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
