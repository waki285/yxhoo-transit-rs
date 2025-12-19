use chrono::{DateTime, FixedOffset, Timelike};
use serde::{self, Deserialize, Deserializer, Serializer};

const FMT_MIN_TZ: &str = "%Y-%m-%dT%H:%M%:z";
const FMT_SEC_TZ: &str = "%Y-%m-%dT%H:%M:%S%:z";

#[allow(dead_code)]
pub fn serialize<S>(dt: &DateTime<FixedOffset>, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let dt = dt.with_second(0).unwrap().with_nanosecond(0).unwrap();
    ser.serialize_str(&dt.format(FMT_MIN_TZ).to_string())
}

pub fn deserialize<'de, D>(de: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    parse_str(&s).map_err(serde::de::Error::custom)
}

fn parse_str(s: &str) -> Result<DateTime<FixedOffset>, String> {
    let mut s = s.to_string();

    // Normalize 'Z' to +00:00
    if s.ends_with('Z') {
        s.pop();
        s.push_str("+00:00");
    }

    // 1) with timezone (datetime)
    if let Ok(dt) = DateTime::parse_from_str(&s, FMT_MIN_TZ)
        .or_else(|_| DateTime::parse_from_str(&s, FMT_SEC_TZ))
    {
        let dt = dt.with_second(0).unwrap().with_nanosecond(0).unwrap();
        return Ok(dt);
    }

    // 2) date-only with timezone: YYYY-MM-DD(+HH:MM)
    //    e.g. 2025-12-18+09:00, 2025-12-18+00:00
    if s.len() > 10 {
        let (date_part, rest) = s.split_at(10);
        if rest.starts_with('+') || rest.starts_with('-') {
            let injected = format!("{date_part}T00:00{rest}");
            let dt = DateTime::parse_from_str(&injected, FMT_MIN_TZ).map_err(|e| e.to_string())?;
            let dt = dt.with_second(0).unwrap().with_nanosecond(0).unwrap();
            return Ok(dt);
        }
    }

    Err("timezone is required".to_string())
}

pub mod option {
    use super::*;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(dt: &Option<DateTime<FixedOffset>>, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match dt {
            Some(v) => super::serialize(v, ser),
            None => ser.serialize_none(),
        }
    }

    #[allow(dead_code)]
    pub fn deserialize<'de, D>(de: D) -> Result<Option<DateTime<FixedOffset>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(de)?;
        match opt {
            Some(s) => super::parse_str(&s)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

#[cfg(feature = "schemars")]
pub fn schema(_gen: &mut schemars::generate::SchemaGenerator) -> schemars::Schema {
    // YYYY-MM-DD
    // YYYY-MM-DDTHH:mm[:ss]
    // with required Z / ±HH:mm
    let pattern = r"^\d{4}-\d{2}-\d{2}(T\d{2}:\d{2}(:\d{2})?)?((Z)|([+-]\d{2}:\d{2}))$";

    let s: schemars::Schema = schemars::json_schema!({
        "type": "string",
        "pattern": pattern,
        "description": "Accepted: YYYY-MM-DD[THH:mm[[:]ss]][Z|±HH:mm]. Seconds are truncated to minute. Timezone is required. If time is omitted, it is treated as 00:00. If dateType parameter is ArrivalTime or DepartureTime, YMDHm are required. If it is FirstTrain or LastTrain, YMD are required. If it is NotSpecified, it is not required.",
        "examples": [
            "2025-12-18+09:00",
            "2025-12-18T09:30+09:00",
            "2025-12-18T09:30Z"
        ]
    });

    s
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct Wrap {
        #[serde(with = "super")]
        date: DateTime<FixedOffset>,
    }

    #[test]
    fn test_serialize_deserialize() {
        let dt = FixedOffset::east_opt(9 * 3600)
            .unwrap()
            .with_ymd_and_hms(2025, 12, 18, 9, 30, 45)
            .unwrap();
        let wrapped = Wrap { date: dt };
        let serialized = serde_json::to_string(&wrapped).unwrap();
        assert_eq!(serialized, r#"{"date":"2025-12-18T09:30+09:00"}"#);

        let deserialized: Wrap = serde_json::from_str(&serialized).unwrap();
        assert_eq!(
            deserialized.date,
            dt.with_second(0).unwrap().with_nanosecond(0).unwrap()
        );
    }
    #[test]
    fn test_deserialize_various_formats() {
        let cases = vec![
            ("2025-12-18+09:00", "2025-12-18T00:00+09:00"),
            ("2025-12-18T09:30+09:00", "2025-12-18T09:30+09:00"),
            ("2025-12-18T09:30Z", "2025-12-18T09:30+00:00"),
        ];
        for (input, expected) in cases {
            let dt: Wrap = serde_json::from_str(&format!(r#"{{"date":"{}"}}"#, input)).unwrap();
            let expected_dt: Wrap =
                serde_json::from_str(&format!(r#"{{"date":"{}"}}"#, expected)).unwrap();

            assert_eq!(dt.date, expected_dt.date, "Failed on input: {}", input);
            assert_eq!(
                serde_json::to_string(&dt).unwrap(),
                format!(r#"{{"date":"{}"}}"#, expected),
                "Failed on input: {}",
                input
            );
        }

        let fail_cases = vec![
            "2025/12/18",
            "2025-12-18T09-30",
            "2025-12-18 09:30",
            "2025-12-18T09:30:61",
            "2025-12-18",
            "2025-12-18T09:30",
            "2025-12-18T09:30:45",
            "2025-12-19T10:35",
            "invalid-string",
        ];
        for input in fail_cases {
            let result: Result<Wrap, _> =
                serde_json::from_str(&format!(r#"{{"date":"{}"}}"#, input));
            assert!(result.is_err(), "Expected failure on input: {}", input);
        }
    }
}
