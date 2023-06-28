// Date utils

use chrono::{NaiveDateTime, DateTime, Utc};

pub fn format_date(timestamp: i64) -> String {
    if timestamp == 0 {
        return "Never".to_string();
    }

    let naive = NaiveDateTime::from_timestamp_millis(timestamp);

    match naive {
        Some(n) => {
            let d: DateTime<Utc> = DateTime::from_utc(n, Utc);
            return d.format("%Y-%m-%d %H:%M:%S").to_string();
        }
        None => {
            return "???".to_string();
        },
    }
}
