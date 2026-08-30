use chrono::{NaiveDateTime, Utc};
use serde::Serialize;
use sqlx::Row;

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
    let rows = sqlx::query(
        "SELECT id, ukey, expdate, lastused, nused, revoked
         FROM keys ORDER BY id",
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| UKey {
            id: row.get::<i64, _>("id") as i32,
            ukey: row.get("ukey"),
            exp_date: row.get("expdate"),
            last_used: row.get("lastused"),
            n_used: row.get::<i64, _>("nused") as i32,
            revoked: row.get::<i64, _>("revoked") != 0,
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
    let result = sqlx::query("SELECT expdate, revoked FROM keys WHERE ukey = ?")
        .bind(key_to_check)
        .fetch_optional(pool)
        .await?;

    match result {
        Some(row) if row.get::<i64, _>("revoked") != 0 => Ok(KeyCheckResult::Revoked),
        Some(row) if row.get::<NaiveDateTime, _>("expdate") < Utc::now().naive_utc() => {
            Ok(KeyCheckResult::Expired)
        }
        Some(_) => Ok(KeyCheckResult::Valid),
        None => Ok(KeyCheckResult::Invalid),
    }
}

pub async fn insert_or_update_key(
    pool: &sqlx::SqlitePool,
    ukey: String,
    exp_date: NaiveDateTime,
) -> Result<i32, sqlx::Error> {
    println!("{}", exp_date);

    let mut transaction = pool.begin().await?;
    sqlx::query(
        "INSERT INTO keys (ukey, expdate) VALUES (?, ?)
         ON CONFLICT (ukey) DO UPDATE SET expdate = excluded.expdate, revoked = 0",
    )
    .bind(&ukey)
    .bind(exp_date)
    .execute(&mut *transaction)
    .await?;

    let row = sqlx::query("SELECT id FROM keys WHERE ukey = ?")
        .bind(&ukey)
        .fetch_one(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(row.get::<i64, _>("id") as i32)
}

pub async fn update_key_last_used(
    pool: &sqlx::SqlitePool,
    key_last_used: String,
) -> Result<(), sqlx::Error> {
    let last_used = Utc::now().naive_utc();
    sqlx::query("UPDATE keys SET lastused = ?, nused = nused + 1 WHERE ukey = ?")
        .bind(last_used)
        .bind(key_last_used)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_revoke_key(
    pool: &sqlx::SqlitePool,
    key_to_revoke: String,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE keys SET revoked = 1 WHERE ukey = ?")
        .bind(key_to_revoke)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_revoke_all_keys(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE keys SET revoked = 1")
        .execute(pool)
        .await?;
    Ok(())
}
