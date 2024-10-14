use std::collections::HashMap;


pub fn parse(raw_urls: &String) -> HashMap<String, String> {
  let mut res: HashMap<String, String> = HashMap::new();

  let parts: Vec<&str> = raw_urls.split(",").collect();

  if parts.len() == 1 {
    res.insert("default".to_owned(), raw_urls.clone());
  } else {
    parts.iter().for_each(|callback| {
      let callback_entry: Vec<&str> = callback.split("@").collect();
      if let Some(entry) = callback_entry.get(0) {
        if let Some(url) = callback_entry.get(1) {
          res.insert(format!("{}", entry), format!("{}", url));
        }
      }
    });
  }

  return res;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_spotify_callbacks() {
    let urls = "default@http://localhost:9000/callback,deep@http://localhost:9000/cookie/callback/".to_string();
    let parsed = parse(&urls);
    let default = parsed.get("default").unwrap();
    assert_eq!(default, "http://localhost:9000/callback");
  }

  #[test]
  fn test_spotify_callback() {
    let url = "http://localhost:9000".to_string();
    let parsed = parse(&url);

    let default = parsed.get("default").unwrap();
    assert_eq!(default, "http://localhost:9000")
  }


}