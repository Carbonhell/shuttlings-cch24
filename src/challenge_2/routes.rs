use crate::challenge_2::structs::{IpRouterDecryptQuery, IpRouterQuery};
use axum::extract::Query;
use axum::response::IntoResponse;
use std::net::Ipv4Addr;

/// Implements task 1 for challenge 2.
pub(crate) async fn ip_router(query_params: Query<IpRouterQuery>) -> impl IntoResponse {
    let from_octets = query_params.0.from.octets();
    let key_octets = query_params.0.key.octets();
    let encrypted_octets: [u8; 4] = from_octets
        .into_iter()
        .zip(key_octets)
        .map(|(from_octet, key_octet)| { from_octet.overflowing_add(key_octet).0 })
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap(); // This unwrap is safe due to the type of encrypted_octets depending on the type of from/key octets
    Ipv4Addr::from(encrypted_octets).to_string()
}

pub(crate) async fn ip_router_decrypt(query_params: Query<IpRouterDecryptQuery>) -> impl IntoResponse {
    let from_octets = query_params.0.from.octets();
    let to_octets = query_params.0.to.octets();
    let decrypted_octets: [u8; 4] = from_octets
        .into_iter()
        .zip(to_octets)
        .map(|(from_octet, to_octet)| { to_octet.overflowing_sub(from_octet).0 })
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap(); // This unwrap is safe due to the type of encrypted_octets depending on the type of from/to octets
    Ipv4Addr::from(decrypted_octets).to_string()
}