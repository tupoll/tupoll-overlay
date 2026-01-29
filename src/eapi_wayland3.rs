use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = "/var/db/repos/tupoll-overlay/profiles/default/linux/amd64/23.0/desktop/wayland/eapi";
    
    let content = r#"8

"#;

    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;

    println!("Успешно записан.");
    Ok(())
}
