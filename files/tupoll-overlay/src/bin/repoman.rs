use std::fs;
use std::path::Path; 

fn main() -> std::io::Result<()> {
    let base_path = Path::new("/var/db/repos/tupoll-overlay/");   
    let text_files = [
   ("sys-apps/repoman-rs/repoman-rs-9999.ebuild", r#"## Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

inherit cargo 

DESCRIPTION="Manifest metadata.xml for repo"
HOMEPAGE="https://github.com"
LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64"

SRC_URI=""
RESTRICT="fetch"

S="${WORKDIR}/${P}/repoman-rs"

RDEPEND=" "
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_unpack() {
    mkdir -p "${WORKDIR}/${P}" || die
    cp -Rp "${FILESDIR}/repoman-rs" "${WORKDIR}/${P}/" || die
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
	     	
}     
  "#), 
      ("sys-apps/repoman-rs/files/repoman/Cargo.toml", r#"[package]
name = "repoman-rs"
version = "0.1.0"
edition = "2024"

[dependencies]
walkdir = "2.4"
sha2 = "0.11"
regex = "1.10"
rayon = "1.8"
colored = "3.0.0"
clap = { version = "4.5", features = ["derive"] }
libc = "0.2.186"

[profile.release]
opt-level = 3
lto = true
panic = "abort"
debug = false "#),
  
       ("sys-apps/repoman-rs/files/repoman-rs/src/main.rs", r##"use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use sha2::{Sha512, Digest};
use regex::Regex;
use rayon::prelude::*;
use colored::Colorize;
use clap::Parser;
use libc;

const DEFAULT_OVERLAY: &str = "/var/db/repos/tupoll-overlay/";
const DISTFILES_DIR: &str = "/var/cache/distfiles";

#[derive(Parser, Debug)]
#[command(name = "repoman-rs", version, about = "Автономный менеджер манифестов Gentoo на Rust")]
struct Args {
    /// Путь к локальному оверлею
    #[arg(short, long)]
    path: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    
    println!("{}", "=== REPOMAN-RS: СИСТЕМА МАНИФЕСТАЦИИ ОВЕРЛЕЕВ ===".bold().green());
    
    let target_path_str = args.path.unwrap_or_else(|| {
    std::env::current_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| DEFAULT_OVERLAY.to_string())
});
    let overlay_path = Path::new(&target_path_str);
    
    if !overlay_path.exists() {
        eprintln!("{} Директория оверлея не найдена: {}", "[ОШИБКА]".red().bold(), overlay_path.display());
        std::process::exit(1);
    }

    println!("{} Сканирование дерева оверлея: {}...", "[1/2]".blue().bold(), overlay_path.display());
    
    let ebuilds: Vec<PathBuf> = WalkDir::new(overlay_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "ebuild"))
        .map(|e| e.into_path())
        .collect();

    if ebuilds.is_empty() {
        println!("{}", "Ебилды не найдены. Проверьте структуру категорий.".yellow());
        return Ok(());
    }

    println!("{} Найдено пакетов: {}. Запуск параллельной обработки...", "[2/2]".blue().bold(), ebuilds.len());
    
    let src_uri_regex = Regex::new(r#"(?m)^SRC_URI=["'](.+?)["']"#).unwrap();

    ebuilds.par_iter().for_each(|ebuild_path| {
        if let Err(e) = process_manifest(ebuild_path, &src_uri_regex) {
            eprintln!("{} Ошибка в {}: {}", "[ОШИБКА]".red().bold(), ebuild_path.display(), e);
        }
    });

    println!("{}", "--------------------------------------------------".green());
    println!("{} Все операции успешно завершены.", "[ГОТОВО]".green().bold());
    Ok(())
}

fn process_manifest(ebuild_path: &Path, src_uri_regex: &Regex) -> io::Result<()> {
    let package_dir = ebuild_path.parent().unwrap();
    let ebuild_file_name = ebuild_path.file_stem().unwrap().to_string_lossy();
    
    let last_dash = ebuild_file_name.rfind('-').unwrap_or(0);
    let p_var = &ebuild_file_name;                    
    let pv_var = &ebuild_file_name[last_dash + 1..];  

    let manifest_path = package_dir.join("Manifest");
    let metadata_path = package_dir.join("metadata.xml");
    let mut manifest_lines = Vec::new();

        // =========================================================================
    // 0. АВТОГЕНЕРАЦИЯ ИЛИ ОБНОВЛЕНИЕ METADATA.XML (Динамический Hostname)
    // =========================================================================
    // Динамически получаем имя хоста из ОС силами libc
    let hostname = unsafe {
        let mut buf = [0; 255];
        if libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) == 0 {
            std::ffi::CStr::from_ptr(buf.as_ptr() as *mut libc::c_char)
                .to_string_lossy()
                .into_owned()
        } else {
            "localhost".to_string()
        }
    };

    let metadata_template = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE pkgmetadata SYSTEM "https://gentoo.org">
<pkgmetadata>
	<maintainer type="person">
		<email>tupoll@{}.local</email>
		<name>tupoll</name>
	</maintainer>
</pkgmetadata>
"#,
        hostname
    );

    // Проверяем: если файла нет ИЛИ его содержимое отличается от актуального шаблона — пишем заново
    let should_write = if metadata_path.exists() {
        fs::read_to_string(&metadata_path).map(|c| c != metadata_template).unwrap_or(true)
    } else {
        true
    };

    if should_write {
        let mut meta_file = File::create(&metadata_path)?;
        meta_file.write_all(metadata_template.as_bytes())?;
        println!("{} Синхронизирован metadata.xml для {} (Host: {})", "[METADATA]".blue(), ebuild_file_name, hostname);
    }


    // =========================================================================
    // 1. СТАТИЧЕСКИЙ АНАЛИЗ ЛОКАЛЬНЫХ ФАЙЛОВ ПАКЕТА (EBUILD и AUX)
    // =========================================================================
    if let Ok(mut file) = File::open(ebuild_path) {
        let size = file.metadata()?.len();
        let mut hasher = Sha512::new();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        hasher.update(&buffer);
        let hash_result: String = hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect();
        
        let file_name = ebuild_path.file_name().unwrap().to_string_lossy();
        manifest_lines.push(format!("EBUILD {} {} SHA512 {}\n", file_name, size, hash_result));
    }

    let files_dir = package_dir.join("files");
    if files_dir.exists() && files_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(files_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() {
                    if let Ok(mut file) = File::open(&path) {
                        let size = file.metadata()?.len();
                        let mut hasher = Sha512::new();
                        let mut buffer = Vec::new();
                        file.read_to_end(&mut buffer)?;
                        hasher.update(&buffer);
                        let hash_result: String = hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect();
                        
                        let file_name = path.file_name().unwrap().to_string_lossy();
                        manifest_lines.push(format!("AUX {} {} SHA512 {}\n", file_name, size, hash_result));
                    }
                }
            }
        }
    }

    // =========================================================================
    // 2. АНАЛИЗ СЕТЕВЫХ АРХИВОВ (DIST) — Только если это НЕ 9999 пакет
    // =========================================================================
    if pv_var != "9999" {
        if let Ok(content) = fs::read_to_string(ebuild_path) {
            if let Some(caps) = src_uri_regex.captures(&content) {
                let mut raw_src_uri = caps.get(1).unwrap().as_str().to_string();

                raw_src_uri = raw_src_uri.replace("${P}", p_var).replace("$P", p_var);
                raw_src_uri = raw_src_uri.replace("${PV}", pv_var).replace("$PV", pv_var);

                let files: Vec<&str> = raw_src_uri
                    .split_whitespace()
                    .filter(|word| word.contains('/') && !word.starts_with('(') && !word.starts_with(')'))
                    .filter_map(|url| url.split('/').last())
                    .map(|f| f.trim_matches(|c| c == '"' || c == '\'' || c == ' '))
                    .collect();

                for filename in files {
                    let distfile_path = PathBuf::from(DISTFILES_DIR).join(filename);

                    if !distfile_path.exists() {
                        println!("{} Исходник {} отсутствует в distfiles. Пропуск.", "[ПРОПУСК]".yellow(), filename);
                        continue;
                    }

                    if let Ok(mut file) = File::open(&distfile_path) {
                        let size = file.metadata()?.len();
                        let mut hasher = Sha512::new();
                        let mut buffer = Vec::new();
                        file.read_to_end(&mut buffer)?;
                        hasher.update(&buffer);
                        let hash_result: String = hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect();

                        manifest_lines.push(format!("DIST {} {} SHA512 {}\n", filename, size, hash_result));
                    }
                }
            }
        }
    }

    // =========================================================================
    // 3. ЗАПИСЬ РЕЗУЛЬТАТА И ВЫВОД
    // =========================================================================
    if !manifest_lines.is_empty() {
        manifest_lines.sort();
        
        let mut f = File::create(&manifest_path)?;
        for line in manifest_lines {
            f.write_all(line.as_bytes())?;
        }
        println!("{} Обновлен Manifest для {}", "[ОК]".green(), ebuild_file_name);
    }

    Ok(())
}
"##), ];
      
 for (rel_path, content) in text_files {
        let full_path = base_path.join(rel_path.trim());
        if let Some(parent) = full_path.parent() {
            
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, content)?;
    }        
    println!("Структура sys-apps/repoman-rs успешно создана ✔️");
    Ok(())
}
