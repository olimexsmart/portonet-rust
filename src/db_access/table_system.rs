use chrono::{Duration, Utc};
use serde::Serialize;

pub async fn check_master_password(
    pool: &sqlx::SqlitePool,
    mp_to_check: String,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("SELECT mp FROM system")
        .fetch_optional(pool)
        .await?;

    match result {
        Some(row) => Ok(row.mp.as_deref() == Some(mp_to_check.as_str())),
        None => {
            insert_default_row(pool).await?;
            Ok(false)
        }
    }
}

#[derive(Serialize)]
pub struct SystemCounter {
    n_openings: i32,
    n_errors: i32,
}

pub async fn get_system_counters(pool: &sqlx::SqlitePool) -> Result<SystemCounter, sqlx::Error> {
    let result = sqlx::query!("SELECT nopenings, nerrors FROM system")
        .fetch_optional(pool)
        .await?;

    match result {
        Some(row) => Ok(SystemCounter {
            n_openings: row.nopenings as i32,
            n_errors: row.nerrors as i32,
        }),
        None => {
            insert_default_row(pool).await?;
            Ok(SystemCounter {
                n_openings: 0,
                n_errors: 0,
            })
        }
    }
}

pub async fn handle_attempt_ok(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE system SET nopenings = nopenings + 1, nattempts = 0")
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn handle_attempt_failed(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    let locked_until = (Utc::now() + Duration::minutes(15)).naive_utc();
    sqlx::query!(
        "UPDATE system
         SET nerrors = nerrors + 1,
             nattempts = nattempts + 1,
             lockeduntil = CASE
                 WHEN nattempts + 1 > 10 THEN ?
                 ELSE lockeduntil
             END",
        locked_until
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn is_system_locked(pool: &sqlx::SqlitePool) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("SELECT lockeduntil FROM system")
        .fetch_optional(pool)
        .await?;

    match result {
        Some(row) => Ok(row
            .lockeduntil
            .is_some_and(|locked_until| locked_until > Utc::now().naive_utc())),
        None => {
            insert_default_row(pool).await?;
            Ok(false)
        }
    }
}

async fn insert_default_row(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO system DEFAULT VALUES")
        .execute(pool)
        .await?;
    Ok(())
}
