use axum::body::Body;
use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use prometheus_client::encoding::text::encode;
use sqlx::{Pool, Sqlite};
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{Router, extract::State, routing::get};
use prometheus_client::metrics::family::Family;
use prometheus_client::registry::Registry;
use serde::Deserialize;

use crate::methods::get_treatment_with_override;
use crate::metrics::metrics::Metrics;

#[derive(Deserialize)]
struct GetTreatmentRequest {
    experiment_id: i64,
    user_id: String,
}

#[derive(Debug)]
struct AppState {
    db: Pool<Sqlite>,
    registry: Registry,
    metrics: Arc<Mutex<Metrics>>,
}

#[tokio::main]
async fn main() {
    // initialize database
    let db = db::db().await.unwrap();

    // initialize metrics
    let metrics = Arc::new(Mutex::new(Metrics {
        triggers: Family::default(),
    }));

    // Create registry and register metrics
    let mut registry = Registry::default();
    {
        let metrics = metrics.lock().await;
        registry.register(
            "triggers_total",
            "Total number of triggers",
            metrics.triggers.clone(),
        );
    }

    // initialize app state
    let state = AppState {
        db,
        registry,
        metrics: metrics.clone(),
    };

    // convert to Arc<Mutex<>> for shared state
    let state = Arc::new(Mutex::new(state));

    // build our application with axum routes
    let app = Router::new()
        .route("/", get(handle_ping))
        // Get treatment without triggering
        .route(
            "/get_treatment/{experiment_id}/{user_id}",
            get(handle_get_treatment),
        )
        // Get treatment and trigger
        .route(
            "/get_treatment_and_trigger/{experiment_id}/{user_id}",
            get(handle_get_treatment_and_trigger),
        )
        // Metrics endpoint
        .route("/metrics", get(metrics_handler))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/**
 * Handler functions for get treatment, with and without trigger
 */
async fn handle_get_treatment(
    State(state): State<Arc<Mutex<AppState>>>,
    axum::extract::Path(GetTreatmentRequest {
        experiment_id,
        user_id,
    }): axum::extract::Path<GetTreatmentRequest>,
) -> Result<axum::Json<Option<String>>, axum::http::StatusCode> {
    // lock mutex to access state
    let state = state.lock().await;

    // fetch treatment
    let res = get_treatment_with_override(&state.db, experiment_id, &user_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(axum::Json(res))
}

async fn handle_get_treatment_and_trigger(
    State(state): State<Arc<Mutex<AppState>>>,
    axum::extract::Path(GetTreatmentRequest {
        experiment_id,
        user_id,
    }): axum::extract::Path<GetTreatmentRequest>,
) -> Result<axum::Json<Option<String>>, axum::http::StatusCode> {
    // lock mutex to access state
    let state = state.lock().await;

    // fetch treatment
    let res = get_treatment_with_override(&state.db, experiment_id, &user_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Record the trigger in metrics if treatment is assigned
    if res.is_some() {
        let metrics = state.metrics.lock().await;
        metrics.record_trigger(experiment_id, res.as_ref().unwrap());
    }

    Ok(axum::Json(res))
}

/**
 * Metrics handler
 */
async fn metrics_handler(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let state = state.lock().await;
    let mut buffer = String::new();
    encode(&mut buffer, &state.registry).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header(
            CONTENT_TYPE,
            "application/openmetrics-text; version=1.0.0; charset=utf-8",
        )
        .body(Body::from(buffer))
        .unwrap()
}

/**
 * Simple ping handler
 */
async fn handle_ping() -> &'static str {
    "pong"
}

mod constants;
mod dao;
mod db;
mod methods;
mod metrics;
