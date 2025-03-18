use rand::distr::Alphanumeric;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::HashMap;

pub struct UrlShortener {
    urls: HashMap<String, String>,
    base_url: String,
}

impl UrlShortener {
    pub fn new(base_url: &str) -> Self {
        Self {
            urls: HashMap::new(),
            base_url: base_url.to_string(),
        }
    }

    pub fn shorten_url(&mut self, long_url: &str) -> String {
        let short_code = ThreadRng::default()
            .sample_iter(Alphanumeric)
            .take(6)
            .map(char::from)
            .collect::<String>();

        self.urls.insert(short_code.clone(), long_url.to_string());

        format!("{}/{}", self.base_url, short_code)
    }

    pub fn redirect(&self, short_code: &str) -> Option<&String> {
        self.urls.get(short_code)
    }
}
