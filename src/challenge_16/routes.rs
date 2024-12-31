use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use headers::{Cookie, HeaderMapExt};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    value: String,
}

pub(crate) async fn wrap(Json(payload): Json<Value>) -> impl IntoResponse {
    tracing::info!("Wrap called with payload {:?}", payload);
    let token = encode(&Header::default(), &Claims { sub: "Santa".to_string(), company: "Santa".to_string(), value: serde_json::to_string(&payload).unwrap() }, &EncodingKey::from_secret("secret".as_ref())).unwrap();
    tracing::info!("Token: {:?}", token);
    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, format!("gift={}", token).parse().unwrap());
    headers
}

pub(crate) async fn unwrap(header_map: HeaderMap) -> impl IntoResponse {
    tracing::info!("Unwrap called with payload {:?}", header_map);
    let cookie = match header_map.typed_get::<Cookie>() {
        Some(c) => c,
        None => return StatusCode::BAD_REQUEST.into_response()
    };
    let gift = match cookie.get("gift") {
        Some(c) => c,
        None => return StatusCode::BAD_REQUEST.into_response()
    };
    let mut validation = Validation::default();
    validation.required_spec_claims.remove("exp");
    validation.validate_exp = false;
    let gift = decode::<Claims>(&gift, &DecodingKey::from_secret("secret".as_ref()), &validation).unwrap();
    tracing::info!("Gift: {:?}", gift.claims.value);
    Json(serde_json::from_str::<Value>(&gift.claims.value).unwrap()).into_response()
}
