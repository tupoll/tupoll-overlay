use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = "/var/db/repos/tupoll-overlay/profiles/repo_name";
    
    let content = r#"tupoll-overlay

"#;

    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;

    println!("Успешно записан.");
    Ok(())
}
