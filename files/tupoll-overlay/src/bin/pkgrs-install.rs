use std::fs;
use std::path::Path; 

fn main() -> std::io::Result<()> {
    let base_path = Path::new("/var/db/repos/tupoll-overlay/");   
    let text_files = [
    ("sys-apps/pkgrs-man/pkgrs-man-9999.ebuild", r#"# Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

PYTHON_COMPAT=( python3_{12..14} )

inherit cargo  

DESCRIPTION="Man for PKGRS"
HOMEPAGE="https://github.com"
LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64"

SRC_URI=""

S="${WORKDIR}/${P}"

RDEPEND="sys-apps/pkgrs    
"
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"


src_unpack() {
    mkdir -p "${S}" || die    
    cp -Rp "${FILESDIR}"/* "${S}/" || die 
    cargo_live_src_unpack   
}


src_compile() {	
	cargo_src_compile 	 
}


src_install() {
	cargo_src_install
}

pkg_postinst() {   
    elog "Это инструкция к ипользованию pkgrs!"
}   "#), 
    ("sys-apps/pkgrs-man/files/Cargo.toml", r#"[package]
name = "pkgrs-man"
version = "0.1.0"
edition = "2024"

[dependencies]
colored = "3.1"

[[bin]]
name = "crab"
path = "src/bin/crab.rs"

[profile.dev]
opt-level = 3
lto = true
panic = "abort"
debug = false
incremental = true  "#),
    ("sys-apps/pkgrs-man/files/src/main.rs", r#"use std::{thread, io::{self, Read, Write}, time::Duration};
use std::process::Command;
use colored::Colorize;
// ==========================================
// ФУНКЦИЯ ТЕКСТА: Подсовываем сюда что угодно
// ==========================================
fn my_custom_logic() {
	println!("{}", "PKGRS — СИСТЕМА УПРАВЛЕНИЯ ПАКЕТАМИ (RUST + GENTOO)".bold().green());
	    println!("\n{}", "ОПИСАНИЕ:".yellow());
        println!("  pkgrs — это обертка над Portage, которая бесшовно объединяет");
        println!("  системные репозитории с экосистемой Cargo. При поиске");
        println!("  выводятся данные из локальной SQLite БД (emerge-cargo-base).");
        println!("\n{}", "КОМАНДЫ:".yellow());       
        println!(" Поиск в Portage и Cargo-base (crates.io):"); println!("{}", "-s, --search <query>".yellow()); 
        println!(" Поиск в  Cargo-base (crates.io):"); println!("{}", "-sc, --search <query>, --search-cargo  <query>".yellow()); 
        println!(" Установка (с автогенерацией ebuild для Rust):"); println!("{}", "-i, --install <pkg>".yellow());    
        println!(" Обновление дерева и кэша БД:"); println!("{}", "--sync".yellow());                 
        println!(" Прямой проброс любых флагов в emerge:"); println!("{}",  "<emerge args>".yellow());
        println!(" Обновление версий пакетов dev-rust:"); println!("{}",  "-u --emerge-update".yellow());           
        println!("\n{}", "Создание ебилдов для cargo-gentoo:".green());
        println!(" Для установки в /usr/lib64 '.so':"); println!("  {}", "-wl, --writer-lib: <pkg> <version>".green()); 
        println!(" Для установки в /usr/lib64 '.so':");println!("  {}", "-wle, --emerge-setup-lib: <pkg> <version>".green()); 
        println!(" Для установки в /usr/bin:");println!("  {}", "-wb, --writer-bin: <pkg> <version>".green()); 
        println!(" Для установки в /usr/bin:");println!("  {}", "-wbe, --emerge-setup-bin: <pkg> <version>".green());
        println!(" Для установки с github /usr/bin '.so'");
        println!(" В <url> введите полный адрес github:");println!("  {}", "-wg, --writer-git: <pkg> <url>".green());
        println!(" Для установки с github /usr/bin., '.so'");println!("  {}", "-wge, --emerge-setup-git: <pkg> <version>".green());
        println!(" Для установки с github /usr/bin '.so'");
        println!(" В <url> введите полный адрес github., использовать когда есть сборка для windows:");println!("  {}", "-wgw, --writer-git-windows: <pkg> <url>".green());
        println!("\n{}", "Пример установки katana где есть установка под windows:
        pkgrs -wgw kanata https://github.com/jtroo/kanata.git
        pkgrs -i --ask kanata".blue());
        println!(" C использованием .pc файлов:");println!("  {}", "-wpa, --writer-pc-all: <name> <url>".green());
        println!(" C использованием .pc файлов:");println!("  {}", "-wpl, --writer-pc-world: <pkg> <version>".green());
        println!("\n {}", "Между <pkg> и <version> не ставить дефис '-'!".red());
        println!("  {}", "eselect-rust-emerge".green()); 
        println!("  {}", "eselect-rust set <команда>  -- переключить на Rust-враппер".yellow());
        println!("  {}", "eselect-rust unset <команда>  -- вернуть стандарт Gentoo".yellow());
        println!("\n{}", "Пример: sudo eselect-rust set emerge".blue());
        println!("  {}", "eselect-python: Смена версии python".yellow());
        println!("\n{}", "Пример: 
        Доступные версии Python:
        1: python3.12
        2: python3.13
        3: python3.14

        Использование:
        sudo eselect-python set <номер>
        sudo eselect-python python set <номер>".blue());
        println!(" Аналог python-exec2 -
        умеет собирать и устанавливать ссылки на бинарные файлы rust
        исходки которых находятся в /usr/lib/python-exec/rust-src:");println!("  {}", "/usr/lib/python-exec/python-exec-2".yellow());
        println!("\n{}", "Пример:
        tupoll@tzfs ~ (master)> ls /usr/lib/python-exec/rust-src/emerge
        Cargo.toml  src/  target/
        sudo /usr/lib/python-exec/python-exec-2 emerge
        python-exec-rs: Запускайте через симлинк (например, emerge)".blue());
        println!("  {}", "rust-helper".yellow());
        println!("\n{}", "Доступные команды:
  rust-helper patch    - внедрить ускоритель
  rust-helper unpatch  - убрать все изменения из emerge
  rust-helper sync     - обновить базу данных".blue());
        
    println!("\x1B[1;32m[SYSTEM OK]\x1B[0m Инициализация мониторинга pkgrs...");
    
    // Выводим инфу о ядре (краб будет прямо над этой строкой)
    if let Ok(output) = Command::new("uname").arg("-snr").output() {
        let kernel = String::from_utf8_lossy(&output.stdout);
        println!("OS: {}", kernel.trim());  
    }
    // Проверяем твой RAM-диск из fstab
    println!("TMPFS: /var/tmp/wm [10M] ACTIVE");
    println!("----------------------------------------");
    println!("   Нажми \x1B[1;31m'Q'\x1B[0m для завершения сессии");
    println!("----------------------------------------");    
}

fn main() {
    // 1. ПРЕДУСТАНОВКА: Чистим старых крабов и прячем курсор
    let _ = Command::new("pkill").arg("-9").arg("-f").arg("crab").status();
    print!("\x1B[?25l");
    let _ = io::stdout().flush();

    // 2. ВЫВОДИМ ТЕКСТ (Твоя функция)
    my_custom_logic();

    // 3. ЗАПУСКАЕМ КРАБА (Он прыгнет на строку вверх от последней печати)
    let mut crab = Command::new("crab")
        .spawn()
        .expect("Бинарник /usr/bin/crab не найден");

    // 4. ПЕРЕХВАТ КЛАВИАТУРЫ (21 век: без Enter и без Эха)
    // stty -echo (не печатать вводимое), -icanon (читать по одному символу)
    let _ = Command::new("stty").arg("-echo").arg("-icanon").status();

    let mut stdin = io::stdin();
    let mut buffer = [0; 1];

    loop {
        // Читаем один байт
        if stdin.read(&mut buffer).is_ok() {
            let ch = buffer[0] as char;
            if ch == 'q' || ch == 'Q' {
                break; // Выход по нажатию Q
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    // 5. ЧИСТКА: Возвращаем всё как было
    let _ = crab.kill();
    let _ = Command::new("stty").arg("sane").status(); // Возвращаем режим терминала
    print!("\x1B[?25h\r\n\x1B[1;33m[EXIT]\x1B[0m Сессия завершена. Краб ушел спать.\n");
    let _ = io::stdout().flush();
}
 "#),
    ("sys-apps/pkgrs-man/files/src/bin/crab.rs", r#"use std::{thread, io::{self, Write}, time::Duration};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn manage_crab(alive: Arc<AtomicBool>) {
    let positions = [3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 30, 27, 24, 21, 18, 15, 12, 9, 6, 3];
    
    // Скрываем курсор, чтобы не мельтишил
    print!("\x1B[?25l");
    let _ = io::stdout().flush();

    while alive.load(Ordering::SeqCst) {
        for &pos in &positions {
            if !alive.load(Ordering::SeqCst) { break; }
            
            // ТОЛЬКО АНИМАЦИЯ: 
            // Сохраняем позицию -> Вверх -> Стираем строку -> Рисуем -> Назад
            print!("\x1B[s\x1B[1A\x1B[G\x1B[K{:>width$}\x1B[u", "🦀", width = pos);
            let _ = io::stdout().flush();
            thread::sleep(Duration::from_millis(400));
        }
    }
    
    // Возвращаем курсор перед выходом
    print!("\x1B[?25h");
    let _ = io::stdout().flush();
}

fn main() {
    let alive = Arc::new(AtomicBool::new(true));
    
    // Запускаем только цикл анимации и ВИСИМ
    // Никаких stdin, никаких read_line, никакой грязи в консоли
    manage_crab(alive);
} "#),
    //---------------------------------------------------
    //          PKGRS                                   |
    //---------------------------------------------------
    ("sys-apps/pkgrs/pkgrs-9999.ebuild", r#"# Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

# Обязательно для сборки Python-модуля через Maturin
DISTUTILS_USE_PEP517=maturin
PYTHON_COMPAT=( python3_{10..14} )

inherit cargo distutils-r1 

DESCRIPTION="Rust manager for Portage"
HOMEPAGE="https://github.com"
LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64"

SRC_URI=""

S="${WORKDIR}/${P}" # Упрощаем путь

RDEPEND="app-portage/tupoll-overlay"
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"


src_unpack() {
    mkdir -p "${S}" || die
    # Копируем всё содержимое папки files (Cargo.toml, src, etc) в корень сборки
    cp -Rp "${FILESDIR}"/* "${S}/" || die
    
    # Теперь cargo_live_src_unpack найдет Cargo.toml прямо в ${S}
    cargo_live_src_unpack   
}


src_compile() {
	export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1
	# Сначала собираем бинарный хелпер (обычный Rust)
	cargo_src_compile 
	 # Флаг для поддержки новых версий Python
    
	# Затем собираем Python-колесо. 
	# Очищаем переменные, чтобы maturin не пытался использовать настройки cargo eclass
	distutils-r1_src_compile
}


src_install() {
	cargo_src_install
    distutils-r1_src_install
    
    # Установка хелпера
    dobin "$(cargo_target_dir)/rust-helper"
    
    # Установка хука (он лежит прямо в ${S} после cp)
    exeinto /etc/pkgrs/postsync.d
    doexe "${S}/99rust-index"
    exeinto /etc/init.d
    doexe "${S}/rust-portage-watcher"
    exeinto /usr/sbin 
    doexe "${S}/rust-portage-watcher.sh" 
}

pkg_postinst() {
    # Авто-патчинг сразу после установки или обновления
    mv -f /usr/bin/python-exec-2 /usr/lib/python-exec/python-exec-2
    elog "Создан /usr/lib/python-exec/python-exec-2"
    mv -f /usr/lib/python-exec/python-exec2 /usr/lib/python-exec/python-exec2.bk
    cp -Rv /usr/lib/python-exec/python-exec-2 /usr/lib/python-exec/python-exec2
    rc-service rust-portage-watcher start
    rc-service rust-portage-watcher status
    elog "Сервис rust-portage-watcher запущен"
    elog "Создан /usr/lib/python-exec/python-exec2"
    /usr/bin/chown_pkgrs
    elog "Всё-права дали pkgrs /etc/pkgrs теперь наше всё"
    if [[ -x /usr/bin/rust-helper ]]; then
        /usr/bin/rust-helper patch
    fi
    elog "Rust-ускоритель активирован. Проверьте скорость: pkgrs -s <запрос>"
}

pkg_prerm() {
    # Чистый откат перед удалением файлов пакета
    if [[ -x /usr/bin/rust-helper ]]; then
        /usr/bin/rust-helper unpatch
    fi
    
}   "#), 
    ("sys-apps/pkgrs/files/Cargo.toml", r#"[package]
name = "pkgrs"
version = "0.1.0"
edition = "2024"

[dependencies]
colored = "2.2"
regex = "1.10"
libc = "0.2"
ureq = { version = "3.2", features = ["json"] }
serde_json = "1.0"
semver = "1.0"
pyo3 = { version = "0.28", features = ["extension-module", "abi3-py310"] }
rayon = "1.8"
walkdir = "2.4"
rusqlite = { version = "0.38", features = ["bundled", "modern_sqlite"] }

[[bin]]
name = "emerge-setup"
path = "src/bin/emerge-setup.rs"

[[bin]]
name = "pkgrs"
path = "src/bin/base.rs"

[[bin]]
name = "emerge-setup-bin"
path = "src/bin/emerge-setup-bin.rs"

[[bin]]
name = "emerge-setup-lib"
path = "src/bin/emerge-setup-lib.rs"

[[bin]]
name = "emerge-cargo-base"
path = "src/bin/emerge-cargo-base.rs"

[[bin]]
name = "eselect-python"
path = "src/bin/eselect-python.rs"

[[bin]]
name = "writer-git"
path = "src/bin/writer-git.rs"

[[bin]]
name = "writer-bin"
path = "src/bin/writer-bin.rs"

[[bin]]
name = "writer-lib"
path = "src/bin/writer-lib.rs"

[[bin]]
name = "emerge-update"
path = "src/bin/emerge-update.rs"

[[bin]]
name = "python-exec-2"
path = "src/bin/python-exec-rs.rs"

[lib]
name = "rust_portage"
crate-type = ["cdylib"]

[[bin]]
name = "rust-helper"
path = "src/bin/helper.rs"

[[bin]]
name = "eselect-rust-emerge"
path = "src/bin/eselect-rust-emerge.rs"

[[bin]]
name = "writer-pc-world"
path = "src/bin/writer_pc_world.rs"

[[bin]]
name = "emerge-rust-world"
path = "src/bin/emerge_rust_world.rs"

[[bin]]
name = "writer-pc-all"
path = "src/bin/writer-pc-all.rs"

[[bin]]
name = "chown_pkgrs"
path = "src/bin/chown_pkgrs.rs"

[[bin]]
name = "writer-git-windows"
path = "src/bin/writer-git-windows.rs"

[[bin]]
name = "helper-cargo-search"
path = "src/bin/helper-cargo-search.rs"	 

[profile.dev]
opt-level = 3
lto = true
panic = "abort"
debug = false
incremental = true 
 "#),
    ("sys-apps/pkgrs/files/src/main.rs", r#"use colored::Colorize;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args[0] == "-h" || args[0] == "--help" {
        println!("{}", "PKGRS — СИСТЕМА УПРАВЛЕНИЯ ПАКЕТАМИ (RUST + GENTOO)".bold().green());
        println!("\n{}", "КОМАНДЫ:".yellow());
        println!("  -s, --search <query>    Поиск в Portage и Cargo-base (crates.io)");
        println!("  -i, --install <pkg>     Установка (с автогенерацией ebuild для Rust)");
        println!("  --sync                  Обновление дерева и кэша БД");
        println!("  <emerge args>           Прямой проброс любых флагов в emerge");
        println!("\n{}", "Читайте pkgrs-man!".red());
        println!("\n{}", "ОПИСАНИЕ:".yellow());
        println!("  pkg — это обертка над Portage, которая бесшовно объединяет");
        println!("  системные репозитории с экосистемой Cargo. При поиске");
        println!("  выводятся данные из локальной SQLite БД (emerge-cargo-base).");
        return;
    }
}
 "#),
    ("sys-apps/pkgrs/files/src/lib.rs", r#"use pyo3::prelude::*;
// Добавляем PyListMethods сюда:
use pyo3::types::{PyList, PyListMethods}; 
use rusqlite::{params, Connection};
use std::fs;
use std::path::Path;

#[pyfunction]
fn power_search(py: Python<'_>, db_path: String, query: String) -> PyResult<Bound<'_, PyList>> {
    let conn = Connection::open(db_path).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    let mut stmt = conn.prepare("SELECT category, name, description FROM pkg_search WHERE pkg_search MATCH ?1 ORDER BY rank").unwrap();
    
    let rows = stmt.query_map(params![query], |row| {
        Ok(format!("\x1b[32m* {}/{}\x1b[0m\n      Description: {}\n", 
            row.get::<_, String>(0)?, 
            row.get::<_, String>(1)?, 
            row.get::<_, String>(2)?))
    }).unwrap();

    // ЗАМЕНА ЗДЕСЬ: создаем пустой список через new_bound
    let results = PyList::empty(py);
 
    
    for row in rows { 
        if let Ok(item) = row { 
            results.append(item)?; // Теперь .append() будет виден благодаря PyListMethods
        } 
    }
    Ok(results)
}

// ... остальной код (sync_index и pymodule) без изменений ...


#[pyfunction]
fn sync_index(db_path: String, repo_path: String) -> PyResult<()> {
    let conn = Connection::open(db_path).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    
    // Очищаем и создаем таблицу
    conn.execute("DROP TABLE IF EXISTS pkg_search", []).ok();
    conn.execute("CREATE VIRTUAL TABLE pkg_search USING fts5(category, name, description)", []).ok();

    let repo = Path::new(&repo_path);
    // Простая логика обхода (можно ускорить через walkdir)
    if let Ok(entries) = fs::read_dir(repo) {
        for entry in entries.flatten() {
            let cat = entry.file_name().to_string_lossy().into_owned();
            if cat.contains("-") { // Грубый фильтр категорий
                let cat_path = entry.path();
                if let Ok(pkgs) = fs::read_dir(cat_path) {
                    for pkg in pkgs.flatten() {
                        let name = pkg.file_name().to_string_lossy().into_owned();
                        // Здесь в идеале нужно парсить metadata.xml или ebuild для Description
                        // Для теста вставим заглушку или базовое имя
                        conn.execute(
                            "INSERT INTO pkg_search (category, name, description) VALUES (?1, ?2, ?3)",
                            params![cat, name, format!("Package {} in {}", name, cat)],
                        ).ok();
                    }
                }
            }
        }
    }
    Ok(())
}

#[pymodule]
fn rust_portage(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(power_search, m)?)?;
    m.add_function(wrap_pyfunction!(sync_index, m)?)?; // РЕГИСТРИРУЕМ ЗДЕСЬ
    Ok(())
}
 "#),
    ("sys-apps/pkgrs/files/99rust-index", r#"#!/bin/bash
# Проверка и применение патча + обновление БД
/usr/bin/rust-helper patch
/usr/bin/rust-helper sync "#),
    ("sys-apps/pkgrs/files/pyproject.toml", r#"[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "rust_portage"
version = "0.1.0"
description = "Rust accelerator"
requires-python = ">=3.13"

[tool.maturin]
# Это заставит maturin искать Cargo.toml в текущей директории S
manifest-path = "Cargo.toml"
# Важно: указываем, что это нативный модуль
module-name = "rust_portage" "#),
    ("sys-apps/pkgrs/files/rust-portage-watcher", r#"#!/sbin/openrc-run
description="Monitors Portage repos and updates Rust index"
command="/usr/sbin/rust-portage-watcher.sh"
command_background=true
pidfile="/run/${RC_SVCNAME}.pid"

# Убираем конфликтующий command_args_background
# OpenRC сам перенаправит выводы в /dev/null, если мы не укажем иное

depend() {
    need localmount
    after bootmisc
}

stop_post() {
    pkill -9 -f rust-portage-watcher.sh 2>/dev/null
    pkill -9 inotifywait 2>/dev/null
    return 0
} "#),
    ("sys-apps/pkgrs/files/rust-portage-watcher.sh", r#"#!/bin/bash
WATCH_PATH="/var/db/repos"
LOG_FILE="/var/log/rust-portage-watcher.log"

# Направляем всё в лог прямо внутри баша
exec > "$LOG_FILE" 2>&1

# Проверка inotifywait
if ! command -v inotifywait &> /dev/null; then
    echo "ERROR: inotifywait not found"
    exit 1
fi

inotifywait -q -m -r -e close_write,move,create,delete "$WATCH_PATH" | while read -r line; do
    sleep 10
    rust-helper sync
done
 "#),
    //---------------------------------------------------
    //          PKGRS-BIN                                |
    //---------------------------------------------------
        ("sys-apps/pkgrs/files/src/bin/base.rs", r##"use std::env;
use std::process::Command;
use std::process::exit;
use std::fs;
use colored::Colorize;
use std::io;

fn setup_rust_src() -> std::io::Result<()> {
    let base_path = "/usr/lib/python-exec/rust-src";
    let emerge_path = format!("{}/emerge", base_path);
    let emerge_src = format!("{}/src", emerge_path);

    // 1. Создаем дерево директорий
    println!("🚀 [Init] Создание дерева директорий в {}...", base_path);
    fs::create_dir_all(&emerge_src)?;

    // 2. Пишем /usr/lib/python-exec/rust-src/equery.rs
    let equery_code = r#"use std::env;
use std::process::Command;
use std::os::unix::process::CommandExt;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 && args[1] == "list" && args[2] == "*" {
        println!("🚀 [Fast-Equery] Список всех пакетов:");
        let _ = Command::new("pkg-check").status();
        return;
    }
    let original_equery = "/usr/lib/python-exec/python3.12/equery";
    let mut cmd = Command::new("python3.14");
    cmd.arg(original_equery);
    if args.len() > 1 { cmd.args(&args[1..]); }
    let _ = cmd.exec();
}"#;
    fs::write(format!("{}/equery.rs", base_path), equery_code)?;

    // 3. Пишем /usr/lib/python-exec/rust-src/emerge/Cargo.toml
    let cargo_toml = r#"[package]
name = "emerge"
version = "0.1.0"
edition = "2024"

[dependencies]
nix = { version = "0.27", features = ["signal", "process"] }
ctrlc = "3.4"
libc = "0.2"

[profile.dev]
opt-level = 3
lto = true
panic = "abort"
debug = false
incremental = true"#;
    fs::write(format!("{}/Cargo.toml", emerge_path), cargo_toml)?;

    // 4. Пишем /usr/lib/python-exec/rust-src/emerge/src/main.rs
    let emerge_main = r#"use std::env;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;

fn main() {
    env::set_var("PORTAGE_INTERNAL_CALLER", "1");
    let path = CString::new("/usr/lib/python-exec/python-exec-2").unwrap();
    let mut args: Vec<CString> = vec![
        CString::new("python-exec-2").unwrap(),
        CString::new("emerge").unwrap(),
    ];
    for arg in env::args_os().skip(1) {
        args.push(CString::new(arg.as_bytes()).unwrap());
    }
    let c_args: Vec<*const libc::c_char> = args.iter()
        .map(|arg| arg.as_ptr())
        .chain(std::iter::once(std::ptr::null()))
        .collect();
    unsafe {
        libc::execv(path.as_ptr(), c_args.as_ptr());
        libc::perror(CString::new("execv failed").unwrap().as_ptr());
    }
}"#;
    fs::write(format!("{}/main.rs", emerge_src), emerge_main)?;

    println!("✅ [Init] Структура rust-src готова.");
    Ok(())
}

fn show_cargo_tools() {
    println!("{}", "=== КУЗНИЦА PKGRS: ШАБЛОНЫ И ГЕНЕРАТОРЫ ===".bold().green());
    println!("\n1. СПРАВКА:          {}", "emerge-setup".cyan());
    println!("2. ШАБЛОНЫ (SETUP):  {}, {}, {}", "setup-bin".cyan(), "setup-lib".cyan(), "setup-git".cyan());
    println!("3. ГЕНЕРАТОРЫ (WR):  {}, {}, {}", "writer-bin".cyan(), "writer-lib".cyan(), "writer-git".cyan());
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args[0] == "-h" || args[0] == "--help" {
        println!("{}", "PKG — СИСТЕМА УПРАВЛЕНИЯ ПАКЕТАМИ (RUST + GENTOO)".bold().green());
        println!("\n{}", "КОМАНДЫ:".yellow());
        println!("  -s, --search <query>    Поиск в Portage и Cargo-base (crates.io)");
        println!("  -i, --install <pkg>     Установка (с автогенерацией ebuild для Rust)");
        println!("  --sync                  Обновление дерева и кэша БД");
        println!("  <emerge args>           Прямой проброс любых флагов в emerge");
        println!("\n{}", "Читайте pkgrs-man!".red());
        println!("\n{}", "ОПИСАНИЕ:".yellow());
        println!("  pkg — это обертка над Portage, которая бесшовно объединяет");
        println!("  системные репозитории с экосистемой Cargo. При поиске");
        println!("  выводятся данные из локальной SQLite БД (emerge-cargo-base).");
        return;
    }

    // 1. Наш внутренний флаг -ic (emerge не участвует)
    if args.iter().any(|a| a == "-ic") {
        show_cargo_tools();
        exit(0);
    }

    // 2. Команда -i (тупо запускает emerge, отрезая сам флаг -i)
    if args[0] == "-i" || args[0] == "--install" {
        let status = Command::new("emerge")
            .args(&args[1..]) // Пробрасываем всё, что после -i (например: -av pkg)
            .status()
            .unwrap_or_else(|_| exit(1));
        exit(status.code().unwrap_or(0));
    }

    // 3. Поиск -s (комбинированный)
    if args[0] == "-s" || args[0] == "--search" {
        let query = args.get(1).expect("Укажите запрос");
        let _ = Command::new("emerge").arg("-s").arg(query).status();
        let _ = Command::new("emerge-cargo-base").arg(query).status();
        exit(0);
    }
    
    // 3.1 Поиск -sc (Поиск в базе Cargo)
    if args[0] == "-sc" || args[0] == "--search-cargo" {
        let query = args.get(1).expect("Укажите запрос");
        let _ = Command::new("emerge-cargo-base").arg(query).status();
        exit(0);
    }
    
    // 3.2 Установка пакета имеющего на выходе .so: -wl (Из базы Cargo)
    if args[0] == "-wl" || args[0]  == "--writer-lib" {
        let status = Command::new("writer-lib")
            .args(&args[1..]) 
            .status()
            .unwrap_or_else(|_| exit(1));
        exit(status.code().unwrap_or(0));
    }
    
     // 3.3 Установка пакета имеющего на выходе .so: -wle (Другая версия)
    if args[0] == "-wle" || args[0]  == "--emerge-setup-lib" {
        let status = Command::new("emerge-setup-lib")
            .args(&args[1..]) 
            .status()
            .unwrap_or_else(|_| exit(1));
        exit(status.code().unwrap_or(0));
    }
     // 3.4 Установка пакета имеющего на выходе bin: -wb 
    if args[0] == "-wb" || args[0]  == "--writer-bin" {
        let status = Command::new("writer-bin")
            .args(&args[1..]) 
            .status()
            .unwrap_or_else(|_| exit(1));
        exit(status.code().unwrap_or(0));
    }
    // 3.5 Установка пакета имеющего на выходе bin: -wbe(Другая версия) 
    if args[0] == "-wbe" || args[0]  == "--emerge-setup-bin" {
        let status = Command::new("emerge-setup-bin")
            .args(&args[1..]) 
            .status()
            .unwrap_or_else(|_| exit(1));
        exit(status.code().unwrap_or(0));
    }
    // 3.6 Установка пакета имеющего на выходе bin .so c git: -wg (Вписывается git-адрес) 
    if args[0] == "-wg" || args[0]  == "--writer-git" {
        let status = Command::new("writer-git")
            .args(&args[1..]) 
            .status()
            .unwrap_or_else(|_| exit(1));
        exit(status.code().unwrap_or(0));
    }
    // 3.7 Установка пакета имеющего на выходе bin .so c git: -wge 
    if args[0] == "-wge" || args[0]  == "--emerge-setup-git" {
        let status = Command::new("emerge-setup-git")
            .args(&args[1..]) 
            .status()
            .unwrap_or_else(|_| exit(1));
        exit(status.code().unwrap_or(0));
    }
    // 3.7 Установка пакета имеющего на выходе bin .so c git: -wgw (Вписывается git-адрес) 
    if args[0] == "-wgw" || args[0]  == "--writer-git-windows" {
        let status = Command::new("writer-git-windows")
            .args(&args[1..]) 
            .status()
            .unwrap_or_else(|_| exit(1));
        exit(status.code().unwrap_or(0));
    }
    // 3.8 Установка пакета имеющего на выходе bin .so c git: -wpa (Вписывается git-адрес) 
    if args[0] == "-wpa" || args[0]  == "--writer-pc-all" {
        let status = Command::new("writer-git-windows")
            .args(&args[1..]) 
            .status()
            .unwrap_or_else(|_| exit(1));
        exit(status.code().unwrap_or(0));
    }
    // 3.9 Установка пакета имеющего на выходе bin .so c git: -wpl
    if args[0] == "-wpl" || args[0]  == "--writer-pc-world" {
        let status = Command::new("writer-pc-world")
            .args(&args[1..]) 
            .status()
            .unwrap_or_else(|_| exit(1));
        exit(status.code().unwrap_or(0));
    }
    // 3.91 Обновление пакетов dev-rust: -u
    if args[0] == "-u" || args[0]  == "--emerge-update" {
        let status = Command::new("emerge-update")
            .args(&args[1..]) 
            .status()
            .unwrap_or_else(|_| exit(1));
        exit(status.code().unwrap_or(0));
    }
    // 4. Все остальное — тоже в emerge напрямую
    let status = Command::new("emerge")
        .args(&args)
        .status()
        .unwrap_or_else(|_| exit(1));
    exit(status.code().unwrap_or(0));
}
   fn finalize_system_takeover() -> io::Result<()> {
    let rust_src_base = "/usr/lib/python-exec/rust-src";
    
    // 1. КОМПИЛЯЦИЯ EQUERY (Напрямую через rustc)
    println!("{} {}", ">>>".green(), "Компиляция equery из исходников rust-src...".bold());
    let equery_src = format!("{}/equery.rs", rust_src_base);
    
    let status_equery = Command::new("rustc")
        .args([
            "-C", "opt-level=z",
            "-C", "panic=abort",
            "-C", "strip=symbols",
            &equery_src,
            "-o", "/usr/bin/equery" // СРАЗУ В /usr/bin (Взрослый путь)
        ])
        .status()?;

    if status_equery.success() {
        println!("  [OK] /usr/bin/equery теперь на Rust.");
    }

    // 2. СБОРКА EMERGE (Через Cargo в его директории)
    println!("{} {}", ">>>".green(), "Сборка emerge-rs через Cargo...".bold());
    let emerge_dir = format!("{}/emerge", rust_src_base);
    
    let status_cargo = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(&emerge_dir)
        .status()?;

    if status_cargo.success() {
        // ПЕРЕНОС EMERGE В СИСТЕМУ
        // Мы кладем его в /usr/bin/emerge, вытесняя старый симлинк/скрипт
        fs::copy(
            format!("{}/target/release/emerge", emerge_dir),
            "/usr/bin/emerge"
        )?;
        println!("  [OK] /usr/bin/emerge теперь на Rust (прокси к python-exec-2).");
    }

    Ok(())
}
 "##),
    ("sys-apps/pkgrs/files/src/bin/chown_pkgrs.rs", r##"use std::{fs, io::{self}, process::Command, path::Path};
use colored::*;

fn main() -> io::Result<()> {
    // 1. СОЗДАНИЕ ОРДЕНА (Юзер и Группа)
    println!("{} {}", ">>>".green(), "Инициализация ордена pkgrs...".bold());
    
    // Создаем группу, если её нет (пусть система сама выберет GID)
    let _ = Command::new("groupadd").arg("-r").arg("pkg").status();
    
    // Создаем пользователя pkg (системный, группы root и portage)
    let status = Command::new("useradd")
        .args(["-r", "-g", "pkgrs", "-G", "root,portage", "-s", "/bin/false", "-M", "pkgrs"])
        .status()?;

    if !status.success() {
        println!("{} {}", "!!!".yellow(), "Пользователь pkg уже существует или создан автоматически.");
    }

    // 2. ВЕЛИКОЕ ПЕРЕСЕЛЕНИЕ (Конфиги)
    let old_path = "/etc/portage";
    let new_path = "/etc/pkgrs";

    if !Path::new(new_path).exists() {
        println!("{} Перенос {} -> {}", ">>>".green(), old_path.cyan(), new_path.cyan());
        // Копируем со всеми атрибутами
        Command::new("cp").args(["-a", old_path, new_path]).status()?;
    }

    // 3. ПРАВА ДОСТУПА (Рекурсивно)
    println!("{} Установка прав доступа на монастырь...", ">>>".green());
    Command::new("chown").args(["-R", "pkg:pkg", new_path]).status()?;
    Command::new("chmod").args(["-R", "775", new_path]).status()?;

    // 4. СИМЛИНК (Главный мост)
    if !fs::symlink_metadata(old_path)?.file_type().is_symlink() {
        println!("{} Создание симлинка {} -> {}", ">>>".magenta(), old_path, new_path);
        let backup = format!("{}.orig", old_path);
        fs::rename(old_path, backup)?;
        std::os::unix::fs::symlink(new_path, old_path)?;
    }

    // 5. PACKAGE.PROVIDED (Блокировка python-exec)
    let prov_dir = format!("{}/profile", new_path);
    let prov_file = format!("{}/package.provided", prov_dir);
    fs::create_dir_all(prov_dir)?;

    let entry = "dev-lang/python-exec-2.4.10";
    let mut content = fs::read_to_string(&prov_file).unwrap_or_default();

    if !content.contains(entry) {
        content.push_str(&format!("\n{}\n", entry));
    } else {
        content = content.replace(&format!("#{}", entry), entry);
    }
    fs::write(prov_file, content)?;

    println!("\n{} Система под полным контролем pkg.", "Аминь!".green().bold());
    Ok(())
}
 "##),
    ("sys-apps/pkgrs/files/src/bin/emerge-cargo-base.rs", r##"use rusqlite::{params, Connection, Result};
use std::env;
use std::process::{Command, exit};
use colored::*;
use regex::Regex;

const DB_PATH: &str = "/var/cache/portage/cargo-base.db";

fn init_db() -> Result<Connection> {
    let conn = Connection::open(DB_PATH)?;
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA busy_timeout = 5000;"
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS packages (
            name TEXT PRIMARY KEY,
            description TEXT
        )",
        [],
    )?;
    Ok(conn)
}

fn print_gentoo_style(name: &str, desc: &str, source: &str) {
    println!("*  {}", name.bold().green());
    // Используем format! вместо сложения через +
    let source_info = format!("[from {}]", source.blue());
    println!("      {} {}", "Latest version available:".cyan(), source_info.white());
    println!("      {} {}\n", "Description:".yellow(), desc.white());
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    if unsafe { libc::getuid() } != 0 {
        eprintln!("{}: {}", " Permission denied".red().bold(), "запустите через sudo.");
        exit(1);
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <query>", "emerge-cargo-base".green());
        exit(1);
    }

    let query = &args[1];
    let mut conn = init_db().unwrap_or_else(|e| {
        eprintln!("Error opening DB: {}", e);
        exit(1);
    });

    let mut found = false;

    // Ограничиваем область видимости stmt, чтобы он "умер" до начала транзакции
    {
        let mut stmt = conn.prepare("SELECT name, description FROM packages WHERE name LIKE ?")?;
        let rows = stmt.query_map(params![format!("%{}%", query)], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        for package in rows {
            let (name, desc) = package?;
            print_gentoo_style(&name, &desc, "cache");
            found = true;
        }
    }

    if !found {
        println!("{} {}", ">>>".green(), "Searching crates.io...".bold());
        
        let output = Command::new("cargo")
            .args(["search", query, "--limit", "20", "--color", "never"])
            .output()?;

        if !output.status.success() {
            eprintln!("Error: cargo search failed.");
            exit(1);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let re = Regex::new(r#"^(\S+)\s*=\s*"([^"]+)"\s*#\s*(.*)$"#)?;
        
        // Теперь conn свободен, можно открывать транзакцию
        let tx = conn.transaction()?;
        for line in stdout.lines() {
            if let Some(caps) = re.captures(line) {
                let p_name = &caps[1];
                let p_ver = &caps[2];
                let p_desc = &caps[3];
                
                let full_pkg = format!("dev-rust/{}-{}", p_name, p_ver);
                print_gentoo_style(&full_pkg, p_desc, "crates.io");

                tx.execute(
                    "INSERT OR REPLACE INTO packages (name, description) VALUES (?1, ?2)",
                    params![full_pkg, p_desc],
                )?;
                found = true;
            }
        }
        tx.commit()?;
    }

    if !found {
        println!("No packages found for '{}'", query.red());
    }

    Ok(())
}
 "##),
    ("sys-apps/pkgrs/files/src/bin/emerge_rust_world.rs", r#"use std::{fs, io::{self, Write}, process::Command};
use colored::*;
use serde_json::{Value, json};

const OVERLAY_PATH: &str = "/var/db/repos/tupoll-overlay/dev-rust";
const TMP_FILE: &str = "/tmp/rust_updates.json";

fn get_remote_version(name: &str) -> Option<String> {
    // ВНИМАНИЕ: Запрос идет без sudo!
    let output = Command::new("curl")
        .args([
            "-s", "-L", "-4",
            "-A", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36",
            &format!("https://crates.io{}", name)
        ])
        .output().ok()?;

    let json: Value = serde_json::from_slice(&output.stdout).ok()?;
    json["crate"]["max_version"].as_str().map(|s| s.to_string())
}

fn main() -> io::Result<()> {
    let uid = unsafe { libc::getuid() };

    if uid != 0 {
        // --- ФАЗА 1: ОБЫЧНЫЙ ПОЛЬЗОВАТЕЛЬ (СЕТЬ) ---
        println!("{}", ">>> Фаза 1: Сбор данных из сети...".cyan());
        let mut results = serde_json::Map::new();
        let entries = fs::read_dir(OVERLAY_PATH)?;

        for entry in entries {
            let name = entry?.file_name().into_string().unwrap();
            print!("  Проверка {}... ", name);
            io::stdout().flush()?;
            
            if let Some(v) = get_remote_version(&name) {
                println!("{}", v.green());
                results.insert(name, json!(v));
            } else {
                println!("{}", "Ошибка".red());
            }
        }
        fs::write(TMP_FILE, serde_json::to_string(&results)?)?;
        println!("\n{} Данные сохранены. Теперь запустите: {}", "Успех!".green(), "sudo !!".bold());
        return Ok(());
    }

    // --- ФАЗА 2: ROOT (УСТАНОВКА) ---
    println!("{}", ">>> Фаза 2: Обновление системы...".magenta());
    let data_str = fs::read_to_string(TMP_FILE).expect("Сначала запустите без sudo!");
    let updates: Value = serde_json::from_str(&data_str).unwrap();

    for (name, remote_v) in updates.as_object().unwrap() {
        let remote_v = remote_v.as_str().unwrap();
        // Тут твоя логика сравнения версий из ebuild и вызов emerge-setup-bin/all
        println!("* Пакет {}: новая версия {}", name, remote_v);
        // ... (вызов инструментов) ...
    }

    Ok(())
}
 "#),
    ("sys-apps/pkgrs/files/src/bin/emerge-setup-bin.rs", r##"use std::fs;
use std::process::Command;
use std::io::Write;
use colored::*;
use std::collections::HashSet;

const OVERLAY_ROOT: &str = "/var/db/repos/tupoll-overlay";
const CATEGORY: &str = "dev-rust";

fn get_system_libs(work_dir: &str) -> String {
    let mut libs = HashSet::new();
    libs.insert("virtual/pkgconfig".to_string());
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--offline"])
        .current_dir(work_dir).output();

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let common_mappings = [
            ("wayland", "dev-libs/wayland"), ("gtk", "gui-libs/gtk"),
            ("glib", "dev-libs/glib"), ("dbus", "sys-apps/dbus"),
            ("openssl", "dev-libs/openssl"), ("sqlite", "dev-db/sqlite"),
        ];
        for (key, atom) in common_mappings {
            if stdout.contains(&format!("{}-sys", key)) { libs.insert(atom.to_string()); }
        }
    }
    libs.into_iter().collect::<Vec<_>>().join("\n\t")
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <pkg> <version>", "pkgrs -wbe".cyan());
        return Ok(());
    }
    let name = &args[1];
    let version = &args[2];

    let pkg_dir = format!("{}/{}/{}", OVERLAY_ROOT, CATEGORY, name);
    let sources_dir = format!("{}/sources-{}", pkg_dir, version);
    let work_dir = format!("{}/{}-{}", sources_dir, name, version);
    
    fs::create_dir_all(&sources_dir)?;

    println!("{} Получение исходников {} v{}...", ">>>".green(), name, version);
    
    // Пытаемся скачать через cargo download
    let dl = Command::new("cargo").args(["download", "-x", &format!("{}=={}", name, version)]).current_dir(&sources_dir).status();

    if dl.is_err() || !dl.unwrap().success() {
        println!("{} Используем встроенный метод получения...", ">>>".yellow());
        // Фолбэк: создаем проект, скачиваем крэйт и перемещаем его исходники
        let tmp_path = format!("{}/tmp_dl", sources_dir);
        let _ = fs::remove_dir_all(&tmp_path);
        Command::new("cargo").args(["new", "tmp_dl"]).current_dir(&sources_dir).output()?;
        Command::new("cargo").args(["add", &format!("{}@{}", name, version)]).current_dir(&tmp_path).output()?;
        
        // Используем vendor, чтобы вытащить исходники самого пакета
        Command::new("cargo").arg("vendor").current_dir(&tmp_path).output()?;
        
        // Находим папку пакета в vendor и перемещаем её в work_dir
        let vendor_path = format!("{}/vendor", tmp_path);
        let pkg_folder = fs::read_dir(&vendor_path)?
            .filter_map(|e| e.ok())
            .find(|e| e.file_name().to_string_lossy().starts_with(name))
            .expect("Пакет не найден в загрузках")
            .path();
            
        fs::rename(pkg_folder, &work_dir)?;
        let _ = fs::remove_dir_all(&tmp_path);
    }

    println!("{} Вендоринг зависимостей...", ">>>".green());
    Command::new("cargo").arg("vendor").current_dir(&work_dir).output()?;

    let rdepend = get_system_libs(&work_dir);
    let ebuild_path = format!("{}/{}-{}.ebuild", pkg_dir, name, version);
    let mut f = fs::File::create(&ebuild_path)?;
    writeln!(f, "#emerge-setup-bin")?;
    writeln!(f, "EAPI=8\ninherit cargo\n\nDESCRIPTION=\"{}\"\nHOMEPAGE=\"https://crates.io/{}\"\nSRC_URI=\"\"\nS=\"${{WORKDIR}}/{}-{}\"", name, name, name, version)?;
    writeln!(f, "LICENSE=\"MIT\"\nSLOT=\"0\"\nKEYWORDS=\"~amd64\"\nRDEPEND=\"\n\t{}\"\nDEPEND=\"${{RDEPEND}}\"\nBDEPEND=\"virtual/pkgconfig\"", rdepend)?;

    writeln!(f, "\nsrc_unpack() {{ cp -Rp \"{}/\"* \"${{WORKDIR}}/\" || die; }}", sources_dir)?;
    
    writeln!(f, "\nsrc_compile() {{
    export CARGO_HOME=\"${{T}}/cargo_home\"
    mkdir -p .cargo || die
    echo \"[source.crates-io]\nreplace-with = 'vendored-sources'\n[source.vendored-sources]\ndirectory = 'vendor'\" > .cargo/config.toml || die
    RUSTFLAGS='-C target-cpu=native' cargo build --release --offline --all-features || die
}}")?;

    writeln!(f, "\nsrc_install() {{
    find target/release -maxdepth 1 -executable -type f ! -name \"*.so\" -exec dobin {{}} +
}}")?;

    println!("{} Финализация...", ">>>".green());
    let _ = Command::new("chown").args(["-R", "portage:portage", &pkg_dir]).status();
    let _ = Command::new("chmod").args(["-R", "go+rX", &pkg_dir]).status();
    Command::new("ebuild").arg(&ebuild_path).arg("digest").status()?;

    let ak_dir = "/etc/portage/package.accept_keywords";
    fs::create_dir_all(ak_dir).ok();
    fs::write(format!("{}/{}", ak_dir, name), format!("{}/{} ~amd64\n", CATEGORY, name))?;

    println!("{} Готово. pkgrs -i -av {}/{}", ">>>".green(), CATEGORY, name);
    Ok(())
}
 "##),
    ("sys-apps/pkgrs/files/src/bin/emerge-setup-git.rs", r##"use rusqlite::{params, Connection, Result};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::process::{Command, exit};
use colored::*;
use regex::Regex;

const DB_PATH: &str = "/var/cache/portage/cargo-base.db";
const OVERLAY_ROOT: &str = "/var/db/repos/tupoll-overlay";
const CATEGORY: &str = "dev-rust";

fn init_db() -> Result<Connection> {
    let conn = Connection::open(DB_PATH)?;
    conn.execute_batch("PRAGMA journal_mode = WAL; PRAGMA busy_timeout = 5000;")?;
    conn.execute("CREATE TABLE IF NOT EXISTS packages (name TEXT PRIMARY KEY, description TEXT)", [])?;
    Ok(conn)
}

fn setup_overlay_metadata() -> io::Result<()> {
    fs::create_dir_all(format!("{}/profiles", OVERLAY_ROOT))?;
    let repo_name_path = format!("{}/profiles/repo_name", OVERLAY_ROOT);
    if !std::path::Path::new(&repo_name_path).exists() {
        fs::write(repo_name_path, "tupoll-overlay")?;
    }
    let cat_file = format!("{}/profiles/categories", OVERLAY_ROOT);
    let mut content = String::new();
    if let Ok(mut f) = File::open(&cat_file) { let _ = f.read_to_string(&mut content); }
    if !content.contains(CATEGORY) {
        let mut f = OpenOptions::new().create(true).append(true).open(&cat_file)?;
        writeln!(f, "{}", CATEGORY)?;
    }
    Ok(())
}

fn create_and_install_ebuild(name: &str) -> io::Result<()> {
    setup_overlay_metadata()?;
    let clean_name = match name.rfind('-') {
        Some(idx) if name[idx+1..].chars().next().map_or(false, |c| c.is_ascii_digit()) => &name[..idx],
        _ => name,
    };

    let dir_path = format!("{}/{}/{}", OVERLAY_ROOT, CATEGORY, clean_name);
    let ebuild_path = format!("{}/{}-9999.ebuild", dir_path, clean_name);

    if !std::path::Path::new(&ebuild_path).exists() {
        println!("{} Настройка ebuild для {}...", ">>>".green(), clean_name);
        let info_output = Command::new("cargo").args(["info", clean_name]).output()?;
        let info_stdout = String::from_utf8_lossy(&info_output.stdout);
        let re_repo = Regex::new(r"(?i)repository:\s+(\S+)").unwrap();
        let repo_url = re_repo.captures(&info_stdout).and_then(|cap| cap.get(1)).map(|m| m.as_str().to_string())
            .unwrap_or_else(|| format!("https://github.com{}", clean_name));

        fs::create_dir_all(&dir_path)?;
                let mut f = File::create(&ebuild_path)?;
        writeln!(f, "#emerge-setup-lib")?;
        writeln!(f, "EAPI=8\ninherit cargo git-r3\n")?;
        writeln!(f, "DESCRIPTION=\"{} - Git Build\"", clean_name)?;
        writeln!(f, "HOMEPAGE=\"{}\"", repo_url)?;
        writeln!(f, "EGIT_REPO_URI=\"{}\"\n", repo_url)?;
        
        writeln!(f, "LICENSE=\"|| ( MIT Apache-2.0 )\"\nSLOT=\"0\"\nKEYWORDS=\"\"\nPROPERTIES=\"live\"\nBDEPEND=\"virtual/pkgconfig\"\n")?;
        
        writeln!(f, "src_unpack() {{\n\tgit-r3_src_unpack\n\tcargo_live_src_unpack\n}}\n")?;
        writeln!(f, "src_install() {{")?;
        writeln!(f, "\tlocal target_dir=\"${{S}}\"")?;
        writeln!(f, "\t# Если есть подпапка с именем пакета (как в tokio), идем туда")?;
        writeln!(f, "\tif [[ -d \"${{S}}/{}\" && -f \"${{S}}/{}/Cargo.toml\" ]]; then", clean_name, clean_name)?;
        writeln!(f, "\t\ttarget_dir=\"${{S}}/{}\"", clean_name)?;
        writeln!(f, "\tfi")?;
        
        writeln!(f, "\tpushd \"${{target_dir}}\" > /dev/null || die")?;
        
        writeln!(f, "\t# Проверяем, есть ли что устанавливать (bin или example)")?;
        writeln!(f, "\tif grep -q '[[bin]]' Cargo.toml 2>/dev/null || [[ -d \"src/bin\" ]] || [[ -f \"src/main.rs\" ]]; then")?;
        writeln!(f, "\t\tcargo_src_install")?;
        writeln!(f, "\telse")?;
        writeln!(f, "\t\teinfo \"Это библиотека. Устанавливаем Cargo.toml как заглушку.\"")?;
        writeln!(f, "\t\tinsinto \"/usr/share/cargo/registry/${{P}}\"")?;
        writeln!(f, "\t\tdoins Cargo.toml")?;
        writeln!(f, "\tfi")?;
        
        writeln!(f, "\tpopd > /dev/null || die")?;
        writeln!(f, "}}")?;
      


        // Размаскировка
        fs::create_dir_all("/etc/portage/package.accept_keywords")?;
        let mut ak = File::create(format!("/etc/portage/package.accept_keywords/{}", clean_name))?;
        writeln!(ak, "{}/{} **", CATEGORY, clean_name)?;
    }

    println!("{} Запуск emerge для {}...", ">>>".green().bold(), clean_name);
    Command::new("pkgrs -i").arg("--ask").arg(format!("{}/{}", CATEGORY, clean_name)).status()?;
    Ok(())
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    if unsafe { libc::getuid() } != 0 { eprintln!("{}: Root req", "Error".red()); exit(1); }
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { println!("Usage: {} <pkg>", "pkgrs -wge".green()); exit(1); }
    let query = &args[1];
    let mut conn = init_db()?;
    let mut found_list = Vec::new();

    {
        let mut stmt = conn.prepare("SELECT name, description FROM packages WHERE name LIKE ?")?;
        let rows = stmt.query_map(params![format!("%{}%", query)], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?;
        for pkg in rows { 
            let (n, d) = pkg?; 
            println!("*  {}/{}\n      {}", CATEGORY.green(), n.bold(), d.white()); 
            found_list.push(n); 
        }
    }

    if found_list.is_empty() {
        println!("{} Crates.io search...", ">>>".green());
        let output = Command::new("cargo").args(["search", query, "--limit", "10"]).output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let re = Regex::new(r#"^(\S+)\s*=\s*"([^"]+)"\s*#\s*(.*)$"#)?;
        let tx = conn.transaction()?;
        for line in stdout.lines() {
            if let Some(caps) = re.captures(line) {
                let n = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                let v = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                let d = caps.get(3).map(|m| m.as_str()).unwrap_or("");
                let full = format!("{}-{}", n, v);
                println!("*  {}/{}\n      {}", CATEGORY.green(), full.bold(), d.white());
                tx.execute("INSERT OR REPLACE INTO packages VALUES (?1, ?2)", params![full, d])?;
                found_list.push(full);
            }
        }
        tx.commit()?;
    }

    if let Some(target) = found_list.first() {
        print!("\n{} Install {}? [y/N]: ", "??".magenta().bold(), target);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() == "y" { create_and_install_ebuild(target)?; }
    }
    Ok(())
}
 "##),
    ("sys-apps/pkgrs/files/src/bin/emerge-setup-lib.rs", r##"use std::fs;
use std::process::Command;
use std::io::Write;
use colored::*;
use std::collections::HashSet;

const OVERLAY_ROOT: &str = "/var/db/repos/tupoll-overlay";
const CATEGORY: &str = "dev-rust";

fn get_system_libs(work_dir: &str) -> String {
    let mut libs = HashSet::new();
    libs.insert("virtual/pkgconfig".to_string());

    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--offline"])
        .current_dir(work_dir)
        .output();

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let common_mappings = [
            ("wayland", "dev-libs/wayland"), ("gtk", "gui-libs/gtk"),
            ("glib", "dev-libs/glib"), ("dbus", "sys-apps/dbus"),
            ("openssl", "dev-libs/openssl"), ("sqlite", "dev-db/sqlite"),
        ];
        for (key, atom) in common_mappings {
            if stdout.contains(&format!("{}-sys", key)) { libs.insert(atom.to_string()); }
        }
    }
    libs.into_iter().collect::<Vec<_>>().join("\n\t")
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <pkg> <version>", "pkgrs -wle".cyan());
        return Ok(());
    }
    let name = &args[1];
    let version = &args[2];
    let lib_name = name.replace("-", "_");

    let pkg_dir = format!("{}/{}/{}", OVERLAY_ROOT, CATEGORY, name);
    let sources_dir = format!("{}/sources-{}", pkg_dir, version);
    let work_dir = format!("{}/build-root", sources_dir);
    
    fs::create_dir_all(&work_dir)?;

    println!("{} Инициализация проекта...", ">>>".green());
    Command::new("cargo").args(["init", "--lib", "--name", "build-root"]).current_dir(&work_dir).output()?;
    Command::new("cargo").args(["add", &format!("{}@{}", name, version)]).current_dir(&work_dir).output()?;

    // ПАТЧ CARGO.TOML
    let toml_path = format!("{}/Cargo.toml", work_dir);
    let mut toml = fs::read_to_string(&toml_path)?;
    // Удаляем старые упоминания lib, если есть
    toml = toml.lines().filter(|l| !l.contains("[lib]") && !l.contains("crate-type")).collect::<Vec<_>>().join("\n");
    toml.push_str(&format!("\n[lib]\nname = \"{}\"\ncrate-type = [\"cdylib\", \"rlib\"]\n", lib_name));
    fs::write(&toml_path, toml)?;

    // ПАТЧ SRC/LIB.RS
    let lib_rs_path = format!("{}/src/lib.rs", work_dir);
    fs::write(&lib_rs_path, format!("pub use {}::*;", lib_name))?;

    println!("{} Вендоринг...", ">>>".green());
    Command::new("cargo").arg("vendor").arg("../vendor").current_dir(&work_dir).output()?;

    let rdepend = get_system_libs(&work_dir);
    let homepage = format!("https://crates.io/{}", name);
    let ebuild_path = format!("{}/{}-{}.ebuild", pkg_dir, name, version);
    
    let mut f = fs::File::create(&ebuild_path)?;
    writeln!(f, "#emerge-setup-lib")?;
    writeln!(f, "EAPI=8\ninherit cargo\n\nDESCRIPTION=\"{} (cdylib)\"\nHOMEPAGE=\"{}\"\nSRC_URI=\"\"\nS=\"${{WORKDIR}}/build-root\"", name, homepage)?;
    writeln!(f, "LICENSE=\"MIT\"\nSLOT=\"0\"\nKEYWORDS=\"~amd64\"\nRDEPEND=\"\n\t{}\"\nDEPEND=\"${{RDEPEND}}\"\nBDEPEND=\"virtual/pkgconfig\"", rdepend)?;

    writeln!(f, "\nsrc_unpack() {{ cp -R \"{}/\"* \"${{WORKDIR}}/\" || die; }}", sources_dir)?;
    writeln!(f, "src_compile() {{
    mkdir -p .cargo
    echo \"[source.crates-io]\nreplace-with = 'vendored-sources'\n[source.vendored-sources]\ndirectory = '../vendor'\" > .cargo/config.toml
    RUSTFLAGS='-C target-cpu=native' cargo build --release --offline
}}")?;
    writeln!(f, "src_install() {{
    find target/release -maxdepth 1 -name \"*.so\" -exec dolib.so {{}} +
    find target/release -maxdepth 1 -executable -type f ! -name \"*.so\" ! -name \"*.a\" -exec dobin {{}} +
}}")?;

    Command::new("ebuild").arg(&ebuild_path).arg("digest").status()?;
    println!("{} Готово. pkgrs -i -av {}/{}", ">>>".green(), CATEGORY, name);

    Ok(())
}
 "##),
    ("sys-apps/pkgrs/files/src/bin/emerge-setup.rs", r#"fn main() {
    println!("Установленны следующие пакеты:
             emerge-setup-git: пишет eduild с github в /usr/bin(unstable)
             emerge-setup-bin: пишет ebuild bin в /usr/bin
             emerge-setup-lib: пишет ebuild  lib в /usr/li64
             emerge-cargo-base: ищет в cargo базе 
             eselect-python: меняет версию python
             writer-git stable emerge-setup-git
             writer-bin stable emerge-setup-bin
             writer-lib stable emerge-setup-lib
             emerge-update: обновление cargo мира
             и так далее....");
} "#),
    ("sys-apps/pkgrs/files/src/bin/emerge-update.rs", r#"use std::{fs, io::{self, Write}, process::Command};
use colored::*;
use semver::Version;

const OVERLAY_PATH: &str = "/var/db/repos/tupoll-overlay/dev-rust";

fn get_cache_version(name: &str) -> Option<String> {
    let output = Command::new("portageq")
        .args(["best_visible", "/", &format!("dev-rust/{}", name)])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() { return None; }
    // Извлекаем чистую версию из атома dev-rust/pkg-1.2.3
    Some(stdout.replace(&format!("dev-rust/{}-", name), ""))
}

fn main() -> io::Result<()> {
    println!("\n{}", "=== [ Gentoo Rust Local Controller ] ===".bold().cyan());
    println!("{} Сервис rust-portage-watcher активен. Работаем с кэшем.\n", ">>>".green());

    // Собираем в вектор, чтобы избежать ошибки "use of moved value"
    let entries: Vec<_> = fs::read_dir(OVERLAY_PATH)?
        .filter_map(|e| e.ok())
        .collect();

    for pkg in entries {
        if !pkg.file_type()?.is_dir() { continue; }
        let name = pkg.file_name().into_string().unwrap();

        // 1. Читаем текущую версию из ebuild
        let mut ebuilds: Vec<String> = fs::read_dir(pkg.path())?
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n.ends_with(".ebuild"))
            .collect();

        if ebuilds.is_empty() { continue; }
        ebuilds.sort();
        let last_ebuild = ebuilds.last().unwrap();
        let current_v = last_ebuild.replace(".ebuild", "").replace(&format!("{}-", name), "");

        print!("* {:<25} [ {} ] ", name.cyan(), current_v.white());
        io::stdout().flush()?;

        // 2. Логика для LIVE (9999)
        if current_v == "9999" {
            println!("{}", "-> LIVE".green());
            continue; 
        }

        // 3. Сравнение с кэшем
        if let Some(cache_v) = get_cache_version(&name) {
            let v_local = Version::parse(&current_v).unwrap_or(Version::parse("0.0.0").unwrap());
            let v_cache = Version::parse(&cache_v).unwrap_or(Version::parse("0.0.0").unwrap());

            if v_cache > v_local {
                println!("-> {} {}", "НОВАЯ:".yellow().bold(), cache_v.yellow());
                print!("  Обновить ebuild? [y/N]: ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                if input.trim().to_lowercase() == "y" {
                    let is_lib = name.contains("-rs") || name.contains("-sys") || name.contains("wayland") || name == "gbm";
                    let tool = if is_lib { "emerge-setup-lib" } else { "emerge-setup-bin" };
                    
                    println!("{} Запуск {} v{}...", ">>>".green(), tool, cache_v);
                    Command::new(tool).args([&name, &cache_v]).status()?;
                }
            } else {
                println!("{}", "[OK]".blue());
            }
        } else {
            println!("{}", "[Нет в кэше]".red());
        }
    }
    Ok(())
}
 "#),
    ("sys-apps/pkgrs/files/src/bin/eselect-python.rs", r#"use std::env;
use std::fs;
use std::process::{Command, exit};

fn main() -> std::io::Result<()> {
    // 0. Проверка на права root
    #[cfg(unix)]
    if unsafe { libc::getuid() } != 0 {
        eprintln!("Ошибка: Запускайте через sudo.");
        exit(1);
    }

    let exec_path = "/usr/lib/python-exec";
    let target_file = "/var/db/repos/tupoll-overlay/dev-util/rust-portage/files/src/bin/python-exec-rs.rs";
    let backup_file = format!("{}.bak", target_file);

    // 1. Сбор версий Python
    let mut pythons: Vec<String> = fs::read_dir(exec_path)?
        .filter_map(|res| res.ok())
        .map(|e| e.file_name().into_string().unwrap_or_default())
        .filter(|name| {
            name.starts_with("python3") 
            && !name.contains(".bk") 
            && !name.contains("rust-src")
            && !name.contains("python-exec")
        })
        .collect();

    pythons.sort();

    if pythons.is_empty() {
        eprintln!("Python не найден в {}", exec_path);
        exit(1);
    }

    // 2. Вывод списка
    println!("Доступные версии Python:");
    for (i, py) in pythons.iter().enumerate() {
        println!("{}: {}", i + 1, py);
    }

    // 3. Обработка аргументов
    let args: Vec<String> = env::args().collect();
    let mut selected_index: Option<usize> = None;

    if args.len() == 4 && args[1] == "python" && args[2] == "set" {
        selected_index = args[3].parse::<usize>().ok();
    } else if args.len() == 3 && args[1] == "set" {
        selected_index = args[2].parse::<usize>().ok();
    }

    if let Some(index) = selected_index {
        if index > 0 && index <= pythons.len() {
            let selected_py = &pythons[index - 1];
            
            // --- НОВОЕ: Создание бэкапа перед изменением ---
            if fs::metadata(target_file).is_ok() {
                fs::copy(target_file, &backup_file)?;
                println!("\n[0/4] Создан бэкап исходника: {}", backup_file);
            }

            // 4. Модификация исходника
            let content = fs::read_to_string(target_file)?;
            let mut new_content = String::new();
            let target_prefix = "let py_versions = [";
            let replacement_line = format!("let py_versions = [\"{}\"];\n", selected_py);

            let mut found = false;
            for line in content.lines() {
                if line.trim().starts_with(target_prefix) {
                    new_content.push_str(&replacement_line);
                    found = true;
                } else {
                    new_content.push_str(line);
                    new_content.push('\n');
                }
            }

            if found {
                fs::write(target_file, new_content)?;
                println!("[1/4] Конфигурация изменена на {}", selected_py);
                
                // 5. Пересборка
                println!("[2/4] Запуск emerge dev-util/rust-portage...");
                let emerge_status = Command::new("emerge")
                    .arg("dev-util/pkg")
                    .status()?;

                if emerge_status.success() {
                    println!("[3/4] Финализация враппера...");
                    
                    let bin_source = "/usr/bin/python-exec-rs";
                    let exec_dest = "/usr/lib/python-exec/python-exec-2";
                    let exec_bk = "/usr/lib/python-exec/python-exec2.bk";

                    // Бэкап системного бинарника и замена
                    if fs::metadata(exec_dest).is_ok() && fs::metadata(exec_bk).is_err() {
                        let _ = fs::rename(exec_dest, exec_bk);
                    }
                    if let Err(e) = fs::copy(bin_source, exec_dest) {
                        eprintln!("Ошибка копирования враппера: {}", e);
                    }

                    // 6. Синхронизация через rust-helper
                    println!("[4/4] Синхронизация: rust-helper sync...");
                    let sync_status = Command::new("rust-helper")
                        .arg("sync")
                        .status();

                    match sync_status {
                        Ok(s) if s.success() => println!("\n[ГОТОВО] Python успешно переключен и синхронизирован!"),
                        _ => eprintln!("\n[!] Ошибка при выполнении rust-helper sync"),
                    }
                } else {
                    eprintln!("\n[!] Ошибка при пересборке пакета. Исходник сохранен в .bak");
                    exit(1);
                }
            }
        } else {
            eprintln!("Ошибка: Неверный индекс {}. Выберите от 1 до {}", index, pythons.len());
        }
    } else {
        println!("\nИспользование:");
        println!("  sudo eselect-python set <номер>");
        println!("  sudo eselect-python python set <номер>");
    }

    Ok(())
}
 "#),
    ("sys-apps/pkgrs/files/src/bin/eselect-rust-emerge.rs", r#"use std::{env, fs, os::unix::fs::symlink, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        println!("🚀 eselect-rust v0.1.0 (Python 3.14/3.13/3.12 ready)");
        println!("--------------------------------------------------");
        println!("Использование:");
        println!("  eselect-rust set <команда>    -- переключить на Rust-враппер");
        println!("  eselect-rust unset <команда>  -- вернуть стандарт Gentoo");
        println!("\nПример: sudo eselect-rust set emerge");
        return Ok(());
    }

    let action = &args[1];
    let cmd_name = &args[2];
    
    let target_path = format!("/usr/bin/{}", cmd_name);
    let my_wrapper = "/usr/lib/python-exec/python-exec-2";
    let gentoo_wrapper = "/usr/lib/python-exec/python-exec2c";

    match action.as_str() {
        "set" => {
            // Проверка, существует ли наш мозг
            if !Path::new(my_wrapper).exists() {
                eprintln!("Ошибка: Бинарник {} не найден. Сначала скомпилируйте main.rs", my_wrapper);
                std::process::exit(1);
            }

            // Удаляем старый симлинк/файл
            if Path::new(&target_path).exists() {
                fs::remove_file(&target_path)?;
            }

            // Создаем новый симлинк на наш враппер
            symlink(my_wrapper, &target_path)?;
            println!("✅ Команда '{}' теперь под управлением python-exec-rs", cmd_name);
            println!("   (Приоритеты: Rust-src -> 3.14 -> 3.13 -> 3.12)");
        }
        "unset" => {
            if Path::new(&target_path).exists() {
                fs::remove_file(&target_path)?;
            }

            // Возвращаем стандартный бинарник Gentoo (python-exec2c)
            if Path::new(gentoo_wrapper).exists() {
                symlink(gentoo_wrapper, &target_path)?;
                println!("🔄 Команда '{}' возвращена под управление стандартного python-exec2c", cmd_name);
            } else {
                eprintln!("Предупреждение: {} не найден, симлинк не восстановлен.", gentoo_wrapper);
            }
        }
        _ => {
            eprintln!("Неизвестное действие: {}. Используйте 'set' или 'unset'.", action);
        }
    }

    Ok(())
}
 "#),
    ("sys-apps/pkgrs/files/src/bin/helper.rs", r##"use std::fs;
use std::process::Command;
use std::path::Path;

const DB_PATH: &str = "/var/cache/portage/rust_index.db";
const REPO_PATH: &str = "/var/db/repos/gentoo";
const MARKER_START: &str = "# --- RUST_ACCELERATOR_START ---";
const MARKER_END: &str = "# --- RUST_ACCELERATOR_END ---";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let action = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match action {
        "patch" => apply_patch(),
        "unpatch" => remove_patch(),
        "sync" => sync_index(),
        _ => {
            println!("Доступные команды:");
            println!("  rust-helper patch    - внедрить ускоритель");
            println!("  rust-helper unpatch  - убрать все изменения из emerge");
            println!("  rust-helper sync     - обновить базу данных");
        }
    }
}

fn sync_index() {
    println!(">>> Индексация дерева Portage...");
    if let Some(parent) = Path::new(DB_PATH).parent() {
        let _ = fs::create_dir_all(parent);
    }
    let status = Command::new("python3")
        .arg("-c")
        .arg(format!("import rust_portage; rust_portage.sync_index('{}', '{}')", DB_PATH, REPO_PATH))
        .status();
    if status.as_ref().map_or(false, |s| s.success()) { println!(">>> Готово."); }
}

fn apply_patch() {
    process_emerge_files(|content| {
        if content.contains(MARKER_START) {
            println!(">>> Уже пропатчен.");
            return None;
        }
        let patch_code = format!(
r#"{}
try:
    import rust_portage, sys, os
    if len(sys.argv) > 1 and sys.argv[1] in ("--search", "-s"):
        db_path = "{}"
        if os.path.exists(db_path):
            query = " ".join(sys.argv[2:])
            if query:
                for res in rust_portage.power_search(db_path, query): print(res)
                sys.exit(0)
except Exception: pass
{}
"#, MARKER_START, DB_PATH, MARKER_END);
        
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        if !lines.is_empty() { lines.insert(1, patch_code); }
        Some(lines.join("\n"))
    });
}

fn remove_patch() {
    process_emerge_files(|content| {
        if !content.contains(MARKER_START) {
            println!(">>> Патч не найден.");
            return None;
        }
        let mut new_lines = Vec::new();
        let mut skip = false;
        for line in content.lines() {
            if line.contains(MARKER_START) { skip = true; continue; }
            if line.contains(MARKER_END) { skip = false; continue; }
            if !skip { new_lines.push(line); }
        }
        println!(">>> Патч удален.");
        Some(new_lines.join("\n"))
    });
}

fn process_emerge_files<F>(mut f: F) 
where F: FnMut(&str) -> Option<String> {
    let base_dir = "/usr/lib/python-exec";
    if let Ok(entries) = fs::read_dir(base_dir) {
        for entry in entries.flatten() {
            let p = entry.path().join("emerge");
            if p.exists() && p.is_file() {
                if let Ok(content) = fs::read_to_string(&p) {
                    if let Some(new_content) = f(&content) {
                        fs::write(&p, new_content).expect("Ошибка записи");
                        println!(">>> Обработан: {:?}", p);
                    }
                }
            }
        }
    }
}
 "##),
    ("sys-apps/pkgrs/files/src/bin/python-exec-rs.rs", r#"use std::{
    env, fs, 
    io::{Read},
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process::Command,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

const RUST_SRC_DIR: &str = "/usr/lib/python-exec/rust-src";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Авто-создание папки для исходников
    if !Path::new(RUST_SRC_DIR).exists() {
        let _ = fs::create_dir_all(RUST_SRC_DIR);
    }

    let args: Vec<String> = env::args().collect();
    if args.is_empty() { return Ok(()); }

    // 1. ОПРЕДЕЛЯЕМ ИМЯ КОМАНДЫ
    let call_path = PathBuf::from(&args[0]);
    let bin_name = call_path.file_name()
        .and_then(|n| n.to_str())
        .ok_or("Failed to get binary name")?;

    // Защита от прямого запуска
    if bin_name == "python-exec-2" || bin_name == "python-exec-rs" {
        eprintln!("python-exec-rs: Запускайте через симлинк (например, emerge)");
        std::process::exit(127);
    }

    // 2. ПРИОРИТЕТ 1: Rust-исходник
    let rust_src = Path::new(RUST_SRC_DIR).join(format!("{}.rs", bin_name));
    if rust_src.exists() {
        return run_rust_script(&rust_src, &args);
    }

    // 3. ПРИОРИТЕТ 2: Поиск по версиям Python
let py_versions = ["python3.13"];
    let mut provider: Option<(PathBuf, String)> = None;

    // Сначала проверяем EPYTHON из окружения
    if let Ok(val) = env::var("EPYTHON") {
        let p = Path::new("/usr/lib/python-exec").join(&val).join(bin_name);
        if p.exists() {
            provider = Some((p, val));
        }
    }

    // Если не нашли, идем по списку приоритетов
    if provider.is_none() {
        for py in py_versions {
            let p = Path::new("/usr/lib/python-exec").join(py).join(bin_name);
            if p.exists() {
                provider = Some((p, py.to_string()));
                break;
            }
        }
    }

    let (script_path, epython_val) = provider.ok_or_else(|| {
        format!("python-exec-rs: команда '{}' не найдена в {:?} или EPYTHON", bin_name, py_versions)
    })?;

    // 4. ПРОВЕРКА ТИПА И ЗАПУСК
    let mut file = fs::File::open(&script_path)?;
    let mut header = [0u8; 4];
    let bytes_read = file.read(&mut header)?;

    // Если ELF
    if bytes_read == 4 && &header == b"\x7fELF" {
        let mut cmd = Command::new(&script_path);
        if args.len() > 1 { cmd.args(&args[1..]); }
        let err = cmd.exec();
        return Err(Box::new(err));
    }

    // Если скрипт -> запускаем через найденный Python
    let mut cmd = Command::new(&epython_val);
    cmd.arg(&script_path);
    if args.len() > 1 {
        cmd.args(&args[1..]);
    }
    
    let err = cmd.exec();
    Err(Box::new(err))
}

fn run_rust_script(path: &Path, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    let build_dir = env::temp_dir().join(format!("rust-exec-{}", hasher.finish()));
    
    let mut deps = String::new();
    for line in content.lines() {
        if line.starts_with("//!") {
            let d = line.trim_start_matches("//!").trim();
            if d.contains('=') { deps.push_str(d); deps.push('\n'); }
        } else if !line.trim().is_empty() { break; }
    }

    fs::create_dir_all(build_dir.join("src"))?;
    let toml = format!("[package]\nname=\"s\"\nversion=\"0.1.0\"\nedition=\"2021\"\n[dependencies]\n{}", deps);
    fs::write(build_dir.join("Cargo.toml"), toml)?;
    fs::write(build_dir.join("src").join("main.rs"), content)?;

    let mut cmd = Command::new("cargo");
    cmd.current_dir(&build_dir).args(["run", "--release", "--quiet", "--"]);
    if args.len() > 1 { cmd.args(&args[1..]); }
    
    let err = cmd.exec();
    eprintln!("python-exec-rs: cargo exec failed: {}", err);
    std::process::exit(1);
}
 "#),
    ("sys-apps/pkgrs/files/src/bin/writer-bin.rs", r##"use std::fs;
use std::process::Command;
use std::io::Write;
use colored::*;
use std::collections::HashSet;

const OVERLAY_ROOT: &str = "/var/db/repos/tupoll-overlay";
const CATEGORY: &str = "dev-rust";

fn get_system_libs(work_dir: &str) -> String {
    let mut libs = HashSet::new();
    libs.insert("virtual/pkgconfig".to_string());
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--offline"])
        .current_dir(work_dir).output();

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let common_mappings = [
            ("wayland", "dev-libs/wayland"),
            ("gtk", "gui-libs/gtk"),
            ("glib", "dev-libs/glib"),
            ("openssl", "dev-libs/openssl"),
            ("libva", "media-libs/libva"), // <--- ДОБАВИТЬ ЭТО
            ("drm", "x11-libs/libdrm"),     // <
        ];
        for (key, atom) in common_mappings {
            if stdout.contains(&format!("{}-sys", key)) { libs.insert(atom.to_string()); }
        }
    }
    libs.into_iter().collect::<Vec<_>>().join("\n\t")
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <pkg> <version>", "pkgrs -wb".cyan());
        return Ok(());
    }
    let name = &args[1];
    let version = &args[2];

    let pkg_dir = format!("{}/{}/{}", OVERLAY_ROOT, CATEGORY, name);
    let sources_dir = format!("{}/sources-{}", pkg_dir, version);
    let work_dir = format!("{}/{}-{}", sources_dir, name, version);
    
    fs::create_dir_all(&sources_dir)?;

    println!("{} Получение исходников {} v{}...", ">>>".green(), name, version);
    
    // Пытаемся скачать через cargo download
    let dl = Command::new("cargo").args(["download", "-x", &format!("{}=={}", name, version)]).current_dir(&sources_dir).status();

    if dl.is_err() || !dl.unwrap().success() {
        println!("{} Используем встроенный метод получения...", ">>>".yellow());
        // Фолбэк: создаем проект, скачиваем крэйт и перемещаем его исходники
        let tmp_path = format!("{}/tmp_dl", sources_dir);
        let _ = fs::remove_dir_all(&tmp_path);
        Command::new("cargo").args(["new", "tmp_dl"]).current_dir(&sources_dir).output()?;
        Command::new("cargo").args(["add", &format!("{}@{}", name, version)]).current_dir(&tmp_path).output()?;
        
        // Используем vendor, чтобы вытащить исходники самого пакета
        Command::new("cargo").arg("vendor").current_dir(&tmp_path).output()?;
        
        // Находим папку пакета в vendor и перемещаем её в work_dir
        let vendor_path = format!("{}/vendor", tmp_path);
        let pkg_folder = fs::read_dir(&vendor_path)?
            .filter_map(|e| e.ok())
            .find(|e| e.file_name().to_string_lossy().starts_with(name))
            .expect("Пакет не найден в загрузках")
            .path();
            
        fs::rename(pkg_folder, &work_dir)?;
        let _ = fs::remove_dir_all(&tmp_path);
    }

    println!("{} Вендоринг зависимостей...", ">>>".green());
    Command::new("cargo").arg("vendor").current_dir(&work_dir).output()?;

    let rdepend = get_system_libs(&work_dir);
    let ebuild_path = format!("{}/{}-{}.ebuild", pkg_dir, name, version);
    let mut f = fs::File::create(&ebuild_path)?;
    writeln!(f, "#writer-bin")?;
    writeln!(f, "EAPI=8\ninherit cargo\n\nDESCRIPTION=\"{}\"\nHOMEPAGE=\"https://crates.io/{}\"\nSRC_URI=\"\"\nS=\"${{WORKDIR}}/{}-{}\"", name, name, name, version)?;
    writeln!(f, "LICENSE=\"MIT\"\nSLOT=\"0\"\nKEYWORDS=\"~amd64\"\nRDEPEND=\"\n\t{}\"\nDEPEND=\"${{RDEPEND}}\"\nBDEPEND=\"virtual/pkgconfig\"", rdepend)?;

    writeln!(f, "\nsrc_unpack() {{ cp -Rp \"{}/\"* \"${{WORKDIR}}/\" || die; }}", sources_dir)?;
    
    writeln!(f, "\nsrc_compile() {{
    export CARGO_HOME=\"${{T}}/cargo_home\"
    mkdir -p .cargo || die
    echo \"[source.crates-io]\nreplace-with = 'vendored-sources'\n[source.vendored-sources]\ndirectory = 'vendor'\" > .cargo/config.toml || die
    RUSTFLAGS='-C target-cpu=native' cargo build --release --offline --all-features || die
}}")?;

    writeln!(f, "\nsrc_install() {{
    find target/release -maxdepth 1 -executable -type f ! -name \"*.so\" -exec dobin {{}} +
}}")?;

    println!("{} Финализация...", ">>>".green());
    let _ = Command::new("chown").args(["-R", "portage:portage", &pkg_dir]).status();
    let _ = Command::new("chmod").args(["-R", "go+rX", &pkg_dir]).status();
    Command::new("ebuild").arg(&ebuild_path).arg("digest").status()?;

    let ak_dir = "/etc/portage/package.accept_keywords";
    fs::create_dir_all(ak_dir).ok();
    fs::write(format!("{}/{}", ak_dir, name), format!("{}/{} ~amd64\n", CATEGORY, name))?;

    println!("{} Готово. pkgrs -i -av {}/{}", ">>>".green(), CATEGORY, name);
    Ok(())
}
 "##),
    ("sys-apps/pkgrs/files/src/bin/writer-git.rs", r##"use std::fs;
use std::io::Write;
use std::process::Command;
use colored::*;

const OVERLAY_ROOT: &str = "/var/db/repos/tupoll-overlay";
const CATEGORY: &str = "dev-rust";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <name> <url>", "pkgrs -wg".cyan());
        return Ok(());
    }
    let name = &args[1];
    let url = &args[2];
    
    let pkg_dir = format!("{}/{}/{}", OVERLAY_ROOT, CATEGORY, name);
    fs::create_dir_all(&pkg_dir)?;

    let ebuild_path = format!("{}/{}-9999.ebuild", pkg_dir, name);
    let mut f = fs::File::create(&ebuild_path)?;

    // Формируем безопасное имя для библиотеки (заменяем - на _)
    let lib_name = name.replace("-", "_");
    writeln!(f, "#writer-git")?;
    writeln!(f, "EAPI=8")?;
    writeln!(f, "inherit cargo git-r3\n")?;
    writeln!(f, "DESCRIPTION=\"{} (live git version)\"", name)?;
    writeln!(f, "EGIT_REPO_URI=\"{}\"", url)?;
    writeln!(f, "LICENSE=\"MIT\"\nSLOT=\"0\"\nKEYWORDS=\"\"\n")?;
    
    writeln!(f, "RDEPEND=\"\"")?;
    writeln!(f, "DEPEND=\"${{RDEPEND}}\"")?;
    writeln!(f, "BDEPEND=\"virtual/pkgconfig\"")?;

    // ВНИМАТЕЛЬНЫЙ ПАТЧ CARGO.TOML
    writeln!(f, "\nsrc_prepare() {{
    default
    if [[ -f \"Cargo.toml\" ]]; then
        einfo \"Внимательный патч Cargo.toml для {0}...\"
        # 1. Если секция [lib] есть, вставляем типы под неё
        if grep -q \"\\[lib\\]\" Cargo.toml; then
            sed -i '/\\[lib\\]/a crate-type = [\"cdylib\", \"rlib\"]' Cargo.toml || die
        else
            # 2. Если секции нет, создаем её с именем пакета
            echo -e \"\\n[lib]\\nname = \\\"{1}\\\"\\ncrate-type = [\\\"cdylib\\\", \\\"rlib\\\"]\" >> Cargo.toml || die
        fi
    fi
}}", name, lib_name)?;

    writeln!(f, "\nsrc_compile() {{
    # Собираем всё: бинарники и динамические библиотеки
    cargo build --release --all-features || die
}}")?;

    writeln!(f, "\nsrc_install() {{
    # Ставим исполняемые файлы
    find target/release -maxdepth 1 -executable -type f ! -name \"*.so*\" ! -name \"*.a\" -exec dobin {{}} +
    # Ставим динамические библиотеки .so
    find target/release -maxdepth 1 -name \"*.so*\" -exec dolib.so {{}} +
}}")?;

    println!("{} Дайджест и права...", ">>>".green());
    let _ = Command::new("chown").args(["-R", "portage:portage", &pkg_dir]).status();
    let _ = Command::new("chmod").args(["-R", "go+rX", &pkg_dir]).status();
    Command::new("ebuild").arg(&ebuild_path).arg("digest").status()?;

    // Авто-размаскировка для LIVE-версий
    let ak_file = format!("/etc/portage/package.accept_keywords/{}", name);
    let mut ak = fs::File::create(ak_file)?;
    writeln!(ak, "{}/{} **", CATEGORY, name)?;

    println!("{} Готово! Пакет {} из Git настроен.", ">>>".green(), name);
    Ok(())
} "##),
    ("sys-apps/pkgrs/files/src/bin/writer-git-windows.rs", r##"use std::fs;
use std::io::Write;
use std::process::Command;
use colored::*;

const OVERLAY_ROOT: &str = "/var/db/repos/tupoll-overlay";
const CATEGORY: &str = "dev-rust";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <name> <url>", "pkgrs -wgw".cyan());
        return Ok(());
    }
    let name = &args[1];
    let url = &args[2];
    
    let pkg_dir = format!("{}/{}/{}", OVERLAY_ROOT, CATEGORY, name);
    fs::create_dir_all(&pkg_dir)?;

    let ebuild_path = format!("{}/{}-9999.ebuild", pkg_dir, name);
    let mut f = fs::File::create(&ebuild_path)?;

    // Формируем безопасное имя для библиотеки (заменяем - на _)
    let lib_name = name.replace("-", "_");
    writeln!(f, "#writer-git-windows")?;
    writeln!(f, "EAPI=8")?;
    writeln!(f, "inherit cargo git-r3\n")?;
    writeln!(f, "DESCRIPTION=\"{} (live git version)\"", name)?;
    writeln!(f, "EGIT_REPO_URI=\"{}\"", url)?;
    writeln!(f, "LICENSE=\"MIT\"\nSLOT=\"0\"\nKEYWORDS=\"\"\n")?;
    
    writeln!(f, "RDEPEND=\"\"")?;
    writeln!(f, "DEPEND=\"${{RDEPEND}}\"")?;
    writeln!(f, "BDEPEND=\"virtual/pkgconfig\"")?;

    // ВНИМАТЕЛЬНЫЙ ПАТЧ CARGO.TOML
    writeln!(f, "\nsrc_prepare() {{
    default
    if [[ -f \"Cargo.toml\" ]]; then
        einfo \"Внимательный патч Cargo.toml для {0}...\"
        # 1. Если секция [lib] есть, вставляем типы под неё
        if grep -q \"\\[lib\\]\" Cargo.toml; then
            sed -i '/\\[lib\\]/a crate-type = [\"cdylib\", \"rlib\"]' Cargo.toml || die
        else
            # 2. Если секции нет, создаем её с именем пакета
            echo -e \"\\n[lib]\\nname = \\\"{1}\\\"\\ncrate-type = [\\\"cdylib\\\", \\\"rlib\\\"]\" >> Cargo.toml || die
        fi
    fi
}}", name, lib_name)?;

    writeln!(f, "\nsrc_compile() {{
    # Собираем всё: бинарники и динамические библиотеки
    cargo build --release --no-default-features --features 'cmd' || die
}}")?;

    writeln!(f, "\nsrc_install() {{
    # Ставим исполняемые файлы
    find target/release -maxdepth 1 -executable -type f ! -name \"*.so*\" ! -name \"*.a\" -exec dobin {{}} +
    # Ставим динамические библиотеки .so
    find target/release -maxdepth 1 -name \"*.so*\" -exec dolib.so {{}} +
}}")?;

    println!("{} Дайджест и права...", ">>>".green());
    let _ = Command::new("chown").args(["-R", "portage:portage", &pkg_dir]).status();
    let _ = Command::new("chmod").args(["-R", "go+rX", &pkg_dir]).status();
    Command::new("ebuild").arg(&ebuild_path).arg("digest").status()?;

    // Авто-размаскировка для LIVE-версий
    let ak_file = format!("/etc/portage/package.accept_keywords/{}", name);
    let mut ak = fs::File::create(ak_file)?;
    writeln!(ak, "{}/{} **", CATEGORY, name)?;

    println!("{} Готово! Пакет {} из Git настроен.", ">>>".green(), name);
    Ok(())
}
 "##),
    ("sys-apps/pkgrs/files/src/bin/writer-lib.rs", r##"use std::fs;
use std::process::Command;
use std::io::Write;
use colored::*;
use std::collections::HashSet;

const OVERLAY_ROOT: &str = "/var/db/repos/tupoll-overlay";
const CATEGORY: &str = "dev-rust";

fn get_system_libs(work_dir: &str) -> String {
    let mut libs = HashSet::new();
    libs.insert("virtual/pkgconfig".to_string());
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--offline"])
        .current_dir(work_dir).output();

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let common_mappings = [
            ("wayland", "dev-libs/wayland"), ("gtk", "gui-libs/gtk"),
            ("glib", "dev-libs/glib"), ("dbus", "sys-apps/dbus"),
            ("openssl", "dev-libs/openssl"), ("sqlite", "dev-db/sqlite"),
            ("gtk4-layer-shell", "gui-libs/gtk4-layer-shell"),
        ];
        for (key, atom) in common_mappings {
            if stdout.contains(&format!("{}-sys", key)) { libs.insert(atom.to_string()); }
        }
    }
    libs.into_iter().collect::<Vec<_>>().join("\n\t")
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <pkg> <version>", "pkgrs -wl".cyan());
        return Ok(());
    }
    let name = &args[1];
    let version = &args[2];
    let lib_name = name.replace("-", "_");

    let pkg_dir = format!("{}/{}/{}", OVERLAY_ROOT, CATEGORY, name);
    let sources_dir = format!("{}/sources-{}", pkg_dir, version);
    let work_dir = format!("{}/build-root", sources_dir);
    
    // 1. Очистка и создание базы
    if std::path::Path::new(&sources_dir).exists() {
        fs::remove_dir_all(&sources_dir)?;
    }
    fs::create_dir_all(&sources_dir)?;

    println!("{} Инициализация проекта-обертки...", ">>>".green());
    // Создаем build-root через cargo
    Command::new("cargo").args(["new", "--lib", "build-root"]).current_dir(&sources_dir).status()?;
    
    // 2. Добавляем зависимость
    Command::new("cargo").args(["add", &format!("{}@{}", name, version)]).current_dir(&work_dir).status()?;

    // 3. Вендоринг во внешнюю папку vendor
    println!("{} Вендоринг зависимостей...", ">>>".green());
    Command::new("cargo").args(["vendor", "../vendor"]).current_dir(&work_dir).status()?;

    // 4. Поиск папки пакета в vendor для path-зависимости
    let vendor_dir = format!("{}/vendor", sources_dir);
    let pkg_folder = fs::read_dir(&vendor_dir)?
        .filter_map(|e| e.ok())
        .find(|e| e.file_name().to_string_lossy().starts_with(name))
        .expect("Пакет не найден в vendor")
        .file_name().into_string().unwrap();

    // 5. Переписываем Cargo.toml на использование path (локально)
    let toml_content = format!(
        "[package]\nname = \"build-root\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n\
        [dependencies]\n{0} = {{ path = \"../vendor/{1}\" }}\n\n\
        [lib]\nname = \"{2}_rs\"\ncrate-type = [\"cdylib\", \"rlib\"]\n",
        name, pkg_folder, lib_name
    );
    fs::write(format!("{}/Cargo.toml", work_dir), toml_content)?;

    // 6. Переписываем src/lib.rs для проброса экспорта
    fs::write(format!("{}/src/lib.rs", work_dir), format!("pub use {}::*;", lib_name))?;

    let rdepend = get_system_libs(&work_dir);
    let ebuild_path = format!("{}/{}-{}.ebuild", pkg_dir, name, version);
    let mut f = fs::File::create(&ebuild_path)?;
    writeln!(f, "#writer-lib")?;
    writeln!(f, "EAPI=8\ninherit cargo\n\nDESCRIPTION=\"{} (wrapped lib)\"\nHOMEPAGE=\"https://crates.io/{}\"\nSRC_URI=\"\"\nS=\"${{WORKDIR}}/build-root\"", name, name)?;
    writeln!(f, "LICENSE=\"MIT\"\nSLOT=\"0\"\nKEYWORDS=\"~amd64\"\nRDEPEND=\"\n\t{}\"\nDEPEND=\"${{RDEPEND}}\"\nBDEPEND=\"virtual/pkgconfig\"", rdepend)?;

    writeln!(f, "\nsrc_unpack() {{ cp -Rp \"{}/\"* \"${{WORKDIR}}/\" || die; }}", sources_dir)?;
    // В src_compile НЕ НУЖЕН конфиг, так как у нас в Cargo.toml стоит path
        writeln!(f, "\nsrc_compile() {{
    # Создаем директорию для конфига
    mkdir -p .cargo || die
    
    # КРИТИЧЕСКИЙ МОМЕНТ: Указываем Cargo использовать вендор для ВСЕХ зависимостей
    echo \"[source.crates-io]
replace-with = 'vendored-sources'

[source.vendored-sources]
directory = '../vendor'\" > .cargo/config.toml || die

    export CARGO_HOME=\"${{T}}/cargo_home\"
    RUSTFLAGS='-C target-cpu=native' cargo build --release --offline || die
}}")?;

    writeln!(f, "\nsrc_install() {{
    find target/release -maxdepth 1 -name \"*.so\" -exec dolib.so {{}} +
    find target/release -maxdepth 1 -executable -type f ! -name \"*.so\" ! -name \"*.a\" -exec dobin {{}} +
}}")?;

    // ПРАВА
    let _ = Command::new("chown").args(["-R", "portage:portage", &pkg_dir]).status();
    let _ = Command::new("chmod").args(["-R", "go+rX", &pkg_dir]).status();
    Command::new("ebuild").arg(&ebuild_path).arg("digest").status()?;

    println!("{} Готово. pkgrs -i -av {}/{}", ">>>".green(), CATEGORY, name);
    Ok(())
}
 "##),
    ("sys-apps/pkgrs/files/src/bin/writer-pc-all.rs", r##"use std::fs;
use std::process::Command;
use std::io::Write;
use colored::*;
use std::collections::HashSet;

const OVERLAY_ROOT: &str = "/var/db/repos/tupoll-overlay";
const CATEGORY: &str = "dev-rust";

fn get_system_libs(work_dir: &str) -> String {
    let mut libs = HashSet::new();
    libs.insert("virtual/pkgconfig".to_string());
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--offline"])
        .current_dir(work_dir).output();

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let common_mappings = [
            ("wayland", "dev-libs/wayland"), ("gtk", "gui-libs/gtk"),
            ("glib", "dev-libs/glib"), ("dbus", "sys-apps/dbus"),
            ("openssl", "dev-libs/openssl"), ("sqlite", "dev-db/sqlite"),
        ];
        for (key, atom) in common_mappings {
            if stdout.contains(&format!("{}-sys", key)) { libs.insert(atom.to_string()); }
        }
    }
    libs.into_iter().collect::<Vec<_>>().join("\n\t")
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <pkg> <version>", "pkgrs -wpa".cyan());
        return Ok(());
    }
    let name = &args[1];
    let version = &args[2];
    let lib_name = name.replace("-", "_");
    let lib_name_rs = format!("{}", lib_name);

    let pkg_dir = format!("{}/{}/{}", OVERLAY_ROOT, CATEGORY, name);
    let sources_dir = format!("{}/sources-{}", pkg_dir, version);
    let work_dir = format!("{}/build-root", sources_dir);
    
    // 1. Подготовка директорий
    if std::path::Path::new(&sources_dir).exists() {
        fs::remove_dir_all(&sources_dir)?;
    }
    fs::create_dir_all(&sources_dir)?;

    println!("{} Инициализация проекта-обертки...", ">>>".green());
    Command::new("cargo").args(["new", "--lib", "build-root"]).current_dir(&sources_dir).status()?;
    Command::new("cargo").args(["add", &format!("{}@{}", name, version)]).current_dir(&work_dir).status()?;

    // 2. Вендоринг
    println!("{} Вендоринг зависимостей...", ">>>".green());
    Command::new("cargo").args(["vendor", "../vendor"]).current_dir(&work_dir).status()?;

    // 3. Настройка Cargo.toml (Force cdylib)
    let vendor_dir = format!("{}/vendor", sources_dir);
    let pkg_folder = fs::read_dir(&vendor_dir)?
        .filter_map(|e| e.ok())
        .find(|e| e.file_name().to_string_lossy().starts_with(name))
        .expect("Пакет не найден в vendor")
        .file_name().into_string().unwrap();

    let toml_content = format!(
        r#"[package]
name = "build-root"
version = "0.1.0"
edition = "2021"

[dependencies]
{0} = {{ path = "../vendor/{1}" }}

[lib]
name = "{2}"
crate-type = ["cdylib", "rlib"]
"#, name, pkg_folder, lib_name_rs
    );
    fs::write(format!("{}/Cargo.toml", work_dir), toml_content)?;
    fs::write(format!("{}/src/lib.rs", work_dir), format!("pub use {}::*;", lib_name))?;

    // 4. Очистка зависимостей для pkg-config
    let rdepend_raw = get_system_libs(&work_dir);
    let pc_requires = rdepend_raw
        .split(|c| c == '\n' || c == '\t' || c == ' ')
        .filter(|s| !s.is_empty() && !s.contains("virtual/") && !s.contains('&') && !s.contains('[') && !s.contains('?'))
        .collect::<Vec<_>>()
        .join(", ");

    // 5. Генерация Ebuild
    let ebuild_path = format!("{}/{}-{}.ebuild", pkg_dir, name, version);
    let mut f = fs::File::create(&ebuild_path)?;
    
    writeln!(f, r#"#writer-pc-all
EAPI=8
inherit cargo

DESCRIPTION="{name} (wrapped rust lib for system integration)"
HOMEPAGE="https://crates.io/{name}"
SRC_URI=""
S="${{WORKDIR}}/build-root"

LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64"
RDEPEND="
	{rdepend_raw}"
DEPEND="${{RDEPEND}}"
BDEPEND="virtual/pkgconfig"

src_unpack() {{
	cp -Rp "{sources_dir}/"* "${{WORKDIR}}/" || die
}}

src_compile() {{
	mkdir -p .cargo || die
	echo "[source.crates-io]
replace-with = 'vendored-sources'
[source.vendored-sources]
directory = '../vendor'" > .cargo/config.toml || die

	export CARGO_HOME="${{T}}/cargo_home"
	RUSTFLAGS='-C target-cpu=native' cargo build --release --offline || die
}}

src_install() {{
	# Установка библиотек .so
	dolib.so target/release/*.so

	# Установка бинарников
	find target/release -maxdepth 1 -executable -type f ! -name "*.so" ! -name "*.a" -exec dobin {{}} + 2>/dev/null

	# Генерация .pc файлов
	local pcdir="/usr/$(get_libdir)/pkgconfig"
	local shared_pcdir="/usr/share/pkgconfig"
	dodir "${{pcdir}}" "${{shared_pcdir}}"

	cat > "${{T}}/{name}.pc" <<EOF
prefix=/usr
exec_prefix=\${{prefix}}
libdir=\${{prefix}}/$(get_libdir)
includedir=\${{prefix}}/share/cargo/registry/{name}-{version}

Name: {name}
Description: {name} native rust wrapper
Version: {version}
Requires: {pc_requires}
Libs: -L\${{libdir}} -l{lib_name}
Cflags: -I\${{includedir}}
EOF

	insinto "${{pcdir}}"
	doins "${{T}}/{name}.pc"
	insinto "${{shared_pcdir}}"
	doins "${{T}}/{name}.pc"
}}
"#, name = name, version = version, lib_name = lib_name, pc_requires = pc_requires, rdepend_raw = rdepend_raw, sources_dir = sources_dir)?;

    // 6. Финализация
    let _ = Command::new("chown").args(["-R", "portage:portage", &pkg_dir]).status();
    Command::new("ebuild").arg(&ebuild_path).arg("digest").status()?;

    println!("{} Готово. pkgrs -i -av {}/{}", ">>>".green(), CATEGORY, name);
    Ok(())
} "##),
    ("sys-apps/pkgrs/files/src/bin/writer_pc_world.rs", r##"use std::fs;
use std::process::Command;
use std::io::Write;
use colored::*;
use std::collections::HashSet;

const OVERLAY_ROOT: &str = "/var/db/repos/tupoll-overlay";
const CATEGORY: &str = "dev-rust";

fn get_system_libs(work_dir: &str) -> String {
    let mut libs = HashSet::new();
    libs.insert("virtual/pkgconfig".to_string());
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--offline"])
        .current_dir(work_dir).output();

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let common_mappings = [
            ("wayland", "dev-libs/wayland"), ("gtk", "gui-libs/gtk"),
            ("glib", "dev-libs/glib"), ("dbus", "sys-apps/dbus"),
            ("openssl", "dev-libs/openssl"), ("sqlite", "dev-db/sqlite"),
        ];
        for (key, atom) in common_mappings {
            if stdout.contains(&format!("{}-sys", key)) { libs.insert(atom.to_string()); }
        }
    }
    libs.into_iter().collect::<Vec<_>>().join("\n\t")
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <pkg> <version>", "pkgrs -wpl".cyan());
        return Ok(());
    }
    let name = &args[1];
    let version = &args[2];
    let lib_name = name.replace("-", "_");
    let lib_name_rs = format!("{}_rs", lib_name);

    let pkg_dir = format!("{}/{}/{}", OVERLAY_ROOT, CATEGORY, name);
    let sources_dir = format!("{}/sources-{}", pkg_dir, version);
    let work_dir = format!("{}/build-root", sources_dir);
    
    // 1. Подготовка директорий
    if std::path::Path::new(&sources_dir).exists() {
        fs::remove_dir_all(&sources_dir)?;
    }
    fs::create_dir_all(&sources_dir)?;

    println!("{} Инициализация проекта-обертки...", ">>>".green());
    Command::new("cargo").args(["new", "--lib", "build-root"]).current_dir(&sources_dir).status()?;
    Command::new("cargo").args(["add", &format!("{}@{}", name, version)]).current_dir(&work_dir).status()?;

    // 2. Вендоринг
    println!("{} Вендоринг зависимостей...", ">>>".green());
    Command::new("cargo").args(["vendor", "../vendor"]).current_dir(&work_dir).status()?;

    // 3. Настройка Cargo.toml (Force cdylib)
    let vendor_dir = format!("{}/vendor", sources_dir);
    let pkg_folder = fs::read_dir(&vendor_dir)?
        .filter_map(|e| e.ok())
        .find(|e| e.file_name().to_string_lossy().starts_with(name))
        .expect("Пакет не найден в vendor")
        .file_name().into_string().unwrap();

    let toml_content = format!(
        r#"[package]
name = "build-root"
version = "0.1.0"
edition = "2021"

[dependencies]
{0} = {{ path = "../vendor/{1}" }}

[lib]
name = "{2}"
crate-type = ["cdylib", "rlib"]
"#, name, pkg_folder, lib_name_rs
    );
    fs::write(format!("{}/Cargo.toml", work_dir), toml_content)?;
    fs::write(format!("{}/src/lib.rs", work_dir), format!("pub use {}::*;", lib_name))?;

    // 4. Очистка зависимостей для pkg-config
    let rdepend_raw = get_system_libs(&work_dir);
    let pc_requires = rdepend_raw
        .split(|c| c == '\n' || c == '\t' || c == ' ')
        .filter(|s| !s.is_empty() && !s.contains("virtual/") && !s.contains('&') && !s.contains('[') && !s.contains('?'))
        .collect::<Vec<_>>()
        .join(", ");

    // 5. Генерация Ebuild
    let ebuild_path = format!("{}/{}-{}.ebuild", pkg_dir, name, version);
    let mut f = fs::File::create(&ebuild_path)?;
    
    writeln!(f, r#"#writer-pc-world
EAPI=8
inherit cargo

DESCRIPTION="{name} (wrapped rust lib for system integration)"
HOMEPAGE="https://crates.io/{name}"
SRC_URI=""
S="${{WORKDIR}}/build-root"

LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64"
RDEPEND="
	{rdepend_raw}"
DEPEND="${{RDEPEND}}"
BDEPEND="virtual/pkgconfig"

src_unpack() {{
	cp -Rp "{sources_dir}/"* "${{WORKDIR}}/" || die
}}

src_compile() {{
	mkdir -p .cargo || die
	echo "[source.crates-io]
replace-with = 'vendored-sources'
[source.vendored-sources]
directory = '../vendor'" > .cargo/config.toml || die

	export CARGO_HOME="${{T}}/cargo_home"
	RUSTFLAGS='-C target-cpu=native' cargo build --release --offline || die
}}

src_install() {{
	# Установка библиотек .so
	dolib.so target/release/*.so

	# Установка бинарников
	find target/release -maxdepth 1 -executable -type f ! -name "*.so" ! -name "*.a" -exec dobin {{}} + 2>/dev/null

	# Генерация .pc файлов
	local pcdir="/usr/$(get_libdir)/pkgconfig"
	local shared_pcdir="/usr/share/pkgconfig"
	dodir "${{pcdir}}" "${{shared_pcdir}}"

	cat > "${{T}}/{name}.pc" <<EOF
prefix=/usr
exec_prefix=\${{prefix}}
libdir=\${{prefix}}/$(get_libdir)
includedir=\${{prefix}}/share/cargo/registry/{name}-{version}

Name: {name}
Description: {name} native rust wrapper
Version: {version}
Requires: {pc_requires}
Libs: -L\${{libdir}} -l{lib_name}
Cflags: -I\${{includedir}}
EOF

	insinto "${{pcdir}}"
	doins "${{T}}/{name}.pc"
	insinto "${{shared_pcdir}}"
	doins "${{T}}/{name}.pc"
}}
"#, name = name, version = version, lib_name = lib_name, pc_requires = pc_requires, rdepend_raw = rdepend_raw, sources_dir = sources_dir)?;

    // 6. Финализация
    let _ = Command::new("chown").args(["-R", "portage:portage", &pkg_dir]).status();
    Command::new("ebuild").arg(&ebuild_path).arg("digest").status()?;

    println!("{} Готово. pkgrs -i -av {}/{}", ">>>".green(), CATEGORY, name);
    Ok(())
} "##),
    ];
      
 for (rel_path, content) in text_files {
        let full_path = base_path.join(rel_path.trim());
        if let Some(parent) = full_path.parent() {            
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, content)?;
    }        
    println!("Структура sys-apps/pkgrs успешно создана ✔️");
    println!("Структура sys-apps/pkgrs-man успешно создана ✔️");
    Ok(())
}
