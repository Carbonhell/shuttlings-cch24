use axum::http::StatusCode;
use axum::response::IntoResponse;
use cargo_manifest::{Manifest, MaybeInherited};
use std::str::FromStr;
use toml::Value;

pub(crate) async fn manifest(body: String) -> impl IntoResponse {
    tracing::info!("Manifest raw input: {:#?}", body);
    let parsed_body = match Manifest::from_str(&body) {
        Ok(m) => m,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid manifest").into_response()
    };
    tracing::info!("Parsed manifest: {:#?}", parsed_body);
    let package = match parsed_body.package {
        Some(p) => p,
        None => {
            tracing::info!("Missing package entry.");
            return StatusCode::NO_CONTENT.into_response();
        }
    };
    match package.keywords {
        Some(MaybeInherited::Local(keywords)) => {
            if !keywords.iter().any(|el| el == "Christmas 2024") {
                return (StatusCode::BAD_REQUEST, "Magic keyword not provided").into_response();
            }
        }
        _ => return (StatusCode::BAD_REQUEST, "Magic keyword not provided").into_response()
    }

    let metadata = match package.metadata {
        Some(m) => m,
        _ => {
            tracing::info!("Missing package.metadata entry.");
            return StatusCode::NO_CONTENT.into_response();
        }
    };
    let orders = match metadata.get("orders") {
        Some(Value::Array(orders)) => orders,
        _ => {
            tracing::info!("Missing package.metadata.orders entry.");
            return StatusCode::NO_CONTENT.into_response();
        }
    };
    let mut result = Vec::new();
    for x in orders {
        if let Value::Table(order) = x {
            let item = match order.get("item") {
                Some(Value::String(s)) => s,
                _ => {
                    tracing::warn!("Invalid order: invalid item.");
                    continue;
                }
            };
            let quantity = match order.get("quantity") {
                Some(Value::Integer(i)) => *i,
                _ => {
                    tracing::warn!("Invalid order: invalid quantity.");
                    continue;
                }
            };
            result.push(format!("{}: {}", item, quantity));
        }
    }

    tracing::info!("Result: {:#?}", result);
    if result.is_empty() {
        return StatusCode::NO_CONTENT.into_response();
    }
    (StatusCode::OK, result.join("\n")).into_response()
}