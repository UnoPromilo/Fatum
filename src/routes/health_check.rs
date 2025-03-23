use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::Serialize;
use sqlx::PgPool;

pub async fn health_check_handler(
    Extension(db): Extension<PgPool>,
) -> (StatusCode, Json<HealthCheckResponse>) {
    let result = run_health_checks(&db).await;
    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(HealthCheckResponse {
                status: "OK".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(HealthCheckResponse {
                status: "OK".to_string(),
            }),
        ),
    }
}

async fn run_health_checks(db: &PgPool) -> Result<(), String> {
    check_db(db).await?;
    Ok(())
}

async fn check_db(db: &PgPool) -> Result<(), String> {
    let simplest_query_result = sqlx::query!("SELECT 1 as ID").fetch_one(db).await;
    match simplest_query_result {
        Ok(_) => Ok(()),
        Err(_) => Err("DATABASE_ERROR".to_string()),
    }
}

#[derive(Serialize)]
pub struct HealthCheckResponse {
    status: String,
}
