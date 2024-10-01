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
    // TODO: way to restore a manifest automatically on previous failure
    let _ = fs::remove_dir_all(".halfwit"); // rm -f
    fs::create_dir(".halfwit")?;
    let mut manifest: HashMap<Uuid, &Path> = HashMap::new();
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

    // copy the files to the manifest
    for path in paths.iter() {
        let file_uuid = Uuid::new_v4();
        manifest.insert(file_uuid, path);
        fs::copy(path, format!(".halfwit/{}", file_uuid))?;
    }
    let mut mfile = File::create(".halfwit/MANIFEST")?;
    write!(mfile, "{:#?}", manifest); // TODO: use serde here so we can deserialize this too?

    #[cfg(windows)]
    let shell = shell.unwrap_or("powershell.exe".to_owned());
    #[cfg(unix)]
    let shell = shell.unwrap_or("sh".to_owned());
    #[cfg(not(any(windows, unix)))]
    compile_error!(
        "Compiling on something that's neither windows nor unix. \
        Please raise an issue and tell me what you're trying to do here. \
        https://github.com/typecasto/halfwit"
    );
    let mut run = Command::new(shell);
    run.arg("-c").arg(&command);
    run.stdin(Stdio::null());
    run.stdout(Stdio::null());
    run.stderr(Stdio::null());

    // Verify that script works as intended
    // TODO: determine if this is useful
    // assert_eq!(run.status()?.success(), false); // TODO: better errors
    // TODO: genericize
    for path in manifest.values() {
        fs::remove_file(path)?;
    }
    // assert_eq!(run.status()?.success(), true); // TODO: better errors
    restore_manifest(&manifest);

    // Start the bisection!
    let mut stack: Vec<Vec<Uuid>> = vec![manifest.keys().map(|x| x.to_owned()).collect()];
    let mut bad: Vec<Uuid> = Vec::new();
    while !stack.is_empty() {
        let next = stack.pop().unwrap();
        // todo: genericize
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
        println!("Testing batch of {} files.", next.len());
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
    println!("Bad files were:");
    for uuid in bad.iter() {
        println!("{}", manifest.get(uuid).unwrap().to_string_lossy());
    }
    restore_manifest(&manifest);

    Ok(())
}

fn restore_manifest(manifest: &HashMap<Uuid, &Path>) -> R<()> {
    for (uuid, path) in manifest {
        fs::copy(format!(".halfwit/{}", uuid), path)?;
    }
    Ok(())
}
