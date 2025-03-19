use rand::distr::Alphanumeric;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::HashMap;

pub struct UrlShortener {
    urls: HashMap<String, String>,
}

impl UrlShortener {
    pub fn new() -> Self {
        Self {
            urls: HashMap::new(),
        }
    }

    pub fn shorten_url(&mut self, long_url: &str) -> String {
        let short_code = ThreadRng::default()
            .sample_iter(Alphanumeric)
            .take(6)
            .map(char::from)
            .collect::<String>();

        self.urls.insert(short_code.clone(), long_url.to_string());

        short_code
    }
}
