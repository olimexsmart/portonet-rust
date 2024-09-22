use chrono::Utc;
use serde::Serialize;

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

#[derive(Serialize)]
pub struct ULog {
    pub id: i32,
    pub api_name: String,
    pub request_date: chrono::NaiveDateTime,
    pub params: Option<String>,
}

pub async fn select_logs(pool: &sqlx::PgPool, limit: i64) -> Result<Vec<ULog>, sqlx::Error> {
    let rows = sqlx::query!("SELECT * FROM logs ORDER BY daterequest DESC LIMIT $1", limit)
        .fetch_all(pool)
        .await?;

    let data = rows
        .into_iter()
        .map(|row| ULog {
            id: row.id,
            api_name: row.apiname,
            request_date: row.daterequest,
            params: row.params,
        })
        .collect();

    Ok(data)
}
