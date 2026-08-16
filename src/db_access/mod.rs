pub mod table_keys;
pub mod table_system;
pub mod table_logs;

pub async fn initialize_database(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS keys (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ukey TEXT NOT NULL UNIQUE,
            expdate DATETIME NOT NULL,
            lastused DATETIME,
            nused INTEGER NOT NULL DEFAULT 0,
            revoked INTEGER NOT NULL DEFAULT 0
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            apiname TEXT NOT NULL,
            daterequest DATETIME NOT NULL,
            params TEXT
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS system (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            datetime DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            nopenings INTEGER NOT NULL DEFAULT 0,
            nerrors INTEGER NOT NULL DEFAULT 0,
            nattempts INTEGER NOT NULL DEFAULT 0,
            lastattempt DATETIME,
            lockeduntil DATETIME
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS logs_daterequest_idx ON logs (daterequest DESC)")
        .execute(pool)
        .await?;

    Ok(())
}
