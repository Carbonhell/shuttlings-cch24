use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Quote {
    id: Uuid,
    author: String,
    quote: String,
    created_at: DateTime<Utc>,
    version: i32,
}

#[axum::debug_handler]
pub(crate) async fn reset_quotes(State(state): State<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    tracing::info!("Resetting quotes");
    let mut locked_state = state.write().await;
    if let Err(e) = sqlx::query("TRUNCATE quotes").execute(&locked_state.pool).await {
        tracing::info!("Error while resetting quotes: {:#?}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    } else {
        tracing::info!("Successfully reset quotes");
        StatusCode::OK.into_response()
    }
}

pub(crate) async fn get_quote(
    Path(id): Path<Uuid>,
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<Json<Quote>, StatusCode> {
    tracing::info!("Getting quote {}", id);
    let pool = &state.read().await.pool;
    let row = sqlx::query("SELECT id, author, quote, created_at, version FROM quotes WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(row) => Ok(Json(Quote {
            id: row.get("id"),
            author: row.get("author"),
            quote: row.get("quote"),
            created_at: row.get("created_at"),
            version: row.get("version"),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub(crate) async fn delete_quote(
    Path(id): Path<Uuid>,
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<Json<Quote>, StatusCode> {
    tracing::info!("Deleting quote {}",id);
    let pool = &state.read().await.pool;
    let row = sqlx::query("DELETE FROM quotes WHERE id = $1 RETURNING id, author, quote, created_at, version")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(row) => Ok(Json(Quote {
            id: row.get("id"),
            author: row.get("author"),
            quote: row.get("quote"),
            created_at: row.get("created_at"),
            version: row.get("version"),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct UpdateQuoteRequest {
    author: String,
    quote: String,
}

pub(crate) async fn update_quote(
    Path(id): Path<Uuid>,
    State(state): State<Arc<RwLock<AppState>>>,
    Json(payload): Json<UpdateQuoteRequest>,
) -> Result<Json<Quote>, StatusCode> {
    tracing::info!("Updating quote {} with payload {:#?}", id, payload);
    let pool = &state.read().await.pool;
    let row = sqlx::query(
        "UPDATE quotes SET author = $1, quote = $2, version = version + 1 WHERE id = $3 RETURNING id, author, quote, created_at, version"
    )
        .bind(&payload.author)
        .bind(&payload.quote)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(row) => Ok(Json(Quote {
            id: row.get("id"),
            author: row.get("author"),
            quote: row.get("quote"),
            created_at: row.get("created_at"),
            version: row.get("version"),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct AddQuoteRequest {
    author: String,
    quote: String,
}

pub(crate) async fn add_quote(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(payload): Json<AddQuoteRequest>,
) -> Result<(StatusCode, Json<Quote>), StatusCode> {
    tracing::info!("Adding quote with payload {:#?}", payload);
    let pool = &state.read().await.pool;
    let id = Uuid::new_v4();
    let row = sqlx::query(
        "INSERT INTO quotes (id, author, quote) VALUES ($1, $2, $3) RETURNING id, author, quote, created_at, version"
    )
        .bind(id)
        .bind(&payload.author)
        .bind(&payload.quote)
        .fetch_one(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(Quote {
            id: row.get("id"),
            author: row.get("author"),
            quote: row.get("quote"),
            created_at: row.get("created_at"),
            version: row.get("version"),
        }),
    ))
}
