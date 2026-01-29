use std::fs;

fn main() -> std::io::Result<()> {
    let path = "/var/db/repos/tupoll-overlay/metadata/repoman/repository.yaml";

    let content = r#"# Регистрационные данные оверлея tupoll-overlay
name: tupoll-overlay
description: "Personal Gentoo overlay for Rust-based system tools and GUI apps"
owner:
  - type: person
    email: "tupoll@example.com"
    name: "tupoll"
status: official
"#;

    // Гарантируем наличие папки
    if let Some(parent) = std::path::Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;

    println!("Файл repository.yaml успешно создан.");
    Ok(())
}
