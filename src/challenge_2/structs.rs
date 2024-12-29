use serde::Deserialize;
use std::net::Ipv4Addr;

#[derive(Deserialize)]
pub(crate) struct IpRouterQuery {
    pub from: Ipv4Addr,
    pub key: Ipv4Addr,
}

#[derive(Deserialize)]
pub(crate) struct IpRouterDecryptQuery {
    pub from: Ipv4Addr,
    pub to: Ipv4Addr,
}