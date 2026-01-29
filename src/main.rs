use std::process::Command;
use std::fs;

fn main() {
    
    let bins: Vec<String> = (1..=19).map(|i| format!("setup{}", i)).collect();

    
    for bin in &bins {
        println!("--- Запуск {} ---", bin);
        
        let status = Command::new("cargo")
            .args(["run", "--bin", bin])
            .status()
            .expect(&format!("Не удалось запустить {}", bin));

        if !status.success() {
            eprintln!("Ошибка при выполнении {}", bin);
        }
    }

    
    println!("\n--- Очистка /usr/bin ---");
    for bin in &bins {
        let path = format!("/usr/bin/{}", bin);
        if std::path::Path::new(&path).exists() {
            match fs::remove_file(&path) {
                Ok(_) => println!("Удалено: {}", path),
                Err(e) => eprintln!("Ошибка удаления {}: {}", path, e),
            }
        }
    }
}
