use anyhow::{anyhow, Context};
use std::path::PathBuf;

mod duration_format;
mod split_set;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(anyhow!("Usage: {} <path_to_splits_file>", args[0]));
    }

    let path = PathBuf::from(&args[1]);

    let split_set = split_set::read_split_set(&path).context("Failed to read splits file")?;
    println!("{:?}", split_set);
    split_set::write_split_set(&split_set, &path)?;

    Ok(())
}
