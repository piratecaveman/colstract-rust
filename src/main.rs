pub mod config;
pub mod render_template;
pub mod structures;
pub mod utils;

use std::env;
use std::path::Path;
use std::path::PathBuf;

use argumentparser::Argument;
use argumentparser::ParsedArguments;
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
    let parsed_arguments = parser.parse_arguments(&std::env::args().collect::<Vec<String>>());
    drop(parser);
    let mut config = create_config(&parsed_arguments);
    config = compose_config(&parsed_arguments, config);
    drop(parsed_arguments);
    let template_containers = [
        PathBuf::from("/usr")
            .join("share")
            .join("colstract")
            .join("templates"),
        get_config_home().join("colstract").join("templates"),
    ];
    let mut templates_paths = Vec::new();
    for path in template_containers {
        collect_templates(&mut templates_paths, &path);
    }

    if let Some(inp) = &(config.input) {
        if config.colors.is_none() {
            let new_config = Config::from(inp);
            let colors = new_config.colors;
            config.colors = colors;
        };
    };

    let output_directory = config.output_directory.clone().map_or_else(
        || {
            eprintln!(
                "{}",
                "Warning: no output dir configured, using default".yellow()
            );
            let home = match env::var("HOME") {
                Ok(val) => PathBuf::from(val),
                Err(_) => {
                    eprintln!("{}", "Could not find home directory".red());
                    eprintln!("{}", "Cannot continue, exiting".red());
                    std::process::exit(1);
                }
            };
            home.join(".cache").join("colstract")
        },
        PathBuf::from,
    );

    if !output_directory.exists() {
        eprintln!("{}", "Warning: output directory does not exist".yellow());
        match std::fs::create_dir_all(&output_directory) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", format!("Could not create dir: {}", e).red());
                std::process::exit(1);
            }
        };
    };

    let mut data = serde_json::from_str(&config.to_json()).unwrap();
    for item in templates_paths {
        let name = match item.file_name() {
            Some(va) => match va.to_str() {
                Some(v) => v,
                None => {
                    eprintln!("Unexpected file name: {:?}", va);
                    continue;
                }
            },
            None => {
                eprintln!("Unexpected file name: {:?}", item.file_name());
                continue;
            }
        };
        match render_template(name, &item, &output_directory, &mut data) {
            Ok(_) => {
                println!("{}", format!("rendered: {}", name).green());
            }
            Err(e) => {
                eprintln!(
                    "{}",
                    format!("An error occured rendering: {}\nError: {}", &name, e)
                );
                continue;
            }
        };
    }

    if let Some(wal) = &config.wallpaper {
        if wal.command.is_some() {
            if let Some(true) = &wal.enable {
                wal.apply_wallpaper();
            }
        }
    }
}

fn compose_config(parsed_arguments: &ParsedArguments, mut config: Config) -> Config {
    if let Some(argumentparser::Value::Word(inp)) = parsed_arguments.get_value("input") {
        config.input = Some(inp.clone());
        config.colors = None;
    };

    if let Some(argumentparser::Value::Word(out)) = parsed_arguments.get_value("output") {
        config.output_directory = Some(out.clone());
    };

    if let Some(argumentparser::Value::Word(walpath)) = parsed_arguments.get_value("wallpaper") {
        match config.wallpaper {
            Some(mut wal) => {
                wal.set_path(walpath);
                config.wallpaper = Some(wal);
            }
            None => {
                let wal = Wallpaper {
                    enable: Some(true),
                    path: Some(walpath.clone()),
                    command: None,
                };
                config.wallpaper = Some(wal);
            }
        };
    };

    if let Some(argumentparser::Value::Vector(com)) =
        parsed_arguments.get_value("wallpaper-command")
    {
        match config.wallpaper {
            Some(mut wal) => {
                wal.set_command(com.clone());
                config.wallpaper = Some(wal);
            }
            None => {
                let wal = Wallpaper {
                    enable: Some(false),
                    path: None,
                    command: Some(com.clone()),
                };
                config.wallpaper = Some(wal);
            }
        };
    };
    config
}

fn create_config(parsed_arguments: &ParsedArguments) -> Config {
    let config_home = get_config_home();
    let config_toml = config_home.join("colstract").join("config.toml");
    let config = match parsed_arguments.get_value("config") {
        Some(val) => {
            if let argumentparser::Value::Word(c) = val {
                Config::from(c)
            } else if config_toml.exists() {
                Config::from(std::fs::read_to_string(&config_toml).unwrap())
            } else {
                eprintln!("{}", "No config files found; using defaults".yellow());
                Config::default()
            }
        }
        None => {
            if config_toml.exists() {
                Config::from(std::fs::read_to_string(&config_toml).unwrap())
            } else {
                eprintln!("{}", "No config files found; using defaults".yellow());
                Config::default()
            }
        }
    };
    config
}

fn get_config_home() -> PathBuf {
    let home = match env::var("HOME") {
        Ok(val) => PathBuf::from(val),
        Err(_) => {
            eprintln!("{}", "Could not find home directory".red());
            eprintln!("{}", "Cannot continue, exiting".red());
            std::process::exit(1);
        }
    };
    match env::var("XDG_CONFIG_HOME") {
        Ok(val) => PathBuf::from(val),
        Err(_) => home.join(".config"),
    }
}

fn collect_templates(collection: &mut Vec<PathBuf>, path: &Path) {
    let files = match std::fs::read_dir(path) {
        Ok(val) => val,
        Err(e) => {
            eprintln!(
                "{}",
                format!("Colud not read directory: {}\nError: {}", path.display(), e).red()
            );
            return;
        }
    };
    for file in files {
        match file {
            Ok(val) => {
                if val.path().is_file() {
                    collection.push(val.path());
                };
            }
            Err(e) => {
                eprintln!(
                    "{}",
                    format!(
                        "Could not read file/dir from template directory\nError: {}",
                        e
                    )
                    .red()
                );
            }
        };
    }
}
