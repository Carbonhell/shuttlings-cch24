use serde::Deserialize;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Deserialize)]
pub(crate) struct Ipv4RouterQuery {
    pub from: Ipv4Addr,
    pub key: Ipv4Addr,
}

#[derive(Deserialize)]
pub(crate) struct Ipv4RouterDecryptQuery {
    pub from: Ipv4Addr,
    pub to: Ipv4Addr,
}

#[derive(Deserialize)]
pub(crate) struct Ipv6RouterQuery {
    pub from: Ipv6Addr,
    pub key: Ipv6Addr,
}

#[derive(Deserialize)]
pub(crate) struct Ipv6RouterDecryptQuery {
    pub from: Ipv6Addr,
    pub to: Ipv6Addr,
}