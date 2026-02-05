use std::fs;
use std::path::Path; 

fn main() -> std::io::Result<()> {
    let base_path = Path::new("/var/db/repos/tupoll-overlay/");   
    let text_files = [
   ("gui-apps/pinnacle-translator/pinnacle-translator-9999.ebuild", r#"# Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

inherit cargo desktop

DESCRIPTION="Translator for pinnacle-wm"
HOMEPAGE="https://github.com"
LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64"

SRC_URI=""
RESTRICT="fetch"

S="${WORKDIR}/${P}/pinnacle-translator"

RDEPEND="    
	gui-wm/pinnacle-gentoo	
"
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_unpack() {
    mkdir -p "${WORKDIR}/${P}" || die
    cp -Rp "${FILESDIR}/pinnacle-translator" "${WORKDIR}/${P}/" || die
    #git-r3_src_unpack
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
}  "#), 

      ("gui-apps/pinnacle-translator/files/pinnacle-translator/Cargo.toml", r#"[package]
name = "pinnacle-translator"
version = "0.1.0"
edition = "2024"

[dependencies]
gtk4 = "0.10"
glib = "0.21"
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
once_cell = "1.18"
  "#),
       ("gui-apps/pinnacle-translator/files/pinnacle-translator/src/main.rs", r#"use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Orientation, ScrolledWindow, TextView, WrapMode, Spinner};
use std::fs;
use once_cell::sync::Lazy;

const TMP_PATH: &str = "/var/tmp/wm/translate.txt";

static RT: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap()
});

async fn translate_call(text: String, target: String) -> String {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36") // Более полный UA
        .build()
        .unwrap_or_default();

    let url = "https://translate.googleapis.com/translate_a/single";
    
    let res = client.get(url)
        .query(&[
            ("client", "gtx"),
            ("sl", "auto"),
            ("tl", &target),
            ("dt", "t"),
            ("ie", "UTF-8"), // Указываем входную кодировку
            ("oe", "UTF-8"), // Указываем выходную кодировку
            ("q", &text),
        ])
        .send()
        .await;

    match res {
        Ok(resp) => {
            let status = resp.status();
            let body_text = resp.text().await.unwrap_or_default();
            
            // Если сервер вернул не 200 OK, выводим статус
            if !status.is_success() {
                println!("Ошибка сервера: {}. Тело: {}", status, body_text);
                return format!("Ошибка сервера ({})", status);
            }

            // Пытаемся разобрать JSON вручную для отладки
            match serde_json::from_str::<serde_json::Value>(&body_text) {
                Ok(json) => {
                    let mut result = String::new();
                    if let Some(outer_array) = json.as_array() {
                        if let Some(segments) = outer_array.get(0).and_then(|v| v.as_array()) {
                            for segment in segments {
                                if let Some(chunk) = segment.get(0).and_then(|v| v.as_str()) {
                                    result.push_str(chunk);
                                }
                            }
                        }
                    }
                    if result.is_empty() { "Ошибка: пустой ответ".into() } else { result }
                }
                Err(e) => {
                    println!("Ошибка парсинга JSON: {}. Ответ был: {}", e, body_text);
                    if body_text.contains("<html") {
                        "Ошибка: Google заблокировал запрос (капча)".to_string()
                    } else {
                        "Ошибка: неверный формат данных".to_string()
                    }
                }
            }
        }
        Err(e) => format!("Ошибка сети: {}", e),
    }
}


fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Rust Translator")
        .default_width(600)
        .default_height(400)
        .build();

    let vbox = GtkBox::new(Orientation::Vertical, 10);
    vbox.set_margin_end(10);
    window.set_child(Some(&vbox));

    let btn_box = GtkBox::new(Orientation::Horizontal, 5);
    vbox.append(&btn_box);

    let spinner = Spinner::new();
    spinner.set_hexpand(true);
    spinner.set_halign(gtk4::Align::End);
    btn_box.append(&spinner);

    let text_view = TextView::new();
    text_view.set_wrap_mode(WrapMode::Word);
    let buffer = text_view.buffer();

    //let directions = [("RU->EN", "en"), ("EN->RU", "ru"), ("RU->ZH", "zh")];
    let directions = [
        ("RU->EN", "en"), 
        ("EN->RU", "ru"), 
        ("RU->ZH", "zh"),
        ("ZH->RU", "ru"), // Китайский -> Русский
        ("ZH->EN", "en"), // Китайский -> Английский
        ("EN->ZH", "zh"), // Английский -> Китайский
    ];


    for (label, code) in directions {
        let btn = Button::with_label(label);
        let buf_clone = buffer.clone();
        let lang = code.to_string();
        let spin_clone = spinner.clone();

        btn.connect_clicked(move |_| {
            let start = buf_clone.start_iter();
            let end = buf_clone.end_iter();
            let mut input = buf_clone.text(&start, &end, false).to_string().trim().to_string();

            if input.is_empty() {
                if let Ok(file_content) = fs::read_to_string(TMP_PATH) {
                    input = file_content.trim().to_string();
                }
            }

            if input.is_empty() { return; }

            spin_clone.start();
            buf_clone.set_text("Перевожу...");
            
            let b = buf_clone.clone();
            let l = lang.clone();
            let s = spin_clone.clone();

            // Безопасно запускаем асинхронную задачу
            glib::MainContext::default().spawn_local(async move {
                let input_str = input.clone();
                let target_lang = l.clone();
                
                // Сетевой запрос в отдельном потоке
                let handle = RT.spawn(async move {
                    translate_call(input_str, target_lang).await
                });

                let result = match handle.await {
                    Ok(res) => res,
                    Err(_) => "Ошибка выполнения запроса".to_string(),
                };

                // Возвращаемся в UI-поток
                s.stop();
                let _ = fs::write(TMP_PATH, &result);
                b.set_text(&result);
            });
        });
        btn_box.append(&btn);
    }

    vbox.append(&ScrolledWindow::builder().vexpand(true).child(&text_view).build());
    window.present();
} 


fn main() {
    let _ = fs::create_dir_all("/var/tmp/wm");
    let app = Application::builder().application_id("com.pinnacle.translator").build();
    app.connect_activate(build_ui);
    app.run();
}    
"#), ];
      
 for (rel_path, content) in text_files {
        let full_path = base_path.join(rel_path.trim());
        if let Some(parent) = full_path.parent() {
            
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, content)?;
    }        
    println!("Структура giu-apps/pinnacle-translate успешно создана.");
    Ok(())
}


      
 
