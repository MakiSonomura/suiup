use indicatif::style::TemplateError;
use std::{borrow::Cow, env::VarError, io};
use sui_assets_info::prelude::UpdaterError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Custom(Cow<'static, str>),
    #[error("{0}")]
    IoError(#[from] io::Error),
    #[error("{0}")]
    VarError(#[from] VarError),
    #[error("{0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("{0}")]
    UpdaterError(#[from] UpdaterError),
    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("{0}")]
    TemplateError(#[from] TemplateError),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
