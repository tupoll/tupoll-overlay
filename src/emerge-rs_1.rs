use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let base = "/var/db/repos/tupoll-overlay/dev-lang/emerge-rs";
    
    // Исходный код main.rs
    let main_rs_content = r#"use std::env;
use std::fs;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;

fn get_epython() -> String {
    if let Ok(val) = env::var("EPYTHON") { return val; }
    if let Ok(content) = fs::read_to_string("/etc/python-exec/python-exec.conf") {
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') { return trimmed.to_string(); }
        }
    }
    Command::new("python")
        .arg("-c")
        .arg("import sys; print(f'python{sys.version_info.major}.{sys.version_info.minor}')")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "python3.13".to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() { return; }
    let mut target = PathBuf::from(&args[0]);
    let mut prev_target: Option<PathBuf> = None;
    while let Ok(link) = fs::read_link(&target) {
        prev_target = Some(target.clone());
        target = if link.is_absolute() { link } else {
            target.parent().unwrap_or(Path::new(".")).join(link)
        };
    }
    let bin_name = target.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let final_script_name = if bin_name == "python-exec2" || bin_name == "python-exec2c" {
        prev_target.and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| {
                eprintln!("{}: python-exec2 is a wrapper, it must not be run directly.", args[0]);
                std::process::exit(127);
            })
    } else if bin_name == "emerge-rs" { "emerge".to_string() } else { bin_name.to_string() };
    let epython = get_epython();
    let final_path = Path::new("/usr/lib/python-exec").join(&epython).join(&final_script_name);
    if !final_path.exists() {
        eprintln!("{}: this Python implementation ({}) is not supported.", final_path.display(), epython);
        std::process::exit(127);
    }
    let mut cmd = Command::new(&epython);
    cmd.arg(&final_path);
    if args.len() > 1 { cmd.args(&args[1..]); }
    let err = cmd.exec();
    eprintln!("Failed to execute {}: {}", epython, err);
    std::process::exit(1);
}"#;

    // Минимальный Cargo.toml
    let cargo_toml_content = r#"[package]
name = "emerge-rs"
version = "1.0.0"
edition = "2021"

[dependencies]
libc = "0.2"
"#;

    // Список путей для записи
    let targets = [
        format!("{}/src/main.rs", base),
        format!("{}/files/src/main.rs", base),
    ];

    // Создаем папки и пишем main.rs
    for path in &targets {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, main_rs_content)?;
    }

    // Пишем Cargo.toml
    fs::write(format!("{}/files/Cargo.toml", base), cargo_toml_content)?;

    println!("Код и конфиги Cargo успешно развернуты в оверлее.");
    Ok(())
}
