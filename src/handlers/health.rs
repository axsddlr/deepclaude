use axum::http::StatusCode;

/// Health check endpoint handler
/// 
/// Returns a 200 OK status to indicate the service is running
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
