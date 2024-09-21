use chrono::Utc;

pub async fn insert_log(
    pool: &sqlx::PgPool,
    api_name: &str,
    params: Option<String>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO logs (apiname, daterequest, params) VALUES($1, $2, $3)",
        api_name,
        Utc::now().naive_local(),
        params
    )
    .execute(pool)
    .await?;

    Ok(())
}
