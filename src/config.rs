use colordata::traits::*;
use colordata::Color;
use serde::Deserialize;
use serde::Serialize;

use crate::structures::Colors;
use crate::structures::Wallpaper;
use crate::utils;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Config {
    pub input: Option<String>,
    pub output_directory: Option<String>,
    pub colors: Option<Colors>,
    pub wallpaper: Option<Wallpaper>,
}

impl Default for Config {
    fn default() -> Self {
        let home = std::path::PathBuf::from(std::env::var("HOME").unwrap());
        Config {
            input: home.join(".Xresources").to_str().map(str::to_string),
            output_directory: home
                .join(".cache")
                .join("colstract")
                .to_str()
                .map(str::to_string),
            colors: None,
            wallpaper: None,
        }
    }
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

    pub fn to_json(&self) -> String {
        macro_rules! hex {
            ($self:ident, $num:literal) => {
                match $self.colors {
                    Some(value) => value.colors[$num].hex(),
                    None => Color::default().hex(),
                }
            };
        }
        macro_rules! rgb {
            ($self:ident, $num:literal) => {
                match $self.colors {
                    Some(value) => value.colors[$num].rgb_stripped(),
                    None => Color::default().rgb_stripped(),
                }
            };
        }
        macro_rules! strip {
            ($self:ident, $num:literal) => {
                match $self.colors {
                    Some(value) => value.colors[$num]
                        .hex()
                        .strip_prefix('#')
                        .unwrap()
                        .to_string(),
                    None => Color::default()
                        .hex()
                        .strip_prefix('#')
                        .unwrap()
                        .to_string(),
                }
            };
        }
        macro_rules! xrgba {
            ($self:ident, $num:literal) => {
                match $self.colors {
                    Some(value) => value.colors[$num].xrgba(),
                    None => Color::default().xrgba(),
                }
            };
        }
        format!(
            r##"{{
            "background": "{background}",
            "background_rgb" : "{background_rgb}",
            "background_strip": "{background_strip}",
            "background_xrgba": "{background_xrgba}",
            "background_alpha_dec": "{alpha}",
            "background_alpha": "{background_hex8}",
            "foreground": "{foreground}",
            "foreground_rgb" : "{foreground_rgb}",
            "foreground_strip": "{foreground_strip}",
            "foreground_xrgba": "{foreground_xrgba}",
            "alpha": "{alpha}",
            "cursor": "{cursor}",
            "cursor_rgb": "{cursor_rgb}",
            "cursor_xrgba": "{cursor_xrgba}",
            "cursor_strip": "{cursor_strip}",
            "color0": "{color0}",
            "color1": "{color1}",
            "color2": "{color2}",
            "color3": "{color3}",
            "color4": "{color4}",
            "color5": "{color5}",
            "color6": "{color6}",
            "color7": "{color7}",
            "color8": "{color8}",
            "color9": "{color9}",
            "color10": "{color10}",
            "color11": "{color11}",
            "color12": "{color12}",
            "color13": "{color13}",
            "color14": "{color14}",
            "color15": "{color15}",
            "color0_rgb": "{color0_rgb}",
            "color1_rgb": "{color1_rgb}",
            "color2_rgb": "{color2_rgb}",
            "color3_rgb": "{color3_rgb}",
            "color4_rgb": "{color4_rgb}",
            "color5_rgb": "{color5_rgb}",
            "color6_rgb": "{color6_rgb}",
            "color7_rgb": "{color7_rgb}",
            "color8_rgb": "{color8_rgb}",
            "color9_rgb": "{color9_rgb}",
            "color10_rgb": "{color10_rgb}",
            "color11_rgb": "{color11_rgb}",
            "color12_rgb": "{color12_rgb}",
            "color13_rgb": "{color13_rgb}",
            "color14_rgb": "{color14_rgb}",
            "color15_rgb": "{color15_rgb}",
            "color0_strip": "{color0_strip}",
            "color1_strip": "{color1_strip}",
            "color2_strip": "{color2_strip}",
            "color3_strip": "{color3_strip}",
            "color4_strip": "{color4_strip}",
            "color5_strip": "{color5_strip}",
            "color6_strip": "{color6_strip}",
            "color7_strip": "{color7_strip}",
            "color8_strip": "{color8_strip}",
            "color9_strip": "{color9_strip}",
            "color10_strip": "{color10_strip}",
            "color11_strip": "{color11_strip}",
            "color12_strip": "{color12_strip}",
            "color13_strip": "{color13_strip}",
            "color14_strip": "{color14_strip}",
            "color15_strip": "{color15_strip}",
            "color0_xrgba": "{color0_xrgba}",
            "color1_xrgba": "{color1_xrgba}",
            "color2_xrgba": "{color2_xrgba}",
            "color3_xrgba": "{color3_xrgba}",
            "color4_xrgba": "{color4_xrgba}",
            "color5_xrgba": "{color5_xrgba}",
            "color6_xrgba": "{color6_xrgba}",
            "color7_xrgba": "{color7_xrgba}",
            "color8_xrgba": "{color8_xrgba}",
            "color9_xrgba": "{color9_xrgba}",
            "color10_xrgba": "{color10_xrgba}",
            "color11_xrgba": "{color11_xrgba}",
            "color12_xrgba": "{color12_xrgba}",
            "color13_xrgba": "{color13_xrgba}",
            "color14_xrgba": "{color14_xrgba}",
            "color15_xrgba": "{color15_xrgba}"
        }}"##,
            background = match self.colors {
                Some(value) => value.background.hex(),
                None => Color::default().hex(),
            },
            background_rgb = match self.colors {
                Some(value) => value.background.rgb_stripped(),
                None => Color::default().rgb_stripped(),
            },
            background_strip = match self.colors {
                Some(value) => value
                    .background
                    .hex()
                    .strip_prefix('#')
                    .unwrap()
                    .to_string(),
                None => Color::default()
                    .hex()
                    .strip_prefix('#')
                    .unwrap()
                    .to_string(),
            },
            background_xrgba = match self.colors {
                Some(value) => value.background.xrgba(),
                None => Color::default().xrgba(),
            },
            background_hex8 = match self.colors {
                Some(value) => value.background.hex8(),
                None => Color::default().hex8(),
            },
            alpha = match self.colors {
                Some(value) => value.background.alpha_f32(),
                None => Color::default().alpha_f32(),
            },
            foreground = match self.colors {
                Some(value) => value.foreground.hex(),
                None => Color::default().hex(),
            },
            foreground_rgb = match self.colors {
                Some(value) => value.foreground.rgb_stripped(),
                None => Color::default().rgb_stripped(),
            },
            foreground_strip = match self.colors {
                Some(value) => value
                    .foreground
                    .hex()
                    .strip_prefix('#')
                    .unwrap()
                    .to_string(),
                None => Color::default()
                    .hex()
                    .strip_prefix('#')
                    .unwrap()
                    .to_string(),
            },
            foreground_xrgba = match self.colors {
                Some(value) => value.foreground.xrgba(),
                None => Color::default().xrgba(),
            },
            cursor = match self.colors {
                Some(value) => value.cursor.hex(),
                None => Color::default().hex(),
            },
            cursor_rgb = match self.colors {
                Some(value) => value.cursor.rgb_stripped(),
                None => Color::default().rgb_stripped(),
            },
            cursor_strip = match self.colors {
                Some(value) => value.cursor.hex().strip_prefix('#').unwrap().to_string(),
                None => Color::default()
                    .hex()
                    .strip_prefix('#')
                    .unwrap()
                    .to_string(),
            },
            cursor_xrgba = match self.colors {
                Some(value) => value.cursor.xrgba(),
                None => Color::default().xrgba(),
            },
            color0 = hex!(self, 0),
            color1 = hex!(self, 1),
            color2 = hex!(self, 2),
            color3 = hex!(self, 3),
            color4 = hex!(self, 4),
            color5 = hex!(self, 5),
            color6 = hex!(self, 6),
            color7 = hex!(self, 7),
            color8 = hex!(self, 8),
            color9 = hex!(self, 9),
            color10 = hex!(self, 10),
            color11 = hex!(self, 11),
            color12 = hex!(self, 12),
            color13 = hex!(self, 13),
            color14 = hex!(self, 14),
            color15 = hex!(self, 15),
            color0_rgb = rgb!(self, 0),
            color1_rgb = rgb!(self, 1),
            color2_rgb = rgb!(self, 2),
            color3_rgb = rgb!(self, 3),
            color4_rgb = rgb!(self, 4),
            color5_rgb = rgb!(self, 5),
            color6_rgb = rgb!(self, 6),
            color7_rgb = rgb!(self, 7),
            color8_rgb = rgb!(self, 8),
            color9_rgb = rgb!(self, 9),
            color10_rgb = rgb!(self, 10),
            color11_rgb = rgb!(self, 11),
            color12_rgb = rgb!(self, 12),
            color13_rgb = rgb!(self, 13),
            color14_rgb = rgb!(self, 14),
            color15_rgb = rgb!(self, 15),
            color0_strip = strip!(self, 0),
            color1_strip = strip!(self, 1),
            color2_strip = strip!(self, 2),
            color3_strip = strip!(self, 3),
            color4_strip = strip!(self, 4),
            color5_strip = strip!(self, 5),
            color6_strip = strip!(self, 6),
            color7_strip = strip!(self, 7),
            color8_strip = strip!(self, 8),
            color9_strip = strip!(self, 9),
            color10_strip = strip!(self, 10),
            color11_strip = strip!(self, 11),
            color12_strip = strip!(self, 12),
            color13_strip = strip!(self, 13),
            color14_strip = strip!(self, 14),
            color15_strip = strip!(self, 15),
            color0_xrgba = xrgba!(self, 0),
            color1_xrgba = xrgba!(self, 1),
            color2_xrgba = xrgba!(self, 2),
            color3_xrgba = xrgba!(self, 3),
            color4_xrgba = xrgba!(self, 4),
            color5_xrgba = xrgba!(self, 5),
            color6_xrgba = xrgba!(self, 6),
            color7_xrgba = xrgba!(self, 7),
            color8_xrgba = xrgba!(self, 8),
            color9_xrgba = xrgba!(self, 9),
            color10_xrgba = xrgba!(self, 10),
            color11_xrgba = xrgba!(self, 11),
            color12_xrgba = xrgba!(self, 12),
            color13_xrgba = xrgba!(self, 13),
            color14_xrgba = xrgba!(self, 14),
            color15_xrgba = xrgba!(self, 15),
        )
    }
}

