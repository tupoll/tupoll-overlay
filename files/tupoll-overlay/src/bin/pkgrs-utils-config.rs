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

pkg_postinst() {
   

    # 2. Выгружаем и активируем отдельный сервис сторожа
    /usr/bin/pkgrs-watcher-setup

    # 3. Стартуем сам сервис через rc-service
    if rc-service pkgrs-watcher status &>/dev/null; then
        rc-service pkgrs-watcher restart
    else
        rc-service pkgrs-watcher start
    fi
}
  "#), 
      ("dev-util/pkgrs-utils/files/pkgrs-utils/Cargo.toml", r#"[package]
name = "pkgrs-util"
version = "0.1.0"
edition = "2024"

[dependencies]
colored = "3.0.0"
libc = "0.2"

[[bin]]
name = "pam_sudo_checker"
path = "src/bin/pam_sudo_checker.rs"

[[bin]]
name = "pkgrs-watcher-daemon"
path = "src/bin/pkgrs-watcher-daemon.rs"

[[bin]]
name = "pkgrs-autologin-setup"
path = "src/bin/pinnacle_autologin_setup.rs"

[profile.release]
opt-level = 3
lto = true
panic = "abort"
debug = false  "#),
       ("dev-util/pkgrs-utils/files/pkgrs-utils/src/bin/pinnacle_autologin_setup.rs", r##"use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const INITTAB_PATH: &str = "/etc/inittab";

fn main() -> std::io::Result<()> {
    // 0. Строгая проверка на root
    #[cfg(unix)]
    if unsafe { libc::getuid() } != 0 {
        eprintln!(" \x1B[1;31m[!] Ошибка:\x1B[0m Требуются права root.");
        std::process::exit(1);
    }

    println!("--- [pinnacle-setup] Настройка сверхбыстрого автологина TTY1... ---");

    // 1. ПЕРЕИМЕНОВАНИЕ СЕРВИСА AGETTY В OPENRC (mv -f)
    let old_agetty = Path::new("/etc/init.d/agetty");
    let new_agetty = Path::new("/etc/init.d/agetty-autologin.tty1");

    if old_agetty.exists() {
        if new_agetty.exists() || new_agetty.symlink_metadata().is_ok() {
            let _ = fs::remove_file(new_agetty);
        }
        fs::rename(old_agetty, new_agetty)?;
        println!("[+] Сервис OpenRC переименован: agetty -> agetty-autologin.tty1");
    }

    // 2. МОДИФИКАЦИЯ /etc/inittab БЕЗ ПОВРЕЖДЕНИЯ ДРУГИХ СТРОК
    let inittab = Path::new(INITTAB_PATH);
    if !inittab.exists() {
        eprintln!("[-] Критическая ошибка: {} отсутствует в системе!", INITTAB_PATH);
        std::process::exit(1);
    }

    let file = File::open(inittab)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    let mut modified = false;

    // Эталонная строка автологина под вашего пользователя
    let target_autologin_line = "c1:12345:respawn:/sbin/agetty --autologin tupoll --noclear 38400 tty1 linux";

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();

        // Если нашли дефолтную строку без автологина — комментируем её знаком #
        if trimmed.contains("agetty") && trimmed.contains("tty1") && !trimmed.contains("--autologin") && !trimmed.starts_with('#') {
            lines.push(format!("#{}", line));
            modified = true;
            continue;
        }

        // Если строка с автологином уже прописана и активна — фиксируем это
        if trimmed == target_autologin_line {
            modified = true; // Изменения уже на месте или не требуются
        }

        lines.push(line);
    }

    // Если целевой строки с автологином вообще не было в файле — принудительно её дописываем
    if !lines.iter().any(|l| l.trim() == target_autologin_line) {
        lines.push(target_autologin_line.to_string());
        modified = true;
        println!("[+] В конец inittab добавлена боевая строка автологина.");
    }

    // Записываем обновленный inittab обратно на диск только если были реальные изменения
    if modified {
        // Делаем резервную копию перед записью на всякий случай
        let backup_path = inittab.with_extension("pkgrs_bak");
        let _ = fs::copy(inittab, backup_path);

        let mut out_file = File::create(inittab)?;
        for line in lines {
            writeln!(out_file, "{}", line)?;
        }
        println!("[🛡️] Файл {} успешно переписан под Monastery Mode.", INITTAB_PATH);
    } else {
        println!("[=] Файл {} уже корректно настроен.", INITTAB_PATH);
    }

    println!("\x1B[1;32m[SUCCESS]\x1B[0m Инициализация автологина завершена. Перезапустите init (telinit q).");
    Ok(())
}  "##),
       ("dev-util/pkgrs-utils/files/pkgrs-utils/src/bin/pkgrs-watcher-setup.rs", r##"use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

// Накатываем на скрипт инициализации OpenRC абсолютные права ядра
const INIT_SCRIPT_CONTENT: &str = r#"#!/sbin/openrc-run
# =========================================================================
# OpenRC Service for pkgrs ecosystem watcher (MAX PRIVILEGES MODE)
# =========================================================================

description="Фоновый сторож экосистемы pkgrs и SUID-прав"
pidfile="/run/pkgrs-watcher.pid"
command="/usr/bin/pkgrs-watcher-daemon"
command_background="yes"

# Строго фиксируем root-контекст для старта
command_user="root:root"

output_log="/var/log/pkgrs-watcher.log"
error_log="/var/log/pkgrs-watcher.log"

depend() {
    need localmount
    after bootmisc
}

start_pre() {
    # Проверяем наличие необходимых бинарников перед стартом
    if [ ! -x "/usr/bin/pam_sudo_checker" ]; then
        eerror "Критические утилиты pkgrs-utils не найдены или не исполняемы!"
        return 1
    fi
}
"#;

fn main() -> std::io::Result<()> {
    #[cfg(unix)]
    if unsafe { libc::getuid() } != 0 {
        eprintln!(" \x1B[1;31m[!] Ошибка:\x1B[0m Запускайте через sudo / под root.");
        std::process::exit(1);
    }

    let target_path = Path::new("/etc/init.d/pkgrs-watcher");

    println!("--- [pkgrs-watcher-setup] Развертывание службы OpenRC с правами ядра... ---");

    if target_path.exists() || target_path.symlink_metadata().is_ok() {
        let _ = fs::remove_file(target_path);
    }

    fs::write(target_path, INIT_SCRIPT_CONTENT)?;
    println!("[+] Создан файл службы OpenRC: {}", target_path.display());

    let mut perms = fs::metadata(target_path)?.permissions();
    perms.set_mode(0o0755);
    fs::set_permissions(target_path, perms)?;
    println!("[🛡️] Права 0755 на скрипт инициализации установлены.");

    let runlevel_dir = Path::new("/etc/runlevels/default");
    let runlevel_link = runlevel_dir.join("pkgrs-watcher");

    if runlevel_dir.exists() {
        if runlevel_link.exists() || runlevel_link.symlink_metadata().is_ok() {
            let _ = fs::remove_file(&runlevel_link);
        }
        
        #[cfg(unix)]
        if std::os::unix::fs::symlink(target_path, &runlevel_link).is_ok() {
            println!("[+] Сервис pkgrs-watcher успешно добавлен в автозагрузку default.");
        }
    }

    println!("\n\x1B[1;32m[SUCCESS]\x1B[0m Отдельный привилегированный сервис успешно развернут!");
    Ok(())
}  "##),
       ("dev-util/pkgrs-utils/files/pkgrs-utils/src/bin/pkgrs-watcher-daemon.rs", r##"use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

// Ловим и close_write, и moved_to (перемещение файлов пакетным менеджером)
const DAEMON_SCRIPT: &str = r#"#!/bin/sh
# =========================================================================
# SYSTEM STABILIZER DAEMON FOR PKGRS ECOSYSTEM
# DIRECT CONTROL MONITORING FOR /usr/bin (REACTIVE ATOMIC MODE)
# =========================================================================

WATCH_PATH="/usr/bin"
LOG_FILE="/var/log/pkgrs-watcher.log"

exec >> "$LOG_FILE" 2>&1

if ! command -v inotifywait >/dev/null 2>&1; then
    echo "[🛡️ WATCHER ERROR] inotifywait не найден в системе! Выход."
    exit 1
fi

echo "[>] Нативный сторож pkgrs успешно активирован прямо на /usr/bin."

# Добавляем событие moved_to, чтобы ловить атомарные замены пакетов!
inotifywait -q -m -e close_write -e moved_to "$WATCH_PATH" | while read -r directory event file; do
    
    sleep 1
        
        # 1. Возвращаем SUID-права на su/sudo через ваш чекер
        if [ -x "/usr/bin/pam_sudo_checker" ]; then
            echo "[>] Стабилизация SUID-битов авторизации..."
            /usr/bin/pam_sudo_checker
       fi
        echo "[🛡️ WATCHER] Система успешно приведена в Монастырский режим."
    
done
"#;

fn main() -> std::io::Result<()> {
    #[cfg(unix)]
    if unsafe { libc::getuid() } != 0 {
        eprintln!("Ошибка: Запускайте от root / через sudo.");
        std::process::exit(1);
    }

    let target_path = Path::new("/usr/bin/pkgrs-watcher-daemon");

    if target_path.exists() || target_path.symlink_metadata().is_ok() {
        let _ = fs::remove_file(target_path);
    }

    fs::write(target_path, DAEMON_SCRIPT)?;

    let mut perms = fs::metadata(target_path)?.permissions();
    perms.set_mode(0o0755);
    fs::set_permissions(target_path, perms)?;

    if std::env::args().any(|arg| arg == "--deploy") {
        println!("[+] Исполняемый компонент демона успешно развернут в /usr/bin/");
    } else {
        let c_str_path = std::ffi::CString::new(target_path.as_os_str().as_encoded_bytes()).unwrap();
        let c_str_arg = std::ffi::CString::new("pkgrs-watcher-daemon").unwrap();
        unsafe {
            libc::execl(c_str_path.as_ptr(), c_str_arg.as_ptr(), std::ptr::null::<*const libc::c_char>());
        }
    }

    Ok(())
}  "##),
       
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
