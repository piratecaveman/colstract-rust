use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash)]
pub struct Wallpaper {
    pub enable: Option<bool>,
    pub path: Option<String>,
    pub command: Option<Vec<String>>,
}
