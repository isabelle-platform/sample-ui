use core::cmp::Ordering::*;
use isabelle_dm::data_model::item::Item;
use std::cmp::Ordering;

pub fn time_cmp(a: &Item, b: &Item) -> Ordering {
    let mut a_day_str = a.safe_str("day_of_the_week", "");
    if a_day_str == "unset" {
        a_day_str = "".to_string();
    }
    let mut b_day_str = b.safe_str("day_of_the_week", "");
    if b_day_str == "unset" {
        b_day_str = "".to_string();
    }
    let a_day_of_week = a_day_str != "";
    let b_day_of_week = b_day_str != "";
    if a_day_of_week && !b_day_of_week {
        return Less;
    } else if !a_day_of_week && b_day_of_week {
        return Greater;
    } else if a_day_of_week && b_day_of_week {
        let days = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"];
        let a_day = days.iter().position(|&r| r == a_day_str).unwrap() as u64;
        let b_day = days.iter().position(|&r| r == b_day_str).unwrap() as u64;
        return (a_day * 86400 + (a.safe_u64("time", 0) % 86400))
            .cmp(&(b_day * 86400 + (b.safe_u64("time", 0) % 86400)));
    } else {
        return a.safe_u64("time", 0).cmp(&b.safe_u64("time", 0));
    }
}

pub fn name_cmp(a: &Item, b: &Item) -> Ordering {
    return a.safe_str("name", "").cmp(&b.safe_str("name", ""));
}

pub fn stime_name_cmp(a: &Item, b: &Item) -> Ordering {
    let res = a.safe_u64("time", 0).cmp(&b.safe_u64("time", 0));
    if res == Equal {
        return name_cmp(a, b);
    }
    return res;
}
