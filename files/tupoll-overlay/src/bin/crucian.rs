use std::fs;
use std::path::Path; 

fn main() -> std::io::Result<()> {
    let base_path = Path::new("/var/db/repos/tupoll-overlay/");   
    let text_files = [
   ("gui-apps/crucian/crucian-9999.ebuild", r#"# Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

inherit cargo desktop

DESCRIPTION="AI Context Handler"
HOMEPAGE="https://github.com"
LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64 ~arm64"

SRC_URI=""
RESTRICT="fetch"

S="${WORKDIR}/${P}/crucian"

RDEPEND="    
	gui-apps/pinnacle-notify
	sci-ml/ollama
	sci-libs/openblas	
"
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_unpack() {
    mkdir -p "${WORKDIR}/${P}" || die
    cp -Rp "${FILESDIR}/crucian" "${WORKDIR}/${P}/" || die
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
	domenu "crucian.desktop"
	domenu "crucian-cli.desktop"	
}   
    
  "#), 
      ("gui-apps/crucian/files/crucian/Cargo.toml", r#"[package]
name = "crucian"
version = "0.1.0"
edition = "2024"

[dependencies]
gtk4 = "0.11"        
glib = "0.22"        
pango = "0.22"       
colored = "3.1"

reqwest = { version = "0.11", features = ["blocking", "json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
parking_lot = "0.12"
libc = "0.2.186"

[[bin]]
name = "file_opener"
path = "src/bin/file_opener.rs"

[[bin]]
name = "file_creator"
path = "src/bin/file_creator.rs"

[[bin]]
name = "crucian-cli"
path = "src/bin/crucian-cli.rs"

[[bin]]
name = "crucian-man"
path = "src/bin/crucian-man.rs"

[profile.dev]
opt-level = 3
lto = true
panic = "abort"
debug = false
incremental = true "#),
  
    ("gui-apps/crucian/files/crucian/crucian.desktop", r#"[Desktop Entry]
Type=Application
Version=1.0
Name=Crucian
GenericName=AI Code Assistant & Editor
Comment=Локальный AI-ассистент и редактор кода на базе Ollama
Exec=/usr/bin/crucian %F
Icon=smartcard
Terminal=false
Categories=Development;IDE;GTK;
MimeType=text/plain;text/x-rust;text/x-python;text/x-lua;text/x-fish; "#),
      ("gui-apps/crucian/files/crucian/crucian-cli.desktop", r#"[Desktop Entry]
Type=Application
Version=1.0
Name=Crucian (Контекст ИИ)
GenericName=AI Context Handler
Comment=Открыть файлы как контекст для ИИ-ассистента Crucian
Exec=/usr/bin/crucian-cli %F
Icon=brain
Terminal=false
NoDisplay=false
Categories=Utility;Development;
MimeType=text/plain;text/x-rust;text/x-python;text/x-lua;text/x-fish;text/x-shellscript; "#),
       ("gui-apps/crucian/files/crucian/src/main.rs", r##"use gtk4::prelude::*;
use gtk4::{
    glib, glib::signal::SignalHandlerId, glib::Propagation, Application, ApplicationWindow, Box, Button, DropDown,
    Entry, Label, Paned, ScrolledWindow, TextTag, TextView,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::{mpsc, Arc};
use std::{fs, thread, time::Duration};
use gtk4::glib::translate::IntoGlib;

mod syntax;
mod search;
#[derive(Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Deserialize)]
struct ChatResponse {
    message: Option<Message>,
    done: bool,
}

#[derive(Deserialize)]
struct OllamaModelsResponse {
    models: Vec<LocalModel>,
}

#[derive(Deserialize)]
struct LocalModel {
    name: String,
}

fn get_history_path() -> std::path::PathBuf {
    glib::user_config_dir().join("crucian/history.json")
}

fn save_history(history: &[Message]) {
    let path = get_history_path();
    if let Ok(json) = serde_json::to_string_pretty(history) {
        let _ = fs::write(path, json);
    }
}

fn load_history() -> Vec<Message> {
    let path = get_history_path();
    if let Ok(data) = fs::read_to_string(path) {
        if let Ok(history) = serde_json::from_str::<Vec<Message>>(&data) {
            return history;
        }
    }
    vec![]
}

fn fetch_local_models() -> Vec<String> {
    let client = reqwest::blocking::Client::new();
    match client.get("http://localhost:11434/api/tags").send() {
        Ok(res) => match res.json::<OllamaModelsResponse>() {
            Ok(json) => json.models.into_iter().map(|m| m.name).collect(),
            Err(_) => vec!["qwen2:1.5b".into()],
        },
        Err(_) => vec!["ollama_offline".into()],
    }
}

fn ask_ollama(
    model: String,
    history_arc: Arc<RwLock<Vec<Message>>>,
    tx_text: mpsc::Sender<String>,
    tx_status: mpsc::Sender<String>,
) {
    let client = reqwest::blocking::Client::new();
    let _ = tx_status.send(format!("✍️ {} думает...", model));

    let payload = ChatRequest {
        model,
        messages: history_arc.read().clone(),
        stream: true,
    };

    if let Ok(response) = client.post("http://localhost:11434/api/chat").json(&payload).send() {
        let reader = std::io::BufReader::new(response);
        for line in std::io::BufRead::lines(reader).flatten() {
            if let Ok(json) = serde_json::from_str::<ChatResponse>(&line) {
                if let Some(msg) = json.message {
                    let _ = tx_text.send(msg.content);
                }
                if json.done {
                    break;
                }
            }
        }
        let _ = tx_status.send("✅ Готово".into());
    }
}

fn build_ui(app: &Application) {
	// 🔥 СБРОС ПАМЯТИ ПОИСКА ПРИ СТАРТЕ ПРИЛОЖЕНИЯ
    // Удаляем старый файл с tmpfs, чтобы новая сессия всегда начиналась с чистого листа (с 0)
    let _ = std::fs::remove_file("/var/tmp/wm/crucian_search");

    // Инициализируем GLib/GTK и запускаем ваше приложение
    gtk4::init().expect("Не удалось инициализировать GTK");
    
    // ... далее ваш неизменный код запуска Application ...

    if app.active_window().is_some() {
        return;
    }

    syntax::load_css();

    let win = ApplicationWindow::builder()
        .application(app)
        .title("Crucian")
        .default_width(1400)
        .default_height(800)
        .build();
    let search_entry = gtk4::Entry::builder()
        .placeholder_text("🔍 Найти в коде...")
        .width_request(180)
        .build();
    let main_horizontal_paned = Paned::new(gtk4::Orientation::Horizontal);
    main_horizontal_paned.set_position(400); 
    win.set_child(Some(&main_horizontal_paned));

    let right_vertical_paned = Paned::new(gtk4::Orientation::Vertical);
    right_vertical_paned.set_position(380); 
    main_horizontal_paned.set_end_child(Some(&right_vertical_paned));

    // --- 1. ЛЕВАЯ ПАНЕЛЬ: ЧАТ ---
    let chat_vbox = Box::new(gtk4::Orientation::Vertical, 10);
    chat_vbox.set_margin_start(10); chat_vbox.set_margin_end(10);
    chat_vbox.set_margin_top(10); chat_vbox.set_margin_bottom(10);
    main_horizontal_paned.set_start_child(Some(&chat_vbox));
    chat_vbox.add_css_class("chat-pane");

    let header_box = Box::new(gtk4::Orientation::Horizontal, 5);
    let available_models = fetch_local_models();
    let model_dropdown = DropDown::from_strings(
        &available_models.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
    );
    let status_label = Label::builder().label("Готов").xalign(0.0).build();
    let spacer = Box::builder().hexpand(true).build(); 
    let open_file_btn = Button::with_label("📂 Открыть");
    let clear_btn = Button::with_label("🗑 Очистить");

        // В левой панели просто добавляем её (саму строку создания отсюда удаляем!):
    header_box.append(&model_dropdown);
    header_box.append(&status_label);
    header_box.append(&spacer);
    header_box.append(&search_entry); // Эта строчка остается неизменной
    header_box.append(&open_file_btn);
    header_box.append(&clear_btn);
    chat_vbox.append(&header_box);


    let ai_view = TextView::builder()
        .editable(false)
        .cursor_visible(false)
        .wrap_mode(gtk4::WrapMode::WordChar)
        .build();

    let buffer = ai_view.buffer();
    buffer.tag_table().add(&TextTag::builder().name("bold").weight(pango::Weight::Bold.into_glib()).build());
    buffer.tag_table().add(&TextTag::builder().name("code").family("monospace").background("#2d2d2d").foreground("#cccccc").paragraph_background("#2d2d2d").build());

    let scroll = ScrolledWindow::builder().vexpand(true).child(&ai_view).build();
    chat_vbox.append(&scroll);

    let saved_data = load_history();
    for msg in &saved_data {
        let mut iter = buffer.end_iter();
        if msg.role == "user" {
            buffer.insert_with_tags_by_name(&mut iter, &format!("\nUser: {}\n", msg.content), &["bold"]);
        } else {
            buffer.insert(&mut iter, "AI:\n");
            let parts = msg.content.split("```");
            let mut is_code_part = false;
            for part in parts {
                let mut end_iter = buffer.end_iter();
                if is_code_part {
                    let clean_code = if part.starts_with('\n') { part } else { part.split_once('\n').map(|x| x.1).unwrap_or(part) };
                    buffer.insert_with_tags_by_name(&mut end_iter, clean_code, &["code"]);
                } else {
                    buffer.insert(&mut end_iter, part);
                }
                is_code_part = !is_code_part;
            }
            buffer.insert(&mut buffer.end_iter(), "\n");
        }
    }

    let input_entry = Entry::builder().placeholder_text("Введите сообщение...").build();
    chat_vbox.append(&input_entry);

    // --- 2. ВЕРХНЯЯ ШИРОКАЯ ПАНЕЛЬ: ЧИСТЫЙ ВЫВОД КОДА ИИ ---
    let code_vbox = Box::new(gtk4::Orientation::Vertical, 5);
    code_vbox.set_margin_start(10); code_vbox.set_margin_end(10);
    code_vbox.set_margin_top(10); code_vbox.set_margin_bottom(10);
    right_vertical_paned.set_start_child(Some(&code_vbox));
    code_vbox.add_css_class("code-pane");

    let code_title_label = Label::builder().label("💻 Код ответа ИИ").xalign(0.0).build();
    code_vbox.append(&code_title_label);

    let code_view = TextView::builder()
        .editable(false)
        .wrap_mode(gtk4::WrapMode::None)
        .monospace(true)
        .build();
    
    code_view.buffer().tag_table().add(&TextTag::builder().name("code_block").background("#1e1e1e").foreground("#d4d4d4").build());

    let code_scroll = ScrolledWindow::builder().vexpand(true).child(&code_view).build();
    code_vbox.append(&code_scroll);

            // --- 3. НИЖНЯЯ ПАНЕЛЬ: РЕДАКТОР С НУМЕРАЦИЕЙ СТРОК ---
    let file_vbox = Box::new(gtk4::Orientation::Vertical, 5);
    file_vbox.set_margin_start(10); file_vbox.set_margin_end(10);
    file_vbox.set_margin_top(10); file_vbox.set_margin_bottom(10);
    right_vertical_paned.set_end_child(Some(&file_vbox));
    file_vbox.add_css_class("file-pane");

    // НАСТРОЙКА РАЗДЕЛИТЕЛЯ: разрешаем нижней панели адаптивно сжиматься
    right_vertical_paned.set_resize_end_child(true);
    right_vertical_paned.set_shrink_end_child(true);

    let file_header_box = Box::new(gtk4::Orientation::Horizontal, 10);
    let file_title_label = Label::builder()
        .label("📄 Системный файл не выбран")
        .xalign(0.0)
        .hexpand(true)
        .build();
        
    let create_file_btn = Button::with_label("➕ Создать");
    let save_file_btn = Button::with_label("💾 Сохранить");
    save_file_btn.set_sensitive(false); 

    file_header_box.append(&file_title_label);
    file_header_box.append(&create_file_btn); 
    file_header_box.append(&save_file_btn);
    file_vbox.append(&file_header_box);

    // Контейнер для выравнивания номеров строк и самого кода по горизонтали
    let editor_layout = Box::new(gtk4::Orientation::Horizontal, 0);
    editor_layout.set_vexpand(true);
    editor_layout.set_hexpand(true);
    file_vbox.append(&editor_layout);

    // 1. Создаем основное поле кода
    let file_view = TextView::builder()
        .editable(true)
        .wrap_mode(gtk4::WrapMode::None) 
        .monospace(true)                 
        .build();

    // 2. Создаем контейнер прокрутки кода (Здесь полоса БУДЕТ отображаться)
    let file_scroll = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true) 
        .child(&file_view)
        .build();

    // 3. Вытаскиваем вертикальный Adjustment прямо из контейнера прокрутки кода
    let code_adjustment = file_scroll.vadjustment();

    // 4. Создаем поле номеров строк
    let line_numbers_view = TextView::builder()
        .editable(false)
        .cursor_visible(false)
        .can_focus(false)
        .monospace(true)
        .build();
    line_numbers_view.add_css_class("line-numbers");
    line_numbers_view.set_width_request(40);

    // 5. Оборачиваем номера строк в ScrolledWindow 
    // ВАЖНО: Ставим Always, чтобы включить логику скроллинга, и вешаем класс для CSS-маскировки
    let line_scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Always) 
        .vadjustment(&code_adjustment)              
        .child(&line_numbers_view)
        .build();
    line_scroll.add_css_class("hidden-scrollbar");

    // 6. Добавляем оба контейнера в горизонтальный Box
    editor_layout.append(&line_scroll);
    editor_layout.append(&file_scroll);

    // 7. ИСЧЕЗНОВЕНИЕ СТРАШНОЙ ВТОРОЙ ПОЛОСЫ ЧЕРЕЗ CSS
    // Физически прокрутка работает, но визуально полоса становится невидимой и сжимается в 0 пикселей
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data(".hidden-scrollbar scrollbar { min-width: 0; width: 0; opacity: 0; padding: 0; margin: 0; }");
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &css_provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    // Инициализируем хранилища пути и ID сигналов
let current_file_path = Arc::new(RwLock::new(None::<std::path::PathBuf>));
let syntax_handler_id = Arc::new(RwLock::new(None::<SignalHandlerId>));

let file_buffer = file_view.buffer();

// Регистрируем теги подсветки
file_buffer.tag_table().add(&TextTag::builder().name("syntax_keyword").foreground("#569cd6").weight(pango::Weight::Bold.into_glib()).build());
file_buffer.tag_table().add(&TextTag::builder().name("syntax_type").foreground("#4ec9b0").build());
file_buffer.tag_table().add(&TextTag::builder().name("syntax_comment").foreground("#7f7f7f").style(pango::Style::Italic).build());

// ДОБАВИТЬ СЮДА:
file_buffer.tag_table().add(&TextTag::builder().name("syntax_string").foreground("#ce9178").build());
file_buffer.tag_table().add(&TextTag::builder().name("syntax_number").foreground("#b5cea8").build());
// В src/main.rs к остальным тегам:
file_buffer.tag_table().add(&TextTag::builder().name("syntax_function").foreground("#dcdcaa").build());
file_buffer.tag_table().add(&TextTag::builder().name("syntax_bracket").foreground("#808080").build());

// Клонируем переменные для перемещения в замыкание
let daemon_id_clone = syntax_handler_id.clone();
let path_clone = current_file_path.clone();
let line_nums_clone = line_numbers_view.clone(); // Больше не будет warning!

let id = file_buffer.connect_changed(move |buf| {
    // 1. Пытаемся получить расширение из пути файла
    let ext_storage = path_clone.read();
    let raw_extension = ext_storage
        .as_ref()
        .and_then(|path| path.extension())
        .and_then(|ext| ext.to_str());

    // 2. Умное определение: приоритет имени, затем шебанг в шапке
   // let final_extension = detect_file_extension(buf, raw_extension);
   let final_extension = syntax::detect_file_extension(buf, raw_extension);
    // 3. Запускаем раскраску кода (передаем как &str)
    syntax::run_syntax_daemon(buf, &daemon_id_clone, final_extension.as_deref());

    // 4. Восстановление нумерации строк
    let line_count = buf.line_count(); 
    let mut numbers_text = String::with_capacity(line_count as usize * 4);
    for i in 1..=line_count {
        use std::fmt::Write;
        let _ = writeln!(numbers_text, "{}", i);
    }

    let nums_buf = line_nums_clone.buffer();
    nums_buf.set_text(&numbers_text);
});


// Сохраняем ID, чтобы демон мог блокировать сигнал
*syntax_handler_id.write() = Some(id);

    let f_view = file_view.clone();
    let f_title = file_title_label.clone();
    let f_save_btn = save_file_btn.clone();
    let file_win = win.clone();
    let path_open_clone = current_file_path.clone();
    
    open_file_btn.connect_clicked(move |_| {
        let file_chooser = gtk4::FileChooserDialog::builder()
            .title("Открыть текстовый файл")
            .transient_for(&file_win)
            .action(gtk4::FileChooserAction::Open)
            .build();

        file_chooser.add_button("Отмена", gtk4::ResponseType::Cancel);
        file_chooser.add_button("Открыть", gtk4::ResponseType::Accept);

        let view = f_view.clone();
        let title = f_title.clone();
        let s_btn = f_save_btn.clone();
        let path_dialog_clone = path_open_clone.clone();

file_chooser.connect_response(move |dialog, response| {
if response == gtk4::ResponseType::Accept {
if let Some(file) = dialog.file() {
if let Some(path) = file.path() {
let std_path: std::path::PathBuf = path;
if let Ok(content) = fs::read_to_string(&std_path) {
let name = std_path.file_name().unwrap_or_default().to_string_lossy();
title.set_label(&format!("📄 Системный файл: {}", name));
view.buffer().set_text(&content);
let mut path_guard = path_dialog_clone.write();
*path_guard = Some(std_path);
s_btn.set_sensitive(true);
}
}
}
}
dialog.destroy();
});
file_chooser.show();
});
    // 1. Обработчик клика по кнопке "➕ Создать"
    let create_buffer = file_view.buffer();
    let create_title = file_title_label.clone();
    let create_save_btn = save_file_btn.clone();
    let path_create_clone = current_file_path.clone();
    let create_view_focus = file_view.clone(); // Клонируем сам виджет для передачи фокуса
    let create_status = status_label.clone();
    
    create_file_btn.connect_clicked(move |_| {
        create_buffer.set_text(""); // Начисто очищаем поле
        create_title.set_label("📄 Новый файл (Не сохранен)");
        create_status.set_label("✍️ Наберите текст кода и нажмите Ctrl+S для сохранения...");
        
        *path_create_clone.write() = None; // Сбрасываем путь
        create_save_btn.set_sensitive(true); // Разрешаем сохранение
        
        // ПРАВКА: Принудительно ставим курсор внутрь текстового поля, чтобы было видно, где писать!
        create_view_focus.grab_focus();
    });

let save_buffer = file_view.buffer();
let save_status = status_label.clone();
let save_title = file_title_label.clone();
let path_save_clone = current_file_path.clone();
    save_file_btn.connect_clicked(move |_| {
        let mut path_guard = path_save_clone.write();
        
        // Если файла еще нет на диске — запрашиваем путь через наш b-файл file_creator
        if path_guard.is_none() {
            use std::process::Command;
            // Автоматически находим папку, где лежит сам запущенный crucian
            if let Ok(mut exe_path) = std::env::current_exe() {
                exe_path.set_file_name("file_creator"); // Меняем crucian на file_creator в пути
                
                match Command::new(&exe_path).output() {
                    Ok(output) => {
                        let stdout_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                        if !stdout_str.is_empty() {
                            *path_guard = Some(std::path::PathBuf::from(stdout_str));
                        }
                    }
                    Err(e) => {
                        save_status.set_label(&format!("❌ Не нашли утилиту сохранения: {}", e));
                        return;
                    }
                }
            }
        }

        // Если путь есть (или только что получили) — записываем текст
        if let Some(ref path) = *path_guard {
            let start = save_buffer.start_iter();
            let end = save_buffer.end_iter();
            let text = save_buffer.text(&start, &end, false);
            
            if fs::write(path, text.as_str()).is_ok() {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                save_title.set_label(&format!("📄 Системный файл: {}", name));
                save_status.set_label("💾 Файл успешно сохранен!");
            }
        }
    });

let chat_history = Arc::new(RwLock::new(saved_data));
let (tx_text, rx_text) = mpsc::channel::<String>();
    let (tx_status, rx_status) = mpsc::channel::<String>();
let gesture = gtk4::GestureClick::new();
ai_view.add_controller(gesture.clone());
let status_click = status_label.clone();
let ai_view_click = ai_view.clone();
gesture.connect_pressed(move |_, _, x, y| {
if let Some(iter) = ai_view_click.iter_at_location(x as i32, y as i32) {
let is_code = iter.tags().iter().any(|t| t.name().map_or(false, |n| n == "code"));
if is_code {
let mut start = iter.clone();
let mut end = iter.clone();
while !start.is_start() {
let mut prev = start.clone();
prev.backward_char();
if !prev.tags().iter().any(|t| t.name().map_or(false, |n| n == "code")) { break; }
start.backward_char();
}
while !end.is_end() {
let mut next = end.clone();
next.forward_char();
if !next.tags().iter().any(|t| t.name().map_or(false, |n| n == "code")) { break; }
end.forward_char();
}
let code = ai_view_click.buffer().text(&start, &end, false);
ai_view_click.display().clipboard().set_text(code.as_str());
status_click.set_label("📋 Код скопирован!");
}
}
});
let clear_history = chat_history.clone();
let clear_buffer = buffer.clone();
let clear_status = status_label.clone();
clear_btn.connect_clicked(move |_| {
let mut guard = clear_history.write();
guard.clear();
save_history(&guard);
clear_buffer.set_text("");
clear_status.set_label("🗑 История очищена");
});
let entry_history = chat_history.clone();
let entry_buffer = buffer.clone();
let available_models_entry = available_models.clone();
let model_dropdown_entry = model_dropdown.clone();
input_entry.connect_activate(move |entry| {
let text = entry.text().to_string();
if text.trim().is_empty() { return; }
entry.set_text("");
let selected_idx = model_dropdown_entry.selected() as usize;
let selected_model = available_models_entry.get(selected_idx).cloned().unwrap_or_else(|| "qwen2:1.5b".into());
let mut guard = entry_history.write();
guard.push(Message { role: "user".into(), content: text.clone() });
save_history(&guard);
let mut iter = entry_buffer.end_iter();
entry_buffer.insert_with_tags_by_name(&mut iter, &format!("\nUser: {}\n\nAI:\n", text), &["bold"]);
let tx_text_clone = tx_text.clone();
let tx_status_clone = tx_status.clone();
let history_send = entry_history.clone();
thread::spawn(move || {
ask_ollama(selected_model, history_send, tx_text_clone, tx_status_clone);
});
});
let buf_gui = buffer.clone();
let scroll_gui = scroll.clone();
let status_gui = status_label.clone();
let history_gui = chat_history.clone();
let c_view = code_view.clone();
let mut is_code = false;
let mut current_ai_text = String::new();
let mut token_accumulator = String::new();
glib::timeout_add_local(Duration::from_millis(20), move || {
while let Ok(s) = rx_status.try_recv() {
status_gui.set_label(&s);
if s == *"✅ Готово" {
let mut guard = history_gui.write();
guard.push(Message { role: "assistant".into(), content: current_ai_text.clone() });
save_history(&guard);
current_ai_text.clear();
token_accumulator.clear();
}
}
let mut text_changed = false;
while let Ok(t) = rx_text.try_recv() {
current_ai_text.push_str(&t);
token_accumulator.push_str(&t);
text_changed = true;
while let Some(pos) = token_accumulator.find("```") {
let before_marker = token_accumulator[..pos].to_string();
let mut iter = buf_gui.end_iter();
if !before_marker.is_empty() {
if is_code {
buf_gui.insert_with_tags_by_name(&mut iter, &before_marker, &["code"]);
let mut c_iter = c_view.buffer().end_iter();
c_view.buffer().insert_with_tags_by_name(&mut c_iter, &before_marker, &["code_block"]);
}
else { buf_gui.insert(&mut iter, &before_marker); }
}
is_code = !is_code;
buf_gui.insert(&mut buf_gui.end_iter(), "\n");
if !is_code {
let mut c_iter = c_view.buffer().end_iter();
c_view.buffer().insert(&mut c_iter, "\n--- Конец блока ---\n\n");
}
token_accumulator = token_accumulator[pos + 3..].to_string();
if is_code {
if let Some(nl_pos) = token_accumulator.find('\n') {
token_accumulator = token_accumulator[nl_pos + 1..].to_string();
}
}
}
}
if text_changed && !token_accumulator.is_empty() {
let mut iter = buf_gui.end_iter();
if is_code {
buf_gui.insert_with_tags_by_name(&mut iter, &token_accumulator, &["code"]);
let mut c_iter = c_view.buffer().end_iter();
c_view.buffer().insert_with_tags_by_name(&mut c_iter, &token_accumulator, &["code_block"]);
}
else { buf_gui.insert(&mut iter, &token_accumulator); }
token_accumulator.clear();
}
if text_changed {
let adj = scroll_gui.vadjustment();
adj.set_value(adj.upper() - adj.page_size());
}
glib::ControlFlow::Continue
});
// --- ПОДКЛЮЧЕНИЕ ЦИКЛИЧЕСКОГО ПОИСКА В РЕДАКТОР ФАЙЛОВ ---
      let search_buffer_clone = file_view.buffer();
    let search_view_clone = file_view.clone();
    let search_entry_clone = search_entry.clone(); // Клонируем для передачи внутрь

    search_entry.connect_activate(move |_| {
        // Вызываем функцию поиска, передавая буфер, вью редактора и саму поисковую строку
        search::find_text(&search_buffer_clone, &search_view_clone, &search_entry_clone);
    });
 


let controller = gtk4::EventControllerKey::new();
let shortcut_buffer = file_view.buffer();
let shortcut_status = status_label.clone();
let shortcut_title = file_title_label.clone();
let path_shortcut_clone = current_file_path.clone();
    controller.connect_key_pressed(move |_, keyval, _, state| {
        if (keyval.name() == Some("S".into()) || keyval.name() == Some("s".into())) 
            && state.contains(gtk4::gdk::ModifierType::CONTROL_MASK) 
        {
            let mut path_guard = path_shortcut_clone.write();
            
            if path_guard.is_none() {
                use std::process::Command;
                if let Ok(mut exe_path) = std::env::current_exe() {
                    exe_path.set_file_name("file_creator");
                    
                    match Command::new(&exe_path).output() {
                        Ok(output) => {
                            let stdout_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                            if !stdout_str.is_empty() {
                                *path_guard = Some(std::path::PathBuf::from(stdout_str));
                            }
                        }
                        Err(e) => {
                            shortcut_status.set_label(&format!("❌ Ошибка вызова сохранения: {}", e));
                            return Propagation::Stop;
                        }
                    }
                }
            }

            if let Some(ref path) = *path_guard {
                let start = shortcut_buffer.start_iter();
                let end = shortcut_buffer.end_iter();
                let text = shortcut_buffer.text(&start, &end, false);
                
                if fs::write(path, text.as_str()).is_ok() {
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    shortcut_title.set_label(&format!("📄 Системный файл: {}", name));
                    shortcut_status.set_label("💾 Файл сохранен через Ctrl+S!");
                }
            }
            return Propagation::Stop;
        }
        Propagation::Proceed
    });

    file_view.add_controller(controller);

           // === НАШ НОВЫЙ МГНОВЕННЫЙ ПЕРЕХВАТ ПЕРЕД ОТРИСОВКОЙ ОКНА ===
    let _file_to_open = std::env::var("CRUCIAN_OPEN_FILE").unwrap_or_default();

       // === НАШ НОВЫЙ МГНОВЕННЫЙ ПЕРЕХВАТ ПЕРЕД ОТРИСОВКОЙ ОКНА ===
    let file_to_open = std::env::var("CRUCIAN_OPEN_FILE").unwrap_or_default();

    if !file_to_open.is_empty() {
        let target_path = std::path::PathBuf::from(&file_to_open);
        if target_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&target_path) {
                // Превращаем в независимую строку, чтобы избежать ошибки заимствования E0505
                let name = target_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                
                // 1. Меняем заголовок панели
                file_title_label.set_label(&format!("📄 Системный файл: {}", name));
                
                // 2. Теперь target_path можно безопасно переместить в замок
                {
                    let mut path_guard = current_file_path.write();
                    *path_guard = Some(target_path);
                }
                
                // 3. Активируем кнопку "Сохранить"
                save_file_btn.set_sensitive(true);
                
                // 4. Записываем текст в буфер (сработает триггер раскраски и строк)
                file_view.buffer().set_text(&content);
                
                // 5. Переводим фокус ввода на редактор
                file_view.grab_focus();
                
                // 6. Обновляем статус-бар напрямую
                status_label.set_label(&format!("📂 Файл '{}' успешно загружен", name));
            }
        }
        // Оборачиваем в unsafe блок, как этого требуют современные версии Rust
        unsafe {
            let _ = std::env::remove_var("CRUCIAN_OPEN_FILE");
        }
    }

    win.show();
}


fn main() {
	     
   
let app = Application::builder()
.application_id("com.example.crucian")
.flags(gtk4::gio::ApplicationFlags::HANDLES_OPEN)
.build();
app.connect_activate(build_ui);
app.connect_open(|app, _files, _hint| {
build_ui(app);
});
app.run_with_args(&[""]);
} "##),
       ("gui-apps/crucian/files/crucian/src/search.rs", r##"use gtk4::prelude::*;
use gtk4::{TextBuffer, TextIter, TextSearchFlags, Entry};
use std::fs;
use std::path::Path;

const MEMORY_FILE: &str = "/var/tmp/wm/crucian_search";

/// Циклический поиск текста с удержанием фокуса ввода для беспрерывного чередования по Enter.
pub fn find_text(buffer: &TextBuffer, view: &gtk4::TextView, search_entry: &Entry) -> bool {
    let search_str = search_entry.text().to_string();
    let trimmed = search_str.trim();

    // 1. Сбрасываем старую желтую маркерную подсветку
    buffer.remove_tag_by_name("search_match", &buffer.start_iter(), &buffer.end_iter());

    if trimmed.is_empty() {
        let _ = fs::remove_file(MEMORY_FILE);
        return false;
    }

    let flags = TextSearchFlags::CASE_INSENSITIVE | TextSearchFlags::VISIBLE_ONLY;

    // 2. Восстанавливаем позицию смещения из tmpfs файла памяти
    let start_iter = if Path::new(MEMORY_FILE).exists() {
        if let Ok(content) = fs::read_to_string(MEMORY_FILE) {
            if let Ok(saved_offset) = content.trim().parse::<i32>() {
                let mut iter = buffer.iter_at_offset(saved_offset);
                // Сдвигаемся строго на 1 символ вперед, чтобы перешагнуть текущее слово
                if !iter.is_end() {
                    iter.forward_char();
                }
                iter
            } else {
                buffer.start_iter()
            }
        } else {
            buffer.start_iter()
        }
    } else {
        buffer.start_iter()
    };

    // 3. Шаг ВПЕРЕД: Ищем слово до конца документа
    if let Some((match_start, match_end)) = start_iter.forward_search(trimmed, flags, None) {
        save_and_highlight(buffer, view, search_entry, &match_start, &match_end);
        return true;
    }

    // 4. ЦИКЛ: Если дошли до конца файла, прыгаем наверх и ищем сначала
    let document_start = buffer.start_iter();
    if let Some((match_start, match_end)) = document_start.forward_search(trimmed, flags, None) {
        save_and_highlight(buffer, view, search_entry, &match_start, &match_end);
        return true;
    }

    let _ = fs::remove_file(MEMORY_FILE);
    false
}

fn save_and_highlight(buffer: &TextBuffer, view: &gtk4::TextView, search_entry: &Entry, start: &TextIter, end: &TextIter) {
    // Пишем новый offset в оперативную память tmpfs
    let current_offset = start.offset();
    let _ = fs::write(MEMORY_FILE, current_offset.to_string());

    // Инициализируем тег подсветки, если его нет
    if buffer.tag_table().lookup("search_match").is_none() {
        let tag = gtk4::TextTag::builder()
            .name("search_match")
            .background("#D67777") // Розовый маркер
            .foreground("#000000") // Черный текст
            .build();
        buffer.tag_table().add(&tag);
    }

    // Накладываем маркер и выделяем текст в редакторе
    buffer.apply_tag_by_name("search_match", start, end);
    buffer.select_range(start, end);
    buffer.place_cursor(end);
    
    // Скроллим экран редактора к найденному месту
    let mut scroll_iter = start.clone();
    view.scroll_to_iter(&mut scroll_iter, 0.0, false, 0.5, 0.5);

    // 🔥 ГЛАВНОЕ ИСПРАВЛЕНИЕ: Намертво возвращаем фокус ввода обратно в строку поиска!
    // Благодаря этому GTK4 не будет блокировать повторные нажатия клавиши Enter
    search_entry.grab_focus();
} "##),
       ("gui-apps/crucian/files/crucian/src/syntax.rs", r##"use gtk4::prelude::*;
use gtk4::{glib::signal::SignalHandlerId, TextBuffer};
use parking_lot::RwLock;
use std::fs;
use std::sync::Arc;

// --- НАДЁЖНОЕ ОПРЕДЕЛЕНИЕ РАСШИРЕНИЯ ЧЕРЕЗ ШЕБАНГ ---
pub fn detect_file_extension(buf: &gtk4::TextBuffer, path_ext: Option<&str>) -> Option<String> {
    if let Some(ext) = path_ext {
        return Some(ext.to_lowercase());
    }

    let start = buf.start_iter();
    let mut end = start.clone();
    end.forward_to_line_end();

    let first_line = buf.text(&start, &end, false).to_string().to_lowercase();

    if first_line.starts_with("#!") {
        if first_line.contains("python") || first_line.contains("py") {
            return Some("py".to_string());
        } else if first_line.contains("lua") {
            return Some("lua".to_string());
        } else if first_line.contains("fish") {
            return Some("fish".to_string());
        } else if first_line.contains("bash") || first_line.contains("sh") {
            return Some("fish".to_string());
        }
    }

    None
}

// --- ДЕМОН ПОДСВЕТКИ СИНТАКСИСА (RUST, LUA, FISH, PYTHON, BASH) ---
pub fn run_syntax_daemon(
    buffer: &TextBuffer, 
    handler_id: &Arc<RwLock<Option<SignalHandlerId>>>,
    file_extension: Option<&str>
) {
    if let Some(id) = handler_id.read().as_ref() {
        buffer.block_signal(id);
    }


    let start_all = buffer.start_iter();
    let end_all = buffer.end_iter();
    

    buffer.remove_tag_by_name("syntax_keyword", &start_all, &end_all);
    buffer.remove_tag_by_name("syntax_type", &start_all, &end_all);
    buffer.remove_tag_by_name("syntax_string", &start_all, &end_all);
    buffer.remove_tag_by_name("syntax_number", &start_all, &end_all);
    buffer.remove_tag_by_name("syntax_function", &start_all, &end_all);
    buffer.remove_tag_by_name("syntax_bracket", &start_all, &end_all);
    buffer.remove_tag_by_name("syntax_comment", &start_all, &end_all);

    let ext = file_extension.unwrap_or("rs").to_lowercase();
    
    let (keywords, types, comment_marker): (&[&str], &[&str], &str) = match ext.as_str() {
        "lua" => (
            &["and", "break", "do", "else", "elseif", "end", "false", "for", "function", "if", "in", "local", "nil", "not", "or", "repeat", "return", "then", "true", "until", "while", "goto"],
            &["print", "require", "type", "tostring", "tonumber", "pairs", "ipairs", "next", "select", "error", "pcall", "xpcall", "setmetatable", "getmetatable", "assert", "math", "table", "string", "io", "os", "coroutine", "debug", "utf8", "package"],
            "--"
        ),
        "fish" => (
            &["and", "begin", "break", "breakpoint", "case", "continue", "else", "end", "exec", "exit", "for", "function", "if", "not", "or", "return", "switch", "while", "command", "builtin"],
            &["set", "echo", "string", "math", "printf", "read", "test", "contains", "count", "status"],
            "#"
        ),
        "py" | "python" => (
            &["False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while", "with", "yield"],
            &["int", "float", "complex", "str", "list", "tuple", "set", "dict", "bool", "bytes", "print", "len", "range", "enumerate", "zip", "open", "Exception"],
            "#"
        ),
        _ => ( 
            &["if", "let", "fn", "match", "loop", "while", "for", "in", "return", "struct", "enum", "impl", "use", "pub", "mut", "unsafe", "const", "mod", "crate", "as", "type", "where"],
            &["String", "Option", "Result", "PathBuf", "Arc", "RwLock", "TextView", "u32", "i32", "usize", "bool", "str", "Self", "self", "Box", "Vec"],
            "//"
        )
    };

    let mut iter = buffer.start_iter();
    let mut expect_function_name = false;

    while !iter.is_end() {
        if iter.starts_word() {
            let mut end_word = iter.clone();
            end_word.forward_word_end();
            let word = buffer.text(&iter, &end_word, false);
            let w = word.as_str();
            
            if expect_function_name {
                buffer.apply_tag_by_name("syntax_function", &iter, &end_word);
                expect_function_name = false;
            } else if keywords.contains(&w) {
                buffer.apply_tag_by_name("syntax_keyword", &iter, &end_word);
                if w == "function" || w == "fn" || w == "def" {
                    expect_function_name = true;
                }
            } else if types.contains(&w) {
                buffer.apply_tag_by_name("syntax_type", &iter, &end_word);
            } else if w.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                buffer.apply_tag_by_name("syntax_number", &iter, &end_word);
            }
            iter = end_word;
        } else {
            let c = iter.char();
            if c == '(' || c == ')' || c == '{' || c == '}' || c == '[' || c == ']' {
                let mut next_char = iter.clone();
                next_char.forward_char();
                buffer.apply_tag_by_name("syntax_bracket", &iter, &next_char);
            }
            if !iter.forward_char() {
                break;
            }
        }
    }

    let mut string_iter = buffer.start_iter();
    while !string_iter.is_end() {
        let char_at = string_iter.char();
        if char_at == '"' || char_at == '\'' {
            let quote = char_at;
            let start_str = string_iter.clone();
            let mut end_str = string_iter.clone();
            
            let mut escaped = false;
            while end_str.forward_char() {
                if end_str.ends_line() {
                    break;
                }
                let c = end_str.char();
                if escaped {
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == quote {
                    end_str.forward_char();
                    break;
                }
            }
            
            buffer.remove_tag_by_name("syntax_bracket", &start_str, &end_str);
            buffer.apply_tag_by_name("syntax_string", &start_str, &end_str);
            string_iter = end_str;
        } else {
            if !string_iter.forward_char() {
                break;
            }
        }
    }

    let mut search_iter = buffer.start_iter();
    while let Some((comment_start, _)) = search_iter.forward_search(comment_marker, gtk4::TextSearchFlags::VISIBLE_ONLY, None) {
        let mut comment_end = comment_start.clone();
        comment_end.forward_to_line_end();
        
        buffer.remove_tag_by_name("syntax_keyword", &comment_start, &comment_end);
        buffer.remove_tag_by_name("syntax_type", &comment_start, &comment_end);
        buffer.remove_tag_by_name("syntax_string", &comment_start, &comment_end);
        buffer.remove_tag_by_name("syntax_number", &comment_start, &comment_end);
        buffer.remove_tag_by_name("syntax_function", &comment_start, &comment_end);
        buffer.remove_tag_by_name("syntax_bracket", &comment_start, &comment_end);
        
        buffer.apply_tag_by_name("syntax_comment", &comment_start, &comment_end);
        
        search_iter = comment_end.clone();
        if !search_iter.is_end() {
            search_iter.forward_char();
        }
    }

    if let Some(id) = handler_id.read().as_ref() {
        buffer.unblock_signal(id);
    }
}

// --- СТАБИЛЬНАЯ ЗАГРУЗКА CSS СТИЛЕЙ ---
pub fn load_css() {
    let provider = gtk4::CssProvider::new();
    let config_dir = gtk4::glib::user_config_dir();
    let css_path = config_dir.join("crucian/style.css");
    
    if let Ok(css_content) = fs::read_to_string(&css_path) {
        provider.load_from_data(&css_content);
        if let Some(display) = gtk4::gdk::Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }
} "##),
        ("gui-apps/crucian/files/crucian/src/bin/crucian-cli.rs", r#"// Файл: src/bin/crucian-cli.rs
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        std::process::exit(1);
    }

    // 1. Собираем и валидируем все переданные файлы
    let mut valid_paths = Vec::new();
    for target_file in &args[1..] {
        let path = Path::new(target_file);
        if let Ok(abs) = path.canonicalize() {
            if abs.exists() {
                valid_paths.push(abs);
            }
        }
    }

    if valid_paths.is_empty() {
        std::process::exit(1);
    }

    // 2. Логика выбора файла
    let selected_path = if valid_paths.len() == 1 {
        // Если файл всего один — сразу выбираем его без кнопок
        &valid_paths[0]
    } else {
        // Формируем аргументы для вызова нативного notify-send с кнопками в один ряд
                // Формируем аргументы для вызова нативного notify-send
        let mut notify_args = vec![
            "Crucian ИИ-Ассистент 🤖".to_string(), // Текст заголовка
            format!("Выберите контекст для Ollama (Очередь: {})", valid_paths.len()), // Текст подзаголовка
            "-i".to_string(), "brain".to_string(), // Можно поставить системную иконку brain, умного робота или оставить терминал
            "-t".to_string(), "0".to_string(),
        ];
        // Выводим только первые 5 файлов, чтобы они гарантированно влезли в ряд
        let max_buttons = std::cmp::min(valid_paths.len(), 5);

        for (index, path) in valid_paths.iter().take(max_buttons).enumerate() {
            let full_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            
            // Очень компактно обрезаем имя (до 7 символов), чтобы влезло как можно больше кнопок
            let display_name = if full_name.len() > 8 {
                format!("{}…", &full_name[..6])
            } else {
                full_name
            };

            notify_args.push(format!("--action={}:{}", index, display_name));
        }

        // Если файлов было больше 5, выводим кнопку-заглушку, иначе — кнопку Выход
        if valid_paths.len() > 5 {
            notify_args.push(format!("--action={}:🎨 Ещё…", valid_paths.len()));
        } else {
            notify_args.push(format!("--action={}:❌", valid_paths.len()));
        }

        // Ждем клика пользователя по кнопке в самом уведомлении waynotify
        let output = Command::new("notify-send")
            .args(&notify_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output();

        let mut chosen_index: Option<usize> = None;

        if let Ok(out) = output {
            let result_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if let Ok(idx) = result_str.parse::<usize>() {
                if idx < valid_paths.len() {
                    chosen_index = Some(idx);
                }
            }
        }

        // ИСПРАВЛЕНО: возвращаем первый элемент вектора в обоих случаях, чтобы типы совпали
        match chosen_index {
            Some(idx) => &valid_paths[idx],
            None => &valid_paths[0], 
        }
    };

    // 3. Запуск основного бинарника Crucian
    let mut exe_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    exe_path.set_file_name("crucian");

    let final_exe = if exe_path.exists() {
        exe_path
    } else {
        PathBuf::from("./crucian")
    };

    let path_str = selected_path.to_string_lossy().to_string();

    let _ = Command::new(&final_exe)
        .env("CRUCIAN_OPEN_FILE", path_str)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
} "#),
       ("gui-apps/crucian/files/crucian/src/bin/crucian-man.rs", r#"use std::{thread, io::{self, Read, Write}, time::Duration};
use std::process::Command;
use colored::Colorize;

fn my_custom_logic() {
	println!("{}", "Crucian — Локальный AI-ассистент и редактор кода на базе Ollama".bold().blue());
        println!("\n{}", "СИНТАКСИС :".yellow());
        println!("crucian [%F]
       crucian-cli [%F]   ");        
        println!("\n{}", "ОПИСАНИЕ:".green());
        println!("Crucian — это легковесный текстовый редактор на GTK4, интегрированный
       с локальными нейросетями через API Ollama. Программа позволяет
       одновременно редактировать код, подсвечивать синтаксис (Rust, Python, 
       Lua, Fish) и общаться с языковой моделью, передавая ей контекст файлов.

       crucian-cli выполняет роль фонового перехватчика файлов из контекстного
       меню ОС. При передаче нескольких файлов утилита вызывает интерактивное
       окно выбора (через notify-send), после чего запускает графический 
       интерфейс с выбранным файлом. При запуске без аргументов выводит
       информацию о статусе системы и инструкцию по настройке.");
        println!("\n{}", "БЕЗОПАСТНОСТЬ :".yellow());
         println!("\n{}", "НЕ СОВЕТУЮ ЗАПУСКАТЬ ОТ root!".red());
        println!("\n{}", "УСТАНОВКА И НАСТРОЙКА OLLAMA (ДЛЯ GENTOO) :".yellow()); 
         println!("Для работы приложения необходим запущенный локальный сервер Ollama.
       Выполните следующие шаги последовательно:

       1. Установка пакета:
          # emerge --ask sci-ml/ollama

       2. Запуск системной службы (OpenRC):
          # rc-service ollama start
          (Для добавления в автозагрузку: # rc-update add ollama default)

       3. Проверка работоспособности порта (сервер должен слушать 11434):
          $ ss -tulpn | grep 11434

       4. Скачивание рекомендуемой по умолчанию модели:
          $ ollama pull qwen2:1.5b

       5. Проверка работы модели в консоли:
          $ ollama run qwen2:1.5b
          $ ollama run llama3       "); 
        println!("\n{}", "НАСТРОЙКА ИНТЕРФЕЙСА (CSS) :".yellow());       
        println!("\n{}", "Приложение поддерживает кастомизацию тем оформления через CSS.
       Глобальный шаблон устанавливается в систему, но для личной настройки
       создайте локальный файл:
          
          mkdir -p ~/.config/crucian
          cp /usr/share/crucian/style.css ~/.config/crucian/style.css

       Вы можете изменять цвета классов подсветки (.syntax_keyword, 
       .syntax_comment, .chat-pane) в этом файле без пересборки проекта.".green());
     println!("\n{}", "Разработано для Gentoo Linux. 2026 г.".blue());  
    // Выводим инфу о ядре (краб будет прямо над этой строкой)
    if let Ok(output) = Command::new("uname").arg("-snr").output() {
        let kernel = String::from_utf8_lossy(&output.stdout);
        println!("OS: {}", kernel.trim());  
    }
    // Проверяем твой RAM-диск из fstab
    println!("TMPFS: /var/tmp/wm [10M] ACTIVE");
    println!("----------------------------------------");
    println!("   Нажми \x1B[1;31m'Q'\x1B[0m для завершения сессии");
    println!("----------------------------------------");    
}

fn main() {
    // 1. ПРЕДУСТАНОВКА: Чистим старых крабов и прячем курсор
    let _ = Command::new("pkill").arg("-9").arg("-f").arg("crab").status();
    print!("\x1B[?25l");
    let _ = io::stdout().flush();

    // 2. ВЫВОДИМ ТЕКСТ (Твоя функция)
    my_custom_logic();

    // 3. ЗАПУСКАЕМ КРАБА (Он прыгнет на строку вверх от последней печати)
    let mut crab = Command::new("crab")
        .spawn()
        .expect("Бинарник /usr/bin/crab не найден");

    // 4. ПЕРЕХВАТ КЛАВИАТУРЫ (21 век: без Enter и без Эха)
    // stty -echo (не печатать вводимое), -icanon (читать по одному символу)
    let _ = Command::new("stty").arg("-echo").arg("-icanon").status();

    let mut stdin = io::stdin();
    let mut buffer = [0; 1];

    loop {
        
        if stdin.read(&mut buffer).is_ok() {
            let ch = buffer[0] as char;
            if ch == 'q' || ch == 'Q' {
                break; // Выход по нажатию Q
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    let _ = crab.kill();
    let _ = Command::new("stty").arg("sane").status(); // Возвращаем режим терминала
    print!("\x1B[?25h\r\n\x1B[1;33m[EXIT]\x1B[0m Сессия завершена. Краб ушел спать.\n");
    let _ = io::stdout().flush();
} "#),
       ("gui-apps/crucian/files/crucian/src/bin/file_creator.rs", r#"use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, FileChooserAction, FileChooserDialog};

fn main() {
    let app = Application::builder()
        .application_id("com.example.filecreator")
        .build();

    app.connect_activate(|app| {
        // Создаем скрытое окно-заглушку для контекста
        let win = ApplicationWindow::builder()
            .application(app)
            .visible(false)
            .build();

        // В GTK4 0.11+ создаем диалог через классический new()
        let file_chooser = FileChooserDialog::new(
            Some("Сохранить новый файл"),
            Some(&win),
            FileChooserAction::Save,
            &[
                ("Отмена", gtk4::ResponseType::Cancel),
                ("Сохранить", gtk4::ResponseType::Accept),
            ],
        );

        file_chooser.set_current_name("untitled.rs");

        file_chooser.connect_response(move |dialog, response| {
            if response == gtk4::ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        let std_path: std::path::PathBuf = path;
                        // Выводим путь в stdout, чтобы crucian его поймал
                        println!("{}", std_path.to_string_lossy());
                    }
                }
            }
            dialog.destroy();
            std::process::exit(0);
        });

        file_chooser.show();
    });

    app.run_with_args(&[""]);
} "#),
       ("gui-apps/crucian/files/crucian/src/bin/file_opener.rs", r#"use gtk4::gio;
use gtk4::prelude::*;
use std::env;
use std::path::Path;

fn main() {
    // Инициализируем GLib/GTK, чтобы работали нативные системные вызовы
    gtk4::init().expect("Не удалось инициализировать GTK");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Если передан аргумент, открываем этот файл напрямую
        let path_str = &args[1];
        open_file_via_gio(path_str);
    } else {
        // Если запущен без аргументов, открываем диалог выбора файла
        open_dialog_selector();
    }
}

fn open_file_via_gio(path_str: &str) {
    let path = Path::new(path_str);
    if !path.exists() {
        eprintln!("❌ Ошибка: Файл не существует: {}", path_str);
        return;
    }

    // Получаем абсолютный путь к файлу
    let absolute_path = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => path.to_path_buf(),
    };

    // Формируем корректный URI-адрес (file://...)
    let uri = format!("file://{}", absolute_path.to_string_lossy());

    // Вызываем xdg-open через глобальный нативный метод AppInfo
    let gio_context: Option<&gio::AppLaunchContext> = None;
    match gio::AppInfo::launch_default_for_uri(&uri, gio_context) {
        Ok(_) => println!("🚀 Файл [{}] успешно передан в xdg-open через URI", path_str),
        Err(e) => eprintln!("❌ Ошибка GIO при открытии URI: {}", e),
    }
}

fn open_dialog_selector() {
    // Создаем фиктивное приложение для управления жизненным циклом диалога
    let app = gtk4::Application::builder()
        .application_id("com.example.fileopener")
        .build();

    app.connect_activate(|_app| {
        let file_chooser = gtk4::FileChooserNative::new(
            Some("Выберите файл для открытия через систему"),
            None::<&gtk4::Window>,
            gtk4::FileChooserAction::Open,
            Some("Открыть"),
            Some("Отмена"),
        );

        file_chooser.connect_response(move |dialog, response| {
            if response == gtk4::ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    // ПРАВКА: file.uri() возвращает GString напрямую, Option не нужен
                    let uri = file.uri();
                    let gio_context: Option<&gio::AppLaunchContext> = None;
                    
                    // Открываем файл по его URI-адресу
                    match gio::AppInfo::launch_default_for_uri(uri.as_str(), gio_context) {
                        Ok(_) => println!("🚀 Файл успешно открыт через систему"),
                        Err(e) => eprintln!("❌ Ошибка GIO: {}", e),
                    }
                }
            }
            dialog.destroy();
            std::process::exit(0);
        });

        file_chooser.show();
    });

    // Запускаем минимальный цикл обработки событий GTK
    app.run_with_args(&[""]);
} "#),
 ];
      
 for (rel_path, content) in text_files {
        let full_path = base_path.join(rel_path.trim());
        if let Some(parent) = full_path.parent() {
            
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, content)?;
    }        
    println!("Структура giu-apps/crucian успешно создана ✔️");
    Ok(())
}
