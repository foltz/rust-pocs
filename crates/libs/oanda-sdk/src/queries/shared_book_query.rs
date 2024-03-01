use chrono::{DateTime, Utc};
use crate::support::helpers::chrono_helpers::oanda_date_format;

#[derive(Default)]
pub struct SharedBookQuery {
    /// Name of the Instrument (required)
    /// i.e. EUR_USD
    pub instrument: String,

    /// The time to fetch books for.
    pub time: Option<DateTime<Utc>>,

    /// The frequency with which to query books
    // - TODO: should this be in the query?
    pub frequency: i32,
}

impl SharedBookQuery {

    pub fn to_querystring(&self) -> String {

        let mut qs = String::from("");

        let add_param = |s: &str, display: &str, mem: &mut String| {
            mem.push_str(&format!("?{}={}", display, s))
        };

        // - we may or may not have these 'optional' attributes:

        if let Some(ref time) = self.time {
            add_param(&oanda_date_format(time), "time", &mut qs)
        }

        qs
    }
}