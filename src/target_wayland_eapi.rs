use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = "/var/db/repos/tupoll-overlay/profiles/targets/amd64/wayland/eapi";
    
    let content = r#"8

"#;

    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;

    println!("Успешно записан.");
    Ok(())
}
