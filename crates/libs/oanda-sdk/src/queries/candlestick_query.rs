use chrono::{DateTime, Utc};
use crate::models::candlestick_models::{CandlestickGranularity, CandlestickPrice};
use crate::support::helpers::chrono_helpers::oanda_date_format;

#[derive(Default)]
pub struct CandlestickQuery {

    /// Name of the Instrument (required)
    /// i.e. EUR_USD
    pub instrument: String,

    /// The granularity of the candlesticks to fetch [default=S5]
    /// Note: this library doesn't allow a default granularity - it has to be specified...
    pub granularity: CandlestickGranularity,

    /// The start of the time range to fetch candlesticks for.
    pub from: Option<DateTime<Utc>>,

    /// The end of the time range to fetch candlesticks for.
    pub to: Option<DateTime<Utc>>,

    /// The Price component(s) to get candlestick data for. Can contain any
    /// combination of the characters
    /// “B” (bid candles)
    /// “A” (ask candles)
    /// “M” (midpoint candles)
    pub price: CandlestickPrice,


    /// The number of candlesticks to return in the response. Count should not
    /// be specified if both the start and end parameters are provided, as the
    /// time range combined with the graularity will determine the number of
    /// candlesticks to return. [default=500, maximum=5000]
    pub count: Option<i32>,

    /// A flag that controls whether the candlestick is “smoothed” or not.
    /// A smoothed candlestick uses the previous candle’s close price as its
    /// open price, while an unsmoothed candlestick uses the first price from
    /// its time range as its open price. [default=False]
    pub smooth: Option<bool>,

    /// A flag that controls whether the candlestick that is covered by the from
    /// time should be included in the results. This flag enables clients to use
    /// the timestamp of the last completed candlestick received to poll for
    /// future candlesticks but avoid receiving the previous candlestick
    /// repeatedly. [default=True]
    pub include_first: Option<bool>,

    /// The hour of the day (in the specified timezone) to use for granularities
    /// that have daily alignments. [default=17, minimum=0, maximum=23]
    pub daily_alignment: Option<i32>,

    /// The timezone to use for the dailyAlignment parameter. Candlesticks with
    /// daily alignment will be aligned to the dailyAlignment hour within the
    /// alignmentTimezone. [default=America/New_York]
    pub alignment_timezone: Option<String>,

    /// The day of the week used for granularities that have weekly alignment.
    /// [default=Friday]
    pub weekly_alignment: Option<String>,
}

impl CandlestickQuery {

    // pub fn new(instrument: String, granularity: CandlestickGranularity, price: CandlestickPrice) -> Self {
    //     Self {
    //         instrument,
    //         granularity,
    //         price,
    //         ..Default::default()
    //     }
    // }
// }

// impl QueryFormatter for CandlesQuery {

    pub fn to_querystring(&self) -> String {

        let mut qs = String::from("");

        let add_param = |s: &str, display: &str, mem: &mut String| {
            mem.push_str(&format!("&{}={}", display, s))
        };

        // - we should always have granularity (even tho the API allows a default)
        qs.push_str(&format!("?granularity={}", self.granularity));

        // - we should always have price (even tho the API allows a default)
        qs.push_str(&format!("&price={}", self.price));

        // result.push_str(&format!("&from={}", self.from.to_rfc3339()));

        // - we may or may not have these 'optional' attributes:


        if let Some(ref from) = self.from {
            // add_param(&from.to_rfc3339(), "from", &mut qs)
            add_param(&oanda_date_format(from), "from", &mut qs);
        }

        if let Some(ref to) = self.to {
            add_param(&oanda_date_format(to), "to", &mut qs);
        }
        if let Some(ref count) = self.count {
            add_param(&count.to_string(), "count", &mut qs);
        }
        if let Some(ref smooth) = self.smooth {
            add_param(&smooth.to_string(), "smooth", &mut qs);
        }
        if let Some(ref include_first) = self.include_first {
            add_param(&include_first.to_string(), "includeFirst", &mut qs);
        }
        if let Some(ref daily_alignment) = self.daily_alignment {
            add_param(&daily_alignment.to_string(), "dailyAlignment", &mut qs);
        }
        if let Some(ref alignment_timezone) = self.alignment_timezone {
            add_param(&alignment_timezone.to_string(), "alignmentTimezone", &mut qs)
        }
        if let Some(ref weekly_alignment) = self.weekly_alignment {
            add_param(&weekly_alignment.to_string(), "weeklyAlignment", &mut qs);
        }

        qs
    }
}

