use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub allowed_languages: Vec<String>,
    pub git_url: String,
}
