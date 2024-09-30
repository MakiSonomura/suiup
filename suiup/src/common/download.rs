use crate::Error;
use crate::Result;
use flate2::read::GzDecoder;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::borrow::Cow;
use std::cmp::min;
use std::io::Write;
use std::time::Instant;
use std::{
    fs::File,
    path::{Path, PathBuf},
};
use sui_assets_info::prelude::Asset;
use tar::Archive;

pub async fn download_to<P: AsRef<Path>>(dir: P, asset: &Asset) -> Result<PathBuf> {
    let url = asset.download_link();
    let filename = asset.name();

    let file = PathBuf::from(dir.as_ref()).join(filename);
    println!("File: {}", file.display());

    let mut f = File::create(&file)?;
    let mut downloaded = 0;

    let client = Client::new();
    let resp = client.get(url).send().await?;
    let total_size = resp
        .content_length()
        .ok_or(Error::Custom("content length".into()))?;
    println!("Total size: {} bytes", total_size);

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
        .progress_chars("#>-"));

    let message = format!("Downloading: {}", url);
    pb.set_message(Cow::Owned(message));
    let mut stream = resp.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item?;
        f.write_all(&chunk)?;
        downloaded = min(downloaded + (chunk.len() as u64), total_size);
        pb.set_position(downloaded);
    }

    pb.finish_with_message(Cow::Owned(format!(
        "Downloaded from {} as {}",
        url,
        file.display()
    )));

    Ok(file)
}

pub async fn extract<P: AsRef<Path>>(path: P, output_path: P) -> Result<()> {
    let start = Instant::now();
    let f = File::open(path)?;
    let decoder = GzDecoder::new(f);
    let mut archive = Archive::new(decoder);
    archive.unpack(output_path)?;
    println!("Extract time usage: {} seconds", start.elapsed().as_secs());
    Ok(())
}
