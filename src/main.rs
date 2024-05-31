#![allow(unused, dead_code)]
use std::{
    collections::{HashMap, VecDeque},
    fs::{self, File},
    io::{Seek, Write},
    path::{Path, PathBuf},
    process::Command,
};

use color_eyre::Result as R;
use uuid::Uuid;

#[fncmd::fncmd]
/// Repeatedly runs a script with different sets of files to figure out which one is causing it to fail.
pub fn main(
    /// Command to be run to determine behavior
    command: String,
    /// Shell to use instead of `sh`
    #[opt(long)]
    shell: Option<String>,
    /// List of files to work with
    files: Vec<PathBuf>,
) -> R<()> {
    color_eyre::install();
    eprintln!(
        "Warning! Halfwit is unfinished software. You should backup all relevant files yourself."
    );
    let _ = fs::remove_dir_all(".halfwit"); // rm -f
    fs::create_dir(".halfwit")?;
    let mut manifest: HashMap<Uuid, &Path> = HashMap::new();

    // copy the files to the manifest
    for path in files.iter() {
        let file_uuid = Uuid::new_v4();
        manifest.insert(file_uuid, path);
        fs::copy(path, format!(".halfwit/{}", file_uuid))?;
    }
    let mut mfile = File::create(".halfwit/MANIFEST")?;
    write!(mfile, "{:#?}", manifest); // todo: use serde here so we can deserialize this too?

    // Verify that script works as intended
    // todo: determine if this is useful
    let mut run = Command::new(shell.unwrap_or("sh".to_owned()));
    run.arg("-c").arg(&command);
    assert_eq!(run.status()?.success(), false); // todo: better errors
    for path in manifest.values() {
        fs::remove_file(path)?;
    }
    println!("files destoyed :(");
    assert_eq!(run.status()?.success(), true); // todo: better errors
    restore_manifest(&manifest);

    // Start the bisection!
    let mut stack: Vec<Vec<Uuid>> = vec![manifest.keys().map(|x| x.to_owned()).collect()];
    let mut bad: Vec<Uuid> = Vec::new();
    while !stack.is_empty() {
        let next = stack.pop().unwrap();
        for (uuid, path) in manifest.iter() {
            if next.contains(uuid) {
                if !path.exists() {
                    fs::copy(format!(".halfwit/{}", uuid), path)?;
                }
            } else {
                if path.exists() {
                    fs::remove_file(path)?;
                }
            }
        }
        // run and handle the results
        match run.status()?.success() {
            true => {
                // do nothing?
            }
            false if next.len() == 1 => {
                println!(
                    "bad element found: {}",
                    manifest.get(&next[0]).unwrap().to_string_lossy()
                );
                bad.push(next[0]);
            }
            false => {
                let (a, b) = next.split_at(next.len() / 2);
                stack.push(a.to_owned());
                stack.push(b.to_owned());
            }
        }
    }

    Ok(())
}

fn restore_manifest(manifest: &HashMap<Uuid, &Path>) -> R<()> {
    for (uuid, path) in manifest {
        fs::copy(format!(".halfwit/{}", uuid), path)?;
    }
    Ok(())
}
