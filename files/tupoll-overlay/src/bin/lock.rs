use std::fs;
use std::path::Path; 

fn main() -> std::io::Result<()> {
    let base_path = Path::new("/var/db/repos/tupoll-overlay/");   
    let text_files = [
   ("gui-apps/pinnacle-lock/pinnacle-lock-9999.ebuild", r#"# Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

inherit cargo desktop

DESCRIPTION="Notify for pinnacle-wm"
HOMEPAGE="https://github.com"
LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64 ~arm64"

SRC_URI=""
RESTRICT="fetch"

S="${WORKDIR}/${P}/pinnacle-lock"

RDEPEND="    
	gui-wm/pinnacle-gentoo
	dev-rust/kanata	
"
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_unpack() {
    mkdir -p "${WORKDIR}/${P}" || die
    cp -Rp "${FILESDIR}/pinnacle-lock" "${WORKDIR}/${P}/" || die
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
	exeinto /etc/pam.d
    doexe "pinnacle-lock"
    mkdir -p /etc/kanata
    exeinto /etc/kanata
    doexe "keymaps-lock.kbd"
    exeinto /usr/sbin
    doexe "kanata-daemon-lock.sh"
    elog "Создайте записи в /etc/sudoers.d:
    Cmnd_Alias PROCESSES = /usr/bin/nice, ..процессы.., /usr/bin/auth-rs
    Cmnd_Alias	REBOOT = /sbin/halt, /sbin/reboot, /sbin/poweroff, /usr/sbin/kanata-daemon-lock.sh
    root ALL=(ALL) ALL
    <пользователь> ALL=(ALL) ALL, NOPASSWD: REBOOT, PROCESSES"     	
}    
  "#), 
      ("gui-apps/pinnacle-lock/files/pinnacle-lock/Cargo.toml", r#"[package]
name = "pinnacle-lock"
version = "0.1.0"
edition = "2024"

[dependencies]
gtk4 = "0.10"
pam-auth = "0.5" 

[[bin]]
name = "auth-rs"
path = "src/bin/auth.rs"


[profile.dev]
opt-level = 3
lto = true
panic = "abort"
debug = false
incremental = true  "#),
  
    ("gui-apps/pinnacle-lock/files/pinnacle-lock/kanata-daemon-lock.sh", r#"#!/bin/bash

mycmd="/usr/bin/kanata --cfg /etc/kantata/keymaps-lock.kbd"
mypid="$TMP/kanata.pid"
mylog="/dev/null" # change for a log file

case "$1" in
  start)
    if [ -e "$mypid" ]; then
      echo "kanata is already running"
      exit 1
    else
      nohup $mycmd &> "$mylog" &
      echo $! > $mypid
      echo "kanata have started in background"
    fi
    ;;
  stop)
    if [ -e "$mypid" ]; then
      kill -15 $(cat "$mypid")
      rm "$mypid"
      echo "kanata have stopped"
    else
      echo "kanata is not running"
      exit 2
    fi
    ;;
  status)
    if [ -e "$mypid" ]; then
      echo "kanata is running"
    else
      echo "kanata is stopped"
    fi
    ;;
  *)
    echo "$(basename $0) start|stop|status"
    ;;
esac "#),  
      ("gui-apps/pinnacle-lock/files/pinnacle-lock/keymaps-lock.kbd", r#"(defcfg
  process-unmapped-keys yes
  
)

(defsrc
  caps grv  i
       j    k    l
  lsft rsft
  lmet lctl rctl   ;; Твои цели
)

(deflayer default
  caps grv  _      ;; caps и grv возвращаем в исходку (или твои алиасы)
       _    _    _
  _    _
  0    1    1      ;; ПЕЧАТАЮТ: Mod=0, Ctrl=1
) "#),
  
       ("gui-apps/pinnacle-lock/files/pinnacle-lock/pinnacle-lock", r#"auth    required    pam_unix.so nullok
account required    pam_unix.so "#),
  
       ("gui-apps/pinnacle-lock/files/pinnacle-lock/src/main.rs", r#"use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label, EventControllerKey};
use std::process::{Command, Stdio, exit};
use std::io::Write;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs;
use std::time::Duration;

fn main() {
    // 1. ЗАПУСК ЖЕЛЕЗНОЙ БЛОКИРОВКИ (Mod4=0, Ctrl=1)
    let _ = Command::new("sudo")
        .arg("/usr/sbin/kanata-daemon-lock.sh")
        .arg("start")
        .status();

    let app = Application::builder()
        .application_id("org.pinnacle.lock")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .fullscreened(true)
        .decorated(false)
        .build();

    // Цифра (Золото)
    let label = Label::builder()
        .label("<lock>")
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .vexpand(true)
        .css_classes(vec!["lock-tag"])
        .build();

    // Краб 🦀
    let crab_label = Label::builder()
        .label("🦀")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::End)
        .margin_bottom(40)
        .css_classes(vec!["crab"])
        .build();

    let vbox = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .build();
    vbox.append(&label);
    vbox.append(&crab_label);
    window.set_child(Some(&vbox));

    // МОНИТОРИНГ ТЭГА (500мс)
    let label_clone = label.clone();
    gtk4::glib::timeout_add_local(Duration::from_millis(500), move || {
        if let Ok(n) = fs::read_to_string("/var/tmp/wm/tag") {
            let n = n.trim();
            let display = match n {
                "1" => " 𝟏 ", "2" => " 𝟐 ", "3" => " 𝟑 ",
                "4" => " 𝟒 ", "5" => " 𝟓 ", "6" => " 𝟔 ",
                _ => " <lock> ",
            };
            label_clone.set_label(display);
        }
        gtk4::glib::ControlFlow::Continue
    });

    // АНИМАЦИЯ КРАБА (400мс)
    let crab_clone = crab_label.clone();
    let positions = [50, 150, 250, 350, 450, 550, 650, 750, 850, 950, 1050, 1150, 1250, 1350, 1450, 1550, 1650, 1750, 1650, 1550, 1450, 1350, 1250, 1150, 1050, 950, 850, 750, 650, 550, 450, 350, 250, 150];
    let mut step = 0;
    gtk4::glib::timeout_add_local(Duration::from_millis(400), move || {
        let pos = positions[step % positions.len()];
        crab_clone.set_margin_start(pos);
        step += 1;
        gtk4::glib::ControlFlow::Continue
    });

    window.set_cursor_from_name(Some("none"));

    // CSS: Синий яд и Золото
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(
        "label.lock-tag { font-size: 120pt; color: #BAA67F; } \
         label.crab { font-size: 80pt; } \
         window { background: #08071B; }"
    );
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("No display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // ОБРАБОТКА ВВОДА
    let password_buf = Rc::new(RefCell::new(String::new()));
    let key_controller = EventControllerKey::new();
    let pass_clone = password_buf.clone();

    key_controller.connect_key_pressed(move |_, key, _, _| {
        let mut p = pass_clone.borrow_mut();
        
        if key == gtk4::gdk::Key::Return {
            let mut child = Command::new("sudo")
                .arg("-n")
                .arg("/usr/bin/auth-rs")
                .stdin(Stdio::piped())
                .spawn()
                .expect("Failed to spawn auth-rs");

            if let Some(mut stdin) = child.stdin.take() {
                let _ = write!(stdin, "{}", *p);
                let _ = stdin.flush();
                drop(stdin); 
            }

            if let Ok(status) = child.wait() {
                if status.success() { 
                    // СНИМАЕМ ЖЕЛЕЗНУЮ БЛОКИРОВКУ ПЕРЕД ВЫХОДОМ
                    let _ = Command::new("sudo")
                        .arg("/usr/sbin/kanata-daemon-lock.sh")
                        .arg("stop")
                        .status();
                    exit(0); 
                }
            }
            p.clear();
        } else if key == gtk4::gdk::Key::BackSpace {
            p.pop();
        } else if let Some(c) = key.to_unicode() {
            p.push(c);
        }
        gtk4::glib::Propagation::Stop
    });

    window.add_controller(key_controller);
    window.present();
} "#), 

     ("gui-apps/pinnacle-lock/files/pinnacle-lock/src/bin/auth.rs", r#"use std::io::{self, Read, Write};
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
} "#),

];
      
 for (rel_path, content) in text_files {
        let full_path = base_path.join(rel_path.trim());
        if let Some(parent) = full_path.parent() {
            
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, content)?;
    }        
    println!("Структура giu-apps/pinnacle-lock успешно создана ✔️");
    Ok(())
}
