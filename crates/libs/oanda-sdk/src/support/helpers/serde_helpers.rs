use std::error::Error;
// use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Deserializer};
// use serde_derive::Deserialize;

// - use this one - since it is less code:
pub fn serde_parse_f32<'de, D>(deserializer: D) -> Result<f32, D::Error> where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    f32::from_str(&s).map_err(serde::de::Error::custom)
}

pub fn serde_parse_decimal<'de, D>(deserializer: D) -> Result<rust_decimal::Decimal, D::Error> where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    rust_decimal::Decimal::from_str(&s).map_err(serde::de::Error::custom)
}

pub fn serde_parse_opt_decimal<'de, D>(deserializer: D) -> Result<Option<rust_decimal::Decimal>, D::Error> where D: Deserializer<'de>
{
    
    todo!();
    // let s = String::deserialize(deserializer)?;
    // if s.is_empty() {
    //     return Ok(None);
    // }
    // let res = rust_decimal::Decimal::from_str(&s);
    // return if res.is_err() {
    //     // return Err(res.map_err(|e| e.to_string()));
    //     let err = res.unwrap_err();
    //     let mapped = res.map_err(serde::de::Error::custom);
    //     // Err(Box::new(err))
    //     Err(mapped)
    // } else {
    //     Ok(Some(res.unwrap()))
    // }
}

// - here's a more flexible example:
// pub fn deserialize_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
//     where
//         D: Deserializer<'de>,
//         T: FromStr + serde::Deserialize<'de>,
//         <T as FromStr>::Err: Display,
// {
//     #[derive(Deserialize)]
//     #[serde(untagged)]
//     enum StringOrTarget<T> {
//         String(String),
//         Target(T),
//     }
//
//     match StringOrTarget::<T>::deserialize(deserializer)? {
//         StringOrTarget::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
//         StringOrTarget::Target(i) => Ok(i),
//     }
// }