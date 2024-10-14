use serde::Deserialize;

pub mod domain;
pub mod client;
pub mod config;
pub mod utils;


/*
  * Struct to obtain data from redirect of spotify
*/
#[derive(Deserialize, Debug)]
pub struct ExchangeToken {
  pub code: String,
  pub state: String
}

