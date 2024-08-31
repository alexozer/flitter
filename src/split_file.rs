use serde::{de, Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SplitFile {
    title: String,
    category: String,
    attempts: u32,
    completed: u32,
    split_names: Vec<String>,
    golds: Vec<Gold>,
    personal_best: PersonalBest,
}

#[derive(Serialize, Deserialize, Debug)]
struct Gold {
    #[serde(with = "duration_format")]
    duration: Duration,
}

#[derive(Serialize, Deserialize, Debug)]
struct PersonalBest {
    attempt: u32,
    splits: Vec<Split>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Split {
    #[serde(with = "duration_format")]
    time: Duration,
}

mod duration_format {
    use core::fmt;
    use std::str::FromStr;
    use std::{sync::LazyLock, time::Duration};

    use regex::{Match, Regex};
    use serde::{de, Deserializer, Serializer};

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let day_sec = 60 * 60 * 24;
        let hour_sec = 60 * 60;
        let minute_sec = 60;
        let duration_secs = duration.as_secs();
        let days = duration_secs / day_sec;
        let hours = (duration_secs % day_sec) / hour_sec;
        let minutes = (duration_secs % hour_sec) / minute_sec;
        let seconds = duration_secs % minute_sec;
        let milliseconds = duration.subsec_millis();
        serializer.serialize_str(&format!(
            "{}:{:02}:{:02}:{:02}.{:03}",
            days, hours, minutes, seconds, milliseconds
        ))
    }

    static DURATION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^(?:(?:(?:(\d+):)?(\d+):)?(\d+):)?(\d+)(?:\.(\d{1,3}))?$").unwrap()
    });

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
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
                let days = match_or_zero(&caps.get(1))?;
                let hours = match_or_zero(&caps.get(2))?;
                let minutes = match_or_zero(&caps.get(3))?;
                let seconds = match_or_zero(&caps.get(4))?;
                let milliseconds = match_or_zero(&caps.get(5))?;

                let secs = seconds + (minutes * 60) + (hours * 60 * 60) + (days * 60 * 60 * 24);
                let dur = Duration::from_secs(secs) + Duration::from_millis(milliseconds);
                Ok(dur)
            }
        }

        deserializer.deserialize_str(DurationVisitor)
    }

    fn match_or_zero<E>(mat: &Option<Match<'_>>) -> Result<u64, E>
    where
        E: de::Error,
    {
        let num = match mat {
            Some(d) => u64::from_str(d.as_str()).map_err(E::custom)?,
            None => 0,
        };
        Ok(num)
    }
}

pub fn read_split_file(path: &Path) -> anyhow::Result<SplitFile> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let split_set: SplitFile = serde_json::from_reader(reader)?;
    Ok(split_set)
}

pub fn write_split_set(split_set: &SplitFile, path: &Path) -> anyhow::Result<()> {
    let file = std::fs::File::create(path)?;
    serde_json::to_writer_pretty(file, split_set)?;
    Ok(())
}
