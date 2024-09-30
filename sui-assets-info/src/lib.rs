pub(crate) mod desc;
pub(crate) mod error;
pub(crate) use error::{Result, UpdaterError};
pub(crate) mod backend;
pub(crate) mod config;

pub mod prelude {
    pub use crate::backend::github::GithubBackend;
    pub use crate::backend::Asset;
    pub use crate::backend::BackEnd;
    pub use crate::desc::{Network, SuiAssetDesc, Version};
    pub use crate::error::UpdaterError;
}
