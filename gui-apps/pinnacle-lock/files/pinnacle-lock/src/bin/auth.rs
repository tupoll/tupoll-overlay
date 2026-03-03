use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

fn main() {
    let mut password = String::new();
    let _ = io::stdin().read_to_string(&mut password);
    let pass = password.trim();

    // Пытаемся выполнить простую команду от имени текущего пользователя через 'su'
    // 'su -c whoami' спросит пароль у stdin.
    let user = std::env::var("USER").unwrap_or_else(|_| "root".into());
    
    let mut child = Command::new("su")
        .arg("-c")
        .arg("exit") // Просто входим и выходим
        .arg(&user)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to run su");

    if let Some(mut stdin) = child.stdin.take() {
        let _ = writeln!(stdin, "{}", pass);
    }

    if child.wait().map(|s| s.success()).unwrap_or(false) {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}
