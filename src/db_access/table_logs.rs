use chrono::Utc;
use serde::Serialize;

pub async fn insert_log(
    pool: &sqlx::SqlitePool,
    api_name: &str,
    params: Option<String>,
) -> Result<(), sqlx::Error> {
    let request_date = Utc::now().naive_utc();
    sqlx::query!(
        "INSERT INTO logs (apiname, daterequest, params) VALUES (?, ?, ?)",
        api_name,
        request_date,
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

pub async fn select_logs(pool: &sqlx::SqlitePool, limit: i64) -> Result<Vec<ULog>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT id as \"id!: i64\", apiname, daterequest, params
         FROM logs ORDER BY daterequest DESC LIMIT ?",
        limit
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| ULog {
            id: row.id as i32,
            api_name: row.apiname,
            request_date: row.daterequest,
            params: row.params,
        })
        .collect())
}
