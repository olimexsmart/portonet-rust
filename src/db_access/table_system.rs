use chrono::{Duration, NaiveDateTime, Utc};
use serde::Serialize;
use std::io;

type SystemState = (i64, i64, i64, Option<NaiveDateTime>, Option<NaiveDateTime>);

async fn latest_state(pool: &sqlx::SqlitePool) -> Result<SystemState, sqlx::Error> {
    sqlx::query_as::<_, SystemState>(
        "SELECT nopenings, nerrors, nattempts, lastattempt, lockeduntil
         FROM system ORDER BY id DESC LIMIT 1",
    )
    .fetch_one(pool)
    .await
}

fn configured_master_password() -> Result<String, sqlx::Error> {
    std::env::var("MASTER_PASSWORD").map_err(|_| {
        sqlx::Error::Configuration(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "MASTER_PASSWORD must be set",
        )))
    })
}

pub async fn check_master_password(
    pool: &sqlx::SqlitePool,
    mp_to_check: String,
) -> Result<bool, sqlx::Error> {
    let configured_mp = configured_master_password()?;
    if configured_mp == mp_to_check {
        // Reset system locked with correct master password
        handle_attempt_ok(pool).await?;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[derive(Serialize)]
pub struct SystemCounter {
    n_openings: i32,
    n_errors: i32,
}

pub async fn get_system_counters(pool: &sqlx::SqlitePool) -> Result<SystemCounter, sqlx::Error> {
    let state = latest_state(pool).await?;

    Ok(SystemCounter {
        n_openings: state.0 as i32,
        n_errors: state.1 as i32,
    })
}

pub async fn handle_attempt_ok(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    let (nopenings, nerrors, _, _, _) = latest_state(pool).await?;
    sqlx::query(
        "INSERT INTO system (nopenings, nerrors, nattempts, lastattempt, lockeduntil)
         VALUES (?, ?, 0, NULL, NULL)",
    )
    .bind(nopenings + 1)
    .bind(nerrors)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn handle_attempt_failed(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    let (nopenings, nerrors, nattempts, lastattempt, previous_lockeduntil) =
        latest_state(pool).await?;
    let new_attempts = nattempts + 1;
    let locked_until = if new_attempts > 5 {
        Some((Utc::now() + Duration::minutes(15)).naive_utc())
    } else {
        previous_lockeduntil
    };

    sqlx::query(
        "INSERT INTO system (nopenings, nerrors, nattempts, lastattempt, lockeduntil)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(nopenings)
    .bind(nerrors + 1)
    .bind(new_attempts)
    .bind(lastattempt)
    .bind(locked_until)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn is_system_locked(pool: &sqlx::SqlitePool) -> Result<bool, sqlx::Error> {
    let lockeduntil = latest_state(pool).await?.4;

    Ok(lockeduntil.is_some_and(|value| value > Utc::now().naive_utc()))
}

pub(crate) async fn insert_default_row(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO system (nopenings, nerrors, nattempts)
         SELECT 0, 0, 0
         WHERE NOT EXISTS (SELECT 1 FROM system)",
    )
    .execute(pool)
    .await?;
    Ok(())
}
