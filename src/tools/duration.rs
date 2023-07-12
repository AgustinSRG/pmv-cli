// Duration utils

use crate::models::MediaType;

pub fn render_media_duration(media_type: MediaType, d: f64) -> String {
    match media_type {
        crate::models::MediaType::Deleted => {
            return "".to_string();
        }
        crate::models::MediaType::Image => {
            return "N/A".to_string();
        }
        crate::models::MediaType::Video | crate::models::MediaType::Audio => {
            return duration_to_string(d);
        }
    }
}

pub fn duration_to_string(d: f64) -> String {
    let mut rest: i64 = d.floor() as i64;

    let hours: i64 = rest / 3600;

    rest = rest % 3600;

    let minutes: i64 = rest / 60;
    let seconds: i64 = rest % 60;

    let mut m_s = minutes.to_string();

    if m_s.chars().count() < 2 {
        m_s = "0".to_owned() + &m_s;
    }

    let mut s_s = seconds.to_string();

    if s_s.chars().count() < 2 {
        s_s = "0".to_owned() + &s_s;
    }

    let mut h_s = hours.to_string();

    if h_s.chars().count() < 2 {
        h_s = "0".to_owned() + &h_s;
    }

    return format!("{h_s}:{m_s}:{s_s}");
}

pub fn parse_duration(duration_str: &str) -> Result<f64, ()> {
    let parts: Vec<&str> = duration_str.trim().split(":").collect();

    if parts.len() == 3 {
        let hours = parts[0].parse::<f64>();

        if hours.is_err() {
            return Err(());
        }

        let minutes = parts[1].parse::<f64>();

        if minutes.is_err() {
            return Err(());
        }

        let seconds = parts[2].parse::<f64>();

        if seconds.is_err() {
            return Err(());
        }

        return Ok((hours.unwrap_or(0.0) * 60.0 * 60.0)
            + (minutes.unwrap_or(0.0) * 60.0)
            + seconds.unwrap_or(0.0));
    } else if parts.len() == 2 {
        let minutes = parts[0].parse::<f64>();

        if minutes.is_err() {
            return Err(());
        }

        let seconds = parts[1].parse::<f64>();

        if seconds.is_err() {
            return Err(());
        }

        return Ok((minutes.unwrap_or(0.0) * 60.0) + seconds.unwrap_or(0.0));
    } else {
        return Err(());
    }
}
