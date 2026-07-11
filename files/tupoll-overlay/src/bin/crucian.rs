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
tokio = "1.52.3"
once_cell = "1.19" 
rusqlite = "0.31"

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
       ("gui-apps/crucian/files/crucian/src/queue_ui.rs", r#"use gtk4::prelude::*;
use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};
use rusqlite::Connection;

/// Полная синхронизация текста на кнопке с базой данных crucian.db
pub fn sync_button_text(queue_btn: &gtk4::Button) {
    let count = crate::db::get_context_count();
    queue_btn.set_label(&format!("📋 Очередь ({})", count));
}

/// 🔥 ФИКС ОШИБКИ E0061: Явно указываем три аргумента для связи с графическим окном!
pub fn handle_queue_click(queue_btn: &gtk4::Button, file_view: &gtk4::TextView, status_label: &gtk4::Label) {
    sync_button_text(queue_btn);
    
    let count = crate::db::get_context_count();
    let msg = format!("Выберите контекст для Ollama (Очередь: {})", count);

    let mut notify_args = vec![
        "Crucian ИИ-Ассистент 🤖".to_string(),
        msg,
        "-i".to_string(), "accessories-text-editor".to_string(),
        "-t".to_string(), "0".to_string(),
    ];

    let mut indexed_paths: Vec<String> = Vec::new();

    // Читаем пути из SQLite
    if let Ok(conn) = Connection::open("/var/tmp/wm/crucian.db") {
        if let Ok(mut stmt) = conn.prepare("SELECT file_path FROM context_queue ORDER BY id ASC") {
            if let Ok(file_iter) = stmt.query_map([], |row| row.get::<_, String>(0)) {
                for (index, file_res) in file_iter.enumerate().take(5) {
                    if let Ok(path) = file_res {
                        let full_name = Path::new(&path)
                            .file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or(&path)
                            .to_string();
                        
                        let display_name = if full_name.len() > 8 {
                            format!("{}…", &full_name[..6])
                        } else {
                            full_name
                        };

                        notify_args.push(format!("--action={}:{}", index, display_name));
                        indexed_paths.push(path);
                    }
                }
            }
        }
    }

    // Кнопка сброса
    notify_args.push(format!("--action=clear:{}:❌", count));

    let queue_btn_to_update = queue_btn.clone();
    let status_to_update = status_label.clone();
    let view_to_update = file_view.clone();

    // Слушаем клики вашего краба из stdout в асинхронном контексте GTK
    gtk4::glib::MainContext::default().spawn_local(async move {
        let output = Command::new("notify-send")
            .args(&notify_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output();

        if let Ok(out) = output {
            let result_str = String::from_utf8_lossy(&out.stdout).trim().to_string();

            if result_str == "clear" {
                let _ = crate::db::clear_context_queue();
                queue_btn_to_update.set_label("📋 Очередь (0)");
                status_to_update.set_label("Очередь путей сброшена. Готов");
            } else if let Ok(idx) = result_str.parse::<usize>() {
                // Если кликнули на файл — выкачиваем его код и обновляем интерфейс
                if idx < indexed_paths.len() {
                    let target_path_str = &indexed_paths[idx];
                    let path_buf = PathBuf::from(target_path_str);
                    
                    if let Ok(content) = std::fs::read_to_string(&path_buf) {
                        view_to_update.buffer().set_text(&content);
                        let name = path_buf.file_name().unwrap_or_default().to_string_lossy();
                        status_to_update.set_label(&format!("📄 Системный файл: {}", name));
                    }
                }
            }
        }
    });
}

/// Вызывается при нажатии на графическую кнопку "❌ Сброс очереди" в основном GTK окне
pub fn handle_reset_click(queue_btn: &gtk4::Button, status_label: &gtk4::Label) {
    let _ = crate::db::clear_context_queue();
    queue_btn.set_label("📋 Очередь (0)");
    status_label.set_label("Очередь путей сброшена. Готов");

    let _ = Command::new("notify-send")
        .arg("Crucian ИИ-Ассистент 🤖")
        .arg("Очередь контекста полностью очищена! ❌")
        .arg("-i").arg("dialog-warning")
        .spawn();
} "#),
       ("gui-apps/crucian/files/crucian/src/db.rs", r#"use rusqlite::{Connection, Result};
use std::fs;

const DB_PATH: &str = "/var/tmp/wm/crucian.db";

/// Функция инициализации БД. Создает таблицу, если её нет.
pub fn init_db() -> Result<()> {
    // Гарантируем, что папка существует
    let _ = fs::create_dir_all("/var/tmp/wm");
    
    let conn = Connection::open(DB_PATH)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS context_queue (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            file_path TEXT NOT NULL UNIQUE
         )",
        [],
    )?;
    Ok(())
}

/// Добавляет путь к файлу в очередь контекста ИИ
pub fn add_file_to_context(path: &str) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;
    // IGNORE спасет от ошибок, если файл добавляется повторно
    conn.execute(
        "INSERT OR IGNORE INTO context_queue (file_path) VALUES (?1)",
        [path],
    )?;
    Ok(())
}

