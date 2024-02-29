use chrono::{DateTime, Local, Utc};

pub fn display_local_time(date_time: DateTime<Utc>) -> String {
    let datetime_local: DateTime<Local> = date_time.with_timezone(&Local);
    datetime_local.format("%d/%m/%Y %H:%M").to_string()
}