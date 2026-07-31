use std::fs;
use std::path::Path; 

fn main() -> std::io::Result<()> {
    let base_path = Path::new("/var/db/repos/tupoll-overlay/");   
    let text_files = [
   ("dev-util/pkgrs-utils/pkgrs-utils-9999.ebuild", r#"## Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

inherit cargo 

DESCRIPTION="dev-rust package management assistant."
HOMEPAGE="https://github.com"
LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64 ~arm64"

SRC_URI=""
RESTRICT="fetch"

S="${WORKDIR}/${P}/pkgrs-utils"

RDEPEND=" "
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_unpack() {
    mkdir -p "${WORKDIR}/${P}" || die
    cp -Rp "${FILESDIR}/pkgrs-utils" "${WORKDIR}/${P}/" || die
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
      ("dev-util/pkgrs-utils/files/pkgrs-utils/Cargo.toml", r#"[package]
name = "pkgrs-util"
version = "0.1.0"
edition = "2024"

[dependencies]
colored = "3.0.0"

[[bin]]
name = "pam_sudo_checker"
path = "src/bin/pam_sudo_checker.rs"

[profile.release]
opt-level = 3
lto = true
panic = "abort"
debug = false  "#),
  
       ("dev-util/pkgrs-utils/files/pkgrs-utils/src/bin/pam_sudo_checker.rs", r#"use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

// Структура для описания PAM-файлов
struct PamFile {
    name: &'static str,
    content: &'static str,
}

fn main() {
    // 1. Настройка конфигураций для файлов в /etc/pam.d
    let pam_files = vec![
        PamFile {
            name: "su",
            content: "auth\t\tsufficient\tpam_rootok.so\nauth\t\trequired\tpam_wheel.so use_uid\nauth\t\tinclude\t\tsystem-auth\naccount\t\tinclude\t\tsystem-auth\npassword\tinclude\t\tsystem-auth\nsession\t\trequired\tpam_env.so\nsession\t\toptional\tpam_xauth.so\n",
        },
        PamFile {
            name: "sudo",
            content: "auth       include      system-auth\naccount    include      system-auth\nsession    include      system-auth\n",
        },
        PamFile {
            name: "sudo-i", // Исправлено: без пробела
            content: "auth       include      system-auth\naccount    include      system-auth\nsession    include      system-auth\n",
        },
        PamFile {
            name: "su-l", // Исправлено: без пробела
            content: "auth\t   include      su\naccount    include      su\npassword   include      su\nsession\t   optional pam_lastlog.so\nsession\t   include      su\n",
        },
    ];

    let pam_dir = Path::new("/etc/pam.d");

    for pam in pam_files {
        let file_path = pam_dir.join(pam.name);
        if !file_path.exists() {
            println!("Файл {:?} отсутствует. Создаю с дефолтным содержимым...", file_path);
            match File::create(&file_path) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(pam.content.as_bytes()) {
                        eprintln!("Ошибка при записи в {:?}: {}", file_path, e);
                    }
                    // Стандартные права для PAM файлов обычно 0644
                    let mut perms = file.metadata().unwrap().permissions();
                    perms.set_mode(0o0644);
                    let _ = std::fs::set_permissions(&file_path, perms);
                }
                Err(e) => eprintln!("Ошибка при создании файла {:?}: {}", file_path, e),
            }
        } else {
            println!("Файл {:?} уже существует.", file_path);
        }
    }

    // 2. Изменение владельца и прав для /usr/bin/sudo и /usr/bin/su
    let binaries = vec!["/usr/bin/sudo", "/usr/bin/su", "/usr/bin/pam_sudo_checker"];

    for bin in binaries {
        if Path::new(bin).exists() {
            println!("Настраиваю права для {}", bin);
            
            // chown root:root
            let chown_status = Command::new("chown")
                .args(["root:root", bin])
                .status();
            
            match chown_status {
                Ok(status) if status.success() => {}
                _ => eprintln!("Ошибка при выполнении chown для {}", bin),
            }

            // chmod 4755 (SUID)
            if let Ok(metadata) = std::fs::metadata(bin) {
                let mut perms = metadata.permissions();
                perms.set_mode(0o4755);
                if let Err(e) = std::fs::set_permissions(bin, perms) {
                    eprintln!("Ошибка при изменении прав (chmod) для {}: {}", bin, e);
                }
            }
        } else {
            eprintln!("Внимание: бинарник {} не найден в системе.", bin);
        }
    }

    // 3. Проверка и создание /etc/sudoers
    let sudoers_path = Path::new("/etc/sudoers");
    if !sudoers_path.exists() {
        println!("/etc/sudoers отсутствует. Создаю с дефолтным содержимым...");
        
        let content = "root ALL=(ALL:ALL) ALL\n\n# Включаем все кастомные конфиги из sudoers.d\n@includedir /etc/sudoers.d\n";
        
        let mut options = OpenOptions::new();
        options.create(true).write(true).truncate(true);
        
        match options.open(sudoers_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(content.as_bytes()) {
                    eprintln!("Ошибка при записи в /etc/sudoers: {}", e);
                }
                let mut perms = file.metadata().unwrap().permissions();
                perms.set_mode(0o0440); 
                if let Err(e) = std::fs::set_permissions(sudoers_path, perms) {
                    eprintln!("Ошибка при установке прав для /etc/sudoers: {}", e);
                }
            }
            Err(e) => eprintln!("Не удалось создать /etc/sudoers: {}", e),
        }
    } else {
        println!("/etc/sudoers уже существует.");
    }
}
"#),
 ];
      
 for (rel_path, content) in text_files {
        let full_path = base_path.join(rel_path.trim());
        if let Some(parent) = full_path.parent() {
            
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, content)?;
    }        
    println!("Структура утилит для Pinnacle OS успешно создана ✔️");
    Ok(())
}
