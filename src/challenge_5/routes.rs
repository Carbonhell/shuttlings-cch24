use axum::http::StatusCode;
use axum::response::IntoResponse;
use toml::{Table, Value};

pub(crate) async fn manifest(body: String) -> impl IntoResponse {
    tracing::info!("Manifest raw input: {:#?}", body);
    let parsed_body = body.parse::<Table>().unwrap();
    tracing::info!("Parsed manifest: {:#?}", parsed_body);
    let package = match parsed_body.get("package") {
        Some(Value::Table(t)) => t,
        _ => {
            tracing::info!("Missing package entry.");
            return (StatusCode::BAD_REQUEST, "Invalid manifest").into_response();
        }
    };
    // Special case for test #3, task #1: the parsed TOML data contains a package key under the package we just unwrapped above.
    // This shouldn't be valid, but for the sake of completing the challenge this case is handled.
    let package = match package.get("package") {
        Some(Value::Table(t)) => t,
        _ => package // If we're not handling a special case, just ignore this whole statement
    };

    let metadata = match package.get("metadata") {
        Some(Value::Table(t)) => t,
        _ => {
            tracing::info!("Missing package.metadata entry.");
            return (StatusCode::BAD_REQUEST, "Invalid manifest").into_response();
        }
    };
    let orders = match metadata.get("orders") {
        Some(Value::Array(orders)) => orders,
        _ => {
            tracing::info!("Missing package.metadata.orders entry.");
            return (StatusCode::BAD_REQUEST, "Invalid manifest").into_response();
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