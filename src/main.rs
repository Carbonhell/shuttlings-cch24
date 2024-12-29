use axum::{routing::get, Router};

#[path = "challenge_-1/mod.rs"]
mod challenge_neg1;
mod challenge_2;

use crate::challenge_2::routes::ip_router_decrypt;
use challenge_2::routes::ip_router;
use challenge_neg1::routes::{hello_world, seek};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/seek", get(seek))
        .route("/2/dest", get(ip_router))
        .route("/2/key", get(ip_router_decrypt));

    Ok(router.into())
}