/// Полностью очищает очередь контекста ИИ (для кнопки Сброс)
pub fn clear_context_queue() -> Result<()> {
    let conn = Connection::open(DB_PATH)?;
    conn.execute("DELETE FROM context_queue", [])?;
    Ok(())
}

/// Возвращает количество файлов в очереди (для счетчика на кнопке)
pub fn get_context_count() -> usize {
    let conn = match Connection::open(DB_PATH) {
        Ok(c) => c,
        Err(_) => return 0,
    };
    
    let mut stmt = match conn.prepare("SELECT COUNT(*) FROM context_queue") {
        Ok(s) => s,
        Err(_) => return 0,
    };
    
    let count: Result<usize> = stmt.query_row([], |row| row.get(0));
    count.unwrap_or(0)
}

/// Считывает весь код из добавленных файлов и склеивает в один большой промпт для ИИ
pub fn compile_aggregated_context() -> String {
    let mut combined_code = String::new();
    
    let conn = match Connection::open(DB_PATH) {
        Ok(c) => c,
        Err(_) => return combined_code,
    };

    let mut stmt = match conn.prepare("SELECT file_path FROM context_queue ORDER BY id ASC") {
        Ok(s) => s,
        Err(_) => return combined_code,
    };

    let file_iter = match stmt.query_map([], |row| row.get::<_, String>(0)) {
        Ok(iter) => iter,
        Err(_) => return combined_code,
    };

    for file_res in file_iter {
        if let Ok(path) = file_res {
            if let Ok(content) = fs::read_to_string(&path) {
                let file_name = std::path::Path::new(&path)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or(&path);
                    
                combined_code.push_str(&format!("--- ИСХОДНЫЙ КОД ФАЙЛА: {} ---\n{}\n\n", file_name, content));
            }
        }
    }

    combined_code
} "#),    
      ("gui-apps/crucian/files/crucian/src/ai.rs", r#"use gtk4::prelude::*;
use gtk4::TextBuffer;
use serde::{Deserialize, Serialize};

/// Собирает текст вокруг каретки для идеальной вставки кода (Fill-in-the-Middle)
pub struct EditorContext {
    pub prefix: String,
    pub suffix: String,
}

pub fn get_editor_context(buffer: &TextBuffer) -> EditorContext {
    let start_all = buffer.start_iter();
    let end_all = buffer.end_iter();

    // 🔥 ФИКС ДЛЯ GTK4 v0.11: Правильно разворачиваем Option со структурой (start, end)
    if let Some((start_sel, end_sel)) = buffer.selection_bounds() {
        // Если текст выделен мышкой, забираем именно его для ИИ
        let selected_text = buffer.text(&start_sel, &end_sel, false).to_string();
        
        EditorContext {
            prefix: selected_text,
            suffix: String::new(), // Суффикс пустой, так как контекст уже в префиксе
        }
    } else {
        // Если выделения нет, собираем текст относительно позиции курсора
        let insert_mark = buffer.get_insert();
        let cursor_iter = buffer.iter_at_mark(&insert_mark);

        let prefix = buffer.text(&start_all, &cursor_iter, false).to_string();
        let suffix = buffer.text(&cursor_iter, &end_all, false).to_string();

        EditorContext { prefix, suffix }
    }
}


// --- СТРУКТУРЫ ДЛЯ СЕРИАЛИЗАЦИИ API ЗАПРОСОВ ---
#[derive(Serialize)]
struct OpenAICompatibleRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAICompatibleResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

/// Асинхронный вызов ИИ-модели по вашей схеме
pub async fn ai_completion_call(
    context: EditorContext, 
    use_local_ollama: bool, 
    api_key: Option<String>
) -> String {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_default();
    
    let prompt = format!(
        "Ты — встроенный ИИ-ассистент разработчика. Допиши код БЕЗ каких-либо пояснений, разметки markdown и тегов ```.\n\
         КОД ДО КУРСОРA:\n{}\n\n\
         КОД ПОСЛЕ КУРСОРA:\n{}\n\n\
         Продолжи код с места разрыва:", 
        context.prefix, context.suffix
    );

    let (url, model_name, authorization) = if use_local_ollama {
        ("http://localhost:11434/v1/chat/completions".to_string(), "qwen2.5-coder:7b".to_string(), None)
    } else {
        ("https://deepseek.com".to_string(), "deepseek-coder".to_string(), api_key)
    };

    let mut request = client.post(&url)
        .json(&OpenAICompatibleRequest {
            model: model_name,
            messages: vec![Message { role: "user".to_string(), content: prompt }],
            temperature: 0.2,
        });

    if let Some(key) = authorization {
        request = request.header("Authorization", format!("Bearer {}", key));
    }

    match request.send().await {
        Ok(resp) => {
            if !resp.status().is_success() {
                return format!("// ❌ Ошибка сервера ({})", resp.status());
            }

            match resp.json::<OpenAICompatibleResponse>().await {
                Ok(ai_data) => {
                    if let Some(choice) = ai_data.choices.first() {
                        choice.message.content.clone()
                    } else {
                        "// ❌ ИИ вернул пустой ответ".into()
                    }
                }
                Err(e) => format!("// ❌ Ошибка парсинга JSON: {}", e),
            }
        }
        Err(e) => format!("// ❌ Ошибка сети ИИ: {}", e),
    }
} "#),
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
use crate::syntax::detect_file_extension;
use once_cell::sync::Lazy;

mod queue_ui;
mod db;
mod ai;
mod syntax;
mod search;

// 🔥 ЖЕЛЕЗОБЕТОННЫЙ ВЫНОС РАНТАЙМА И ОЧЕРЕДИ НА САМЫЙ ВЕРХНИЙ УРОВЕНЬ ФАЙЛА:
static AI_RT: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap()
});

