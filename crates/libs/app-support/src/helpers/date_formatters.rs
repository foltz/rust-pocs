use chrono::{DateTime, Utc};

pub fn date_with_seconds(date: DateTime<Utc>) -> String {
    date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

pub fn now_with_seconds() -> String {
    Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}