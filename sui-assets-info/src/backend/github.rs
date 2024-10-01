use crate::desc::{Network, SuiAssetDesc};
use crate::{Result, UpdaterError};
use reqwest::{header::USER_AGENT, Client};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubAsset {
    #[serde(rename = "browser_download_url")]
    pub download_url: String,
    pub name: String,
}

// impl Asset for GitHubAsset {
//     fn name() -> &'static str {
//         "GithubAsset"
//     }
// }

#[derive(Debug, Deserialize, Clone)]
pub struct GithubRelease {
    #[serde(rename = "tag_name")]
    pub desc: String,
    pub assets: Vec<GitHubAsset>,
}

#[derive(Debug, Clone)]
pub struct GithubBackend(GithubBackendImpl);
impl Default for GithubBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl GithubBackend {
    pub async fn get_tag(&self, desc: &SuiAssetDesc) -> Result<GitHubAsset> {
        self.0.get_release_by_tag(desc).await
    }

    pub async fn get_latest(&self, network: Option<Network>) -> Result<GitHubAsset> {
        self.0.get_latest_release(network).await
    }

    pub fn new() -> Self {
        Self(GithubBackendImpl)
    }
}

#[derive(Debug, Clone)]
struct GithubBackendImpl;
impl GithubBackendImpl {
    const API_PREFIX: &'static str = "https://api.github.com/repos/MystenLabs/sui/releases";
    const UA: &'static str =
        "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0";

    async fn get_release_by_tag(&self, desc: &SuiAssetDesc) -> Result<GitHubAsset> {
        let api_url = format!(
            "https://api.github.com/repos/MystenLabs/sui/releases/tags/{}",
            desc.desc()
        );

        let client = Client::new();
        let resp = client
            .get(api_url)
            .header(USER_AGENT, Self::UA)
            .send()
            .await?;
        let release: GithubRelease = resp.json().await?;

        release
            .assets
            .into_iter()
            .find(|asset| SuiAssetDesc::from_quints(&asset.name).is_ok())
            .ok_or(UpdaterError::InvalidData(
                "no release to match the required network".into(),
            ))
    }

    async fn get_latest_release(&self, network: Option<Network>) -> Result<GitHubAsset> {
        let api_url = format!("{}?page=1&per_page=10", Self::API_PREFIX);

        let filter = |release: &GithubRelease| -> bool {
            if let Ok(desc) = SuiAssetDesc::from_twins(&release.desc) {
                network.map(|n| *desc.network() == n).unwrap_or(true)
            } else {
                false
            }
        };

        let client = Client::new();
        let resp = client
            .get(api_url)
            .header(USER_AGENT, Self::UA)
            .send()
            .await?;

        if let Some(release) = resp
            .json::<Vec<GithubRelease>>()
            .await?
            .into_iter()
            .find(filter)
        {
            if let Some(asset) = release
                .assets
                .iter()
                .find(|asset| SuiAssetDesc::from_quints(&asset.name).is_ok())
            {
                return Ok(asset.clone());
            }
        }

        Err(UpdaterError::InvalidData(
            "no release to match the required network".into(),
        ))
    }
}
