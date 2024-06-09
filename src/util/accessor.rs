use crate::components::pagination::PageQuery;
use isabelle_dm::data_model::item::*;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

pub fn unset_zero() -> u64 {
    return 0;
}

pub fn unset_max() -> u64 {
    return u64::MAX;
}

pub fn unset_str() -> String {
    return "".to_string();
}

pub fn unset_empty() -> String {
    return "".to_string();
}

pub fn current_page<T: yew::Component>(ctx: &Context<T>) -> u64 {
    let location = ctx.link().location().unwrap();

    location.query::<PageQuery>().map(|it| it.page).unwrap_or(1)
}

pub fn get_parent(map: &HashMap<u64, Item>, se: &Item) -> Option<Item> {
    let id = se.safe_id("parent_id", 0);
    if map.contains_key(&id) {
        return Some(map[&id].clone());
    }
    return None;
}
