use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use cargo_manifest::{Manifest, MaybeInherited};
use std::str::FromStr;
use toml::Value;

pub(crate) async fn manifest(headers: HeaderMap, body: String) -> impl IntoResponse {
    tracing::info!("Manifest raw input: {:#?}", body);
    let parsed_body = match extract_cargo_toml(&body, headers.get("Content-Type").unwrap().to_str().unwrap()) {
        Ok(manifest) => manifest,
        Err(CargoTomlExtractError::UnsupportedMimeType) => {
            tracing::info!("Invalid manifest: unsupported media type");
            return StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response();
        }
        Err(_) => {
            tracing::info!("Invalid manifest: unknown error occurred");
            return (StatusCode::BAD_REQUEST, "Invalid manifest").into_response();
        }
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
#[derive(Debug, Clone)]
enum CargoTomlExtractError {
    InvalidManifest,
    UnsupportedMimeType,
}
// This could be refactored to be a custom extractor perhaps
fn extract_cargo_toml(body: &str, mime_type: &str) -> Result<Manifest, CargoTomlExtractError> {
    let value = match mime_type {
        "application/yaml" => serde_yaml::from_str::<Value>(body).map_err(|_| CargoTomlExtractError::InvalidManifest)?,
        "application/json" => serde_json::from_str::<Value>(body).map_err(|_| CargoTomlExtractError::InvalidManifest)?,
        "application/toml" => toml::from_str::<Value>(body).map_err(|_| CargoTomlExtractError::InvalidManifest)?,
        _ => Err(CargoTomlExtractError::UnsupportedMimeType)?
    };
    let toml_string = toml::ser::to_string_pretty(&value).unwrap();
    match Manifest::from_str(&toml_string) {
        Ok(m) => Ok(m),
        Err(_) => Err(CargoTomlExtractError::InvalidManifest)
    }
}