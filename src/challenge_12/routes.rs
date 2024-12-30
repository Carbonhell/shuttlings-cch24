use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::{Arc, RwLock};

pub(crate) async fn board(State(state): State<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    let locked_state = state.read().unwrap();
    let board = locked_state.board.to_string();
    tracing::info!("Returning board:\n{}", board);
    board
}

pub(crate) async fn reset_board(State(state): State<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    let mut locked_state = state.write().unwrap();
    locked_state.reset_board();
    let board = locked_state.board.to_string();
    tracing::info!("Returning board:\n{}", board);
    board
}

