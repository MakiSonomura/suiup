use std::path::PathBuf;

use sui_assets_info::prelude::{Asset, SuiAssetDesc};

use crate::common::{download_to, extract, set_symlink, write_setting};
use crate::{process::config::Config, LatestOpt, Process, Result, UpdateOpt};

async fn extract_and_set(asset: &Asset, filepath: &PathBuf, config: &Config<'_>) -> Result<()> {
    let toolkit_dir = &config.toolkits_dir;
    let binary_dir = &config.bin;

    let desc = SuiAssetDesc::from_quints(asset.name())?;
    let toolkit_dir = toolkit_dir.join(desc.desc());

    println!("{} ---> {}", filepath.display(), toolkit_dir.display());
    extract(filepath, &toolkit_dir).await?;
    set_symlink(&toolkit_dir, binary_dir).await?;
    write_setting(&desc, &binary_dir).await?;
    Ok(())
}

pub(crate) async fn run(process: &Process<'_>, opt: UpdateOpt) -> Result<()> {
    let config = Config::from_env(process)?;
    let backend = opt.backend;
    println!("Downloading: {}, Backend: {}", &opt.toolkit_desc, backend);
    let asset_desc = SuiAssetDesc::from_twins(&opt.toolkit_desc)?;
    let asset = backend.fetch_specific(&asset_desc).await?;
    println!("Asset file: {}", asset.name());
    let file = download_to(&config.download_dir, &asset).await?;
    extract_and_set(&asset, &file, &config).await?;
    Ok(())
}

pub(crate) async fn run_latest(process: &Process<'_>, opt: LatestOpt) -> Result<()> {
    let config = Config::from_env(process)?;
    let network = opt.network;
    let backend = opt.backend;
    println!("Network: {}, Backend: {}", network, backend);
    let asset = backend.fetch_latest(Some(opt.network)).await?;
    println!("Asset file: {}", asset.name());
    let file = download_to(&config.download_dir, &asset).await?;
    extract_and_set(&asset, &file, &config).await?;
    Ok(())
}
