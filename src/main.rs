use axum::routing::post;
use axum::{routing::get, Router};
use leaky_bucket::RateLimiter;
use std::sync::Arc;
use std::time::Duration;

#[path = "challenge_-1/mod.rs"]
mod challenge_neg1;
mod challenge_2;
mod challenge_5;
mod challenge_9;

use crate::challenge_2::routes::{ipv4_router_decrypt, ipv6_router, ipv6_router_decrypt};
use crate::challenge_5::routes::manifest;
use crate::challenge_9::routes::milk;
use challenge_2::routes::ipv4_router;
use challenge_neg1::routes::{hello_world, seek};

struct AppState {
    rate_limiter: RateLimiter,
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let shared_state = Arc::new(AppState {
        rate_limiter: RateLimiter::builder()
            .max(5)
            .initial(5)
            .interval(Duration::from_millis(1000))
            .build()
    });
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/seek", get(seek))
        .route("/2/dest", get(ipv4_router))
        .route("/2/key", get(ipv4_router_decrypt))
        .route("/2/v6/dest", get(ipv6_router))
        .route("/2/v6/key", get(ipv6_router_decrypt))
        .route("/5/manifest", post(manifest))
        .route("/9/milk", post(milk))
        .with_state(shared_state);

    Ok(router.into())
}
