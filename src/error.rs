use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    AlreadyExists(String),
    #[error("{0}")]
    Invalid(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("expected a mappings document: {0}")]
    Yaml(#[from] serde_yaml_ng::Error),
    #[error("expected an OSCAL System Security Plan: {0}")]
    Json(#[from] serde_json::Error),
}
