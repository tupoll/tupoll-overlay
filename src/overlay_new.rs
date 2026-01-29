use std::fs;
use std::process::Command;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let make_conf = "/etc/portage/make.conf";
    let repos_conf = "/etc/portage/repos.conf/tupoll-overlay.conf";
    let old_path = "/var/db/repos/tupoll-overlay";
    let tmp_path = "/tmp/tupoll-overlay";

    let target_line = "PORTDIR_OVERLAY=\"/var/db/repos/tupoll-overlay${PORTDIR_OVERLAY}\"";
    let tmp_line = "PORTDIR_OVERLAY=\"/tmp/tupoll-overlay${PORTDIR_OVERLAY}\"";

    println!("🚀 Начинаю процесс...");

    // 1. Изменение tupoll-overlay.conf
    let repos_content = fs::read_to_string(repos_conf)?;
    fs::write(repos_conf, repos_content.replace(old_path, tmp_path))?;

    // 2. Изменение make.conf
    let make_content = fs::read_to_string(make_conf)?;
    let modified_make = make_content.replace(target_line, &format!("#{}\n{}", target_line, tmp_line));
    fs::write(make_conf, modified_make)?;

    // 3. Перенос существующей директории (если есть)
    if Path::new(old_path).exists() {
        Command::new("mv").args([old_path, tmp_path]).status()?;
        println!("✅ Старый оверлей перемещен в /tmp.");
    }

    // --- НОВАЯ СЕКЦИЯ: РАБОТА С GIT И СБОРКА ---
    println!("📥 Клонирование и сборка...");
    
    // Клонируем в /tmp (или в текущую папку, если нужно)
    let work_dir = "/tmp/tupoll-build";
    if Path::new(work_dir).exists() { fs::remove_dir_all(work_dir)?; }
    
    Command::new("git")
        .args(["clone", "https://github.com/tupoll/tupoll-overlay.git", work_dir])
        .status()?;

    // Переходим в директорию и собираем
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(work_dir)
        .status()?;

    if status.success() {
        println!("✅ Сборка завершена. Запуск бинарника...");
        Command::new("./tupoll-overlay")
            .current_dir(format!("{}/target/release", work_dir))
            .status()?;
    } else {
        println!("❌ Ошибка сборки Cargo.");
    }
    // ------------------------------------------

    // 4. Выполнение системных команд Portage
    println!("🔄 Обновление eix...");
    Command::new("eix-update").status()?;

    println!("🔍 Проверка emerge...");
    Command::new("emerge").args(["--ask", "app-portage/tupoll-overlay"]).status()?;

    // 5. Откат make.conf
    let current_make = fs::read_to_string(make_conf)?;
    let final_make = current_make
        .replace(&format!("{}\n", tmp_line), "")
        .replace(&format!("\n{}", tmp_line), "")
        .replace(&format!("#{}", target_line), target_line);
    fs::write(make_conf, final_make)?;

    println!("✨ Все операции завершены.");
    Ok(())
}
