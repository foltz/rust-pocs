use chrono::{DateTime, SecondsFormat, Utc};

// println!("a {}", sun_bod.to_string());
// println!("b {}", sun_bod.to_rfc2822());
// println!("c {}", sun_bod.to_rfc3339());
// println!("d {}", sun_bod.to_rfc3339_opts(SecondsFormat::AutoSi, true));
// println!("e {}", sun_bod.to_rfc3339_opts(SecondsFormat::Micros, true));
// println!("f {}", sun_bod.to_rfc3339_opts(SecondsFormat::Millis, true));
// println!("g {}", sun_bod.to_rfc3339_opts(SecondsFormat::Nanos, true));
// - output:
// a 2023-08-20 21:03:00 UTC
// b Sun, 20 Aug 2023 21:03:00 +0000
// c 2023-08-20T21:03:00+00:00
// d 2023-08-20T21:03:00Z
// e 2023-08-20T21:03:00.000000Z
// f 2023-08-20T21:03:00.000Z
// g 2023-08-20T21:03:00.000000000Z

pub fn oanda_format(time: DateTime<Utc>) -> String {
    time.to_rfc3339_opts(SecondsFormat::Millis, true)
}

pub fn oanda_date_format(date: &DateTime<Utc>) -> String {
    date.to_rfc3339_opts(SecondsFormat::Millis, true)
}
