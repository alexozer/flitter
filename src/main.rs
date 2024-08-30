use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use std::time::Duration;

mod duration_format;

#[derive(Serialize, Deserialize, Debug)]
struct Speedrun {
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

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(anyhow!("Usage: {} <path_to_splits_file>", args[0]));
    }

    let file = std::fs::File::open(&args[1]).context(format!("Failed to open {}", &args[1]))?;
    let reader = std::io::BufReader::new(file);
    let speedrun: Speedrun =
        serde_json::from_reader(reader).context(format!("Failed to load {}", &args[1]))?;

    println!("{:?}", speedrun);
    Ok(())
}
