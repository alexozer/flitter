use anyhow::anyhow;
use serde::{de, Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct SplitFile {
    pub title: String,
    pub category: String,
    pub attempts: u32,
    pub completed: u32,
    pub split_names: Vec<String>,
    pub golds: Vec<Option<Gold>>,
    pub personal_best: PersonalBest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gold {
    #[serde(with = "duration_format")]
    pub duration: Duration,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonalBest {
    pub attempt: u32,
    pub splits: Vec<Option<Split>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Split {
    #[serde(with = "duration_format")]
    pub time: Duration,
}

mod duration_format {
    use core::fmt;
    use std::str::FromStr;
    use std::{sync::LazyLock, time::Duration};

    use regex::{Match, Regex};
    use serde::{de, Deserializer, Serializer};

    use crate::utils::format_duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format_duration(*duration, 3))
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
                formatter.write_str("a string in the format D:H:MM:SS.mmm")
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
    if split_set.golds.len() != split_set.split_names.len() {
        return Err(anyhow!("Split name count does not match gold count"));
    }
    if split_set.personal_best.splits.len() != split_set.split_names.len() {
        return Err(anyhow!(
            "Split name count does not match personal best split count"
        ));
    }
    Ok(split_set)
}

pub fn write_split_set(split_set: &SplitFile, path: &Path) -> anyhow::Result<()> {
    let file = std::fs::File::create(path)?;
    serde_json::to_writer_pretty(file, split_set)?;
    Ok(())
}
