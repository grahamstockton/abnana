use sqlx::{Pool, Sqlite, SqlitePool};

use crate::constants::DATABASE_URL;

pub async fn db() -> Result<Pool<Sqlite>, sqlx::Error> {
    // Create database connection pool
    let pool = SqlitePool::connect(DATABASE_URL).await?;

    // Run migrations to create database
    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
