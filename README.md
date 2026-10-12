# xmip-core-archive-mssql

SQL Server archive target: one item is one row of an archive table. A technology of [xmip-core-archive](https://github.com/IlleNilsson/xmip-core-archive).

The store is the capability's `archive::sql::SqlArchive`; this crate is its
`archive::sql::Database`, `SqlServer` — the SQL the server speaks (T-SQL, `sql::TSQL`,
from `xmip-core-library-sql`) and the connection through
[xmip-core-transport-mssql](https://github.com/IlleNilsson/xmip-core-transport-mssql).
An archive here is `SqlArchive::<SqlServer>::new(server, database, user)`.

## Toolchain

`rust-toolchain.toml` pins the toolchain for the whole estate. Do not change it
here.

## Verification

The included workflow is manual-only and calls the versioned shared workflow at
`IlleNilsson/.github@v1`.
