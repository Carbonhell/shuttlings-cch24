use axum::{routing::get, Router};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect};

/// Implements task 1 for challenge -1.
async fn hello_world() -> &'static str {
    "Hello, bird!"
}

/// Implements task 2 for challenge -1.
/// Must pass a custom status code due to the default one being a 303 See Other.
async fn seek() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("Location", "https://www.youtube.com/watch?v=9Gc4QTqslN4".parse().unwrap());
    (StatusCode::FOUND, Redirect::to("https://www.youtube.com/watch?v=9Gc4QTqslN4"))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world)).route("/-1/seek", get(seek));

    Ok(router.into())
}
