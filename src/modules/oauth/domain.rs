use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct SpotifyToken {
  pub access_token: String,
  pub token_type: String,
  pub expires_in: u16,
  pub refresh_token : String,
  pub scope: String
}

impl SpotifyToken {
  pub fn to_string(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

