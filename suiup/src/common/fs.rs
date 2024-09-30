use crate::Result;
use crate::{process::config::Config, Error};
use std::fs;
use sui_assets_info::prelude::SuiAssetDesc;

pub fn list_all_toolkits(config: &Config<'_>) -> Result<Vec<SuiAssetDesc>> {
    let toolkit_dir = &config.toolkits_dir;
    let mut res = Vec::new();
    match fs::read_dir(toolkit_dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => {
                        if e.path().is_dir() {
                            if let Some(name) = e.path().file_name() {
                                let name = name.to_string_lossy();
                                if let Ok(desc) = SuiAssetDesc::from_twins(&name) {
                                    res.push(desc);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        return Err(Error::Custom(
                            format!("Error reading entry {}: {}", toolkit_dir.display(), e).into(),
                        ));
                    }
                }
            }
        }
        Err(e) => {
            return Err(Error::Custom(
                format!("Error reading directory {}: {}", toolkit_dir.display(), e).into(),
            ));
        }
    }

    Ok(res)
}
