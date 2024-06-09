use crate::util::dt_conv::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;

pub fn get_simple_date_ts(event: InputEvent) -> u64 {
    let target = event.target();

    if target.is_none() {
        return 0;
    }

    let binding = target.unwrap();
    let hie = binding.dyn_ref::<HtmlInputElement>();
    if hie.is_none() {
        return 0;
    }

    let s: String = hie.unwrap().clone().value().into();
    let ndt = date2ts(s, "00:00".to_string());
    return ndt;
}

pub fn get_simple_date_ts_str(event: InputEvent) -> String {
    let target = event.target();

    if target.is_none() {
        return "".to_string();
    }

    let binding = target.unwrap();
    let hie = binding.dyn_ref::<HtmlInputElement>();
    if hie.is_none() {
        return "".to_string();
    }

    let s: String = hie.unwrap().clone().value().into();
    return s;
}

pub fn get_simple_time_ts(event: InputEvent) -> u64 {
    let target = event.target();

    if target.is_none() {
        return 0;
    }

    let binding = target.unwrap();
    let hie = binding.dyn_ref::<HtmlInputElement>();
    if hie.is_none() {
        return 0;
    }

    let s: String = hie.unwrap().clone().value().into();
    let ndt = date2ts("1970-01-01".to_string(), s);
    return ndt;
}

pub fn get_input(event: InputEvent) -> String {
    let target = event.target();

    if target.is_none() {
        return "".to_string();
    }

    let binding = target.unwrap();
    let hie = binding.dyn_ref::<HtmlInputElement>();
    if hie.is_none() {
        return "".to_string();
    }

    let s: String = hie.unwrap().clone().value().into();
    return s;
}

pub fn get_input_u64(event: InputEvent) -> u64 {
    let target = event.target();

    if target.is_none() {
        return 0;
    }

    let binding = target.unwrap();
    let hie = binding.dyn_ref::<HtmlInputElement>();
    if hie.is_none() {
        return 0;
    }

    let s: String = hie.unwrap().clone().value().into();
    return s.parse::<u64>().unwrap_or(0);
}

pub fn get_select_input(event: InputEvent) -> String {
    let target = event.target();

    if target.is_none() {
        return "".to_string();
    }

    let binding = target.unwrap();
    let hie = binding.dyn_ref::<HtmlSelectElement>();
    if hie.is_none() {
        return "".to_string();
    }

    let s: String = hie.unwrap().clone().value().into();
    return s;
}

pub fn get_select_input_id(event: InputEvent) -> u64 {
    let target = event.target();

    if target.is_none() {
        return u64::MAX;
    }

    let binding = target.unwrap();
    let hie = binding.dyn_ref::<HtmlSelectElement>();
    if hie.is_none() {
        return u64::MAX;
    }

    let s: String = hie.unwrap().clone().value().into();
    return s.parse::<u64>().unwrap_or(u64::MAX);
}

pub fn get_text_content(event: InputEvent) -> String {
    let target = event.target();

    if target.is_none() {
        return "bad1".to_string();
    }

    let binding = target.unwrap();
    let hie = binding.dyn_ref::<HtmlTextAreaElement>();
    if hie.is_none() {
        return "bad2".to_string();
    }

    let s: String = hie.unwrap().clone().value().into();
    return s;
}

pub fn get_checked(event: InputEvent) -> bool {
    let target = event.target();

    if target.is_none() {
        return false;
    }

    let binding = target.unwrap();
    let hie = binding.dyn_ref::<HtmlInputElement>();
    if hie.is_none() {
        return false;
    }

    let b: bool = hie.unwrap().clone().checked().into();
    return b;
}
