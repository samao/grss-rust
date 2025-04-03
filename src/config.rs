use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub comfy: bool,
    pub foo: i64,
    pub last_modified: i64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: "default".to_string(),
            comfy: true,
            foo: 42,
            last_modified: Utc::now().timestamp(),
        }
    }
}
