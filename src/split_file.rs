use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::utils::{format_duration, Prefix, Sign};

#[derive(Serialize, Deserialize, Debug)]
pub struct SplitFile {
    pub title: String,
    pub category: String,
    pub attempts: u32,
    pub completed: u32,
    pub split_names: Vec<String>,
    pub golds: Vec<Option<Gold>>,
    pub personal_best: PersonalBest,

    #[serde(skip)]
    file_path: PathBuf,
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

    use crate::utils::{format_duration, Prefix, Sign};

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format_duration(
            *duration,
            3,
            Sign::Positive,
            Prefix::NoneOrMinus,
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
                formatter.write_str("a string in the format D:H:MM:SS.mmm")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let caps = DURATION_REGEX
                    .captures(value)
                    .ok_or_else(|| E::custom("invalid duration format"))?;

                let days = get_or_zero(&caps.get(1))?;
                let hours = get_or_zero(&caps.get(2))?;
                let minutes = get_or_zero(&caps.get(3))?;
                let seconds = get_or_zero(&caps.get(4))?;
                let milliseconds = get_or_zero_padded(&caps.get(5))?;

                let secs = seconds + (minutes * 60) + (hours * 60 * 60) + (days * 60 * 60 * 24);
                let dur = Duration::from_secs(secs) + Duration::from_millis(milliseconds);
                Ok(dur)
            }
        }

        deserializer.deserialize_str(DurationVisitor)
    }

    fn get_or_zero<E>(mat: &Option<Match<'_>>) -> Result<u64, E>
    where
        E: de::Error,
    {
        let num = match mat {
            Some(d) => u64::from_str(d.as_str()).map_err(E::custom)?,
            None => 0,
        };
        Ok(num)
    }

    fn get_or_zero_padded<E>(mat: &Option<Match<'_>>) -> Result<u64, E>
    where
        E: de::Error,
    {
        let num = match mat {
            Some(d) => {
                let mut num_str = d.as_str().to_string();
                while num_str.len() < 3 {
                    num_str.push('0');
                }
                u64::from_str(&num_str).unwrap()
            }
            None => 0,
        };
        Ok(num)
    }
}

pub fn read_split_file(path: &Path) -> anyhow::Result<SplitFile> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut split_file: SplitFile = serde_json::from_reader(reader)?;
    split_file.file_path = path.to_owned();

    if split_file.split_names.is_empty() {
        return Err(anyhow!("Split names cannot be empty"));
    }
    if split_file.personal_best.splits.is_empty() {
        return Err(anyhow!("Personal best cannot be empty"));
    }
    if split_file.golds.is_empty() {
        return Err(anyhow!("Golds cannot be empty"));
    }

    if split_file.golds.len() != split_file.split_names.len() {
        return Err(anyhow!(
            "Split name count ({}) does not match gold count ({})",
            split_file.split_names.len(),
            split_file.golds.len()
        ));
    }
    if split_file.personal_best.splits.len() != split_file.split_names.len() {
        return Err(anyhow!(
            "Split name count ({}) does not match personal best split count ({})",
            split_file.split_names.len(),
            split_file.personal_best.splits.len(),
        ));
    }

    if split_file.personal_best.splits.last().unwrap().is_none() {
        return Err(anyhow!("Last split of personal best cannot be null"));
    }

    for i in 0..split_file.split_names.len() {
        let pb_split = split_file.personal_best.splits[i].as_ref();
        let gold = split_file.golds[i].as_ref();
        let prev_pb_split = if i == 0 {
            None
        } else {
            split_file.personal_best.splits[i - 1].as_ref()
        };

        if let (Some(pb_split), Some(prev_pb_split)) = (pb_split, prev_pb_split) {
            if pb_split.time < prev_pb_split.time {
                return Err(anyhow!(
                    "Split {} ({}) is earlier than previous split",
                    i + 1,
                    format_duration(pb_split.time, 3, Sign::Positive, Prefix::NoneOrMinus)
                ));
            }
            if let Some(gold) = gold {
                let pb_dur = pb_split.time - prev_pb_split.time;
                if gold.duration > pb_dur {
                    return Err(anyhow!(
                        "Gold {} ({}) is longer than the PB segment",
                        i + 1,
                        format_duration(gold.duration, 3, Sign::Positive, Prefix::NoneOrMinus)
                    ));
                }
            }
        }
    }

    Ok(split_file)
}

pub fn write_split_file(split_file: &SplitFile) -> anyhow::Result<()> {
    let file = std::fs::File::create(&split_file.file_path)?;
    serde_json::to_writer_pretty(file, split_file)?;
    Ok(())
}
