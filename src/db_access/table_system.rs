pub async fn check_master_password(
    pool: sqlx::PgPool,
    mp_to_check: String,
) -> Result<bool, sqlx::Error> {
    let res = sqlx::query!("SELECT mp FROM system").fetch_one(&pool).await;

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



// 
async fn insert_default_row(pool: sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO system DEFAULT VALUES")
        .execute(&pool)
        .await?;

    Ok(())
}
