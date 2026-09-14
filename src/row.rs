//! One item as one row, the SQL Server part: the INSERT that stores it and
//! asks for the id through the OUTPUT clause, and the dialect the shared
//! row code is given — bracketed identifiers, and the bytes as the `0x…`
//! binary literal T-SQL itself writes, which a `varbinary` column stores
//! as the bytes and answers in the same form; a text column that kept the
//! literal verbatim answers it too, and either comes back as the bytes.
//! The columns, the SELECT and the row read back are the capability's
//! `archive::row` (ADR-0044).

use archive::row::Dialect;
use archive::{ArchiveItem, metadata};
use mssql::{binary, quote_identifier, quote_literal};

/// What SQL Server does its own way, handed to the shared row code.
pub const DIALECT: Dialect = Dialect {
    quote_identifier,
    bytes_expression: "bytes",
    column_bytes: binary::column_bytes,
};

/// The statement that stores `item` in `table` at `archived_at`, answering
/// with the new row's `id` through the OUTPUT clause.
#[must_use]
pub fn insert_sql(table: &str, item: &ArchiveItem, archived_at: &str) -> String {
    format!(
        "INSERT INTO {} (data_type, identifier, bytes, metadata, archived_at) \
         OUTPUT INSERTED.id VALUES ({}, {}, {}, {}, {})",
        DIALECT.table_name(table),
        quote_literal(&item.data_type),
        quote_literal(&item.identifier),
        binary::hex_literal(&item.bytes),
        quote_literal(&metadata::encode(&item.metadata)),
        quote_literal(archived_at)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item() -> ArchiveItem {
        ArchiveItem {
            data_type: "json".to_string(),
            identifier: "it's #1".to_string(),
            bytes: vec![0x7b, 0xff],
            metadata: vec![("source".to_string(), "playground".to_string())],
        }
    }

    #[test]
    fn the_insert_names_the_five_columns_and_asks_for_the_id() {
        let sql = insert_sql("audit.archive", &item(), "2026-09-09T12:00:00Z");
        assert!(sql.starts_with(
            "INSERT INTO [audit].[archive] \
             (data_type, identifier, bytes, metadata, archived_at) OUTPUT INSERTED.id \
             VALUES (N'json', N'it''s #1', 0x7bff, "
        ));
        assert!(sql.ends_with("N'2026-09-09T12:00:00Z')"), "{sql}");
        assert_eq!(
            DIALECT.select_sql("Archive", 41),
            "SELECT data_type, identifier, bytes, metadata FROM [Archive] WHERE id = 41"
        );
    }

    #[test]
    fn a_row_in_either_bytes_form_is_the_item_again() {
        let original = item();
        let hex = vec![
            Some("json".to_string()),
            Some("it's #1".to_string()),
            Some("0x7bff".to_string()),
            Some(metadata::encode(&original.metadata)),
        ];
        assert_eq!(DIALECT.item_from_row(&hex, "here").expect("row"), original);
        let text = vec![
            Some("json".to_string()),
            Some("it's #1".to_string()),
            Some("plain".to_string()),
            Some(String::new()),
        ];
        let restored = DIALECT.item_from_row(&text, "here").expect("row");
        assert_eq!(restored.bytes, b"plain");
        assert!(restored.metadata.is_empty());
    }
}
