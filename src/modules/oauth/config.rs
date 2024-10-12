use dotenvy;
use std::env;
use crate::errors::{Error, Result};
use validator::Validate;

#[derive(Debug, Validate)]
pub struct ClientConfig {
  #[validate(length(min = 5, message = "ClientConfig property seems to bee to small"))]
  pub client_id: String,
  #[validate(length(min = 5, message = "ClientConfig property seems to bee to small"))]
  pub secret: String,
  #[validate(url(message = "ClientConfig property is not a valid url"))]
  pub callback_url: String
}

impl ClientConfig {
 
  pub fn from_env() -> Result<Self> {
    Self::from_env_file(".env")
  }

  pub fn from_env_file(source_file: &str) -> Result<Self> {
 
    if let Err(e) = dotenvy::from_filename(source_file) {
      return Err(Error::ConfigError(format!("{}", e)));
    }

    let config = ClientConfig{
      client_id: env::var("SPOTIFY_CLIENT_ID").expect("missing SPOTIFY_CLIENT_ID"),
      secret: env::var("SPOTIFY_SECRET").expect("missing SPOTIFY_SECRET"),
      callback_url: env::var("SPOTIFY_CALLBACK").expect("missing SPOTIFY_CALLBACK")
    };

    config.check()
  }

  fn check(self) -> Result<Self> {
    match self.validate() {
      Ok(_) => Ok(self),
      Err(e) => Err(Error::ConfigError( format!("{}", e)))
    }
  }

}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  #[should_panic]
  fn test_read_from_missing_env() {
    // shoyuld panic as file does not exists
    ClientConfig::from_env_file("missing.file").unwrap();
  }

  #[test]
  fn test_validates_callback() {
    match ClientConfig::from_env_file(".env.wrong_callback") {
      Ok(config) => {
        panic!("check callback validation: {:?}", config);
      },
      Err(e) => {
        match e {
          Error::ConfigError(msg) => {
            assert_eq!(msg, "callback_url: ClientConfig property is not a valid url");
          },
          _ =>  { panic!("validation type differ from ConfigError") }
        }
      }
    };
  }

}