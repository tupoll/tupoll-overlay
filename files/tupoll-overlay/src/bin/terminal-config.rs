use std::fs;
use std::path::Path; 

fn main() -> std::io::Result<()> {
    let base_path = Path::new("/var/db/repos/tupoll-overlay/");   
    let text_files = [
   ("gui-apps/pinnacle-terminal/pinnacle-terminal-9999.ebuild", r#"# Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

inherit cargo desktop

DESCRIPTION="Terminal for pinnacle-wm"
HOMEPAGE="https://github.com"
LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64"

SRC_URI=""
RESTRICT="fetch"

S="${WORKDIR}/${P}/pinnacle-terminal"

RDEPEND="    
	gui-wm/pinnacle-gentoo	
"
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_unpack() {
    mkdir -p "${WORKDIR}/${P}" || die
    cp -Rp "${FILESDIR}/pinnacle-terminal" "${WORKDIR}/${P}/" || die   
	cargo_live_src_unpack
}

src_configure() {
	cargo_gen_config
}

src_compile() {
	cargo_src_compile
}

src_install() {
	cargo_src_install	
}   "#), 

      ("gui-apps/pinnacle-terminal/files/pinnacle-terminal/Cargo.toml", r#"[package]
name = "pinnacle-terminal"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
dirs = "5.0"
libc = "0.2"
  "#),
       ("gui-apps/pinnacle-terminal/files/pinnacle-terminal/src/main.rs", r#"use std::process::{Command};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::fs;

#[derive(serde::Deserialize, Default)]
struct VteConfig {
    font: Option<String>,
    scrollbar: Option<bool>,
    foreground: Option<String>, 
    background: Option<String>,
    transparent: Option<i32>,
    scrollback: Option<i32>,
}

fn main() {
    let home = std::env::var("HOME").unwrap();
    let config_dir = PathBuf::from(home).join(".config/vte4");
    let toml_path = config_dir.join("config.toml");

    let mut vte_args: Vec<String> = std::env::args().skip(1).collect();

    // Читаем настройки из файла
    if let Ok(content) = fs::read_to_string(&toml_path) {
        if let Ok(cfg) = toml::from_str::<VteConfig>(&content) {
            if let Some(f) = cfg.font { vte_args.push(format!("--font={}", f)); }
            if let Some(false) = cfg.scrollbar { vte_args.push("--no-scrollbar".into()); }
            // Здесь мы прокидываем БЕЛЫЙ цвет из конфига в систему
            if let Some(fg) = cfg.foreground { vte_args.push(format!("--foreground-color={}", fg)); }
            if let Some(bg) = cfg.background { vte_args.push(format!("--background-color={}", bg)); }
            if let Some(t) = cfg.transparent { vte_args.push(format!("--transparent={}", t)); }
            if let Some(n) = cfg.scrollback { vte_args.push(format!("--scrollback-lines={}", n)); }
        }
    }

    // Трюк: Прячем GnuTLS и запускаем Fish
    vte_args.push("--".into());
    vte_args.push("fish".into());
    vte_args.push("-c".into());
    vte_args.push("clear && exec fish".into());

    let mut cmd = Command::new("/usr/bin/vte-2.91");
    cmd.args(&vte_args);

    // Глушим ошибки GnuTLS наглухо
    unsafe {
        cmd.pre_exec(|| {
            let dev_null = libc::open("/dev/null\0".as_ptr() as *const libc::c_char, libc::O_WRONLY);
            libc::dup2(dev_null, libc::STDERR_FILENO);
            Ok(())
        });
    }

    cmd.spawn().expect("Failed").wait().ok();
}   
"#), 
      ("gui-apps/pinnacle-terminal/files/pinnacle-terminal/Pinnacle Terminal.desktop", r#"[Desktop Entry]
Type=Application
Name=VTE Pinnacle Terminal
Comment=Custom VTE terminal wrapper
Exec=/usr/bin/pinnacle-terminal
Icon=utilities-terminal
Terminal=false
Categories=System;TerminalEmulator;
"#),
];
      
 for (rel_path, content) in text_files {
        let full_path = base_path.join(rel_path.trim());
        if let Some(parent) = full_path.parent() {
            
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, content)?;
    }        
    println!("Структура giu-apps/pinnacle-terminal успешно создана ✔️");
    Ok(())
}
