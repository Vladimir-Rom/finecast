use serde::Deserialize;
use url;
use regex::Regex;

#[derive(Deserialize, Debug, Clone)]
pub struct Root {
    pub podcasts: Podcasts,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Podcasts {
    pub filtered: Vec<Filtered>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Filtered {
    pub title: String,
    pub source_url: url::Url,
    pub route: String,
    #[serde(with = "serde_regex")]
    pub filter: Regex,
}