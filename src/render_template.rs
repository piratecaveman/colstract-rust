pub fn render_template(
    name: &str,
    path: &str,
    output_dir: &str,
    data: &mut serde_json::Map<String, serde_json::Value>,
) -> Result<(), handlebars::RenderError> {
    let mut handler = handlebars::Handlebars::new();
    handler.register_template_file(name, path)?;
    let mut output = std::path::PathBuf::from(output_dir);
    if !output.exists() {
        std::fs::create_dir_all(&output)?;
    };
    output = output.join(name);
    let file = std::fs::File::create(output)?;
    handler.render_to_write(name, &data, file)?;
    Ok(())
}

#[test]
fn lets_do_this() {
    let conf = crate::config::Config::from(
        r##"input = "/home/user/.Xresources"
    output_directory = "/home/user/.cache/colstract"
    
    [wallpaper]
    enable = true
    path = "/home/user/Pictures/wall.png"
    command = ["feh", "--bg-fill", "/home/user/Pictures/wall.png"]
    "##,
    );
    let xres = xreader::Xresources::from(
        r##"! special
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

    *.colors16:     #fabeca"##,
    );
    let mut json_value: serde_json::Value = serde_json::from_str(&xres.to_json()).unwrap();
    dbg!(&json_value);
    let data = json_value.as_object_mut().unwrap();
    data.insert(
        "wallpaper".to_string(),
        serde_json::json!(conf.wallpaper.unwrap().path),
    );
    let templates = [
        "colors",
        "colors-kitty.conf",
        "colors-konsole.colorscheme",
        "colors-nqq.css",
        "colors-oomox",
        "colors-putty.reg",
        "colors-rofi-dark.rasi",
        "colors-rofi-light.rasi",
        "colors-speedcrunch.json",
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
        "colors.css",
        "colors.hs",
        "colors.json",
        "colors.scss",
        "colors.sh",
        "colors.styl",
        "colors.Xresources",
        "colors.yml",
    ];
    for item in templates {
        render_template(
            item,
            &format!("assets/templates/{}", item),
            "/tmp/try-harder",
            data,
        )
        .unwrap();
    }
}
