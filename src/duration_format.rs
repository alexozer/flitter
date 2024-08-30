use serde::{de, Deserializer, Serializer};
use std::str::FromStr;
use std::sync::LazyLock;
use std::{fmt, time::Duration};

pub(crate) fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let secs = duration.as_secs();
    let mins = secs / 60;
    let secs = secs % 60;
    let millis = duration.subsec_millis();
    let s = format!("{}:{:02}.{:03}", mins, secs, millis);
    serializer.serialize_str(&s)
}

static DURATION_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^(\d+):(\d{2})\.(\d{3})$").unwrap());

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    struct DurationVisitor;

    impl<'de> de::Visitor<'de> for DurationVisitor {
        type Value = Duration;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string in the format MM:SS.mmm")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let caps = DURATION_REGEX
                .captures(value)
                .ok_or_else(|| E::custom("invalid duration format"))?;

            let mins = u64::from_str(caps.get(1).unwrap().as_str()).map_err(E::custom)?;
            let secs = u64::from_str(caps.get(2).unwrap().as_str()).map_err(E::custom)?;
            let millis = u64::from_str(caps.get(3).unwrap().as_str()).map_err(E::custom)?;

            Ok(Duration::from_millis(
                mins * 60 * 1000 + secs * 1000 + millis,
            ))
        }
    }

    deserializer.deserialize_str(DurationVisitor)
}
