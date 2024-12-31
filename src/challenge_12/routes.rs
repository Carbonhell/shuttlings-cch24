use crate::challenge_12::structs::{GameState, Player, TileType};
use crate::AppState;
use axum::extract::{Path, State};
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

pub(crate) async fn place(State(state): State<Arc<RwLock<AppState>>>, Path((team, column)): Path<(String, usize)>) -> impl IntoResponse {
    tracing::info!("Placing entry in grid:\n{}\n{}", team, column);
    let mut locked_state = state.write().unwrap();
    let player = match Player::try_from(team.as_str()) {
        Ok(player) => player,
        Err(_) => {
            tracing::info!("Error: Invalid player");
            return StatusCode::BAD_REQUEST.into_response();
        }
    };
    if !(1..=4).contains(&column) {
        tracing::info!("Error: column not in 1-4");
        return StatusCode::BAD_REQUEST.into_response();
    }
    match locked_state.board.check_winner() {
        GameState::Win(_) | GameState::NoWin => {
            tracing::info!("Error: game finished");
            (StatusCode::SERVICE_UNAVAILABLE, locked_state.board.to_string()).into_response()
        }
        GameState::Pending => {
            tracing::info!("Game pending");
            let place_result = locked_state.board.place(TileType::from(&player), column - 1);
            tracing::info!("Result of placement: {:?}", place_result);
            match place_result {
                Ok(_) => {
                    let board = locked_state.board.to_string();
                    tracing::info!("Returning board:\n{}", board);
                    board.into_response()
                }
                Err(_) => {
                    tracing::info!("Error: column full");
                    (StatusCode::SERVICE_UNAVAILABLE, locked_state.board.to_string()).into_response()
                }
            }
        }
    }
}