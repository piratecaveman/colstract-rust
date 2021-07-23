pub mod command;
pub mod config;
pub mod render_template;
pub mod structures;
pub mod utils;

use std::env;
use std::path::PathBuf;

use argumentparser::Argument;
use argumentparser::Parser;
use text_colorizer::Colorize;

use crate::config::Config;
use crate::render_template::render_template;
use crate::structures::Wallpaper;

fn main() {
    let mut parser = Parser::with_capacity(5);
    parser.add_argument(
        Argument::with_type("word")
            .name("config")
            .invoke_with("--config")
            .invoke_with("-c")
            .required(false),
    );
    parser.add_argument(
        Argument::with_type("word")
            .name("input")
            .invoke_with("--input")
            .invoke_with("-i")
            .required(false),
    );
    parser.add_argument(
        Argument::with_type("word")
            .name("output")
            .invoke_with("--output")
            .invoke_with("-o")
            .required(false),
    );
    parser.add_argument(
        Argument::with_type("word")
            .name("wallpaper")
            .invoke_with("--wallpaper")
            .invoke_with("-w")
            .required(false),
    );
    parser.add_argument(
        Argument::with_type("vector")
            .name("wallpaper-command")
            .invoke_with("--wallpaper-command")
            .invoke_with("-W")
            .required(false),
    );

    // started parsing the arguments
    let home = match env::var("HOME") {
        Ok(val) => PathBuf::from(val),
        Err(_) => {
            eprintln!("{}", "Could not find home directory".red());
            eprintln!("{}", "Cannot continue, exiting".red());
            std::process::exit(1);
        }
    };
    let config_home = match env::var("XDG_CONFIG_HOME") {
        Ok(val) => PathBuf::from(val),
        Err(_) => home.join(".config"),
    };
    let config_toml = config_home.join("colstract").join("config.toml");
    let parsed_arguments = parser.parse_arguments(&std::env::args().collect::<Vec<_>>());
    let config = match parsed_arguments.get_value("config") {
        Some(val) => {
            if let argumentparser::Value::Word(c) = val {
                Config::from(c)
            } else if config_toml.exists() {
                Config::from(std::fs::read_to_string(&config_toml).unwrap())
            } else {
                eprintln!("{}", "No config files found using defaults".yellow());
                Config::default()
            }
        }
        None => {
            if config_toml.exists() {
                Config::from(std::fs::read_to_string(&config_toml).unwrap())
            } else {
                eprintln!("{}", "No config files found using defaults".yellow());
                Config::default()
            }
        }
    };
}
