use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;

pub(crate) async fn milk(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    if state.rate_limiter.try_acquire(1) {
        (StatusCode::OK, "Milk withdrawn\n")
    } else {
        (StatusCode::TOO_MANY_REQUESTS, "No milk available\n")
    }
}