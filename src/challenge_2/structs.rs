use std::net::Ipv4Addr;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct IpRouterQuery{
    pub from: Ipv4Addr,
    pub key: Ipv4Addr
}