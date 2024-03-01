use chrono::{DateTime, Utc};
use crate::support::helpers::chrono_helpers::oanda_date_format;

#[derive(Default)]
pub struct PricingQuery {
    /// Comma separated names of the Instrument (required)
    /// i.e. EUR_USD  or EUR_USD,USD_CAD
    pub instruments: String,

    /// The time to fetch books for.
    pub since: Option<DateTime<Utc>>,

    /// The frequency with which to query books
    // - TODO: should this be in the query?
    pub frequency: i32,

    /// The account_id with which to query books
    pub account_id: String,
}

impl PricingQuery {

    pub fn to_querystring(&self) -> String {

        let mut qs = String::from("");

        let add_param = |s: &str, display: &str, mem: &mut String| {
            mem.push_str(&format!("&{}={}", display, s))
        };

        // - we should always have instruments
        qs.push_str(&format!("?instruments={}", self.instruments));

        // - we may or may not have these 'optional' attributes:

        if let Some(ref since) = self.since {
            add_param(&oanda_date_format(since), "since", &mut qs)
        }

        qs
    }
}