pub async fn init_db_with_tables(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    // system
    sqlx::query(
        r#"
        CREATE TABLE "system" (
            "id" SERIAL PRIMARY KEY,
            "mp" TEXT,
            "nopenings" INTEGER NOT NULL DEFAULT 0,
            "nerrors" INTEGER NOT NULL DEFAULT 0,
            "nattempts" INTEGER NOT NULL DEFAULT 0,
            "lastattempt" TIMESTAMP,
            "lockeduntil" TIMESTAMP
        )
    "#,
    )
    .execute(pool)
    .await?;
    // logs
    sqlx::query(
        r#"
        CREATE TABLE "logs" (
            "id" SERIAL PRIMARY KEY,
            "apiname" TEXT NOT NULL,
            "daterequest" TIMESTAMP NOT NULL,
            "params" TEXT
        )
    "#,
    )
    .execute(pool)
    .await?;
    // keys
    sqlx::query(
        r#"
        CREATE TABLE "keys" (
            "id" SERIAL PRIMARY KEY,
            "ukey" TEXT NOT NULL,
            "expdate" TIMESTAMP NOT NULL,
            "lastused" TIMESTAMP,
            "nused" INTEGER NOT NULL DEFAULT 0,
            "revoked" SMALLINT NOT NULL DEFAULT 0,
            UNIQUE ("ukey")
        )
    "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
