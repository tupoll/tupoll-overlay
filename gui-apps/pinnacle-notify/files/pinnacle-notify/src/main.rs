use std::process::Command;
use std::fs;
use std::env;
use std::path::PathBuf;

fn main() {
    // 1. Собираем пути
    let home = env::var("HOME").expect("Переменная HOME не найдена");
    let config_dir = PathBuf::from(&home).join(".config/waynotify");
    let css_path = config_dir.join("style.css");

    // 2. Создаем папку ~/.config/waynotify
    if !config_dir.exists() {
        if let Err(e) = fs::create_dir_all(&config_dir) {
            eprintln!("❌ Ошибка создания папки конфига: {}", e);
        }
    }

    // 3. Пишем эталонный ядовито-синий CSS, если файла нет
    if !css_path.exists() {
        let full_css = r#"
.notification-popup {
    background-color: #050a15;
    border: 1px solid #BAA67F;
    border-radius: 10px;
    padding: 12px;
    box-shadow: inset 0 0 15px rgba(0, 85, 255, 0.1), 0 4px 15px rgba(0, 0, 0, 0.6);
}
.notification-summary {
    color: #ffffff;
    font-weight: bold;
    font-size: 11pt;
}
.notification-body {
    font-family: "monospace";
    color: #BAA67F;
    font-size: 10pt;
    padding-top: 4px;
}
.crab-anim {
    font-size: 14pt;
    color: #BAA67F;
    text-shadow: 0 0 5px rgba(186, 166, 127, 0.4);
}
.notification-app-name {
    font-size: 0;
    opacity: 0;
}
"#;
        if let Err(e) = fs::write(&css_path, full_css) {
            eprintln!("❌ Ошибка записи style.css: {}", e);
        } else {
            println!("✨ Создан эталонный CSS: {:?}", css_path);
        }
    }

    // 4. Убиваем старые процессы waynotify
    let _ = Command::new("pkill").args(["-f", "waynotify"]).status();

    // 5. Запуск демона
    println!("🚀 Запускаю WayNotify...");
    
    let script_path = format!("/usr/sbin/waynotify");

    let status = Command::new("python3")
        .arg(script_path)
        .spawn();

    match status {
        Ok(_) => println!("✔ Демон запущен. Ожидай появления краба!"),
        Err(e) => eprintln!("❌ Не удалось запустить питон: {}", e),
    }
}
