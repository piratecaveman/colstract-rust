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

    if parsed_arguments.get_value("input").is_some() {
        println!(
            "Getting config colors from {}",
            config.input.as_ref().unwrap()
        );
        let new_config = match std::fs::read_to_string(config.input.as_ref().unwrap()) {
            Ok(val) => Config::from(&val),
            Err(e) => {
                println!(
                    "Error: {}\n while reading config from {}",
                    e,
                    config.input.as_ref().unwrap()
                );
                println!("No changes will be made to the program");
                Config::default()
            }
        };
        if let Some(col) = new_config.colors {
            config.colors = Some(col);
        };
    } else if config.colors.is_some() {
        println!("Colors provided in the config will be used");
    };

    if config.colors.is_none() {
        println!("No colors provided");
        println!("Nothing to render, exiting");
        std::process::exit(0);
    }

    let mut list_of_templates = Vec::<PathBuf>::new();
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

    add_templates(&templates_path, &mut list_of_templates);

    match env::var("XDG_CONFIG_HOME") {
        Ok(val) => {
            let p = PathBuf::from(val).join("colstract").join("templates");
            if p.exists() {
                templates_path = p;
                add_templates(&templates_path, &mut list_of_templates);
            };
        }
        Err(_) => {
            if let Ok(val) = env::var("HOME") {
                let p = PathBuf::from(val)
                    .join(".config")
                    .join("colstract")
                    .join("templates");
                if p.exists() {
                    templates_path = p;
                    add_templates(&templates_path, &mut list_of_templates);
                };
            }
        }
    };
    let mut data: serde_json::Value = serde_json::from_str(&config.to_json()).unwrap();
    let data = data.as_object_mut().unwrap();
    println!("Using templates from: {}", templates_path.to_string_lossy());
    if list_of_templates.is_empty() {
        println!("No templates to render, exiting");
        std::process::exit(0);
    };
    for template in list_of_templates {
        let name = match template.file_name() {
            Some(nam) => nam.to_str().unwrap().to_string(),
            None => continue,
        };
        render_template(
            &name,
            template.to_str().unwrap(),
            &config.output_directory.as_ref().unwrap(),
            data,
        )
        .unwrap();
    }
    println!("Template rendering complete");
    if let Some(wal) = config.wallpaper {
        if let Some(true) = wal.enable {
            if let Some(command) = wal.command {
                crate::command::run_command(&command).unwrap();
            } else {
                println!("wallpaper enabled but no command provided. Aborting.");
            };
        } else {
            println!("wallpaper not enabled");
        };
    } else {
        println!("No settings for wallpaper, ignoring.")
    };
}

fn add_templates(templates_path: &std::path::Path, list_of_paths: &mut Vec<PathBuf>) {
    if templates_path.exists() && templates_path.is_dir() {
        let paths = std::fs::read_dir(templates_path).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.is_file() {
                list_of_paths.push(path);
            };
        }
    };
}
