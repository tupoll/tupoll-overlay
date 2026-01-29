use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let base = "/var/db/repos/tupoll-overlay";
    let content = "test overlay\n";

    // Список всех файлов для создания
    let files = [
        "dev-lang/emerge-rs/emerge-rs-1.0.0.ebuild",
        "dev-lang/emerge-rs/src/main.rs",
        "dev-lang/emerge-rs/files/Cargo.toml",
        "dev-lang/emerge-rs/files/src/main.rs",
        "gui-wm/pinnacle/pinnacle-9999.ebuild",
        "licenses/Microsoft-Font",
        "media-video/soxbar/soxbar-9999.ebuild",
        "metadata/repoman/qa_data.yaml",
        "metadata/repoman/repository.yaml",
        "metadata/layout.conf",
        "profiles/profiles.desc",
        "profiles/repo_name",
        "profiles/default/linux/amd64/23.0/desktop/wayland/eapi",
        "profiles/default/linux/amd64/23.0/desktop/wayland/parent",
        "profiles/default/linux/amd64/23.0/desktop/wayland/wayland/eapi",
        "profiles/default/linux/amd64/23.0/desktop/wayland/wayland/parent",
        "profiles/targets/amd64/wayland/eapi",
        "profiles/targets/amd64/wayland/make.defaults",
        "profiles/targets/amd64/wayland/use.force",
        "profiles/targets/amd64/wayland/use.mask",
    ];

    for file_rel_path in files {
        let full_path = Path::new(base).join(file_rel_path);
        
        // Создаем родительские папки для файла
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Пишем контент
        fs::write(full_path, content)?;
    }

    // Создаем пустые папки, в которых нет файлов (updates и т.д.)
    fs::create_dir_all(format!("{}/profiles/updates", base))?;

    println!("Структура tupoll-overlay успешно создана.");
    Ok(())
}

