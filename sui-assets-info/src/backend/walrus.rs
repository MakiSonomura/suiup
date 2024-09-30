use crate::desc::Network;

use crate::Result;

#[derive(Debug, Clone)]
pub struct WalrusBackend(WalrusBackendImpl);

impl Default for WalrusBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl WalrusBackend {
    pub async fn get_latest(&self, _network: Option<Network>) -> Result<WalrusAsset> {
        Ok(WalrusAsset {})
    }

    pub fn new() -> Self {
        Self(WalrusBackendImpl {})
    }
}

#[derive(Debug, Clone)]
struct WalrusBackendImpl;

#[derive(Debug)]
pub struct WalrusAsset {}
