//use std::fmt::Display;
//use std::str::FromStr;
//use std::num::ParseIntError;

//use serde::{de, Deserialize, Deserializer, Serializer};
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(value: &Option<f32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(ref v) = *value{
      return serializer.collect_str(&v.to_string())
    }
    serializer.serialize_none()
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        return Ok(Some(s.parse::<f32>().map_err(serde::de::Error::custom)?))
    }
		Ok(None)
}

