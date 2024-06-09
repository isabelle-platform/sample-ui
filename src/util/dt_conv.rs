use chrono::{DateTime, NaiveDateTime, Utc};

pub fn date2ts(date: String, time: String) -> u64 {
    #![allow(warnings)]
    let ndt = NaiveDateTime::parse_from_str(
        &(date.to_string() + " " + &time.to_string()),
        "%Y-%m-%d %H:%M",
    );
    return ndt.unwrap().timestamp() as u64;
}

pub fn ts2date(ts: u64) -> DateTime<Utc> {
    #![allow(warnings)]
    let mut datetime = ts;

    if datetime == 0 {
        datetime = chrono::Local::now().timestamp() as u64;
    }

    let naive = NaiveDateTime::from_timestamp(datetime as i64, 0);
    DateTime::from_utc(naive, Utc)
}

pub fn ts2datetimestr(ts: u64) -> String {
    #![allow(warnings)]
    let mut datetime = ts;

    if datetime == 0 {
        datetime = chrono::Local::now().timestamp() as u64;
    }

    let naive = NaiveDateTime::from_timestamp(datetime as i64, 0);
    let utc_date_time: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    let newdate = utc_date_time.format("%Y-%m-%d");
    let newtime = utc_date_time.format("%H:%M");
    newdate.to_string() + " " + &newtime.to_string()
}

pub fn ts2datestr(ts: u64) -> String {
    #![allow(warnings)]
    let mut datetime = ts;

    if datetime == 0 {
        datetime = chrono::Local::now().timestamp() as u64;
    }

    let naive = NaiveDateTime::from_timestamp(datetime as i64, 0);
    let utc_date_time: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    let newdate = utc_date_time.format("%Y-%m-%d");
    newdate.to_string()
}

pub fn ts2timestr(ts: u64) -> String {
    #![allow(warnings)]
    let mut datetime = ts;

    if datetime == 0 {
        datetime = chrono::Local::now().timestamp() as u64;
    }

    let naive = NaiveDateTime::from_timestamp(datetime as i64, 0);
    let utc_date_time: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    let newtime = utc_date_time.format("%H:%M");
    newtime.to_string()
}
