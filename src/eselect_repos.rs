use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = Path::new("/etc/portage/repos.conf/eselect-repo.conf");
    let repo_header = "[guru]";
    let config_block = "\n[guru]\nlocation = /var/db/repos/guru\nsync-type = git\nsync-uri = https://github.com/gentoo-mirror/guru.git\n";

    // 1. Создаем родительские директории, если их нет
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // 2. Проверяем наличие заголовка [guru] в файле
    let already_exists = if path.exists() {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        reader.lines().any(|line| line.unwrap_or_default().trim() == repo_header)
    } else {
        false
    };

    // 3. Если записи нет — дописываем
    if !already_exists {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        
        file.write_all(config_block.as_bytes())?;
        println!("Репозиторий GURU успешно добавлен в {}", path.display());
    } else {
        println!("Конфигурация GURU уже присутствует в {}", path.display());
    }

    Ok(())
}
