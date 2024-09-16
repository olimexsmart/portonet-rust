use chrono::{Duration, Utc};
use serde::Serialize;

pub async fn check_master_password(
    pool: &sqlx::PgPool,
    mp_to_check: String,
) -> Result<bool, sqlx::Error> {
    let res = sqlx::query!("SELECT mp FROM system").fetch_one(pool).await;

    match res {
        Ok(row) => {
            match row.mp {
                Some(mp) => Ok(mp == mp_to_check), // Compare if not NULL
                None => Ok(false),                 // If `mp` is NULL, return false
            }
        }
        Err(sqlx::Error::RowNotFound) => {
            insert_default_row(pool).await?;
            // Return false since the table was uninitialized and no password matched
            Ok(false)
        }
        Err(err) => Err(err),
    }
}

// Counters
#[derive(Serialize)]
pub struct SystemCounter {
    n_openings: i32,
    n_errors: i32,
}

pub async fn get_system_counters(pool: &sqlx::PgPool) -> Result<SystemCounter, sqlx::Error> {
    let res = sqlx::query!("SELECT nopenings, nerrors FROM system")
        .fetch_one(pool)
        .await;

    match res {
        Ok(row) => Ok(SystemCounter {
            n_openings: row.nopenings,
            n_errors: row.nerrors,
        }),
        Err(sqlx::Error::RowNotFound) => {
            insert_default_row(pool).await?;
            // Return zero-zero since row has been init now
            Ok(SystemCounter {
                n_openings: 0,
                n_errors: 0,
            })
        }
        Err(err) => Err(err),
    }
}

pub async fn handle_attempt_ok(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE system SET nopenings = nopenings + 1, nattempts = 0")
        .execute(pool)
        .await?;

    Ok(())
}

// Password brute force protection
// Handle wrong master password and access keys attempts
// Lock system for 15 minutes if more than 10 wrong attemps in a row
pub async fn handle_attempt_failed(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    // Increment over error counter
    sqlx::query!("UPDATE system SET nerrors = nerrors + 1")
        .execute(pool)
        .await?;
    // Increment attempt counter
    sqlx::query!("UPDATE system SET nattempts = nattempts + 1")
        .execute(pool)
        .await?;
    // Get current number of attempts
    let res = sqlx::query!("SELECT nattempts FROM system")
        .fetch_one(pool)
        .await;
    match res {
        Ok(res) => {
            let n_attempts = res.nattempts;
            if n_attempts > 10 {
                sqlx::query!(
                    "UPDATE system SET lockeduntil = $1",
                    (Utc::now() + Duration::minutes(15)).naive_utc()
                )
                .execute(pool)
                .await?;
            }
            Ok(())
        }
        Err(sqlx::Error::RowNotFound) => {
            insert_default_row(pool).await?;
            // Return zero-zero since row has been init now
            Ok(())
        }
        Err(err) => Err(err),
    }
}

pub async fn is_system_locked(pool: &sqlx::PgPool) -> Result<bool, sqlx::Error> {
    let res = sqlx::query!("SELECT lockeduntil FROM system")
        .fetch_one(pool)
        .await;

    match res {
        Ok(res) => {
            match res.lockeduntil {
                // TODO return remaining time to unlocked state
                Some(lu) => Ok(lu > Utc::now().naive_local()), // Compare if not NULL
                None => Ok(false),                             // If `mp` is NULL, return false
            }
        }
        Err(sqlx::Error::RowNotFound) => {
            insert_default_row(pool).await?;
            // Return zero-zero since row has been init now
            Ok(false)
        }
        Err(err) => Err(err),
    }
}

/*
 * PRIVATE
 */
async fn insert_default_row(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO system DEFAULT VALUES")
        .execute(pool)
        .await?;

    Ok(())
}
