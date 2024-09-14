use chrono::NaiveDateTime;
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

pub async fn insert_or_update_key(
    pool: sqlx::PgPool,
    ukey: String,
    exp_date: NaiveDateTime,
) -> Result<i32, sqlx::Error> {
    // Insert the key into the database and return the inserted id
    let result = sqlx::query!(
        "INSERT INTO keys (ukey, expdate) VALUES ($1, $2)
         ON CONFLICT (ukey) DO UPDATE SET expdate = EXCLUDED.expdate
         RETURNING id",
        ukey,
        exp_date
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(row) => Ok(row.id),
        Err(err) => Err(err),
    }
}
