use dotenvy;
use std::env;
use std::collections::HashMap;
use super::utils::callback;

use validator::Validate;
use crate::errors::{Error, Result};

#[derive(Debug, Validate)]
pub struct ClientConfig {
  #[validate(length(min = 5, message = "ClientConfig property seems to bee to small"))]
  pub client_id: String,
  #[validate(length(min = 5, message = "ClientConfig property seems to bee to small"))]
  pub secret: String,
  pub callback_urls: HashMap<String, String> 
}


impl ClientConfig {
 
  pub fn from_env() -> Result<Self> {
    Self::from_env_file(".env")
  }

  pub fn from_env_file(source_file: &str) -> Result<Self> {
 
    if let Err(e) = dotenvy::from_filename(source_file) {
      return Err(Error::ConfigError(format!("{}", e)));
    }

    let callbacks = match env::var("SPOTIFY_CALLBACK") {
      Ok(single) => { single },
      Err(_) => {
        env::var("SPOTIFY_CALLBACKS").expect("SPOTIFY_CALLBACK or SPOTIFY_CALLBACKS should be defined")
      }
    };

    if callbacks.is_empty() {
      panic!("SPOTIFY_CALLBACK or SPOTIFY_CALLBACKS should be defined");
    }

    let config = ClientConfig{
      client_id: env::var("SPOTIFY_CLIENT_ID").expect("missing SPOTIFY_CLIENT_ID"),
      secret: env::var("SPOTIFY_SECRET").expect("missing SPOTIFY_SECRET"),
      callback_urls: callback::parse(&callbacks)
    };

    match config.validate() {
      Ok(_) => {Ok(config)},
      Err(e) => {
        Err(Error::ConfigError(e.to_string()))
      }
    }
  }

  pub fn get_callback_by_key(&self, key: &'static str) -> String {
    match self.callback_urls.get(key) {
      Some(u) => {u.clone()},
      None => { format!("--!!NEED TO REGISTER {} CALLBALCK!!--", key) }
    }
  }


}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  #[should_panic]
  fn test_read_from_missing_env() {
    // should panic as file does not exists
    ClientConfig::from_env_file("missing.file").unwrap();
  }

  #[test]
  #[should_panic]
  fn test_validates_callback() {
    // shoyuld panic askey is not defined
    ClientConfig::from_env_file(".env.wrong_callback").unwrap();
  }

}