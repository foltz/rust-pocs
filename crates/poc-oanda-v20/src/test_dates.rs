use chrono::{Datelike, DateTime, Timelike, TimeZone, Utc};
use chrono_tz::America::New_York;

pub fn get_sun_bod() -> DateTime<Utc> {

    New_York.with_ymd_and_hms(
        2023, 08, 20,
        17, 03, 0
    ).unwrap().with_timezone(&Utc)
}

pub fn get_mon_eod() -> DateTime<Utc> {

    New_York.with_ymd_and_hms(
        2023, 08, 21,
        16, 58, 0
    ).unwrap().with_timezone(&Utc)
}

pub fn get_valid_book_time() -> DateTime<Utc> {

    New_York.with_ymd_and_hms(
        2023, 08, 21,
        16, 00, 0
    ).unwrap().with_timezone(&Utc)
}

pub fn get_utc_now() -> DateTime<Utc> {
    Utc::now()
}

#[test]
fn can_calc_now() {
    let utc_now = Utc::now();
    let ny_now = utc_now.with_timezone(&New_York);
    let ny_date = New_York.with_ymd_and_hms(
        ny_now.year(), ny_now.month(), ny_now.day(),
        ny_now.hour(), ny_now.minute(), ny_now.second()
        //17, 03, 0
    ).unwrap();

    println!("{}-{}-{} {}:{}:{}",
             ny_now.year(), ny_now.month(), ny_now.day(),
             ny_now.hour(), ny_now.minute(), ny_now.second()
    );
    assert_eq!(ny_now.to_rfc2822(), ny_date.to_rfc2822());
}