use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = "/var/db/repos/tupoll-overlay/profiles/targets/amd64/wayland/use.force";
    
    let content = r#"wayland
lua_single_target_lua5-4
efistub

"#;

    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;

    println!("Успешно записан.");
    Ok(())
}
