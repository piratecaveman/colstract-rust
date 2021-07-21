pub mod command;
pub mod config;
pub mod render_template;
pub mod structures;
pub mod utils;

use std::env;
use std::path::PathBuf;

use argumentparser::Argument;
use argumentparser::Parser;

use crate::config::Config;
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
    let parsed_arguments = parser.parse_arguments(&std::env::args().collect::<Vec<_>>());
    let home = PathBuf::from(std::env::var("HOME").unwrap());

    let config_path = match parsed_arguments.get_value("config") {
        Some(path) => match path {
            argumentparser::Value::Word(s) => PathBuf::from(s),
            _ => unreachable!(),
        },
        None => home.join(".config").join("colstract").join("config.toml"),
    };

    let config_content = match std::fs::read_to_string(&config_path) {
        Ok(value) => Some(value),
        Err(e) => {
            eprintln!(
                "could not read the config file: {}\nat path: {}",
                e,
                &config_path.to_string_lossy()
            );
            None
        }
    };
    // a working config ready to use on every run
    let mut config = match &config_content {
        Some(val) => Config::from(val),
        None => {
            eprintln!("Using defaults for config");
            Config::default()
        }
    };

    // overriding config options with arguments
    if let Some(argumentparser::Value::Word(inp)) = parsed_arguments.get_value("input") {
        config.input = Some(inp.to_string());
    };

    if let Some(argumentparser::Value::Word(output)) = parsed_arguments.get_value("output") {
        config.output_directory = Some(output.to_string());
    };

    if let Some(argumentparser::Value::Word(path)) = parsed_arguments.get_value("wallpaper") {
        match config.wallpaper {
            Some(mut wal) => {
                wal.path = Some(path.to_string());
                config.wallpaper = Some(wal);
            }
            None => {
                let wal = Wallpaper {
                    enable: Some(true),
                    path: Some(path.to_string()),
                    command: None,
                };
                config.wallpaper = Some(wal);
            }
        };
    };

    if let Some(argumentparser::Value::Vector(command)) =
        parsed_arguments.get_value("wallpaper-command")
    {
        match config.wallpaper {
            Some(mut wal) => {
                wal.command = Some(command.clone());
                config.wallpaper = Some(wal);
            }
            None => {
                let wal = Wallpaper {
                    enable: Some(false),
                    path: None,
                    command: Some(command.clone()),
                };
                config.wallpaper = Some(wal);
            }
        };
    };

    // check if templates directory exists
    // it should have been created at the build time
    let mut templates_path = PathBuf::from("/usr")
        .join("share")
        .join("colstract")
        .join("templates");
    println!(
        "Default templates path: {}",
        templates_path.to_string_lossy()
    );
    match env::var("XDG_CONFIG_HOME") {
        Ok(val) => {
            let p = PathBuf::from(val).join("colstract").join("templates");
            if p.exists() {
                templates_path = p;
            };
        }
        Err(_) => {
            if let Ok(val) = env::var("HOME") {
                let p = PathBuf::from(val).join("colstract").join("templates");
                if p.exists() {
                    templates_path = p;
                };
            }
        }
    };
    println!("Using templates from: {}", templates_path.to_string_lossy());
}
