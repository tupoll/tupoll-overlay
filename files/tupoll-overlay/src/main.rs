// Copyright 2006-2026 Gentoo Authors
// Distributed under the terms of the GNU General Public License v2

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

const OVERLAY_ROOT: &str = "/var/db/repos/tupoll-overlay";

fn main() -> std::io::Result<()> {
    // Получаем точку запуска текущего бинарника (он запущен из target/release/)
    let current_exe = env::current_exe().unwrap_or_default();
    let current_dir = current_exe.parent().unwrap_or_else(|| Path::new("."));

    // Вычисляем базовые пути к домашней директории для миграции репозитория
    let home_env = env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let repo_source = format!("{}/tupoll-overlay", home_env);
    let _target_repo_dir = "/var/db/repos/tupoll-overlay/app-portage/tupoll-overlay";

    // =========================================================================
    // ФАЗА 1: ЛОКАЛЬНОЕ РАЗВЕРТЫВАНИЕ И МИГРАЦИЯ (Запуск руками из target/release)
    // =========================================================================
    println!(">>> [Rust-Core] Фаза 1: Локальное развертывание и миграция репозитория...");

    // 1. Создаем структуру и переносим репозиторий на его ПМЖ в дерево Portage
    let target_app_path = format!("{}/app-portage", OVERLAY_ROOT);
    fs::create_dir_all(&target_app_path)?;

    if Path::new(&repo_source).exists() {
        println!(">>> Перенос репозитория в системное дерево: {}", target_app_path);
        let _ = Command::new("mv").args(["-f", &repo_source, &target_app_path]).status();
    }

    // 2. Выставляем системные права доступа оверлея для Portage
    let _ = Command::new("chown").args(["-R", "portage:portage", OVERLAY_ROOT]).status();
    let _ = Command::new("chmod").args(["-R", "go+rX", OVERLAY_ROOT]).status();

    // 3. ХИРУРГИЧЕСКИЙ ПЕРЕНОС ГОТОВОЙ КУЗНИЦЫ БИНАРНИКОВ В /usr/bin/
    println!(">>> [Rust-Core] Регистрация скомпилированных утилит в системе...");
    
    // Перечень бинарников, которые Cargo УЖЕ собрал в вашей папке target/release/
    let binary_names = [
        "tupoll-overlay-amd64", "tupoll-overlay-arm64", "pinnacle-install",
        "translator-config", "pinnacle-terminal-config", "pinnacle-wallpaper-config",
        "pinnacle-notify-config", "pinnacle-lock-config", "pinnacle-screenshot-config",
        "pinnacle-fm-config", "pkgrs-install", "crucian-config",
        "repoman-config", "servo-install", "pkgrs-utils-config",
    ];

    // Копируем готовые файлы прямо из папки запуска инсталлятора в /usr/bin
    for bin in &binary_names {
        let local_bin_path = current_dir.join(bin);
        if local_bin_path.exists() {
            let system_bin_path = format!("/usr/bin/{}", bin);
            fs::copy(&local_bin_path, &system_bin_path)?;
            
            // Выставляем права на исполнение
            let _ = Command::new("chmod").args(["+x", &system_bin_path]).status();
        }
    }

    // =========================================================================
    // ФАЗА 2: СКВОЗНОЙ ЗАПУСК КОНВЕЙЕРА (В рамках того же процесса)
    // =========================================================================
    println!(">>> [Rust-Core] Фаза 2: Запуск конвейера конфигурации Pinnacle OS...");

    // Полная архитектурная обойма утилит в /usr/bin/
    let configs = [
        "/usr/bin/tupoll-overlay-amd64", 
        "/usr/bin/tupoll-overlay-arm64", 
        "/usr/bin/pinnacle-install",
        "/usr/bin/translator-config",
        "/usr/bin/pinnacle-terminal-config",
        "/usr/bin/pinnacle-wallpaper-config",
        "/usr/bin/pinnacle-notify-config",
        "/usr/bin/pinnacle-lock-config",
        "/usr/bin/pinnacle-screenshot-config",
        "/usr/bin/pinnacle-fm-config",
        "/usr/bin/pkgrs-install",
        "/usr/bin/crucian-config",
        "/usr/bin/repoman-config",
        "/usr/bin/servo-install",
        "/usr/bin/pkgrs-utils-config",
    ];

    // Последовательно исполняем утилиты кузницы настроек через Fish
    for config in &configs {
        if Path::new(config).exists() {
            let _ = Command::new("fish").args(["-c", config]).status();
        }
    }

    // =========================================================================
    // ТОТАЛЬНАЯ АННИГИЛЯЦИЯ ВСЕЙ КУЗНИЦЫ (ОЧИСТКА СИСТЕМЫ)
    // =========================================================================
    println!(">>> Очистка утилит инициализации...");
    
    // Сносим все внешние сателлиты из /usr/bin/
    for config in &configs {
        if Path::new(config).exists() {
            let _ = fs::remove_file(config);
            println!("  [-] Снесен бинарник: {}", config);
        }
    }

    // Полностью уничтожаем временную папку клонирования в домашней директории
    if Path::new(&repo_source).exists() {
        let _ = fs::remove_dir_all(&repo_source);
    }

    // И в самом финале текущий запущенный бинарник стирает сам себя из target/release/
    if current_exe.exists() {
        let _ = fs::remove_file(current_exe);
        println!("  [-] Временный оркестратор успешно самоликвидировался.");
    }

    println!("Всё почищено 🗑. Исходники ebuild сохранены. Установка Pinnacle OS полностью завершена.");
    Ok(())
}
