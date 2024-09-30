use crate::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};
use sui_assets_info::prelude::SuiAssetDesc;

use std::io::{BufWriter, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting<'a> {
    current_toolkit: &'a str,
}

impl<'a> Setting<'a> {
    pub fn new(current_toolkit: &'a str) -> Self {
        Self { current_toolkit }
    }
}

pub async fn write_setting<P: AsRef<Path>>(desc: &SuiAssetDesc, path: P) -> Result<()> {
    if !path.as_ref().exists() {
        fs::create_dir_all(&path)?;
    }
    let setting_file = PathBuf::from(path.as_ref()).join("setting.json");
    let file = File::create(setting_file)?;

    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &Setting::new(&desc.desc()))?;
    writer.flush()?;
    Ok(())
}

pub async fn set_symlink<P: AsRef<Path>>(src: P, dst: P) -> Result<()> {
    if dst.as_ref().exists() {
        std::fs::remove_dir_all(&dst)?;
    }
    #[cfg(unix)]
    std::os::unix::fs::symlink(src, dst)?;
    #[cfg(windows)]
    junction::create(src, dst)?;

    Ok(())
}