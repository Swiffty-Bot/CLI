use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub allowed_languages: Vec<String>,
    pub git_url: String,
}