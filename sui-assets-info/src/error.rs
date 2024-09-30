use std::borrow::Cow;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UpdaterError {
    #[error("{0}")]
    InvalidData(Cow<'static, str>),

    #[error("parse {} error", desc)]
    ParseAssetDescError { desc: &'static str },

    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),
}

pub(crate) type Result<T> = std::result::Result<T, UpdaterError>;
