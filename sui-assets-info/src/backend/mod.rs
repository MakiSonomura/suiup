use std::fmt::Display;
use std::str::FromStr;

use github::{GitHubAsset, GithubBackend};
use walrus::{WalrusAsset, WalrusBackend};

use crate::desc::{Network, SuiAssetDesc};
use crate::{Result, UpdaterError};
pub(crate) mod github;
pub(crate) mod walrus;

#[derive(Debug)]
pub enum Asset {
    Github(GitHubAsset),
    Walrus(WalrusAsset),
}

impl Asset {
    pub fn download_link(&self) -> &str {
        match self {
            Self::Github(asset) => &asset.download_url,
            Self::Walrus(_) => unimplemented!(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Github(asset) => &asset.name,
            Self::Walrus(_) => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BackEnd {
    WalrusBackend(WalrusBackend),
    GithubBackend(GithubBackend),
}

impl FromStr for BackEnd {
    type Err = UpdaterError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "walrus" | "Walrus" => Ok(Self::WalrusBackend(WalrusBackend::new())),
            "github" | "Github" => Ok(Self::GithubBackend(GithubBackend::new())),
            _ => Err(Self::Err::InvalidData(
                format!("invalid backend: {}", s).into(),
            )),
        }
    }
}

impl Display for BackEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::GithubBackend(_) => "github",
            Self::WalrusBackend(_) => "walrus",
        };
        write!(f, "{}", s)
    }
}

impl BackEnd {
    pub async fn fetch_latest(&self, network: Option<Network>) -> Result<Asset> {
        match self {
            Self::GithubBackend(backend) => Ok(Asset::Github(backend.get_latest(network).await?)),
            Self::WalrusBackend(backend) => Ok(Asset::Walrus(backend.get_latest(network).await?)),
        }
    }

    pub async fn fetch_specific(&self, desc: &SuiAssetDesc) -> Result<Asset> {
        match self {
            Self::GithubBackend(backend) => Ok(Asset::Github(backend.get_tag(desc).await?)),
            Self::WalrusBackend(_) => todo!(),
        }
    }
}
