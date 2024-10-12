use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct SpotifyUserImage { 
  pub url: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotifyUser {
  pub id: String,
  pub display_name: String,
  pub email: String,
  pub images: Vec<SpotifyUserImage>
}