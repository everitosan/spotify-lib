use std::result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("[config error]: {0}")]
  ConfigError(String),
  #[error("[oauth error]: {0}")]
  OAuth(String),
  #[error("[unknown error]")]
  Unknown
}

pub type Result<T> = result::Result<T, Error>;