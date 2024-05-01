// Date utils

use chrono::{DateTime, Utc};

pub fn format_date(timestamp: i64) -> String {
    if timestamp == 0 {
        return "Never".to_string();
    }

    let naive = DateTime::from_timestamp_millis(timestamp);

    match naive {
        Some(n) => {
            let d: DateTime<Utc> = DateTime::from_naive_utc_and_offset(n.naive_utc(), Utc);
            d.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        None => "???".to_string(),
    }
}
