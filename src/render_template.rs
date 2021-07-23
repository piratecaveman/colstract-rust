use std::path::Path;

pub fn render_template(
    name: &str,
    path: &Path,
    output_dir: &Path,
    data: &mut serde_json::Map<String, serde_json::Value>,
) -> Result<(), handlebars::RenderError> {
    let mut handler = handlebars::Handlebars::new();
    handler.register_template_file(name, path)?;
    let mut output = std::path::PathBuf::from(output_dir);
    if !output.exists() {
        eprintln!("{} does not exist", output.display());
        eprintln!("Creating {}", output.display());
        std::fs::create_dir_all(&output)?;
    };
    output = output.join(name);
    let file = std::fs::File::create(output)?;
    handler.render_to_write(name, &data, file)?;
    Ok(())
}

#[test]
fn lets_test() {
    use std::path::PathBuf;
    let templates = [
        "colors",
        "colors.css",
        "colors.hs",
        "colors.json",
        "colors-kitty.conf",
        "colors-konsole.colorscheme",
        "colors-nqq.css",
        "colors-oomox",
        "colors-putty.reg",
        "colors-rofi-dark.rasi",
        "colors-rofi-light.rasi",
        "colors.scss",
        "colors.sh",
        "colors-speedcrunch.json",
        "colors.styl",
        "colors-sway",
        "colors-themer.js",
        "colors-tilix.json",
        "colors-tty.sh",
        "colors-vscode.json",
        "colors-wal-dmenu.h",
        "colors-wal-dwm.h",
        "colors-wal-st.h",
        "colors-wal-tabbed.h",
        "colors-wal.vim",
        "colors-waybar.css",
        "colors.Xresources",
        "colors.yml",
    ];
    let config = crate::Config::from(
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
    cursor = "#cac0a9""##,
    );
    let jason = config.to_json();
    let mut data_opt: serde_json::Value = serde_json::from_str(&jason).unwrap();
    let data = data_opt.as_object_mut().unwrap();
    for item in templates {
        render_template(
            item,
            &PathBuf::from(&format!("assets/templates/{}", item)),
            &PathBuf::from("/tmp/templates"),
            data,
        )
        .unwrap();
    }
}
