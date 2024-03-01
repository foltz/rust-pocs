use chrono::{DateTime, Utc};
use crate::support::helpers::chrono_helpers::oanda_date_format;

#[derive(Default)]
pub struct PricingStreamQuery {

    /// Comma separated names of the Instrument (required)
    /// i.e. EUR_USD  or EUR_USD,USD_CAD
    pub instruments: String,

    /// Whether to include initial state when the stream is opened.
    pub snapshot: Option<bool>,

    /// The account_id to open the stream on.
    pub account_id: String,
}

impl PricingStreamQuery {

    pub fn to_querystring(&self) -> String {

        let mut qs = String::from("");

        let add_param = |s: &str, display: &str, mem: &mut String| {
            mem.push_str(&format!("&{}={}", display, s))
        };
        
        // - we should always have an instrument
        qs.push_str(&format!("?instruments={}", self.instruments));

        // - we may or may not have these 'optional' attributes:

        if let Some(ref snapshot) = self.snapshot {
            add_param(&snapshot.to_string(), "snapshot", &mut qs)
        }

        qs
    }
}