# Spotify-lib

Unofficial spotify lib for rust

## Installation
```toml
[dependencies]
spotify-lib = { git="https://github.com/everitosan/spotify-lib", version="0.1.0" }
```

### Use

```rust
use spotify_lib::modules::oauth::{
  client::{Client, Scopes}, 
  config::ClientConfig
};

let config = ClientConfig::from_env().unwrap(); // Will search for .env file
let config_1 = ClientConfig::from_env_file("custom.env".to_owned()).unwrap();

let client = Client::new(config);
let scopes: Vec<Scopes> = vec![Scopes::UserReadEmail, Scopes::UserReadPrivate];
let state = uuid::Uuid::new_v4().to_string();
let redirect = client.get_token_url(scopes, &state);

// Exchange token

let spotify_token = client.exchange_token(&code).await.unwrap();

```

### Env Vars
| Name | Description |
| -- | -- |
| SPOTIFY_CLIENT_ID | Client id provided by spotify | 
| SPOTIFY_SECRET | Secret provided by spotify | 
| SPOTIFY_CALLBACK | Callback seted in spotify dashboard | 
