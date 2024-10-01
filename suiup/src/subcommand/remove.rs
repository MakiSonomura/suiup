use crate::{
    common::{list_all_toolkits, setting::Setting},
    process::config::Config,
    Error, Process, RemoveOpt, Result,
};
use std::fs;
use sui_assets_info::prelude::SuiAssetDesc;

pub(crate) fn remove_toolkit(process: &Process<'_>, opt: RemoveOpt) -> Result<()> {
    let config = Config::from_env(process)?;
    let binary_dir = &config.bin;

    if SuiAssetDesc::from_twins(&opt.desc).is_ok() {
        let installed_toolkits = list_all_toolkits(&config)?;
        if installed_toolkits
            .into_iter()
            .any(|t| t.desc() == opt.desc)
        {
            let desc_dir = config.toolkits_dir.join(&opt.desc);
            let setting_file = binary_dir.join("setting.json");
            if let Ok(setting) = fs::read_to_string(&setting_file) {
                let setting: Setting = serde_json::from_str(&setting)?;
                dbg!(&setting);
                if opt.desc == setting.current() {
                    println!("Removing a using toolkit: {}", &opt.desc);
                    if binary_dir.is_symlink() {
                        fs::remove_dir_all(binary_dir)?;
                    }
                }
            }
            fs::remove_dir_all(desc_dir)?;
            Ok(())
        } else {
            Err(Error::Custom(
                format!("{} is not installed", opt.desc).into(),
            ))
        }
    } else {
        Err(Error::Custom(
            format!("Invalid input desc: {}", opt.desc).into(),
        ))
    }
}
