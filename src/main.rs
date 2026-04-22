use aho_corasick::AhoCorasick;
use anyhow::{Ok, Result};
use std::{
    cmp::min,
    fs::{self},
    path::{Path, PathBuf},
    sync::{
        Arc,
        mpsc::{self},
    },
    thread,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Search query
    query: String,

    /// Maximum depth to reach
    #[arg(short, long)]
    max_depth: Option<usize>,

    /// Exclusions
    #[arg(short, long)]
    exclusion: Option<Vec<String>>,

    /// Root Dir
    #[arg(short, long)]
    root_dir: Option<PathBuf>,
}

struct FoundEntry {
    path: PathBuf,
}

struct FlatBedConfig {
    remaining_depth: usize,
    exclusion: Vec<String>,
}

fn main() -> Result<()> {
    let Args {
        query,
        max_depth,
        exclusion,
        root_dir,
    } = Args::parse();
    let query_arc = Arc::new(query.clone());
    let (sender, receiver) = mpsc::channel::<FoundEntry>();
    let max_threads_size = 9;
    let default_exclusion = vec![String::from(".git"), String::from("node_modules")];
    let mut exclusion = exclusion.unwrap_or_default();
    exclusion.extend(default_exclusion);
    let candidates = flatbed_dir(
        &root_dir.unwrap_or(PathBuf::from(".")),
        FlatBedConfig {
            remaining_depth: max_depth.unwrap_or(10),
            exclusion,
        },
    )?;
    let threads_size = min(max_threads_size, candidates.len() / 3);
    // dbg!(candidates.len());
    let chunk_size = (candidates.len() as f64 / threads_size as f64).ceil() as usize;
    let mut handles = Vec::new();

    for i in 0..threads_size {
        let sender = sender.clone();
        let slice_begin = min(chunk_size * i, candidates.len());
        let slice_end = min(slice_begin + chunk_size, candidates.len());
        // dbg!(slice_begin, slice_end);
        let chunk_slice = &candidates[slice_begin..slice_end];
        let chunk_vec = Vec::from(chunk_slice);
        let q = Arc::clone(&query_arc);
        let ac = AhoCorasick::new([&*q])?;
        handles.push(thread::spawn(move || -> Result<()> {
            for file in chunk_vec {
                let std::result::Result::Ok(text) = fs::read_to_string(&file) else {
                    continue;
                };
                let res = ac.find(&text);
                if res.is_some() {
                    sender.send(FoundEntry { path: file })?;
                }
            }
            Ok(())
        }));
    }

    drop(sender);

    for h in handles {
        h.join().unwrap()?;
    }

    for e in receiver.iter() {
        println!("{}", e.path.display());
    }

    Ok(())
}

fn flatbed_dir(path: &Path, config: FlatBedConfig) -> Result<Vec<PathBuf>> {
    let FlatBedConfig {
        remaining_depth,
        exclusion,
    } = config;
    let dir = fs::read_dir(path)?;
    let mut result: Vec<PathBuf> = Vec::new();
    for entry in dir.into_iter() {
        let entry = entry?;
        if remaining_depth == 0 {
            return Ok(Vec::new());
        }
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if exclusion.iter().any(|e| e == name_str.as_ref()) {
            continue;
        }
        if entry.file_type()?.is_dir() {
            result.extend(flatbed_dir(
                &entry.path(),
                FlatBedConfig {
                    remaining_depth: remaining_depth - 1,
                    exclusion: exclusion.clone(),
                },
            )?);
        } else {
            result.push(entry.path());
        }
    }
    Ok(result)
}
