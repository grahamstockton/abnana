use crate::dao::treatments_dao::{get_override_treatment, get_treatment};
use anyhow::Result;

pub async fn get_treatment_with_override(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    experiment_id: i64,
    user_id: &str,
) -> Result<Option<String>> {
    // First check for override
    match get_override_treatment(pool, experiment_id, user_id).await {
        Ok(Some(treatment)) => Ok(Some(treatment)),
        _ => get_treatment(pool, experiment_id, user_id).await,
    }
}
