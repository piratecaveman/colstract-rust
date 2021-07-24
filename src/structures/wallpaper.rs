use serde::Deserialize;
use serde::Serialize;
use text_colorizer::Colorize;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash)]
pub struct Wallpaper {
    pub enable: Option<bool>,
    pub path: Option<String>,
    pub command: Option<Vec<String>>,
}

impl Wallpaper {
    pub fn apply_wallpaper(&self) {
        if let Some(true) = self.enable {
            if let Some(com) = &self.command {
                let mut process = std::process::Command::new(&com[0]);
                process.args(&com[1..]);
                println!("{}", format!("Running command {}", &com.join(" ")).green());
                let output = match process.output() {
                    Ok(o) => o,
                    Err(e) => {
                        eprintln!("{}", format!("Could not apply wallpaper: {}", e).red());
                        return;
                    }
                };
                if !output.stdout.is_empty() {
                    println!(
                        "{}",
                        format!("Stdout: {}", String::from_utf8_lossy(&output.stdout)).green()
                    );
                };
                if !output.stderr.is_empty() {
                    println!(
                        "{}",
                        format!("Stderr: {}", String::from_utf8_lossy(&output.stderr)).red()
                    );
                };
            }
        } else {
            eprintln!("{}", "wallpaper not enabled".yellow());
        };
    }

    pub fn set_path(&mut self, path: &str) {
        self.path = Some(path.to_string());
    }

    pub fn set_status(&mut self, status: bool) {
        self.enable = Some(status);
    }

    pub fn set_command(&mut self, command: Vec<String>) {
        self.command = Some(command);
    }
}
