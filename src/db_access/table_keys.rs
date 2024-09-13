use serde::Serialize;

#[derive(Serialize)]
pub struct UKey {
    pub id: i32,
    pub ukey: String,
    pub exp_date: Option<chrono::NaiveDateTime>,
    pub last_used: Option<chrono::NaiveDateTime>,
    pub n_used: i32,
    pub revoked: bool,
}

// Handler function to fetch data from Postgres
pub async fn get_keys(pool: sqlx::PgPool) -> Result<Vec<UKey>, sqlx::Error> {
    let rows = sqlx::query!("SELECT * FROM keys").fetch_all(&pool).await?;

    let data = rows
        .into_iter()
        .map(|row| UKey {
            id: row.id,
            ukey: row.ukey,
            exp_date: row.expdate,
            last_used: row.lastused,
            n_used: row.nused,
            revoked: row.revoked != 0,
        })
        .collect();

    Ok(data)
}
