#![allow(unused, dead_code)]
use std::{
    collections::HashMap,
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
    for path in manifest.values() {
        fs::remove_file(path)?;
    }
    println!("files destoyed :(");
    // assert_eq!(run.status()?.success(), false);
    restore_manifest(&manifest)?;
    println!("files restoyed :)");

    Ok(())
}

fn restore_manifest(manifest: &HashMap<Uuid, &Path>) -> R<()> {
    for (uuid, path) in manifest {
        fs::copy(format!(".halfwit/{}", uuid), path)?;
    }
    Ok(())
}
