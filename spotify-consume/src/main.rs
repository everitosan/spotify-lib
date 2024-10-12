use spotify_lib::modules::oauth::{client::{Client, Scopes}, config::ClientConfig};

#[tokio::main]
async fn main() {

  let config = ClientConfig::from_env().unwrap();
  let client = Client::new(config);
  let scopes: Vec<Scopes> = vec![Scopes::UserReadEmail, Scopes::UserReadPrivate];
  let state = uuid::Uuid::new_v4().to_string();
  let redirect = client.get_redirect_url(scopes, &state);
  println!("{}", redirect);
  let token = String::from("AQAbaYnPMCZZTOe5or_24-bWFm67LpDPya99JRbZoe52OW9CXPOTO2FF9lPu01rHmU0jiSQxaMBV9ZBUBtC7aENu8q2_7BGu8rvvRxoEibE0VYAaBHJm_20C7Ht-DaEEpcxvyuVJ-OnRUP1tNmxdhcED_SqRZQJ-lcH55KoXjPQWkRFkcggzw0lo43OlBcDh2n3B7nFOByIXbM8JqMw");
  let spotify_token = client.exchange_token(&token).await.unwrap();
  println!("{:?}", spotify_token);
}
