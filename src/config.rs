use colordata::traits::*;
use colordata::Color;
use serde::Deserialize;
use serde::Serialize;

use crate::structures::Colors;
use crate::structures::Wallpaper;

#[derive(Debug, Default, Clone, Hash, Serialize, Deserialize)]
pub struct Config {
    pub input: Option<String>,
    pub output_directory: Option<String>,
    pub colors: Option<Colors>,
    pub wallpaper: Option<Wallpaper>,
}

impl Config {
    pub fn from_toml_str(s: &str) -> Self {
        let conf: toml::Value = match toml::from_str(s) {
            Ok(some_result) => some_result,
            Err(e) => panic!("an error occured while parsing the TOML: {}", e),
        };

        let input: Option<String> = conf.get("input").map(|s| s.as_str().unwrap().to_string());
        let output_directory = conf
            .get("output_directory")
            .map(|s| s.as_str().unwrap().to_string());
        let wallpaper = conf.get("wallpaper").map(|f| Wallpaper {
            enable: if let Some(e) = f.get("enable") {
                match e {
                    toml::Value::Boolean(b) => Some(*b),
                    _ => None,
                }
            } else {
                None
            },
            path: f.get("path").map(|f| f.as_str().unwrap().to_string()),
            command: f.get("command").map(|x| {
                x.as_array()
                    .unwrap()
                    .clone()
                    .iter()
                    .map(|k| k.as_str().unwrap().to_string())
                    .collect::<Vec<String>>()
            }),
        });
        let colors = conf.get("colors").map(|f| Colors {
            background: Color::from(f.get("background").unwrap().as_str().unwrap()),
            foreground: Color::from(f.get("foreground").unwrap().as_str().unwrap()),
            cursor: Color::from(f.get("cursor").unwrap().as_str().unwrap()),
            colors: [
                Color::from(f.get("color0").unwrap().as_str().unwrap()),
                Color::from(f.get("color1").unwrap().as_str().unwrap()),
                Color::from(f.get("color2").unwrap().as_str().unwrap()),
                Color::from(f.get("color3").unwrap().as_str().unwrap()),
                Color::from(f.get("color4").unwrap().as_str().unwrap()),
                Color::from(f.get("color5").unwrap().as_str().unwrap()),
                Color::from(f.get("color6").unwrap().as_str().unwrap()),
                Color::from(f.get("color7").unwrap().as_str().unwrap()),
                Color::from(f.get("color8").unwrap().as_str().unwrap()),
                Color::from(f.get("color9").unwrap().as_str().unwrap()),
                Color::from(f.get("color10").unwrap().as_str().unwrap()),
                Color::from(f.get("color11").unwrap().as_str().unwrap()),
                Color::from(f.get("color12").unwrap().as_str().unwrap()),
                Color::from(f.get("color13").unwrap().as_str().unwrap()),
                Color::from(f.get("color14").unwrap().as_str().unwrap()),
                Color::from(f.get("color15").unwrap().as_str().unwrap()),
            ],
        });
        Config {
            input,
            output_directory,
            colors,
            wallpaper,
        }
    }

    pub fn from_xresource_str(s: &str) -> Self {
        let background_regex =
            regex::Regex::new(r#".*background:\s*?(#[a-fA-F0-9]{6,8})\s?"#).unwrap();
        let foreground_regex =
            regex::Regex::new(r#".*foreground:\s*?(#[a-fA-F0-9]{6,8})\s?"#).unwrap();
        let cursor_regex =
            regex::Regex::new(r#".*cursorColor:\s*?(#[a-fA-F0-9]{6,8})\s?"#).unwrap();
        let colors_regex =
            regex::Regex::new(r#".*(color([0-9]{1,2})):\s*?(#[a-fA-F0-9]{6,8})\s?"#).unwrap();

        let mut background_color = Color::default();
        let mut foreground_color = Color::default();
        let mut cursor_color = Color::default();
        let mut colors_all = [Color::default(); 16];
        for line in s.split('\n') {
            if let Some(capt) = background_regex.captures(line) {
                match &capt[1].len() {
                    7 | 4 => {
                        background_color = Color::from_hex(&capt[1]);
                    }
                    9 | 5 => {
                        background_color = Color::from_hex8(&capt[1]);
                    }
                    _ => panic!("Invalid hex in: {}", line),
                }
                continue;
            };
            if let Some(capt) = foreground_regex.captures(line) {
                match &capt[1].len() {
                    7 | 4 => {
                        foreground_color = Color::from_hex(&capt[1]);
                    }
                    9 | 5 => {
                        foreground_color = Color::from_hex8(&capt[1]);
                    }
                    _ => panic!("Invalid hex in: {}", line),
                }
                continue;
            };
            if let Some(capt) = cursor_regex.captures(line) {
                match &capt[1].len() {
                    7 | 4 => {
                        cursor_color = Color::from_hex(&capt[1]);
                    }
                    9 | 5 => {
                        cursor_color = Color::from_hex8(&capt[1]);
                    }
                    _ => panic!("Invalid hex in: {}", line),
                }
                continue;
            };
            if let Some(capt) = colors_regex.captures(line) {
                match &capt[3].len() {
                    7 | 4 => {
                        let index = (&capt[2]).parse::<usize>().unwrap();
                        if index > 15 {
                            eprintln!("Ignoring color: {}", &capt[3]);
                            continue;
                        };
                        colors_all[index] = Color::from_hex(&capt[3]);
                    }
                    9 | 5 => {
                        let index = (&capt[2]).parse::<usize>().unwrap();
                        if index > 15 {
                            eprintln!("Ignoring color: {}", &capt[3]);
                            continue;
                        };
                        colors_all[index] = Color::from_hex(&capt[3]);
                    }
                    _ => panic!("Invalid hex in: {}", line),
                }
                continue;
            };
        }
        let colors = Colors {
            background: background_color,
            foreground: foreground_color,
            cursor: cursor_color,
            colors: colors_all,
        };
        Config {
            input: None,
            output_directory: None,
            wallpaper: None,
            colors: Some(colors),
        }
    }
}

// impl From<&String> for Config {
//     fn from(s: &String) -> Self {
//         Config::from(s.as_str())
//     }
// }

// impl From<String> for Config {
//     fn from(s: String) -> Self {
//         Config::from(s.as_str())
//     }
// }

#[test]
fn test_conf() {
    let string = r##"input = "/home/user/.Xresources"
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
    "##;
    let conf = Config::from_toml_str(string);
    dbg!(conf);
}
