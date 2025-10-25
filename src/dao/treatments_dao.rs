use anyhow::Result;

/**
 * GETTERS FOR TREATMENTS AND OVERRIDES
 */
pub async fn get_treatment(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    experiment_id: i64,
    user_id: &str,
) -> Result<Option<String>> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT treatment_id FROM treatments WHERE experiment_id = ? AND user_id = ?",
    )
    .bind(experiment_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.0))
}

pub async fn get_override_treatment(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    experiment_id: i64,
    user_id: &str,
) -> Result<Option<String>> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT treatment_id FROM overrides WHERE experiment_id = ? AND user_id = ?",
    )
    .bind(experiment_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.0))
}

/**
 * SETTERS FOR TREATMENTS AND OVERRIDES
 */
#[allow(dead_code)]
pub async fn set_treatment(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    experiment_id: i64,
    user_id: &str,
    treatment_id: &str,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO treatments (experiment_id, user_id, treatment_id) VALUES (?, ?, ?)
         ON CONFLICT(experiment_id, user_id) DO UPDATE SET treatment_id = excluded.treatment_id",
    )
    .bind(experiment_id)
    .bind(user_id)
    .bind(treatment_id)
    .execute(pool)
    .await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn set_override(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    experiment_id: i64,
    user_id: &str,
    treatment_id: &str,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO overrides (experiment_id, user_id, treatment_id) VALUES (?, ?, ?)
         ON CONFLICT(experiment_id, user_id) DO UPDATE SET treatment_id = excluded.treatment_id",
    )
    .bind(experiment_id)
    .bind(user_id)
    .bind(treatment_id)
    .execute(pool)
    .await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn create_experiment(pool: &sqlx::Pool<sqlx::Sqlite>, name: &str) -> Result<i64> {
    let result = sqlx::query("INSERT INTO experiments (name) VALUES (?)")
        .bind(name)
        .execute(pool)
        .await?;

    Ok(result.last_insert_rowid())
}

/**
 * DELETE FUNCTIONS
 */
#[allow(dead_code)]
pub async fn delete_override(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    experiment_id: i64,
    user_id: &str,
) -> Result<()> {
    sqlx::query("DELETE FROM overrides WHERE experiment_id = ? AND user_id = ?")
        .bind(experiment_id)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn delete_experiment(pool: &sqlx::Pool<sqlx::Sqlite>, experiment_id: i64) -> Result<()> {
    sqlx::query("DELETE FROM experiments WHERE experiment_id = ?")
        .bind(experiment_id)
        .execute(pool)
        .await?;

    Ok(())
}
