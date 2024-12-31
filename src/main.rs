use axum::routing::{delete, post, put};
use axum::{routing::get, Router};
use leaky_bucket::RateLimiter;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

#[path = "challenge_-1/mod.rs"]
mod challenge_neg1;
mod challenge_2;
mod challenge_5;
mod challenge_9;
mod challenge_12;
mod challenge_16;
mod challenge_19;
mod challenge_23;

use crate::challenge_12::routes::{board, place, reset_board};
use crate::challenge_12::structs::Grid;
use crate::challenge_16::routes::{unwrap, wrap};
use crate::challenge_19::routes::{add_quote, delete_quote, get_quote, reset_quotes, update_quote};
use crate::challenge_2::routes::{ipv4_router_decrypt, ipv6_router, ipv6_router_decrypt};
use crate::challenge_23::routes::{get_ornament, get_present, star};
use crate::challenge_5::routes::manifest;
use crate::challenge_9::routes::{milk, refill};
use challenge_2::routes::ipv4_router;
use challenge_neg1::routes::{hello_world, seek};

#[derive(Debug)]
struct AppState {
    rate_limiter: RateLimiter,
    board: Grid,
    pool: PgPool,
}


impl AppState {
    fn new(pool: PgPool) -> Self {
        Self {
            rate_limiter: RateLimiter::builder()
                .max(5)
                .initial(5)
                .interval(Duration::from_millis(1000))
                .build(),
            board: Default::default(),
            pool: pool,
        }
    }
    fn reset_bucket(&mut self) {
        self.rate_limiter = RateLimiter::builder()
            .max(5)
            .initial(5)
            .interval(Duration::from_millis(1000))
            .build()
    }

    fn reset_board(&mut self) {
        self.board = Grid::default();
    }
}

type SharedState = Arc<RwLock<AppState>>;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let shared_state = SharedState::new(RwLock::new(AppState::new(pool)));
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
        .route("/16/wrap", post(wrap))
        .route("/16/unwrap", get(unwrap))
        .route("/19/reset", post(reset_quotes))
        .route("/19/cite/:id", get(get_quote))
        .route("/19/remove/:id", delete(delete_quote))
        .route("/19/undo/:id", put(update_quote))
        .route("/19/draft", post(add_quote))
        .route("/23/star", get(star))
        .route("/23/present/:color", get(get_present))
        .route("/23/ornament/:state/:n", get(get_ornament))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(shared_state);

    Ok(router.into())
}
