// Copyright 2006-2026 Gentoo Authors
// Distributed under the terms of the GNU General Public License v2

use std::env;
use std::fs;
use std::path::{Path};
use std::process::Command;

const OVERLAY_ROOT: &str = "/var/db/repos/tupoll-overlay";

fn main() -> std::io::Result<()> {
    // 1. ОПРЕДЕЛЯЕМ ТОЧКУ ЗАПУСКА ПРОЦЕССА
    let current_exe = env::current_exe().unwrap_or_default();
    let is_system_launch = current_exe.starts_with("/usr/bin");

    // Вычисляем базовые пути к домашней директории для миграции репозитория
    let home_env = env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let repo_source = format!("{}/tupoll-overlay", home_env);

    if !is_system_launch {
        // =========================================================================
        // ФАЗА 1: ПЕРВИЧНЫЙ ЗАПУСК ИЗ ДОМАШНЕЙ ПАПКИ (sudo ./tupoll-overlay)
        // =========================================================================
        println!(">>> [Rust-Core] Фаза 1: Подготовка и миграция репозитория Portage...");

        let target_app_path = format!("{}/app-portage", OVERLAY_ROOT);
        fs::create_dir_all(&target_app_path)?;

        if Path::new(&repo_source).exists() {
            println!(">>> Перенос репозитория в системное дерево: {}", target_app_path);
            let _ = Command::new("mv").args(["-f", &repo_source, &target_app_path]).status();
        }

        // Выставляем системные права доступа оверлея для Portage
        let _ = Command::new("chown").args(["-R", "portage:portage", OVERLAY_ROOT]).status();
        let _ = Command::new("chmod").args(["-R", "go+rX", OVERLAY_ROOT]).status();

        // Генерируем Manifest для ebuild оверлея
        let target_ebuild = format!("{}/tupoll-overlay/app-portage/tupoll-overlay/tupoll-overlay-9999.ebuild", target_app_path);
        println!(">>> Preparation: ebuild {} manifest", target_ebuild);
        let _ = Command::new("ebuild").arg(&target_ebuild).arg("manifest").status();

        // Ставим оверлей официально через emerge
        println!(">>> Установка пакета через emerge...");
        let _ = Command::new("emerge").args(["--oneshot", "app-portage/tupoll-overlay"]).status();

        println!(" [+] Первичная миграция и запуск сборки успешно завершены.");

    } else {
        // =========================================================================
        // ФАЗА 2: СИСТЕМНЫЙ ЗАПУСК ИЗ /usr/bin/ ЧЕРЕЗ (pkg_postinst)
        // =========================================================================
        println!(">>> [Rust-Core] Фаза 2: Запуск конвейера конфигурации Pinnacle OS...");

        // Полная обойма конфигураторов, включая специализированный ARM64 бинарник выгрузки профилей
        let configs = [
            "/usr/bin/tupoll-overlay-arm64", // Запустится и развернет ARM64-конфигурацию на Raspberry Pi 5
            "/usr/bin/tupoll-overlay",       // Самоидентификация в списке для удаления
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
        // АВТО-РАЗМАСКИРОВКА И ПОСЛЕДНИЙ В СИСТЕМЕ ВЫЗОВ EMERGE
        // =========================================================================
        println!(">>> [Rust-Core] Инжекция ключевых слов для repoman-rs и pkgrs...");
        let portage_ak_dir = "/etc/portage/package.accept_keywords";
        let portage_ak_file = format!("{}/pro", portage_ak_dir);

        if !Path::new(portage_ak_dir).exists() {
            let _ = fs::create_dir_all(portage_ak_dir);
        }

        // Прописываем ключевые слова для обеих архитектур
        let keywords_payload = "sys-apps/repoman-rs ~amd64 ~arm64\nsys-apps/pkgrs ~amd64 ~arm64\n";
        let _ = fs::write(&portage_ak_file, keywords_payload);

        println!(">>> [Rust-Core] Принудительная установка sys-apps/repoman-rs и sys-apps/pkgrs...");
        let _ = Command::new("emerge")
            .args(["--oneshot", "sys-apps/repoman-rs", "sys-apps/pkgrs"])
            .status();

        // =========================================================================
        // ТОТАЛЬНАЯ АННИГИЛЯЦИЯ ВСЕЙ КУЗНИЦЫ ИЗ /usr/bin/
        // =========================================================================
        println!(">>> Очистка утилит инициализации...");
        for config in &configs {
            if Path::new(config).exists() {
                let _ = fs::remove_file(config);
                println!("  [-] Снесен бинарник: {}", config);
            }
        }

        // Финально стираем самого себя из /usr/bin/tupoll-overlay
        if current_exe.exists() {
            let _ = fs::remove_file(current_exe);
        }

        // Зачищаем остатки исходников в домашней папке
        if Path::new(&repo_source).exists() {
            let _ = fs::remove_dir_all(&repo_source);
        }

        println!("Всё почищено 🗑. Установка Pinnacle OS полностью завершена.");
    }

    Ok(())
}