// Глобальная переменная для совместимости структуры с вашим queue_ui.rs
static SHARED_CONTEXT_QUEUE: Lazy<Arc<RwLock<Vec<String>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(Vec::new()))
});


static IS_SYNTAX_PAINTING: Lazy<std::sync::Arc<parking_lot::RwLock<bool>>> = Lazy::new(|| {
    std::sync::Arc::new(parking_lot::RwLock::new(false))
});



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

 /// Считает строки в основном редакторе и обновляет панель номеров
pub fn update_line_numbers(file_buffer: &gtk4::TextBuffer, line_numbers_buffer: &gtk4::TextBuffer) {
    let line_count = file_buffer.line_count();
    
    // Генерируем строку вида "1\n2\n3\n4\n..."
    let mut numbers_text = String::new();
    for i in 1..=line_count {
        numbers_text.push_str(&format!("{}\n", i));
    }
    
    // Записываем полученный текст в панель номеров
    line_numbers_buffer.set_text(&numbers_text);
}


fn build_ui(app: &Application) {
	// 🔥 СБРОС ПАМЯТИ ПОИСКА ПРИ СТАРТЕ ПРИЛОЖЕНИЯ
    // Удаляем старый файл с tmpfs, чтобы новая сессия всегда начиналась с чистого листа (с 0)
    let _ = std::fs::remove_file("/var/tmp/wm/crucian_search");
        // Инициализируем SQLite базу данных перед запуском окон
    if let Err(e) = db::init_db() {
        println!("Ошибка инициализации БД: {}", e);
    }


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
    
    
         // --- 1. ЛЕВАЯ ПАНЕЛЬ: ЧАТ И ТУЛБАР (СТРОГИЙ ПОРЯДОК ОБЪЯВЛЕНИЯ) ---
    let chat_vbox = Box::new(gtk4::Orientation::Vertical, 10);
    chat_vbox.set_margin_start(10); chat_vbox.set_margin_end(10);
    chat_vbox.set_margin_top(10); chat_vbox.set_margin_bottom(10);
    main_horizontal_paned.set_start_child(Some(&chat_vbox));
    chat_vbox.add_css_class("chat-pane");

    let header_box = Box::new(gtk4::Orientation::Vertical, 5);
    let available_models = fetch_local_models();
    let model_dropdown = DropDown::from_strings(
        &available_models.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
    );
    let status_label = Label::builder().label("Готов").xalign(0.0).build();
    let spacer = Box::builder().hexpand(true).build(); 
    let open_file_btn = Button::with_label("📂 Открыть");
    let clear_btn = Button::with_label("🗑 Очистить");

    // 🔥 СНАЧАЛА ОБЪЯВЛЯЕМ КНОПКИ: Теперь они гарантированно существуют для всего нижнего кода!
    let queue_btn = Button::with_label("📋 Очередь (0)");
    let reset_queue_btn = Button::with_label("❌ Сброс очереди");

    // Упаковываем элементы в ваш тулбар по вашей схеме
    header_box.append(&model_dropdown);
    header_box.append(&status_label);
    header_box.append(&spacer);
    header_box.append(&search_entry); // Эта строчка остается неизменной
    header_box.append(&open_file_btn);
    header_box.append(&clear_btn);
    
    // Теперь спокойно добавляем кнопки — они объявлены строкой выше!
    header_box.append(&queue_btn);
    header_box.append(&reset_queue_btn);
    chat_vbox.append(&header_box);

        // --- СВЯЗКА КНОПОК С ОТДЕЛЬНЫМ ФАЙЛОМ-ФУНКЦИЕЙ (ТОЧНЫЕ ИМЕНА БУФЕРОВ) ---
    // Кнопка мгновенно считает данные из базы при инициализации окна
    crate::queue_ui::sync_button_text(&queue_btn);

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

    let code_scroll = ScrolledWindow::builder().vexpand(true).child(&ai_view).build();
    chat_vbox.append(&code_scroll);



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
// 🔥 ВСТАВЛЯЕМ СВЯЗКУ КНОПОК И КРАБА СЮДА:
    // Теперь file_view гарантированно существует и виден компилятору!
    crate::queue_ui::sync_button_text(&queue_btn);

    let queue_btn_clone = queue_btn.clone();
    let status_lbl_clone = status_label.clone();
    let view_for_queue = file_view.clone();
    let title_for_queue = status_label.clone();

    queue_btn.connect_clicked(move |btn| {
        crate::queue_ui::handle_queue_click(btn, &view_for_queue, &title_for_queue);
    });

    reset_queue_btn.connect_clicked(move |_| {
        crate::queue_ui::handle_reset_click(&queue_btn_clone, &status_lbl_clone);
    });

    // Финальный вывод главного окна на экран:
    win.present();
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
    let file_buffer = file_view.buffer();
    let lines_buffer = line_numbers_view.buffer();

    // 1. Генерируем начальную цифру "1" при запуске редактора
    update_line_numbers(&file_buffer, &lines_buffer);

    // 2. Обновляем цифры при вводе текста
    let lines_buf_clone = lines_buffer.clone();
             
    
          // 🔥 БРОНЕБОЙНЫЙ АБСОЛЮТНЫЙ ФИЛЬТР С ЗАЩИТОЙ ОТ НУЛЕВЫХ СБРОСОВ
    file_buffer.connect_closure(
        "mark-set",
        false,
        gtk4::glib::closure!(move |_buf: gtk4::TextBuffer, iter: gtk4::TextIter, mark: gtk4::TextMark| {
            if mark.name().as_deref() == Some("insert") {
                
                // Если сейчас идет перекраска текста синтаксическим демоном — намертво игнорируем!
                if *IS_SYNTAX_PAINTING.read() {
                    return;
                }

                let current_offset = iter.offset();
                
                // 🔥 ГЛАВНЫЙ Unix-ФИЛЬТР: Записываем офсет ТОЛЬКО если он больше нуля!
                // Любые системные сбросы GTK4 в ноль во время connect_changed просто пролетят мимо,
                // и в файле crucian_cursor останется лежать нетронутая живая координата.
                if current_offset > 0 {
                    let _ = std::fs::write("/var/tmp/wm/crucian_cursor", format!("{}", current_offset));
                }
            }
        }),
    );

    file_buffer.connect_changed(move |buf| {
        update_line_numbers(buf, &lines_buf_clone);
    });

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
    while let Some(child) = editor_layout.first_child() {
        editor_layout.remove(&child);
    }
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

             // --- 9. АСИНХРОННЫЙ КОНВЕЙЕР ИИ (ИСПРАВЛЕННЫЙ АВТОСКРОЛЛ ЧАТА) ---
    let _prompt_buffer = input_entry.buffer();     
    let chat_buffer = buffer.clone();              
    let current_file_buffer = file_view.buffer();  

    let key_controller = gtk4::EventControllerKey::new();
    let prompt_clone = input_entry.clone();
    let chat_buf_clone = chat_buffer.clone(); 
    let file_buf_clone = current_file_buffer.clone(); 
    let status_clone = status_label.clone();
    
    // 🔥 ФИКС АВТОСКРОЛЛА: Клонируем само левое окно ОТОБРАЖЕНИЯ чата
    let ai_view_clone = ai_view.clone(); 

    key_controller.connect_key_pressed(move |_, keyval, _, modifier| {
        if (keyval == gtk4::gdk::Key::Return || keyval == gtk4::gdk::Key::KP_Enter) 
            && modifier.contains(gtk4::gdk::ModifierType::CONTROL_MASK) 
        {
            let user_question = prompt_clone.text().to_string();
            if user_question.trim().is_empty() {
                return false.into();
            }

            let mut end_iter = chat_buf_clone.end_iter();
            chat_buf_clone.insert_with_tags_by_name(&mut end_iter, &format!("\nUser: {}\n", user_question), &["bold"]);

            prompt_clone.set_text("");
            status_clone.set_label("ИИ думает...");

            let mut status_iter = chat_buf_clone.end_iter();
            chat_buf_clone.insert(&mut status_iter, "\n🤖 ИИ генерирует ответ, пожалуйста подождите...\n");

            let start_status_offset = chat_buf_clone.end_iter().offset() - 47; 

            let ai_context = ai::get_editor_context(&file_buf_clone);
            
            let b_chat = chat_buf_clone.clone();
            let b_lower = file_buf_clone.clone(); // Ссылка на правый редактор
            let s_label = status_clone.clone();
            
            // Пробрасываем клон левого TextView внутрь асинхронной задачи
            let view_chat_scroll = ai_view_clone.clone(); 

            let use_local = true; 
            let api_key = Some("your_key".to_string());

            gtk4::glib::MainContext::default().spawn_local(async move {
                
                let handle = AI_RT.spawn(async move {
                    ai::ai_completion_call(ai_context, use_local, api_key).await
                });

                let result = match handle.await {
                    Ok(res) => res,
                    Err(_) => "❌ Ошибка выполнения фонового потока ИИ".to_string(),
                };

                s_label.set_label("Готов");

                // Текстовый поиск и удаление маркера из ПРАВОГО редактора кода
                let wait_signature = " 🤖⏳ [Генерация...] ";
                let search_iter = b_lower.start_iter();
                if let Some((match_start, match_end)) = search_iter.forward_search(
                    wait_signature,
                    gtk4::TextSearchFlags::VISIBLE_ONLY,
                    None
                ) {
                    b_lower.delete(&mut match_start.clone(), &mut match_end.clone());
                }
                
                if let Some(mark) = b_lower.mark("ai_wait_start") {
                    b_lower.delete_mark(&mark);
                }

                // Чистим временную строчку статуса ожидания внутри ЛЕВОГО чата
                let mut del_start = b_chat.iter_at_offset(start_status_offset);
                let mut del_end = b_chat.end_iter();
                b_chat.delete(&mut del_start, &mut del_end);

                // Выводим ответ ИИ в левый чат
                let mut ai_iter = b_chat.end_iter();
                b_chat.insert(&mut ai_iter, "AI:\n");
                
                let parts = result.split("```");
                let mut is_code_part = false;
                for part in parts {
                    let mut current_end = b_chat.end_iter();
                    if is_code_part {
                        let clean_code = if part.starts_with('\n') { part } else { part.split_once('\n').map(|x| x.1).unwrap_or(part) };
                        b_chat.insert_with_tags_by_name(&mut current_end, clean_code, &["code"]);
                    } else {
                        b_chat.insert(&mut current_end, part);
                    }
                    is_code_part = !is_code_part;
                }
                b_chat.insert(&mut b_chat.end_iter(), "\n");

                
                // Берем самый свежий конец буфера чата
                let mut chat_end_iter = b_chat.end_iter();
                // Насильно приказываем левому окну прокрутиться к этой точке
                view_chat_scroll.scroll_to_iter(&mut chat_end_iter, 0.0, false, 0.0, 0.0);
            });

            return true.into(); 
        }
        false.into()
    });

    input_entry.add_controller(key_controller);

  
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
let _daemon_id_clone = syntax_handler_id.clone();
let _path_clone = current_file_path.clone();
let _line_nums_clone = line_numbers_view.clone(); // Больше не будет warning!

    // Клонируем переменные для потока
    let daemon_id_clone = syntax_handler_id.clone();
    let file_view_clone = file_view.clone();
    let current_ext = detect_file_extension(&file_buffer, None); 

            let handler_signal_id = file_buffer.connect_changed(move |buf| {
        // 1. Намертво блокируем запись курсора в файл перед началом покраски
        *IS_SYNTAX_PAINTING.write() = true;

        let ext_str = current_ext.as_deref();
        crate::syntax::run_syntax_daemon(
            buf, 
            &file_view_clone, 
            &daemon_id_clone, 
            ext_str
        );

        // 2. Покраска завершена, открываем доступ сигналу mark-set
        *IS_SYNTAX_PAINTING.write() = false;
    });

    *syntax_handler_id.write() = Some(handler_signal_id);



    // Записываем полученный SignalHandlerId в наш потокобезопасный RwLock
  //  *syntax_handler_id.write() = Some(handler_signal_id);

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
                        
                        if let Ok(content) = std::fs::read_to_string(&std_path) {
                            let name = std_path.file_name().unwrap_or_default().to_string_lossy();
                            let full_path_str = std_path.to_string_lossy().to_string();
                            
                            // 1. Выводим текст в редактор
                            view.buffer().set_text(&content);
                            
                            // 2. Обновляем плашку с именем файла
                            title.set_label(&format!("📄 Системный файл: {}", name));
                            
                            // 3. Активируем кнопку сохранения
                            s_btn.set_sensitive(true);
                            
                            // 4. Запоминаем путь в переменной редактора
                            if let mut p_lock = path_dialog_clone.write() {
                                *p_lock = Some(std_path);
                            }

                            // 🔥 КУДА ПРАВИЛЬНЕЙ: Железно впечатываем путь в SQLite crucian.db!
                            // Никакие кнопки или ИИ при этом не дергаются. Тихо сохраняем в базу.
                            let _ = db::add_file_to_context(&full_path_str);
                        }
                    }
                }
            }
            dialog.destroy(); // Закрываем диалог выбора файла
        });

        file_chooser.present(); // Выводим окно на экран в GTK4
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
let c_view = file_view.clone();
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
           
               // 🔥 ДОБАВЛЯЕМ НЕДОСТАЮЩИЙ БЛОК ЧТЕНИЯ АРГУМЕНТОВ CLI:
    // Считываем пути к файлам, которые вы передали при старте в терминале
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        for file_path in args.iter().skip(1) {
            // Проверяем, существует ли файл на диске перед записью
            if std::path::Path::new(file_path).exists() {
                // Железно пишем путь в вашу базу crucian.db
                let _ = db::add_file_to_context(file_path);
            }
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
}
"##),
       ("gui-apps/crucian/files/crucian/src/syntax.rs", r##"use gtk4::prelude::*;
use gtk4::{glib::signal::SignalHandlerId, TextBuffer};
use parking_lot::RwLock;
use std::fs;
use std::sync::Arc;

const CURSOR_FILE: &str = "/var/tmp/wm/crucian_cursor";

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

pub fn run_syntax_daemon(
    buffer: &TextBuffer, 
    view: &gtk4::TextView,
    handler_id: &Arc<RwLock<Option<SignalHandlerId>>>,
    file_extension: Option<&str>
) {
       // 1. ЗАПОМИНАЕМ ТЕКУЩИЙ ОФСЕТ ИЗ БУФЕРА
    let insert_mark = buffer.get_insert();
    let current_iter = buffer.iter_at_mark(&insert_mark);
    let mut saved_offset = current_iter.offset();

    // Если буфер сбросился в 0 во время очистки, восстанавливаем из файла tmpfs
    if saved_offset == 0 {
        if let Ok(coords) = fs::read_to_string(CURSOR_FILE) {
            if let Ok(offset) = coords.trim().parse::<i32>() {
                if offset > 0 {
                    saved_offset = offset;
                }
            }
        }
    }

    // Теперь безопасно блокируем сигналы
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

          // 2. СИЛОВОЕ ВОССТАНАВЛИВАЕМ КУРСОР ИЗ ТЕКСТА ФАЙЛА-ПАМЯТИ
    let mut saved_offset = 0;
    if let Ok(coords) = fs::read_to_string(CURSOR_FILE) {
        if let Ok(offset) = coords.trim().parse::<i32>() {
            if offset > 0 {
                saved_offset = offset;
            }
        }
    }

    // Если в файле был сохранен реальный офсет, намертво возвращаем каретку туда
    if saved_offset > 0 {
        let mut reset_cursor_iter = buffer.iter_at_offset(saved_offset);
        buffer.place_cursor(&reset_cursor_iter);

        // 3. УДЕРЖИВАЕМ ВЬЮПОРТ ЭКРАНА НА МЕСТЕ
        let mut scroll_iter = reset_cursor_iter.clone();
        view.scroll_to_iter(&mut scroll_iter, 0.0, false, 0.0, 0.0);
    } else {
        // Подстраховка на случай пустого файла
        let mut fallback_iter = buffer.start_iter();
        buffer.place_cursor(&fallback_iter);
    }

    if let Some(id) = handler_id.read().as_ref() {
        buffer.unblock_signal(id);
    }
}


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
}
 "##),
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
}  "#),
        ("gui-apps/crucian/files/crucian/src/bin/crucian-cli.rs", r#"use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use rusqlite::Connection;

const DB_PATH: &str = "/var/tmp/wm/crucian.db";

/// Запись пути к файлу в SQLite базу данных crucian.db
fn db_add_file(path: &str) {
    if let Ok(conn) = Connection::open(DB_PATH) {
        let _ = conn.execute(
            "INSERT OR IGNORE INTO context_queue (file_path) VALUES (?1)",
            [path],
        );
    }
}

/// Полная очистка очереди контекста в базе данных (кнопка Сброс)
fn db_clear_queue() {
    if let Ok(conn) = Connection::open(DB_PATH) {
        let _ = conn.execute("DELETE FROM context_queue", []);
    }
}

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
    let mut should_open_gui = true;
    let selected_path = if valid_paths.len() == 1 {
        // Если файл всего один — сразу выбираем его и пишем в SQLite
        let path_str = valid_paths[0].to_string_lossy().to_string();
        db_add_file(&path_str);
        Some(valid_paths[0].clone())
    } else {
        // Формируем аргументы для вызова нативного notify-send
        let mut notify_args = vec![
            "Crucian ИИ-Ассистент 🤖".to_string(),
            format!("Выберите контекст для Ollama (Очередь: {})", valid_paths.len()),
            "-i".to_string(), "accessories-text-editor".to_string(), // Используем иконку редактора
            "-t".to_string(), "0".to_string(),
        ];
        
        // Выводим только первые 5 файлов, чтобы они гарантированно влезли в ряд
        let max_buttons = std::cmp::min(valid_paths.len(), 5);

        for (index, path) in valid_paths.iter().take(max_buttons).enumerate() {
            let full_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            
            let display_name = if full_name.len() > 8 {
                format!("{}…", &full_name[..6])
            } else {
                full_name
            };

            notify_args.push(format!("--action={}:{}", index, display_name));
        }

        // Кнопка сброса/выхода вешается на экшен "clear"
        notify_args.push(format!("--action=clear:{}", "3:❌"));

        // Ждем клика пользователя по кнопке в самом уведомлении waynotify
        let output = Command::new("notify-send")
            .args(&notify_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output();

        let mut chosen_path: Option<PathBuf> = None;

        if let Ok(out) = output {
            let result_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
            
            if result_str == "clear" {
                // 🔥 ЖЕЛЕЗОБЕТОННЫЙ СБРОС: Пользователь кликнул на 3:❌
                db_clear_queue();
                should_open_gui = false; // Передумали открывать редактор, просто сбросили очередь путей!
                
                // Шлём финальное уведомление через ваш waynotify с желтым треугольником warning
                let _ = Command::new("notify-send")
                    .arg("Crucian ИИ-Ассистент 🤖")
                    .arg("Очередь контекста полностью очищена! ❌")
                    .arg("-i").arg("dialog-warning")
                    .spawn();
            } else if let Ok(idx) = result_str.parse::<usize>() {
                if idx < valid_paths.len() {
                    let path_str = valid_paths[idx].to_string_lossy().to_string();
                    // 🔥 ЗАПИСЬ КОНТЕКСТА: Пишем выбранный файл в базу данных
                    db_add_file(&path_str);
                    chosen_path = Some(valid_paths[idx].clone());
                }
            }
        }

        // Если пользователь закрыл оверлей без клика по кнопкам, открываем первый по дефолту
        if chosen_path.is_none() && should_open_gui {
            let path_str = valid_paths[0].to_string_lossy().to_string();
            db_add_file(&path_str);
            chosen_path = Some(valid_paths[0].clone());
        }

        chosen_path
    };

    // 3. Запуск основного бинарника Crucian (только если не нажали Сброс)
    if should_open_gui {
        if let Some(final_path) = selected_path {
            let mut exe_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
            exe_path.set_file_name("crucian");

            let final_exe = if exe_path.exists() {
                exe_path
            } else {
                PathBuf::from("./crucian")
            };

            let path_str = final_path.to_string_lossy().to_string();

            let _ = Command::new(&final_exe)
                .env("CRUCIAN_OPEN_FILE", path_str)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();
        }
    }
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
