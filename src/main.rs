#![allow(unused, dead_code)]
use glob::glob;
use std::{
    collections::{HashMap, VecDeque},
    fs::{self, File},
    io::{Seek, Write},
    os,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use color_eyre::Result as R;
use uuid::Uuid;

#[fncmd::fncmd]
/// Repeatedly runs a script with different sets of files to figure out which one is causing it to fail.
pub fn main(
    /// Command to be run to determine behavior
    command: String,
    /// Shell to use instead of `sh` or `cmd`
    #[opt(long, short)]
    shell: Option<String>,
    /// List of files or globs to work with.
    files: Vec<String>,
) -> R<()> {
    color_eyre::install();
    eprintln!(
        "Warning! Halfwit is unfinished software. You should backup all relevant files yourself."
    );
    let mut paths: Vec<PathBuf> = Vec::new();
    for path in files {
        if let Ok(paths_to_add) = glob(&path) {
            for path in paths_to_add {
                if let Ok(path) = path {
                    if path.is_file() {
                        paths.push(path);
                    }
                }
            }
        }
    }
    println!("paths: {:#?}", &paths);
}
