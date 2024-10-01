use crate::{common::setting::Setting, process::config::Config, Process, RemoveOpt, Result};
use std::{fs, path::PathBuf};
use sui_assets_info::prelude::SuiAssetDesc;

pub(crate) fn remove_toolkit(process: &Process<'_>, opt: RemoveOpt) -> Result<()> {
    let config = Config::from_env(process)?;
    let binary_dir = &config.bin;

    let setting_file = PathBuf::from(&binary_dir).join("setting.json");
    let setting = fs::read_to_string(setting_file)?;

    let setting: Setting = serde_json::from_str(&setting)?;
    if SuiAssetDesc::from_twins(&opt.desc).is_ok() {
        let desc_dir = config.toolkits_dir.join(&opt.desc);
        fs::remove_dir_all(desc_dir)?;
        if opt.desc == setting.current() {
            println!("Removing a using toolkit: {}", &opt.desc);
            let binary_dir = &config.bin;
            if binary_dir.is_symlink() {
                fs::remove_dir_all(binary_dir)?;
            }
        }
    } else {
        eprintln!("Invalid input desc: {}", opt.desc);
    }

    Ok(())
}
