use std::str::FromStr;
use std::fmt::Display;

use serde::{
    Serializer,
    Deserialize,
    Deserializer,
};

use serde_json;

use super::hit::Hit;

pub fn stred_int_to_float<'a, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
    D: Deserializer<'a>
{
    let int = String::deserialize(deserializer)?;

    int.parse().map_err(serde::de::Error::custom)
}

pub fn tags_mode_serialize<S>(tags_exclusive: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let str_opt = if *tags_exclusive { "OR" } else { "AND" };

    serializer.serialize_str(&str_opt)
}

pub fn ordering_serialize<S>(ascending: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let str_opt = if *ascending { "asc" } else { "desc" };

    serializer.serialize_str(&str_opt)
}

pub fn hits_deserialize<'a, D>(deserializer: D) -> Result<Vec<Hit>, D::Error>
where
    D: Deserializer<'a>,
{
    let hits_str = String::deserialize(deserializer)?;

    serde_json::from_str::<Vec<Hit>>(&hits_str).map_err(serde::de::Error::custom)
}