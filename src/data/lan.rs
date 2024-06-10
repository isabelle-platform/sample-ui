use std::collections::HashMap;
use crate::FetchState;
use isabelle_dm::data_model::item::Item;
use serde::{Serialize, Deserialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct LanPoolRegisteredMac {
    pub mac: String,
    pub ipv4: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct LanPool {
    pub id: u64,
    pub enable: bool,
    pub ipv4_start: String,
    pub ipv4_end: String,
    pub interface: String,
    pub routers: Vec<String>,
    pub client_subnet_mask: String,
    pub lease_time: u64,

    pub registered: Vec<LanPoolRegisteredMac>,

    pub domain: String,

    pub dns: Vec<String>,

    pub broadcast: String,
}
