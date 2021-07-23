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
        let default = Color::default();
        format!(
            r##"{{
	"alpha": "{alpha}",
    "alpha_percentage": "{alpha_percentage}",
	"background": "{background}",
	"background_rgb": "{background_rgb}",
	"background_rgba": "{background_rgba}",
	"background_hex8": "{background_hex8}",
	"background_xrgba": "{background_xrgba}",
	"background_alpha": "{background_alpha}",
	"background_hex_stripped": "{background_hex_stripped}",
	"background_hex8_stripped": "{background_hex8_stripped}",
	"background_rgb_stripped": "{background_rgb_stripped}",
	"background_rgba_stripped": "{background_rgba_stripped}",
	"foreground": "{foreground}",
	"foreground_rgb": "{foreground_rgb}",
	"foreground_rgba": "{foreground_rgba}",
	"foreground_hex8": "{foreground_hex8}",
	"foreground_xrgba": "{foreground_xrgba}",
	"foreground_alpha": "{foreground_alpha}",
	"foreground_hex_stripped": "{foreground_hex_stripped}",
	"foreground_hex8_stripped": "{foreground_hex8_stripped}",
	"foreground_rgb_stripped": "{foreground_rgb_stripped}",
	"foreground_rgba_stripped": "{foreground_rgba_stripped}",
	"cursor": "{cursor}",
	"cursor_rgb": "{cursor_rgb}",
	"cursor_rgba": "{cursor_rgba}",
	"cursor_hex8": "{cursor_hex8}",
	"cursor_xrgba": "{cursor_xrgba}",
	"cursor_alpha": "{cursor_alpha}",
	"cursor_hex_stripped": "{cursor_hex_stripped}",
	"cursor_hex8_stripped": "{cursor_hex8_stripped}",
	"cursor_rgb_stripped": "{cursor_rgb_stripped}",
	"cursor_rgba_stripped": "{cursor_rgba_stripped}",
	"color0": "{color0}",
	"color0_rgb": "{color0_rgb}",
	"color0_rgba": "{color0_rgba}",
	"color0_hex8": "{color0_hex8}",
	"color0_xrgba": "{color0_xrgba}",
	"color0_alpha": "{color0_alpha}",
	"color0_hex_stripped": "{color0_hex_stripped}",
	"color0_hex8_stripped": "{color0_hex8_stripped}",
	"color0_rgb_stripped": "{color0_rgb_stripped}",
	"color0_rgba_stripped": "{color0_rgba_stripped}",
	"color1": "{color1}",
	"color1_rgb": "{color1_rgb}",
	"color1_rgba": "{color1_rgba}",
	"color1_hex8": "{color1_hex8}",
	"color1_xrgba": "{color1_xrgba}",
	"color1_alpha": "{color1_alpha}",
	"color1_hex_stripped": "{color1_hex_stripped}",
	"color1_hex8_stripped": "{color1_hex8_stripped}",
	"color1_rgb_stripped": "{color1_rgb_stripped}",
	"color1_rgba_stripped": "{color1_rgba_stripped}",
	"color2": "{color2}",
	"color2_rgb": "{color2_rgb}",
	"color2_rgba": "{color2_rgba}",
	"color2_hex8": "{color2_hex8}",
	"color2_xrgba": "{color2_xrgba}",
	"color2_alpha": "{color2_alpha}",
	"color2_hex_stripped": "{color2_hex_stripped}",
	"color2_hex8_stripped": "{color2_hex8_stripped}",
	"color2_rgb_stripped": "{color2_rgb_stripped}",
	"color2_rgba_stripped": "{color2_rgba_stripped}",
	"color3": "{color3}",
	"color3_rgb": "{color3_rgb}",
	"color3_rgba": "{color3_rgba}",
	"color3_hex8": "{color3_hex8}",
	"color3_xrgba": "{color3_xrgba}",
	"color3_alpha": "{color3_alpha}",
	"color3_hex_stripped": "{color3_hex_stripped}",
	"color3_hex8_stripped": "{color3_hex8_stripped}",
	"color3_rgb_stripped": "{color3_rgb_stripped}",
	"color3_rgba_stripped": "{color3_rgba_stripped}",
	"color4": "{color4}",
	"color4_rgb": "{color4_rgb}",
	"color4_rgba": "{color4_rgba}",
	"color4_hex8": "{color4_hex8}",
	"color4_xrgba": "{color4_xrgba}",
	"color4_alpha": "{color4_alpha}",
	"color4_hex_stripped": "{color4_hex_stripped}",
	"color4_hex8_stripped": "{color4_hex8_stripped}",
	"color4_rgb_stripped": "{color4_rgb_stripped}",
	"color4_rgba_stripped": "{color4_rgba_stripped}",
	"color5": "{color5}",
	"color5_rgb": "{color5_rgb}",
	"color5_rgba": "{color5_rgba}",
	"color5_hex8": "{color5_hex8}",
	"color5_xrgba": "{color5_xrgba}",
	"color5_alpha": "{color5_alpha}",
	"color5_hex_stripped": "{color5_hex_stripped}",
	"color5_hex8_stripped": "{color5_hex8_stripped}",
	"color5_rgb_stripped": "{color5_rgb_stripped}",
	"color5_rgba_stripped": "{color5_rgba_stripped}",
	"color6": "{color6}",
	"color6_rgb": "{color6_rgb}",
	"color6_rgba": "{color6_rgba}",
	"color6_hex8": "{color6_hex8}",
	"color6_xrgba": "{color6_xrgba}",
	"color6_alpha": "{color6_alpha}",
	"color6_hex_stripped": "{color6_hex_stripped}",
	"color6_hex8_stripped": "{color6_hex8_stripped}",
	"color6_rgb_stripped": "{color6_rgb_stripped}",
	"color6_rgba_stripped": "{color6_rgba_stripped}",
	"color7": "{color7}",
	"color7_rgb": "{color7_rgb}",
	"color7_rgba": "{color7_rgba}",
	"color7_hex8": "{color7_hex8}",
	"color7_xrgba": "{color7_xrgba}",
	"color7_alpha": "{color7_alpha}",
	"color7_hex_stripped": "{color7_hex_stripped}",
	"color7_hex8_stripped": "{color7_hex8_stripped}",
	"color7_rgb_stripped": "{color7_rgb_stripped}",
	"color7_rgba_stripped": "{color7_rgba_stripped}",
	"color8": "{color8}",
	"color8_rgb": "{color8_rgb}",
	"color8_rgba": "{color8_rgba}",
	"color8_hex8": "{color8_hex8}",
	"color8_xrgba": "{color8_xrgba}",
	"color8_alpha": "{color8_alpha}",
	"color8_hex_stripped": "{color8_hex_stripped}",
	"color8_hex8_stripped": "{color8_hex8_stripped}",
	"color8_rgb_stripped": "{color8_rgb_stripped}",
	"color8_rgba_stripped": "{color8_rgba_stripped}",
	"color9": "{color9}",
	"color9_rgb": "{color9_rgb}",
	"color9_rgba": "{color9_rgba}",
	"color9_hex8": "{color9_hex8}",
	"color9_xrgba": "{color9_xrgba}",
	"color9_alpha": "{color9_alpha}",
	"color9_hex_stripped": "{color9_hex_stripped}",
	"color9_hex8_stripped": "{color9_hex8_stripped}",
	"color9_rgb_stripped": "{color9_rgb_stripped}",
	"color9_rgba_stripped": "{color9_rgba_stripped}",
	"color10": "{color10}",
	"color10_rgb": "{color10_rgb}",
	"color10_rgba": "{color10_rgba}",
	"color10_hex8": "{color10_hex8}",
	"color10_xrgba": "{color10_xrgba}",
	"color10_alpha": "{color10_alpha}",
	"color10_hex_stripped": "{color10_hex_stripped}",
	"color10_hex8_stripped": "{color10_hex8_stripped}",
	"color10_rgb_stripped": "{color10_rgb_stripped}",
	"color10_rgba_stripped": "{color10_rgba_stripped}",
	"color11": "{color11}",
	"color11_rgb": "{color11_rgb}",
	"color11_rgba": "{color11_rgba}",
	"color11_hex8": "{color11_hex8}",
	"color11_xrgba": "{color11_xrgba}",
	"color11_alpha": "{color11_alpha}",
	"color11_hex_stripped": "{color11_hex_stripped}",
	"color11_hex8_stripped": "{color11_hex8_stripped}",
	"color11_rgb_stripped": "{color11_rgb_stripped}",
	"color11_rgba_stripped": "{color11_rgba_stripped}",
	"color12": "{color12}",
	"color12_rgb": "{color12_rgb}",
	"color12_rgba": "{color12_rgba}",
	"color12_hex8": "{color12_hex8}",
	"color12_xrgba": "{color12_xrgba}",
	"color12_alpha": "{color12_alpha}",
	"color12_hex_stripped": "{color12_hex_stripped}",
	"color12_hex8_stripped": "{color12_hex8_stripped}",
	"color12_rgb_stripped": "{color12_rgb_stripped}",
	"color12_rgba_stripped": "{color12_rgba_stripped}",
	"color13": "{color13}",
	"color13_rgb": "{color13_rgb}",
	"color13_rgba": "{color13_rgba}",
	"color13_hex8": "{color13_hex8}",
	"color13_xrgba": "{color13_xrgba}",
	"color13_alpha": "{color13_alpha}",
	"color13_hex_stripped": "{color13_hex_stripped}",
	"color13_hex8_stripped": "{color13_hex8_stripped}",
	"color13_rgb_stripped": "{color13_rgb_stripped}",
	"color13_rgba_stripped": "{color13_rgba_stripped}",
	"color14": "{color14}",
	"color14_rgb": "{color14_rgb}",
	"color14_rgba": "{color14_rgba}",
	"color14_hex8": "{color14_hex8}",
	"color14_xrgba": "{color14_xrgba}",
	"color14_alpha": "{color14_alpha}",
	"color14_hex_stripped": "{color14_hex_stripped}",
	"color14_hex8_stripped": "{color14_hex8_stripped}",
	"color14_rgb_stripped": "{color14_rgb_stripped}",
	"color14_rgba_stripped": "{color14_rgba_stripped}",
	"color15": "{color15}",
	"color15_rgb": "{color15_rgb}",
	"color15_rgba": "{color15_rgba}",
	"color15_hex8": "{color15_hex8}",
	"color15_xrgba": "{color15_xrgba}",
	"color15_alpha": "{color15_alpha}",
	"color15_hex_stripped": "{color15_hex_stripped}",
	"color15_hex8_stripped": "{color15_hex8_stripped}",
	"color15_rgb_stripped": "{color15_rgb_stripped}",
	"color15_rgba_stripped": "{color15_rgba_stripped}",
    "wallpaper": "{wallpaper}"
}}"##,
            alpha = self
                .colors
                .map(|f| f.background.alpha_f32())
                .unwrap_or(1.0f32),
            alpha_percentage = self
                .colors
                .map(|f| f
                    .background
                    .alpha_percentage()
                    .round()
                    .clamp(0.0f32, 100.0f32) as u8)
                .unwrap_or(100),
            background = self
                .colors
                .map(|f| f.background.hex())
                .unwrap_or_else(|| default.hex()),
            background_rgb = self
                .colors
                .map(|f| f.background.rgb())
                .unwrap_or_else(|| default.rgb()),
            background_rgba = self
                .colors
                .map(|f| f.background.rgba())
                .unwrap_or_else(|| default.rgba()),
            background_hex8 = self
                .colors
                .map(|f| f.background.hex8())
                .unwrap_or_else(|| default.hex8()),
            background_xrgba = self
                .colors
                .map(|f| f.background.xrgba())
                .unwrap_or_else(|| default.xrgba()),
            background_alpha = self
                .colors
                .map(|f| f.background.alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            background_hex_stripped = self
                .colors
                .map(|f| f.background.hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            background_hex8_stripped = self
                .colors
                .map(|f| f.background.hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            background_rgb_stripped = self
                .colors
                .map(|f| f.background.rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            background_rgba_stripped = self
                .colors
                .map(|f| f.background.rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            foreground = self
                .colors
                .map(|f| f.foreground.hex())
                .unwrap_or_else(|| default.hex()),
            foreground_rgb = self
                .colors
                .map(|f| f.foreground.rgb())
                .unwrap_or_else(|| default.rgb()),
            foreground_rgba = self
                .colors
                .map(|f| f.foreground.rgba())
                .unwrap_or_else(|| default.rgba()),
            foreground_hex8 = self
                .colors
                .map(|f| f.foreground.hex8())
                .unwrap_or_else(|| default.hex8()),
            foreground_xrgba = self
                .colors
                .map(|f| f.foreground.xrgba())
                .unwrap_or_else(|| default.xrgba()),
            foreground_alpha = self
                .colors
                .map(|f| f.foreground.alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            foreground_hex_stripped = self
                .colors
                .map(|f| f.foreground.hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            foreground_hex8_stripped = self
                .colors
                .map(|f| f.foreground.hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            foreground_rgb_stripped = self
                .colors
                .map(|f| f.foreground.rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            foreground_rgba_stripped = self
                .colors
                .map(|f| f.foreground.rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            cursor = self
                .colors
                .map(|f| f.cursor.hex())
                .unwrap_or_else(|| default.hex()),
            cursor_rgb = self
                .colors
                .map(|f| f.cursor.rgb())
                .unwrap_or_else(|| default.rgb()),
            cursor_rgba = self
                .colors
                .map(|f| f.cursor.rgba())
                .unwrap_or_else(|| default.rgba()),
            cursor_hex8 = self
                .colors
                .map(|f| f.cursor.hex8())
                .unwrap_or_else(|| default.hex8()),
            cursor_xrgba = self
                .colors
                .map(|f| f.cursor.xrgba())
                .unwrap_or_else(|| default.xrgba()),
            cursor_alpha = self
                .colors
                .map(|f| f.cursor.alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            cursor_hex_stripped = self
                .colors
                .map(|f| f.cursor.hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            cursor_hex8_stripped = self
                .colors
                .map(|f| f.cursor.hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            cursor_rgb_stripped = self
                .colors
                .map(|f| f.cursor.rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            cursor_rgba_stripped = self
                .colors
                .map(|f| f.cursor.rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            color0 = self
                .colors
                .map(|f| f.colors[0].hex())
                .unwrap_or_else(|| default.hex()),
            color0_rgb = self
                .colors
                .map(|f| f.colors[0].rgb())
                .unwrap_or_else(|| default.rgb()),
            color0_rgba = self
                .colors
                .map(|f| f.colors[0].rgba())
                .unwrap_or_else(|| default.rgba()),
            color0_hex8 = self
                .colors
                .map(|f| f.colors[0].hex8())
                .unwrap_or_else(|| default.hex8()),
            color0_xrgba = self
                .colors
                .map(|f| f.colors[0].xrgba())
                .unwrap_or_else(|| default.xrgba()),
            color0_alpha = self
                .colors
                .map(|f| f.colors[0].alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            color0_hex_stripped = self
                .colors
                .map(|f| f.colors[0].hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            color0_hex8_stripped = self
                .colors
                .map(|f| f.colors[0].hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            color0_rgb_stripped = self
                .colors
                .map(|f| f.colors[0].rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            color0_rgba_stripped = self
                .colors
                .map(|f| f.colors[0].rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            color1 = self
                .colors
                .map(|f| f.colors[1].hex())
                .unwrap_or_else(|| default.hex()),
            color1_rgb = self
                .colors
                .map(|f| f.colors[1].rgb())
                .unwrap_or_else(|| default.rgb()),
            color1_rgba = self
                .colors
                .map(|f| f.colors[1].rgba())
                .unwrap_or_else(|| default.rgba()),
            color1_hex8 = self
                .colors
                .map(|f| f.colors[1].hex8())
                .unwrap_or_else(|| default.hex8()),
            color1_xrgba = self
                .colors
                .map(|f| f.colors[1].xrgba())
                .unwrap_or_else(|| default.xrgba()),
            color1_alpha = self
                .colors
                .map(|f| f.colors[1].alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            color1_hex_stripped = self
                .colors
                .map(|f| f.colors[1].hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            color1_hex8_stripped = self
                .colors
                .map(|f| f.colors[1].hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            color1_rgb_stripped = self
                .colors
                .map(|f| f.colors[1].rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            color1_rgba_stripped = self
                .colors
                .map(|f| f.colors[1].rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            color2 = self
                .colors
                .map(|f| f.colors[2].hex())
                .unwrap_or_else(|| default.hex()),
            color2_rgb = self
                .colors
                .map(|f| f.colors[2].rgb())
                .unwrap_or_else(|| default.rgb()),
            color2_rgba = self
                .colors
                .map(|f| f.colors[2].rgba())
                .unwrap_or_else(|| default.rgba()),
            color2_hex8 = self
                .colors
                .map(|f| f.colors[2].hex8())
                .unwrap_or_else(|| default.hex8()),
            color2_xrgba = self
                .colors
                .map(|f| f.colors[2].xrgba())
                .unwrap_or_else(|| default.xrgba()),
            color2_alpha = self
                .colors
                .map(|f| f.colors[2].alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            color2_hex_stripped = self
                .colors
                .map(|f| f.colors[2].hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            color2_hex8_stripped = self
                .colors
                .map(|f| f.colors[2].hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            color2_rgb_stripped = self
                .colors
                .map(|f| f.colors[2].rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            color2_rgba_stripped = self
                .colors
                .map(|f| f.colors[2].rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            color3 = self
                .colors
                .map(|f| f.colors[3].hex())
                .unwrap_or_else(|| default.hex()),
            color3_rgb = self
                .colors
                .map(|f| f.colors[3].rgb())
                .unwrap_or_else(|| default.rgb()),
            color3_rgba = self
                .colors
                .map(|f| f.colors[3].rgba())
                .unwrap_or_else(|| default.rgba()),
            color3_hex8 = self
                .colors
                .map(|f| f.colors[3].hex8())
                .unwrap_or_else(|| default.hex8()),
            color3_xrgba = self
                .colors
                .map(|f| f.colors[3].xrgba())
                .unwrap_or_else(|| default.xrgba()),
            color3_alpha = self
                .colors
                .map(|f| f.colors[3].alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            color3_hex_stripped = self
                .colors
                .map(|f| f.colors[3].hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            color3_hex8_stripped = self
                .colors
                .map(|f| f.colors[3].hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            color3_rgb_stripped = self
                .colors
                .map(|f| f.colors[3].rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            color3_rgba_stripped = self
                .colors
                .map(|f| f.colors[3].rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            color4 = self
                .colors
                .map(|f| f.colors[4].hex())
                .unwrap_or_else(|| default.hex()),
            color4_rgb = self
                .colors
                .map(|f| f.colors[4].rgb())
                .unwrap_or_else(|| default.rgb()),
            color4_rgba = self
                .colors
                .map(|f| f.colors[4].rgba())
                .unwrap_or_else(|| default.rgba()),
            color4_hex8 = self
                .colors
                .map(|f| f.colors[4].hex8())
                .unwrap_or_else(|| default.hex8()),
            color4_xrgba = self
                .colors
                .map(|f| f.colors[4].xrgba())
                .unwrap_or_else(|| default.xrgba()),
            color4_alpha = self
                .colors
                .map(|f| f.colors[4].alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            color4_hex_stripped = self
                .colors
                .map(|f| f.colors[4].hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            color4_hex8_stripped = self
                .colors
                .map(|f| f.colors[4].hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            color4_rgb_stripped = self
                .colors
                .map(|f| f.colors[4].rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            color4_rgba_stripped = self
                .colors
                .map(|f| f.colors[4].rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            color5 = self
                .colors
                .map(|f| f.colors[5].hex())
                .unwrap_or_else(|| default.hex()),
            color5_rgb = self
                .colors
                .map(|f| f.colors[5].rgb())
                .unwrap_or_else(|| default.rgb()),
            color5_rgba = self
                .colors
                .map(|f| f.colors[5].rgba())
                .unwrap_or_else(|| default.rgba()),
            color5_hex8 = self
                .colors
                .map(|f| f.colors[5].hex8())
                .unwrap_or_else(|| default.hex8()),
            color5_xrgba = self
                .colors
                .map(|f| f.colors[5].xrgba())
                .unwrap_or_else(|| default.xrgba()),
            color5_alpha = self
                .colors
                .map(|f| f.colors[5].alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            color5_hex_stripped = self
                .colors
                .map(|f| f.colors[5].hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            color5_hex8_stripped = self
                .colors
                .map(|f| f.colors[5].hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            color5_rgb_stripped = self
                .colors
                .map(|f| f.colors[5].rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            color5_rgba_stripped = self
                .colors
                .map(|f| f.colors[5].rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            color6 = self
                .colors
                .map(|f| f.colors[6].hex())
                .unwrap_or_else(|| default.hex()),
            color6_rgb = self
                .colors
                .map(|f| f.colors[6].rgb())
                .unwrap_or_else(|| default.rgb()),
            color6_rgba = self
                .colors
                .map(|f| f.colors[6].rgba())
                .unwrap_or_else(|| default.rgba()),
            color6_hex8 = self
                .colors
                .map(|f| f.colors[6].hex8())
                .unwrap_or_else(|| default.hex8()),
            color6_xrgba = self
                .colors
                .map(|f| f.colors[6].xrgba())
                .unwrap_or_else(|| default.xrgba()),
            color6_alpha = self
                .colors
                .map(|f| f.colors[6].alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            color6_hex_stripped = self
                .colors
                .map(|f| f.colors[6].hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            color6_hex8_stripped = self
                .colors
                .map(|f| f.colors[6].hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            color6_rgb_stripped = self
                .colors
                .map(|f| f.colors[6].rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            color6_rgba_stripped = self
                .colors
                .map(|f| f.colors[6].rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            color7 = self
                .colors
                .map(|f| f.colors[7].hex())
                .unwrap_or_else(|| default.hex()),
            color7_rgb = self
                .colors
                .map(|f| f.colors[7].rgb())
                .unwrap_or_else(|| default.rgb()),
            color7_rgba = self
                .colors
                .map(|f| f.colors[7].rgba())
                .unwrap_or_else(|| default.rgba()),
            color7_hex8 = self
                .colors
                .map(|f| f.colors[7].hex8())
                .unwrap_or_else(|| default.hex8()),
            color7_xrgba = self
                .colors
                .map(|f| f.colors[7].xrgba())
                .unwrap_or_else(|| default.xrgba()),
            color7_alpha = self
                .colors
                .map(|f| f.colors[7].alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            color7_hex_stripped = self
                .colors
                .map(|f| f.colors[7].hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            color7_hex8_stripped = self
                .colors
                .map(|f| f.colors[7].hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            color7_rgb_stripped = self
                .colors
                .map(|f| f.colors[7].rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            color7_rgba_stripped = self
                .colors
                .map(|f| f.colors[7].rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            color8 = self
                .colors
                .map(|f| f.colors[8].hex())
                .unwrap_or_else(|| default.hex()),
            color8_rgb = self
                .colors
                .map(|f| f.colors[8].rgb())
                .unwrap_or_else(|| default.rgb()),
            color8_rgba = self
                .colors
                .map(|f| f.colors[8].rgba())
                .unwrap_or_else(|| default.rgba()),
            color8_hex8 = self
                .colors
                .map(|f| f.colors[8].hex8())
                .unwrap_or_else(|| default.hex8()),
            color8_xrgba = self
                .colors
                .map(|f| f.colors[8].xrgba())
                .unwrap_or_else(|| default.xrgba()),
            color8_alpha = self
                .colors
                .map(|f| f.colors[8].alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            color8_hex_stripped = self
                .colors
                .map(|f| f.colors[8].hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            color8_hex8_stripped = self
                .colors
                .map(|f| f.colors[8].hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            color8_rgb_stripped = self
                .colors
                .map(|f| f.colors[8].rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            color8_rgba_stripped = self
                .colors
                .map(|f| f.colors[8].rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            color9 = self
                .colors
                .map(|f| f.colors[9].hex())
                .unwrap_or_else(|| default.hex()),
            color9_rgb = self
                .colors
                .map(|f| f.colors[9].rgb())
                .unwrap_or_else(|| default.rgb()),
            color9_rgba = self
                .colors
                .map(|f| f.colors[9].rgba())
                .unwrap_or_else(|| default.rgba()),
            color9_hex8 = self
                .colors
                .map(|f| f.colors[9].hex8())
                .unwrap_or_else(|| default.hex8()),
            color9_xrgba = self
                .colors
                .map(|f| f.colors[9].xrgba())
                .unwrap_or_else(|| default.xrgba()),
            color9_alpha = self
                .colors
                .map(|f| f.colors[9].alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            color9_hex_stripped = self
                .colors
                .map(|f| f.colors[9].hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            color9_hex8_stripped = self
                .colors
                .map(|f| f.colors[9].hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            color9_rgb_stripped = self
                .colors
                .map(|f| f.colors[9].rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            color9_rgba_stripped = self
                .colors
                .map(|f| f.colors[9].rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            color10 = self
                .colors
                .map(|f| f.colors[10].hex())
                .unwrap_or_else(|| default.hex()),
            color10_rgb = self
                .colors
                .map(|f| f.colors[10].rgb())
                .unwrap_or_else(|| default.rgb()),
            color10_rgba = self
                .colors
                .map(|f| f.colors[10].rgba())
                .unwrap_or_else(|| default.rgba()),
            color10_hex8 = self
                .colors
                .map(|f| f.colors[10].hex8())
                .unwrap_or_else(|| default.hex8()),
            color10_xrgba = self
                .colors
                .map(|f| f.colors[10].xrgba())
                .unwrap_or_else(|| default.xrgba()),
            color10_alpha = self
                .colors
                .map(|f| f.colors[10].alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            color10_hex_stripped = self
                .colors
                .map(|f| f.colors[10].hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            color10_hex8_stripped = self
                .colors
                .map(|f| f.colors[10].hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            color10_rgb_stripped = self
                .colors
                .map(|f| f.colors[10].rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            color10_rgba_stripped = self
                .colors
                .map(|f| f.colors[10].rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            color11 = self
                .colors
                .map(|f| f.colors[11].hex())
                .unwrap_or_else(|| default.hex()),
            color11_rgb = self
                .colors
                .map(|f| f.colors[11].rgb())
                .unwrap_or_else(|| default.rgb()),
            color11_rgba = self
                .colors
                .map(|f| f.colors[11].rgba())
                .unwrap_or_else(|| default.rgba()),
            color11_hex8 = self
                .colors
                .map(|f| f.colors[11].hex8())
                .unwrap_or_else(|| default.hex8()),
            color11_xrgba = self
                .colors
                .map(|f| f.colors[11].xrgba())
                .unwrap_or_else(|| default.xrgba()),
            color11_alpha = self
                .colors
                .map(|f| f.colors[11].alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            color11_hex_stripped = self
                .colors
                .map(|f| f.colors[11].hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            color11_hex8_stripped = self
                .colors
                .map(|f| f.colors[11].hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            color11_rgb_stripped = self
                .colors
                .map(|f| f.colors[11].rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            color11_rgba_stripped = self
                .colors
                .map(|f| f.colors[11].rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            color12 = self
                .colors
                .map(|f| f.colors[12].hex())
                .unwrap_or_else(|| default.hex()),
            color12_rgb = self
                .colors
                .map(|f| f.colors[12].rgb())
                .unwrap_or_else(|| default.rgb()),
            color12_rgba = self
                .colors
                .map(|f| f.colors[12].rgba())
                .unwrap_or_else(|| default.rgba()),
            color12_hex8 = self
                .colors
                .map(|f| f.colors[12].hex8())
                .unwrap_or_else(|| default.hex8()),
            color12_xrgba = self
                .colors
                .map(|f| f.colors[12].xrgba())
                .unwrap_or_else(|| default.xrgba()),
            color12_alpha = self
                .colors
                .map(|f| f.colors[12].alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            color12_hex_stripped = self
                .colors
                .map(|f| f.colors[12].hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            color12_hex8_stripped = self
                .colors
                .map(|f| f.colors[12].hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            color12_rgb_stripped = self
                .colors
                .map(|f| f.colors[12].rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            color12_rgba_stripped = self
                .colors
                .map(|f| f.colors[12].rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            color13 = self
                .colors
                .map(|f| f.colors[13].hex())
                .unwrap_or_else(|| default.hex()),
            color13_rgb = self
                .colors
                .map(|f| f.colors[13].rgb())
                .unwrap_or_else(|| default.rgb()),
            color13_rgba = self
                .colors
                .map(|f| f.colors[13].rgba())
                .unwrap_or_else(|| default.rgba()),
            color13_hex8 = self
                .colors
                .map(|f| f.colors[13].hex8())
                .unwrap_or_else(|| default.hex8()),
            color13_xrgba = self
                .colors
                .map(|f| f.colors[13].xrgba())
                .unwrap_or_else(|| default.xrgba()),
            color13_alpha = self
                .colors
                .map(|f| f.colors[13].alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            color13_hex_stripped = self
                .colors
                .map(|f| f.colors[13].hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            color13_hex8_stripped = self
                .colors
                .map(|f| f.colors[13].hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            color13_rgb_stripped = self
                .colors
                .map(|f| f.colors[13].rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            color13_rgba_stripped = self
                .colors
                .map(|f| f.colors[13].rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            color14 = self
                .colors
                .map(|f| f.colors[14].hex())
                .unwrap_or_else(|| default.hex()),
            color14_rgb = self
                .colors
                .map(|f| f.colors[14].rgb())
                .unwrap_or_else(|| default.rgb()),
            color14_rgba = self
                .colors
                .map(|f| f.colors[14].rgba())
                .unwrap_or_else(|| default.rgba()),
            color14_hex8 = self
                .colors
                .map(|f| f.colors[14].hex8())
                .unwrap_or_else(|| default.hex8()),
            color14_xrgba = self
                .colors
                .map(|f| f.colors[14].xrgba())
                .unwrap_or_else(|| default.xrgba()),
            color14_alpha = self
                .colors
                .map(|f| f.colors[14].alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            color14_hex_stripped = self
                .colors
                .map(|f| f.colors[14].hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            color14_hex8_stripped = self
                .colors
                .map(|f| f.colors[14].hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            color14_rgb_stripped = self
                .colors
                .map(|f| f.colors[14].rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            color14_rgba_stripped = self
                .colors
                .map(|f| f.colors[14].rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            color15 = self
                .colors
                .map(|f| f.colors[15].hex())
                .unwrap_or_else(|| default.hex()),
            color15_rgb = self
                .colors
                .map(|f| f.colors[15].rgb())
                .unwrap_or_else(|| default.rgb()),
            color15_rgba = self
                .colors
                .map(|f| f.colors[15].rgba())
                .unwrap_or_else(|| default.rgba()),
            color15_hex8 = self
                .colors
                .map(|f| f.colors[15].hex8())
                .unwrap_or_else(|| default.hex8()),
            color15_xrgba = self
                .colors
                .map(|f| f.colors[15].xrgba())
                .unwrap_or_else(|| default.xrgba()),
            color15_alpha = self
                .colors
                .map(|f| f.colors[15].alpha_f32())
                .unwrap_or_else(|| default.alpha_f32()),
            color15_hex_stripped = self
                .colors
                .map(|f| f.colors[15].hex_stripped())
                .unwrap_or_else(|| default.hex_stripped()),
            color15_hex8_stripped = self
                .colors
                .map(|f| f.colors[15].hex8_stripped())
                .unwrap_or_else(|| default.hex8_stripped()),
            color15_rgb_stripped = self
                .colors
                .map(|f| f.colors[15].rgb_stripped())
                .unwrap_or_else(|| default.rgb_stripped()),
            color15_rgba_stripped = self
                .colors
                .map(|f| f.colors[15].rgba_stripped())
                .unwrap_or_else(|| default.rgba_stripped()),
            wallpaper = match &self.wallpaper {
                Some(wal) => wal.path.clone().unwrap_or_default(),
                None => String::new(),
            }
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
    assert!(serde_json::from_str::<serde_json::Value>(&jason).is_ok());
}
