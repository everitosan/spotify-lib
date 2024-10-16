
use core::fmt;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use reqwest::{self, StatusCode};

use super::config::ClientConfig;
use super::domain::SpotifyToken;
use crate::constants::SPOTIFY_URL;
use crate::errors::{Result, Error};

static SPOTIFY_RESPONSE_TYPE: &'static str = "response_type=code&";
static SPOTIFY_TOKEN_URL: &'static str = "/api/token";

pub enum Scopes {
  UserReadPrivate, //user-read-private
  UserReadEmail, //user-read-email
  PlaylistReadPrivate, //playlist-read-private
  PlaylistReadCollaborative, //playlist-read-collaborative
  UserLibraryRead, //user-library-read
  UserTopRead, //user-top-read
}

impl fmt::Display for Scopes {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Scopes::UserReadPrivate => write!(f, "user-read-private"),
      Scopes::UserReadEmail => write!(f, "user-read-email"),
      Scopes::PlaylistReadPrivate => write!(f, "playlist-read-private"),
      Scopes::PlaylistReadCollaborative => write!(f, "playlist-read-collaborative"),
      Scopes::UserLibraryRead => write!(f, "user-library-read"),
      Scopes::UserTopRead => write!(f, "user-top-read"),
    }
  }
}

pub struct OauthClient<'a> {
  config: &'a ClientConfig
}


impl<'a> OauthClient <'a> {
  pub fn new(config: &'a ClientConfig) -> Self {
    Self {
      config: config
    }
  }
  
  pub fn get_redirect_url(&self, scopes: Vec<Scopes>, state: &String, callback_key: &'static str) -> String {
    let str_scopes: Vec<String> = scopes.iter().map(|s| s.to_string() ).collect();
    let client_id = format!("client_id={}&", self.config.client_id);
    let redirect_uri = format!("redirect_uri={}&", self.config.get_callback_by_key(callback_key));
    let state = format!("state={}&", state);
    let scopes = format!("scope={}", str_scopes.join("%20"));
    
    let mut url = format!("{}/authorize?", SPOTIFY_URL);
    
    url.push_str(&client_id);
    url.push_str(SPOTIFY_RESPONSE_TYPE);
    url.push_str(&redirect_uri);
    url.push_str(&state);
    url.push_str(&scopes);
    
    return url;
  }

  pub async fn exchange_token(&self, code: &String, callback_key: &'static str) -> Result<SpotifyToken> {
    let credentials = BASE64_STANDARD.encode(format!("{}:{}", self.config.client_id, self.config.secret));

    let params = [
      ("code", code),
      ("redirect_uri", &self.config.get_callback_by_key(callback_key)),
      ("grant_type", &"authorization_code".to_string())
    ];
  
    let client = reqwest::Client::new();
  
    let response = match client.post(format!("{}{}", SPOTIFY_URL,SPOTIFY_TOKEN_URL))
      .form(&params)
      .header(reqwest::header::AUTHORIZATION, format!("Basic {}", credentials))
      .send()
      .await {
        Ok(r) => { 
          if r.status() != StatusCode::OK {
            return Err(Error::OAuth("Bad code".to_string()));
          } else { r }
        }
        Err(e) => {
          return Err(Error::OAuth(format!("{} possible network", e)))
        }
      };

    // let text = &response.text().await.unwrap();
    // println!("{}", text);
    // return Err(Error::OAuth(format!("mismatch response")));
  
    match response.json::<SpotifyToken>().await {
      Ok(r) => { return Ok(r) },
      Err(e) => {
        return Err(Error::OAuth(format!("{} mismatch response", e)))
      }
    };
  }
}


#[cfg(test)]
mod test {
  use super::*;
  use uuid::Uuid;
  
  #[test]
  fn test_get_redirect_url() {
    
    let config = ClientConfig::from_env_file(".env.example").unwrap();
    let client = OauthClient::new(&config);
    let scopes: Vec<Scopes> = vec![ Scopes::UserReadPrivate, Scopes::UserReadEmail ];
    let state = Uuid::new_v4().to_string();
    
    let redirect = client.get_redirect_url(scopes, &state, "default");
    let expected = format!("https://accounts.spotify.com/authorize?client_id=83jjr8ujrjrj0wrkr8i&response_type=code&redirect_uri=http://myapp.com/callback&state={}&scope=user-read-private%20user-read-email", state);
    
    assert_eq!(redirect, expected);
  }
}