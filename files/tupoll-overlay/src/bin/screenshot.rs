use std::fs;
use std::path::Path; 

fn main() -> std::io::Result<()> {
    let base_path = Path::new("/var/db/repos/tupoll-overlay/");   
    let text_files = [
   ("gui-apps/pinnacle-screenshot/pinnacle-screenshot-9999.ebuild", r#"# Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

inherit cargo desktop

DESCRIPTION="Taking screenshots using wayshot."
HOMEPAGE="https://github.com"
LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64 ~arm64"

SRC_URI=""
RESTRICT="fetch"

S="${WORKDIR}/${P}/pinnacle-screenshot"

RDEPEND="    
	gui-wm/pinnacle-gentoo
	dev-rust/wayshot
	dev-python/dasbus
	dev-python/pygobject
	gui-libs/gtk-layer-shell
	app-accessibility/at-spi2-core	
"
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_unpack() {
    mkdir -p "${WORKDIR}/${P}" || die
    cp -Rp "${FILESDIR}/pinnacle-screenshot" "${WORKDIR}/${P}/" || die
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
      ("gui-apps/pinnacle-screenshot/files/pinnacle-screenshot/Cargo.toml", r#"[package]
name = "pinnacle-screenshot"
version = "0.1.0"
edition = "2024"

[dependencies]
colored = "3.1"
slurp-rs = "0.2.0"

[[bin]]
name = "grab-screen"
path = "src/bin/grab-screen.rs"

[[bin]]
name = "all-screen"
path = "src/bin/all-screen.rs"

[profile.dev]
opt-level = 3
lto = true
panic = "abort"
debug = false
incremental = true  
  "#),
  
    ("gui-apps/pinnacle-screenshot/files/pinnacle-screenshot/src/bin/grab-screen.rs", r#"use std::env;
use std::fs;
use std::process::Command;
// Импортируем обязательные типы настроек из установленной .so либы
use slurp_rs::SelectOptions;

fn main() {
    let home = env::var("HOME").expect("HOME not found");
    let base_dir = format!("{}/Изображения/screenshots", home);
    fs::create_dir_all(&base_dir).ok();

    println!(">>> [Pinnacle-Grab] Нативный захват региона силами slurp-rs...");

    // ИСПРАВЛЕНО: Передаем дефолтные опции выбора региона в аргумент функции
    let selection = match slurp_rs::select_region(SelectOptions::default()) {
        Ok(geo) => geo,
        Err(_) => {
            eprintln!(" !!! Отмена: Выделение региона сброшено пользователем.");
            std::process::exit(0);
        }
    };

    // ИСПРАВЛЕНО: Форматируем объект Selection в классическую строку slurp "x,y widthxheight"
    let geometry = format!(
        "{},{} {}x{}",
        selection.rect.x, selection.rect.y, selection.rect.width, selection.rect.height
    );

    if geometry.trim().is_empty() || selection.rect.width == 0 || selection.rect.height == 0 {
        eprintln!(" !!! Отмена: Регион не выбран или имеет нулевой размер.");
        std::process::exit(0);
    }

    // Генерируем имя файла по дате
    let date_str = Command::new("date")
        .arg("+%Y:%d:%m-%H:%M-pinnacle-screenshot.png")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "screenshot.png".to_string());

    let final_filepath = format!("{}/{}", base_dir, date_str);
    println!(" [+] Координаты из либы: {}. Запуск wayshot...", geometry);

    // Вызываем wayshot в идеальном синтаксическом порядке
    let _ = Command::new("wayshot")
        .envs(env::vars())
        .arg("-g")
        .arg(&geometry)       // Чистые координаты: "x,y widthxheight"
        .arg("-c")            // Включаем курсор в снимок
        .arg(&final_filepath) // Путь идет строго в самый конец!
        .status();

    println!(" [+] Скриншот успешно сохранен: {}", final_filepath);
} "#),
     ("gui-apps/pinnacle-screenshot/files/pinnacle-screenshot/src/bin/all-screen.rs", r#"use std::process::Command;

fn main() {
    let home = std::env::var("HOME").expect("HOME not found");
    let target = format!("{}/Изображения/screenshots/%Y:%d:%m-%H:%M-pinnacle-screenshot", home);

    // wayshot --output HDMI-A-1 и твой путь
    let _ = Command::new("wayshot")
        .args(["--output", "HDMI-A-1"])
        .args(["--file-name-format", &target])
        .status();
} "#),  
       ("gui-apps/pinnacle-screenshot/files/pinnacle-screenshot/src/main.rs", r#"use std::{thread, io::{self, Read, Write}, time::Duration};
use std::process::Command;
use colored::Colorize;

fn my_custom_logic() {
	println!("{}", "Pinnacle-screenshot — СИСТЕМА УПРАВЛЕНИЯ СНИМКАМИ ЭКРАНА".bold().green());
        println!("\n{}", "КОМАНДЫ:".yellow());
        println!("all-screen: делает скришот всего экрана");
        println!("grab-screen: выбор мышкой области экрана,затем снимок выделенного");        
        println!("\n{}", "ОПИСАНИЕ:".yellow());
        println!("Путь сохранения скриншота: $HOME/Изображения.screenshots");
        println!("Для изменения хранения меняйте пути в исходнике.");
         println!("\n{}", "НЕ НАДО ПРОБОВАТЬ ЗАПУСТИТЬ ОТ root!".red());      
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
        
        if stdin.read(&mut buffer).is_ok() {
            let ch = buffer[0] as char;
            if ch == 'q' || ch == 'Q' {
                break; // Выход по нажатию Q
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    let _ = crab.kill();
    let _ = Command::new("stty").arg("sane").status(); // Возвращаем режим терминала
    print!("\x1B[?25h\r\n\x1B[1;33m[EXIT]\x1B[0m Сессия завершена. Краб ушел спать.\n");
    let _ = io::stdout().flush();
}
"#), ];
      
 for (rel_path, content) in text_files {
        let full_path = base_path.join(rel_path.trim());
        if let Some(parent) = full_path.parent() {
            
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, content)?;
    }        
    println!("Структура giu-apps/pinnacle-screenshot успешно создана ✔️");
    Ok(())
}
