use chrono::{Duration, NaiveDateTime, Utc};
use serde::Serialize;
use std::io;

type SystemState = (
    i64,
    i64,
    i64,
    Option<NaiveDateTime>,
    Option<NaiveDateTime>,
);

async fn latest_state(pool: &sqlx::SqlitePool) -> Result<Option<SystemState>, sqlx::Error> {
    sqlx::query_as::<_, SystemState>(
        "SELECT nopenings, nerrors, nattempts, lastattempt, lockeduntil
         FROM system ORDER BY id DESC LIMIT 1",
    )
    .fetch_optional(pool)
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
    if latest_state(pool).await?.is_none() {
        insert_default_row(pool).await?;
    }
    Ok(configured_mp == mp_to_check)
}

#[derive(Serialize)]
pub struct SystemCounter {
    n_openings: i32,
    n_errors: i32,
}

pub async fn get_system_counters(pool: &sqlx::SqlitePool) -> Result<SystemCounter, sqlx::Error> {
    let state = match latest_state(pool).await? {
        Some(state) => state,
        None => {
            insert_default_row(pool).await?;
            (0, 0, 0, None, None)
        }
    };

    Ok(SystemCounter {
        n_openings: state.0 as i32,
        n_errors: state.1 as i32,
    })
}

pub async fn handle_attempt_ok(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    let (nopenings, nerrors, _, lastattempt, lockeduntil) = latest_or_default(pool).await?;
    sqlx::query(
        "INSERT INTO system (nopenings, nerrors, nattempts, lastattempt, lockeduntil)
         VALUES (?, ?, 0, ?, ?)",
    )
    .bind(nopenings + 1)
    .bind(nerrors)
    .bind(lastattempt)
    .bind(lockeduntil)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn handle_attempt_failed(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    let (nopenings, nerrors, nattempts, lastattempt, previous_lockeduntil) =
        latest_or_default(pool).await?;
    let new_attempts = nattempts + 1;
    let locked_until = if new_attempts > 10 {
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
    let lockeduntil = match latest_state(pool).await? {
        Some(state) => state.4,
        None => {
            insert_default_row(pool).await?;
            None
        }
    };

    Ok(lockeduntil.is_some_and(|value| value > Utc::now().naive_utc()))
}

async fn latest_or_default(pool: &sqlx::SqlitePool) -> Result<SystemState, sqlx::Error> {
    match latest_state(pool).await? {
        Some(state) => Ok(state),
        None => {
            insert_default_row(pool).await?;
            Ok((0, 0, 0, None, None))
        }
    }
}

async fn insert_default_row(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO system DEFAULT VALUES")
        .execute(pool)
        .await?;
    Ok(())
}
