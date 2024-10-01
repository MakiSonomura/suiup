use std::path::PathBuf;

use super::Process;
use crate::Result;
use serde::Serialize;
use std::fs;

#[derive(Debug, Serialize)]
pub(crate) struct Config<'a> {
    pub suiup_dir: PathBuf,
    pub toolkits_dir: PathBuf,
    pub update_hash_dir: PathBuf,
    pub download_dir: PathBuf,
    pub current_dir: PathBuf,
    pub bin: PathBuf,

    #[serde(skip)]
    pub process: &'a Process<'a>,
}

fn create_dirs_if_not_exists(paths: &[&PathBuf]) -> Result<()> {
    for path in paths {
        if fs::metadata(path).is_err() {
            fs::create_dir_all(path)?;
        }
    }
    Ok(())
}

impl<'a> Config<'a> {
    pub(crate) fn from_env(process: &'a Process) -> Result<Self> {
        let current_dir = process.current_dir()?;

        let suiup_dir = process.suiup_home()?;

        let toolkits_dir = suiup_dir.join("toolkits");
        let update_hash_dir = suiup_dir.join("update-hashes");
        let download_dir = suiup_dir.join("downloads");
        let bin = suiup_dir.join("bin");
        create_dirs_if_not_exists(&[&suiup_dir, &toolkits_dir, &download_dir])?;
        Ok(Self {
            suiup_dir,
            toolkits_dir,
            update_hash_dir,
            download_dir,
            current_dir,
            bin,
            process,
        })
    }
}
