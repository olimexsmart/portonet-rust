use chrono::{NaiveDateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct UKey {
    pub id: i32,
    pub ukey: String,
    pub exp_date: chrono::NaiveDateTime,
    pub last_used: Option<chrono::NaiveDateTime>,
    pub n_used: i32,
    pub revoked: bool,
}

// Handler function to fetch data from Postgres
pub async fn select_keys(pool: &sqlx::PgPool) -> Result<Vec<UKey>, sqlx::Error> {
    let rows = sqlx::query!("SELECT * FROM keys").fetch_all(pool).await?;

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

pub enum KeyCheckResult {
    Valid,
    Expired,
    Revoked,
    Invalid,
}

pub async fn check_key(
    pool: &sqlx::PgPool,
    key_to_check: String,
) -> Result<KeyCheckResult, sqlx::Error> {
    let res = sqlx::query!(
        "SELECT expdate, revoked FROM keys WHERE ukey = $1",
        key_to_check
    )
    .fetch_one(pool)
    .await;

    match res {
        Ok(row) => {
            if row.revoked != 0 {
                Ok(KeyCheckResult::Revoked)
            } else if row.expdate < Utc::now().naive_utc() {
                Ok(KeyCheckResult::Expired)
            } else {
                Ok(KeyCheckResult::Valid)
            }
        }
        Err(sqlx::Error::RowNotFound) => Ok(KeyCheckResult::Invalid),
        Err(err) => Err(err),
    }
}

pub async fn insert_or_update_key(
    pool: &sqlx::PgPool,
    ukey: String,
    exp_date: NaiveDateTime,
) -> Result<i32, sqlx::Error> {
    // Insert the key into the database and return the inserted id
    let result = sqlx::query!(
        "INSERT INTO keys (ukey, expdate) VALUES ($1, $2)
         ON CONFLICT (ukey) DO UPDATE SET expdate = EXCLUDED.expdate, revoked = 0
         RETURNING id",
        ukey,
        exp_date
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(row) => Ok(row.id),
        Err(err) => Err(err),
    }
}

pub async fn update_revoke_key(
    pool: &sqlx::PgPool,
    key_to_revoke: String,
) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE keys SET revoked = 1 WHERE ukey = $1", key_to_revoke)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn update_revoke_all_keys(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE keys SET revoked = 1")
        .execute(pool)
        .await?;

    Ok(())
}
