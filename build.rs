use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let home = env::var("HOME").expect("cargo:warning=Could not find $HOME");
    let templates_dir = PathBuf::from(home)
        .join(".config")
        .join("colstract")
        .join("templates");
    if !templates_dir.exists() {
        fs::create_dir_all(&templates_dir).expect("Could not create the required folders");
    };

    let assets = PathBuf::from("assets");
    let templates = assets.join("templates");
    let sample_config = assets.join("config.toml");

    fs::copy(
        &sample_config,
        &templates_dir.parent().unwrap().join("config.toml"),
    )
    .unwrap();

    let files = fs::read_dir(templates).expect("cargo:warning=Could not read from the assets dir");
    for file in files {
        let file = file.unwrap();
        let name = file.file_name().into_string().unwrap();
        fs::copy(file.path(), templates_dir.join(name)).unwrap();
    }
}
