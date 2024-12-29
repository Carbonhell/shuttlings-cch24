use crate::challenge_2::structs::{Ipv4RouterDecryptQuery, Ipv4RouterQuery, Ipv6RouterDecryptQuery, Ipv6RouterQuery};
use axum::extract::Query;
use axum::response::IntoResponse;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ops::BitXor;

/// Implements task 1 for challenge 2.
pub(crate) async fn ipv4_router(query_params: Query<Ipv4RouterQuery>) -> impl IntoResponse {
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

/// Implements task 2 for challenge 2.
pub(crate) async fn ipv4_router_decrypt(query_params: Query<Ipv4RouterDecryptQuery>) -> impl IntoResponse {
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

/// Implements task 3 for challenge 2.
pub(crate) async fn ipv6_router(query_params: Query<Ipv6RouterQuery>) -> impl IntoResponse {
    let from_octets = query_params.0.from.octets();
    let key_octets = query_params.0.key.octets();
    let decrypted_octets: [u8; 16] = from_octets
        .into_iter()
        .zip(key_octets)
        .map(|(from_octet, key_octet)| { from_octet.bitxor(key_octet) })
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap(); // This unwrap is safe due to the type of encrypted_octets depending on the type of from/key octets
    Ipv6Addr::from(decrypted_octets).to_string()
}

pub(crate) async fn ipv6_router_decrypt(query_params: Query<Ipv6RouterDecryptQuery>) -> impl IntoResponse {
    let from_octets = query_params.0.from.octets();
    let to_octets = query_params.0.to.octets();
    let decrypted_octets: [u8; 16] = from_octets
        .into_iter()
        .zip(to_octets)
        .map(|(from_octet, to_octet)| { from_octet.bitxor(to_octet) })
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap(); // This unwrap is safe due to the type of encrypted_octets depending on the type of from/key octets
    Ipv6Addr::from(decrypted_octets).to_string()
}