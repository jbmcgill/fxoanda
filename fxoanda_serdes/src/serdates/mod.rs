use serde::{Deserialize, Deserializer, Serializer};
use chrono::prelude::*;
pub fn serialize<S>(value: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(ref v) = *value{
      return serializer.collect_str(&v.to_rfc3339())
    }
    serializer.serialize_none()
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        if s == "0" {
            return Ok(None);
        }else{
            let d = DateTime::from_utc(
                            DateTime::<FixedOffset>::parse_from_rfc3339(&s)
                            .unwrap()
                            .naive_utc(),
                        Utc);
            return Ok(Some(d));
            //return Ok(Some(Utc::now()));
        }
    }
	Ok(None)
}

