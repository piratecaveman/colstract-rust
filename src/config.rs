use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigColors {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub color0: String,
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub color4: String,
    pub color5: String,
    pub color6: String,
    pub color7: String,
    pub color8: String,
    pub color9: String,
    pub color10: String,
    pub color11: String,
    pub color12: String,
    pub color13: String,
    pub color14: String,
    pub color15: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Wallpaper {
    pub enable: Option<bool>,
    pub path: String,
    pub command: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub input: String,
    pub output_directory: String,
    pub wallpaper: Option<Wallpaper>,
    pub colors: Option<ConfigColors>,
}

impl From<&str> for Config {
    fn from(s: &str) -> Self {
        toml::from_str(s).unwrap()
    }
}

impl From<String> for Config {
    fn from(s: String) -> Self {
        Config::from(s.as_str())
    }
}

impl From<&String> for Config {
    fn from(s: &String) -> Self {
        Config::from(s.as_str())
    }
}

#[test]
fn works_or_not() {
    let conf: Config = toml::from_str(
        r##"input = "/home/user/.Xresources"
    output_directory = "/home/user/.cache/colstract"
    
    [wallpaper]
    enable = true
    path = "/home/user/Pictures/wall.png"
    command = ["feh", "--bg-fill", "/home/user/Pictures/wall.png"]
    
    [colors]
    color0 = "#242837"
    color1 = "#f14360"
    color2 = "#aecc00"
    color3 = "#ff9d35"
    color4 = "#75b0ff"
    color5 = "#c651e5"
    color6 = "#4ce7ff"
    color7 = "#fbe1a3"
    color8 = "#7e818b"
    color9 = "#ff89b5"
    color10 = "#b8cc66"
    color11 = "#ffc380"
    color12 = "#bfd9ff"
    color13 = "#d2a1e6"
    color14 = "#99f5ff"
    color15 = "#e3d8be"
    background = "#1c1f2b"
    foreground = "#cac0a9"
    cursor = "#cac0a9"
    "##,
    )
    .unwrap();
    dbg!(conf);
}
