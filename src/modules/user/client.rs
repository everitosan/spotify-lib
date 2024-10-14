use crate::modules::oauth::config::ClientConfig;

pub struct UserClient {
  config: ClientConfig
}

impl UserClient {
  pub fn new(config: ClientConfig) -> Self {
    Self {
      config
    }
  }
  
}