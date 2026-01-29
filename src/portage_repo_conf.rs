use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = "/etc/portage/repos.conf/tupoll-overlay.conf";
    
    let content = r#"[tupoll-overlay]
location = /var/db/repos/tupoll-overlay
auto-sync = no
priority = 1

"#;

    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;

    println!("Успешно записан.");
    Ok(())
}
