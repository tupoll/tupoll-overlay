use gtk4::prelude::*;
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
}
