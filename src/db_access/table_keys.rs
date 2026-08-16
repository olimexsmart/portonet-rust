use chrono::{NaiveDateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct UKey {
    pub id: i32,
    pub ukey: String,
    pub exp_date: NaiveDateTime,
    pub last_used: Option<NaiveDateTime>,
    pub n_used: i32,
    pub revoked: bool,
}

pub async fn select_keys(pool: &sqlx::SqlitePool) -> Result<Vec<UKey>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT id as \"id!: i64\", ukey, expdate, lastused, nused, revoked
         FROM keys ORDER BY id"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| UKey {
            id: row.id as i32,
            ukey: row.ukey,
            exp_date: row.expdate,
            last_used: row.lastused,
            n_used: row.nused as i32,
            revoked: row.revoked != 0,
        })
        .collect())
}

pub enum KeyCheckResult {
    Valid,
    Expired,
    Revoked,
    Invalid,
}

pub async fn check_key(
    pool: &sqlx::SqlitePool,
    key_to_check: &str,
) -> Result<KeyCheckResult, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT expdate, revoked FROM keys WHERE ukey = ?",
        key_to_check
    )
    .fetch_optional(pool)
    .await?;

    match result {
        Some(row) if row.revoked != 0 => Ok(KeyCheckResult::Revoked),
        Some(row) if row.expdate < Utc::now().naive_utc() => Ok(KeyCheckResult::Expired),
        Some(_) => Ok(KeyCheckResult::Valid),
        None => Ok(KeyCheckResult::Invalid),
    }
}

pub async fn insert_or_update_key(
    pool: &sqlx::SqlitePool,
    ukey: String,
    exp_date: NaiveDateTime,
) -> Result<i32, sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query!(
        "INSERT INTO keys (ukey, expdate) VALUES (?, ?)
         ON CONFLICT (ukey) DO UPDATE SET expdate = excluded.expdate, revoked = 0",
        ukey,
        exp_date
    )
    .execute(&mut *transaction)
    .await?;

    let row = sqlx::query!("SELECT id as \"id!: i64\" FROM keys WHERE ukey = ?", ukey)
        .fetch_one(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(row.id as i32)
}

pub async fn update_key_last_used(
    pool: &sqlx::SqlitePool,
    key_last_used: String,
) -> Result<(), sqlx::Error> {
    let last_used = Utc::now().naive_utc();
    sqlx::query!(
        "UPDATE keys SET lastused = ? WHERE ukey = ?",
        last_used,
        key_last_used
    )
    .execute(pool)
    .await?;
    Ok(()) 
}

pub async fn update_revoke_key(
    pool: &sqlx::SqlitePool,
    key_to_revoke: String,
) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE keys SET revoked = 1 WHERE ukey = ?", key_to_revoke)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_revoke_all_keys(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE keys SET revoked = 1")
        .execute(pool)
        .await?;
    Ok(())
}