impl From<&str> for Config {
    fn from(s: &str) -> Self {
        let document_type = utils::detect_string_type(s);
        match document_type {
            0 => Config::from_toml_str(s),
            1 => Config::from_xresource_str(s),
            _ => panic!("Unrecognized document format"),
        }
    }
}

impl From<&String> for Config {
    fn from(s: &String) -> Self {
        Config::from(s.as_str())
    }
}

impl From<String> for Config {
    fn from(s: String) -> Self {
        Config::from(s.as_str())
    }
}

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
    let conf = Config::from(string);
    dbg!(conf);

    let string = r#"! special
    *.foreground:   #cac0a9
    *.background:   #1c1f2b
    *.cursorColor:  #cac0a9
    
    ! black
    *.color0:       #242837
    *.color8:       #7e818b
    
    ! red
    *.color1:       #f14360
    *.color9:       #ff89b5
    
    ! green
    *.color2:       #aecc00
    *.color10:      #b8cc66
    
    ! yellow
    *.color3:       #ff9d35
    *.color11:      #ffc380
    
    ! blue
    *.color4:       #75b0ff
    *.color12:      #bfd9ff
    
    ! magenta
    *.color5:       #c651e5
    *.color13:      #d2a1e6
    
    ! cyan
    *.color6:       #4ce7ff
    *.color14:      #99f5ff
    
    ! white
    *.color7:       #fbe1a3
    *.color15:      #e3d8be

    *.colors16:     #fabeca
    "#;
    let conf = Config::from(string);
    dbg!(conf);
}

#[test]
fn json_test() {
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
    let conf = Config::from(string);
    let jason = conf.to_json();
    assert!(serde_json::from_str::<serde_json::Value>(&jason).is_ok())
}
