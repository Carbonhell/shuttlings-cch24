use axum::routing::post;
use axum::{routing::get, Router};
use leaky_bucket::RateLimiter;
use std::sync::{Arc, RwLock};
use std::time::Duration;

#[path = "challenge_-1/mod.rs"]
mod challenge_neg1;
mod challenge_2;
mod challenge_5;
mod challenge_9;
mod challenge_12;

use crate::challenge_12::routes::{board, place, reset_board};
use crate::challenge_12::structs::Grid;
use crate::challenge_2::routes::{ipv4_router_decrypt, ipv6_router, ipv6_router_decrypt};
use crate::challenge_5::routes::manifest;
use crate::challenge_9::routes::{milk, refill};
use challenge_2::routes::ipv4_router;
use challenge_neg1::routes::{hello_world, seek};

#[derive(Debug)]
struct AppState {
    rate_limiter: RateLimiter,
    board: Grid,
}

impl AppState {
    fn reset_bucket(&mut self) {
        self.rate_limiter = AppState::default().rate_limiter
    }

    fn reset_board(&mut self) {
        self.board = AppState::default().board;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            rate_limiter: RateLimiter::builder()
                .max(5)
                .initial(5)
                .interval(Duration::from_millis(1000))
                .build(),
            board: Default::default(),
        }
    }
}

type SharedState = Arc<RwLock<AppState>>;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let shared_state = SharedState::default();
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/seek", get(seek))
        .route("/2/dest", get(ipv4_router))
        .route("/2/key", get(ipv4_router_decrypt))
        .route("/2/v6/dest", get(ipv6_router))
        .route("/2/v6/key", get(ipv6_router_decrypt))
        .route("/5/manifest", post(manifest))
        .route("/9/milk", post(milk))
        .route("/9/refill", post(refill))
        .route("/12/board", get(board))
        .route("/12/reset", post(reset_board))
        .route("/12/place/:team/:column", post(place))
        .with_state(shared_state);

    Ok(router.into())
}
