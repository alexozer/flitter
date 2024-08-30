use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use std::{path::Path, time::Duration};

use crate::duration_format;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SplitSet {
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

pub fn read_split_set(path: &Path) -> anyhow::Result<SplitSet> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let split_set: SplitSet = serde_json::from_reader(reader)?;
    Ok(split_set)
}

pub fn write_split_set(split_set: &SplitSet, path: &Path) -> anyhow::Result<()> {
    let file = std::fs::File::create(path)?;
    serde_json::to_writer_pretty(file, split_set)?;
    Ok(())
}
