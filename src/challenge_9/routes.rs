use crate::AppState;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const LITER_TO_GALLON: f32 = 0.264172;
const LITRE_TO_UK_PINT: f32 = 1.7597539864;

// https://stackoverflow.com/questions/69834142/how-to-only-allow-one-field-or-the-other-with-serde
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
enum UnitConversion {
    Liters {
        liters: f32,
    },
    Gallons {
        gallons: f32,
    },
    Litres {
        litres: f32,
    },
    Pints {
        pints: f32,
    },
}

pub(crate) async fn milk(State(state): State<Arc<AppState>>, header_map: HeaderMap, body: String) -> impl IntoResponse {
    if state.rate_limiter.try_acquire(1) {
        if is_content_type_json(&header_map) {
            tracing::info!("Handling unit conversion request with body {:#?}", body);
            let unit_conversion_request = match serde_json::from_str::<UnitConversion>(&body) {
                Ok(body) => body,
                Err(_) => {
                    tracing::info!("Invalid JSON");
                    return StatusCode::BAD_REQUEST.into_response();
                }
            };
            match unit_conversion_request {
                UnitConversion::Liters { liters } => Json(UnitConversion::Gallons { gallons: liters * LITER_TO_GALLON }).into_response(),
                UnitConversion::Gallons { gallons } => Json(UnitConversion::Liters { liters: gallons / LITER_TO_GALLON }).into_response(),
                UnitConversion::Litres { litres } => { Json(UnitConversion::Pints { pints: litres * LITRE_TO_UK_PINT }).into_response() }
                UnitConversion::Pints { pints } => { Json(UnitConversion::Litres { litres: pints / LITRE_TO_UK_PINT }).into_response() }
            }
        } else {
            tracing::info!("Handling milk withdraw request");
            (StatusCode::OK, "Milk withdrawn\n").into_response()
        }
    } else {
        tracing::info!("Too many requests");
        (StatusCode::TOO_MANY_REQUESTS, "No milk available\n").into_response()
    }
}

fn is_content_type_json(headers: &HeaderMap) -> bool {
    match headers.get("Content-Type") {
        Some(content_type) => {
            match content_type.to_str() {
                Ok(content_type) => content_type.starts_with("application/json"),
                Err(_) => false,
            }
        }
        None => false,
    }
}