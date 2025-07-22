//! DateTime serialization utilities following the standard defined in 
//! [DATETIME_STANDARD.md](../../docs/DATETIME_STANDARD.md)

use chrono::{DateTime, TimeZone, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

/// Serializes a DateTime<Utc> to a string in the standard format
pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.format(FORMAT).to_string();
    serializer.serialize_str(&s)
}

/// Deserializes a string in the standard format to a DateTime<Utc>
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&s, FORMAT)
        .map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_serialize() {
        let dt = Utc.with_ymd_and_hms(2025, 7, 22, 1, 42, 45).unwrap().with_nanosecond(82000000).unwrap();
        let serialized = serialize(&dt, serde_json::value::Serializer).unwrap();
        assert_eq!(serialized, "\"2025-07-22T01:42:45.082Z\"");
    }

    #[test]
    fn test_deserialize() {
        let dt = deserialize::<_, DateTime<Utc>>(serde_json::Value::from("2025-07-22T01:42:45.082Z")).unwrap();
        assert_eq!(dt.timestamp(), 1753155765);
        assert_eq!(dt.timestamp_subsec_millis(), 82);
    }
}