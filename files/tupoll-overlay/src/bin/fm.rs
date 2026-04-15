use std::fs;
use std::path::Path; 

fn main() -> std::io::Result<()> {
    let base_path = Path::new("/var/db/repos/tupoll-overlay/");   
    let text_files = [
   ("gui-apps/pinnacle-fm/pinnacle-fm-9999.ebuild", r#"# Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

inherit cargo desktop

DESCRIPTION="Notify for pinnacle-wm"
HOMEPAGE="https://github.com"
LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64"

SRC_URI=""
RESTRICT="fetch"

S="${WORKDIR}/${P}/pinnacle-fm"

RDEPEND="    
	gui-wm/pinnacle-gentoo	
"
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_unpack() {
    mkdir -p "${WORKDIR}/${P}" || die
    cp -Rp "${FILESDIR}/pinnacle-fm" "${WORKDIR}/${P}/" || die
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
	domenu "pinnacle-fm.desktop"	
}   "#), 
      ("gui-apps/pinnacle-fm/files/pinnacle-fm/Cargo.toml", r#"[package]
name = "pinnacle-fm"
version = "0.1.0"
edition = "2024"

[dependencies]
gtk4 = { version = "0.10", features = ["v4_10"] }
libc = "0.2.182"
serde = { version = "1.0", features = ["derive"] }
toml = "1.1"
glib = "0.22" 
rusqlite = { version = "0.39", features = ["bundled"] }
colored = "3.1"
pango = "0.22.0"

# Это превращает src/lib.rs в библиотеку, которую можно импортировать
[lib]
name = "pinnacle_fm"
path = "src/lib.rs"

# Это превращает твой файл с интерфейсом в исполняемый бинарник
[[bin]]
name = "pinnacle-fm"
path = "src/bin/pinnacle-fm.rs"

[[bin]]
name = "pinnacle-copy"
path = "src/bin/copy.rs"

[[bin]]
name = "pinnacle_preview_sql"
path = "src/bin/pinnacle_preview_sql.rs"

[[bin]]
name = "pinnacle-fm-man"
path = "src/bin/man.rs"


[profile.dev]
opt-level = 3
lto = true
panic = "abort"
debug = false
incremental = true
 "#),

("gui-apps/pinnacle-fm/files/pinnacle-fm/src/icon.rs", r#" use std::path::Path;
use std::process::Command;
use std::fs;
use gtk4 as gtk; // Убедитесь, что используете gtk4
use gtk::prelude::*;



// В функции append_nerd_label
pub fn append_nerd_label(container: &gtk::Box, info: &IconInfo) {
    let label = gtk::Label::builder()
        .use_markup(true)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    // font_desc="32" — это примерно 128px визуально для иконок
    label.set_markup(&format!(
        "<span foreground=\"{}\" font_desc=\"48\">{}</span>", 
        info.color, info.symbol
    ));

    // Чтобы иконка не сжималась
    label.set_size_request(128, 128); 
    container.append(&label);
}

/// Структура для удобного хранения символа и цвета
pub struct IconInfo {
    pub symbol: &'static str,
    pub color: &'static str,
}

/// Основная функция палитры
pub fn get_file_info(path: &Path) -> IconInfo {
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let name_lower = file_name.to_lowercase();

    // 1. ПАПКИ — Ядовито-сине-фиолетовый
    if path.is_dir() {
        return IconInfo { symbol: "\u{f07b}", color: "#2b00ff" };
    }

    // 2. СКРЫТЫЕ ФАЙЛЫ — Серый глаз
    if file_name.starts_with('.') {
        return IconInfo { symbol: "\u{f070}", color: "#555753" };
    }

    // 3. АРХИВЫ — Золотисто-янтарный (включая bz2)
    let archive_exts = [".tar.gz", ".tar.xz", ".tar.bz2", ".zip", ".rar", ".7z", ".bz2", ".gz", ".xz"];
    if archive_exts.iter().any(|&ext| name_lower.ends_with(ext)) {
        return IconInfo { symbol: "\u{f1c6}", color: "#ce5c00" };
    }

    // 4. ОСТАЛЬНЫЕ ФАЙЛЫ ПО РАСШИРЕНИЮ
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let (s, c) = match ext.to_lowercase().as_str() {
        // Исполняемые и скрипты (Золотистый терминал)
        "sh" | "bash" | "zsh" | "fish" | "bat" | "exe" | "bin" => ("\u{f489}", "#d4a017"),
        
        // Программирование
        "rs"  => ("\u{e7a8}", "#e44c30"), // Rust
        "py"  => ("\u{e73c}", "#3572a5"), // Python
        "lua" => ("\u{e620}", "#51a0cf"), // Lua
        "js"  => ("\u{e74e}", "#f1e05a"), // JS
        "cpp" => ("\u{e61d}", "#004482"), // C++

        // МУЗЫКА (Ярко-пурпурный)
        "mp3" | "flac" | "wav" | "m4a" | "ogg" | "opus" | "aac" => ("\u{f001}", "#d33682"),

        // ВИДЕО (Сочный красный)
        "avi" | "mp4" | "mkv" | "mov" | "webm" | "wmv" | "flv" => ("\u{f16a}", "#e91224"),

        // ИЗОБРАЖЕНИЯ (Фиолетовый)
        "jpg" | "jpeg" | "png" | "svg" | "webp" | "gif" | "ico" => ("\u{f1c5}", "#75507b"),

        // ДОКУМЕНТЫ
        "pdf" => ("\u{f1c1}", "#ad2224"),
        "txt" | "md" | "conf" | "json" | "toml" | "yaml" => ("\u{f15c}", "#eeeeec"),

        // Дефолтный файл (Серый)
        _ => ("\u{f15b}", "#888a85"),
    };

    IconInfo { symbol: s, color: c }
}

/// Генерирует миниатюру видео в tmpfs через ffmpeg
// В src/icon.rs
#[allow(dead_code)]
pub fn get_media_thumbnail(path: &Path) -> Option<String> {
    let cache_dir = "/var/tmp/wm/pinnacle-cache/";
    let _ = fs::create_dir_all(cache_dir);

    let file_name = path.file_name()?.to_str()?;
    let thumb_path = format!("{}{}.jpg", cache_dir, file_name);

    if Path::new(&thumb_path).exists() {
        return Some(thumb_path);
    }

    // Универсальная команда для видео и аудио (берет первый кадр/обложку)
    let status = Command::new("ffmpeg")
        .args(&[
            "-i", path.to_str()?,
            "-vframes", "1",
            "-q:v", "4",
            "-vf", "scale=128:-1",
            &thumb_path,
            "-y", "-loglevel", "quiet"
        ])
        .status();

    if status.map_or(false, |s| s.success()) { Some(thumb_path) } else { None }
} "#),
		
    ("gui-apps/pinnacle-fm/files/pinnacle-fm/pinnacle-fm.desktop", r#"[Desktop Entry]
Name=Pinnacle FM
Comment=Rust & GTK4 File Manager
Exec=pinnacle-fm %u
Icon=system-file-manager
Terminal=false
Type=Application
Categories=System;FileTools;FileManager;Utility;
MimeType=inode/directory;
StartupWMClass=io.github.pinnacle_fm "#),
    ("gui-apps/pinnacle-fm/files/pinnacle-fm/src/bin/pinnacle-fm.rs", r#"use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{
    gio, glib, Application, ApplicationWindow, Box, Label, Orientation, 
    MenuButton, Button, CssProvider, Entry, ScrolledWindow, Revealer, ListBox
};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use ::glib::format_size;
#[path = "../icon.rs"] // Добавили две точки, так как файл теперь уровнем выше
mod icon;
//use gtk::glib as other_glib;


#[derive(Deserialize, Serialize, Default, Clone)]
struct Config {
    default_path: String,
    associations: HashMap<String, String>,
    show_hidden: bool,
    show_size: bool,
    show_type: bool,
    show_time: bool,
    folders_first: bool,
}

fn main() -> glib::ExitCode {
    let _ = prepare_environment();
    pinnacle_fm::DbEngine::setup();
    pinnacle_fm::DbEngine::seed_home_dirs(); // Наполнит базу базовыми папками
    let app = Application::builder()
        .application_id("io.github.pinnacle_fm")
        .build();
    app.connect_activate(build_ui);
    app.run()
}

// --- ВСПОМОГАТЕЛЬНЫЕ ФУНКЦИИ ---

fn save_config(path: &PathBuf, config: &Config) {
    if let Ok(toml) = toml::to_string(config) {
        let _ = fs::write(path, toml);
    }
}

fn load_config(path: &PathBuf) -> Config {
    fs::read_to_string(path)
        .ok()
        .and_then(|c| toml::from_str(&c).ok())
        .unwrap_or_default()
}

fn prepare_environment() -> std::io::Result<()> {
    let home = env::var("HOME").expect("HOME not found");
    let config_dir = PathBuf::from(&home).join(".config/pinnacle-fm");
    fs::create_dir_all(&config_dir)?;
    let config_path = config_dir.join("config.toml");
    if !config_path.exists() {
        save_config(&config_path, &Config::default());
    }
    let css_path = config_dir.join("style.css");
    let css_content = "
    .error-note { 
        background: #ff5555;
        color: white; 
        border-radius: 5px; 
        padding: 4px; 
        font-weight: bold; 
        font-family: 'MesloLGSDZ Nerd Font';
        font-size: 14px;
    }
    .success-note { 
        background: #50fa7b; 
        color: #282a36; 
        border-radius: 5px; 
        padding: 4px; 
    }
   
    .file-card:hover .file-icon-nf {
    color: #f9e2af; /* Цвет меняется при наведении на карточку */
    transform: scale(1.2);
    text-shadow: 0 0 10px rgba(249, 226, 175, 0.5); /* Легкое свечение */
}
    ";
    let _ = fs::write(&css_path, css_content);   
    Ok(())
}

fn show_toast(revealer: &Revealer, label: &Label, message: &str, is_error: bool) {
    label.set_label(message);
    label.set_css_classes(&[if is_error { "error-note" } else { "success-note" }]);
    revealer.set_reveal_child(true);
    let rev = revealer.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(3), move || {
        rev.set_reveal_child(false);
        glib::ControlFlow::Break
    });
}


    fn fill_file_list(list: &ListBox, path: &Path, cfg: &Config) {
    pinnacle_fm::DbEngine::store_path(&path.to_string_lossy());
    
    // Очищаем старый список
    while let Some(child) = list.first_child() {
        list.remove(&child);
    }

    if let Ok(entries) = fs::read_dir(path) {
        let mut entries_vec: Vec<_> = entries.flatten().collect();
        // Сортировка: папки сверху
        entries_vec.sort_by(|a, b| {
            let a_dir = a.path().is_dir();
            let b_dir = b.path().is_dir();
            if a_dir != b_dir { b_dir.cmp(&a_dir) }
            else { a.file_name().cmp(&b.file_name()) }
        });

               for entry in entries_vec {
            let p = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if !cfg.show_hidden && name.starts_with('.') { continue; }

            // ПОЛНОСТЬЮ ВЕРТИКАЛЬНАЯ СТРОКА
            let row_box = Box::new(Orientation::Vertical, 4); // 4 — отступ между этажами
            row_box.set_widget_name(&p.to_string_lossy());
            row_box.set_margin_start(12);
            row_box.set_margin_bottom(8); // Отступ между файлами

            // 1. Иконка (Верхний этаж)
            let icon = gtk::Image::from_icon_name(if p.is_dir() { "folder-symbolic" } else { "text-x-generic-symbolic" });
//            let icon_text = if p.is_dir() { "📁" } else { "📄" };
//            let icon = gtk::Label::new(Some(icon_text));

            icon.set_pixel_size(32);
            icon.set_halign(gtk::Align::Start); // Прижать к левому краю
            row_box.append(&icon);

            // 2. Имя (Второй этаж)
            let label = Label::builder()
                .label(&name)
                .halign(gtk::Align::Start) // Вместо xalign используем halign
                .build();
            row_box.append(&label);

            // 3. Размер (Третий этаж)
           let metadata = entry.metadata().ok();
           let bytes = metadata.map(|m| m.len()).unwrap_or(0);

// Вызываем функцию (теперь ворнинг точно уйдет)
let size_str = if p.is_dir() { String::new() } else { format_size(bytes).to_string() };

let size_label = Label::builder()
    .label(&size_str)
    .halign(gtk::Align::Start) // В списке лучше прижать к левому краю
    .css_classes(["dim-label"])
    .build();

row_box.append(&size_label);
            // 4. Кнопка меню (Нижний этаж)
            let menu_btn = MenuButton::builder()
                .icon_name("view-more-symbolic")
                .css_classes(["flat"])
                .halign(gtk::Align::Start)
                .build();
            
            let menu = pinnacle_fm::FileMenu::create_for_path(p.to_string_lossy().to_string(), path.to_string_lossy().to_string());
            menu_btn.set_popover(Some(&menu));
            row_box.append(&menu_btn);

            list.append(&row_box);
        }

    }
}

  fn fill_file_grid(list: &ListBox, path: &Path, cfg: &Config, last_selected: Rc<RefCell<String>>) {
    pinnacle_fm::DbEngine::store_path(&path.to_string_lossy());
    
    // 1. Очистка старого содержимого
    while let Some(child) = list.first_child() {
        list.remove(&child);
    }

    // 2. Расчет колонок (динамика от 2 до 6 в зависимости от ширины)
    let width = list.width();
    let columns = (width / 160).clamp(2, 6);

    if let Ok(entries) = fs::read_dir(path) {
        let mut entries_vec: Vec<_> = entries.flatten()
            .filter(|e| cfg.show_hidden || !e.file_name().to_string_lossy().starts_with('.'))
            .collect();

        // Сортировка: папки всегда сверху
        entries_vec.sort_by(|a, b| {
            let a_dir = a.path().is_dir();
            let b_dir = b.path().is_dir();
            if a_dir != b_dir { b_dir.cmp(&a_dir) }
            else { a.file_name().cmp(&b.file_name()) }
        });

        // 3. Создание сетки через горизонтальные ряды
        for chunk in entries_vec.chunks(columns as usize) {
            let row_layout = Box::builder()
                .orientation(Orientation::Horizontal)
                .homogeneous(true) // Одинаковая ширина колонок
                .spacing(12)
                .margin_start(12)
                .margin_end(12)
                .margin_bottom(15)
                .build();

            for entry in chunk {
                let p = entry.path();
                let p_str = p.to_string_lossy().to_string();
                let name = entry.file_name().to_string_lossy().to_string();

                // КАРТОЧКА ФАЙЛА (Лэйаут)
                let item_box = Box::new(Orientation::Vertical, 6);
                item_box.add_css_class("file-card"); // Класс для подсветки из CSS
                item_box.set_focusable(true);
                item_box.set_cursor_from_name(Some("pointer"));

 // 1. Создаем контейнер и сразу ставим Nerd-иконку как заглушку
let icon_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
icon_container.set_size_request(128, 128);

let info = icon::get_file_info(&p);
icon::append_nerd_label(&icon_container, &info);

// 2. Подготовленные данные для отложенной загрузки
let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
let p_path = p.to_path_buf();
let c_box = icon_container.clone();

// 3. Отложенная загрузка (выполнится сразу после отрисовки окна)
if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "webp" | "gif" | "mp4" | "mkv" | "mp3" | "flac") {
    glib::idle_add_local(move || {
        let thumb_path = if matches!(ext.as_str(), "mp4" | "mkv" | "mp3" | "flac") {
            icon::get_media_thumbnail(&p_path)
        } else {
            Some(p_path.to_string_lossy().into_owned())
        };

        if let Some(t) = thumb_path {
            if let Ok(pb) = gtk::gdk_pixbuf::Pixbuf::from_file_at_size(&t, 128, 128) {
                // Если картинка загрузилась, убираем Nerd-иконку и ставим превью
                if let Some(child) = c_box.first_child() {
                    c_box.remove(&child);
                }
                let img = gtk::Image::from_pixbuf(Some(&pb));
                img.set_pixel_size(128);
                c_box.append(&img);
            }
        }
        glib::ControlFlow::Break // Выполнить один раз
    });
}

item_box.append(&icon_container);



                // Имя (центровка + перенос + фикс GString)
                let label = Label::builder()
                    .label(name.clone())
                    .halign(gtk::Align::Center)
                    .wrap(true)
                    .wrap_mode(gtk::pango::WrapMode::WordChar)
                    .max_width_chars(15)
                    .build();
                item_box.append(&label);
                
       let metadata = entry.metadata().ok();
       let size_text = if p.is_dir() {
        String::new() 
      } else {
    // ВЫЗЫВАЕМ ТУ САМУЮ ФУНКЦИЮ format_size
       let bytes = metadata.map(|m| m.len()).unwrap_or(0);
       format_size(bytes).to_string() 
};

let size_label = Label::builder()
    .label(size_text)
    .halign(gtk::Align::Center)
    .css_classes(["dim-label"]) // Делаем текст чуть бледнее (если есть в CSS)
    .build();

if !p.is_dir() {
    item_box.append(&size_label);
}

                // Кнопка меню (MenuButton, а не Builder!)
                let menu_btn = MenuButton::builder()
                    .icon_name("view-more-symbolic")
                    .css_classes(["flat"])
                    .halign(gtk::Align::Center)
                    .build();
                let menu = pinnacle_fm::FileMenu::create_for_path(p_str.clone(), path.to_string_lossy().to_string());
                menu_btn.set_popover(Some(&menu));
                item_box.append(&menu_btn);

                // --- ЛОГИКА МЫШИ (ОДИНАРНЫЙ КЛИК) ---
                let gesture = gtk::GestureClick::new();
                let p_clone = p.clone();
                let list_clone = list.clone();
                let ls_c = last_selected.clone(); // Клон для записи выбора
                let cfg_clone = cfg.clone();

               gesture.connect_released(move |_, n_press, _, _| {
    if n_press == 1 {
        // 1. Запоминаем выбор для меню
        *ls_c.borrow_mut() = p_clone.to_string_lossy().to_string();

        if p_clone.is_dir() {
            // Переход по папке
            fill_file_grid(&list_clone, &p_clone, &cfg_clone, ls_c.clone());
        } else {
            // --- ЛОГИКА АССОЦИАЦИЙ ИЗ ВАШЕГО КОДА ---
            let ext = p_clone.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
            
            // Проверяем ваш HashMap associations
            if let Some(app_id) = cfg_clone.associations.get(&ext) {
                if let Some(app) = gio::AppInfo::all().into_iter()
                    .find(|a| a.id().map(|id| id == *app_id).unwrap_or(false)) 
                {
                    let _ = app.launch(&[gio::File::for_path(&p_clone)], None::<&gio::AppLaunchContext>);
                    return;
                }
            }
            // Если в конфиге пусто — запускаем системный по умолчанию
            let _ = gio::AppInfo::launch_default_for_uri(
                &format!("file://{}", p_clone.display()), 
                None::<&gio::AppLaunchContext>
            );
        }
    }
});

                item_box.add_controller(gesture);

                row_layout.append(&item_box);
            }
            list.append(&row_layout);
        }
    }
    
    // ВАЖНО: Убираем синюю полосу на весь ряд
    list.set_selection_mode(gtk::SelectionMode::None);
}

// --- ДИАЛОГИ ---

fn show_create_dialog(parent: &ApplicationWindow, current_dir: PathBuf, is_dir: bool, list: ListBox, cfg: Config) {
    let dialog = gtk::Window::builder()
        .title(if is_dir { "Новая папка" } else { "Новый файл" })
        .transient_for(parent)
        .modal(true)
        .default_width(300)
        .build();

    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.set_margin_end(12);

    let entry = Entry::builder().placeholder_text("Название...").build();
    vbox.append(&entry);

    let btn = Button::builder().label("Создать").css_classes(["suggested-action"]).build();
    
    let d_c = dialog.clone();
    let l_c = list.clone();
    let p_c = current_dir.clone();
    let cfg_c = cfg.clone();

    btn.connect_clicked(move |_| {
        let name = entry.text().to_string().trim().to_string();
        if !name.is_empty() {
            let res = if is_dir {
                fs::create_dir(p_c.join(&name))
            } else {
                fs::File::create(p_c.join(&name)).map(|_| ())
            };

            if res.is_ok() {
                fill_file_list(&l_c, &p_c, &cfg_c);
                d_c.close();
            }
        }
    });

    vbox.append(&btn);
    dialog.set_child(Some(&vbox));
    dialog.present();
}

fn show_rename_dialog(parent: &ApplicationWindow, current_path: String, list: ListBox, cfg: Config) {
    let old_path = PathBuf::from(&current_path);
    let old_name = old_path.file_name().unwrap_or_default().to_string_lossy().to_string();
    let parent_path = old_path.parent().unwrap_or(Path::new("/")).to_path_buf();

    let dialog = gtk::Window::builder().title("Переименовать").transient_for(parent).modal(true).default_width(300).build();
    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.set_margin_end(12);

    let entry = Entry::builder().text(&old_name).build();
    vbox.append(&entry);

    let btn = Button::builder().label("ОК").css_classes(["suggested-action"]).build();
    let d_c = dialog.clone();
    let l_c = list.clone();
    let cfg_c = cfg.clone();

    btn.connect_clicked(move |_| {
        let new_name = entry.text().to_string().trim().to_string();
        if !new_name.is_empty() && new_name != old_name {
            let new_path = parent_path.join(new_name);
            if fs::rename(&old_path, &new_path).is_ok() {
                fill_file_list(&l_c, &parent_path, &cfg_c);
                d_c.close();
            }
        }
    });

    vbox.append(&btn);
    dialog.set_child(Some(&vbox));
    dialog.present();
}

fn show_associations_dialog(parent: &ApplicationWindow, config_path: &PathBuf) {
    let dialog = gtk::Window::builder()
        .title("Ассоциации")
        .transient_for(parent)
        .modal(true)
        .default_width(380)
        .default_height(500)
        .build();

    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.set_margin_end(12);

    let ext_entry = Entry::builder().placeholder_text("Расширение (txt)").build();
    vbox.append(&ext_entry);

    let list_box = ListBox::builder().selection_mode(gtk::SelectionMode::Single).build();
    for app in gio::AppInfo::all() {
        if let Some(id) = app.id() {
            let l = Label::builder().label(&*app.display_name()).xalign(0.0).margin_start(6).name(id.to_string()).build();
            list_box.append(&l);
        }
    }
    let scroll = ScrolledWindow::builder().vexpand(true).min_content_height(250).child(&list_box).build();
    vbox.append(&scroll);

    let btn = Button::builder().label("Сохранить").css_classes(["suggested-action"]).build();
    let d_c = dialog.clone(); let e_c = ext_entry.clone(); let l_c = list_box.clone(); let p_c = config_path.clone();
    btn.connect_clicked(move |_| {
        if let Some(row) = l_c.selected_row() {
            let label = row.child().unwrap().downcast::<Label>().unwrap();
            let mut cfg = load_config(&p_c);
            cfg.associations.insert(e_c.text().to_string().trim().replace('.', "").to_lowercase(), label.widget_name().to_string());
            save_config(&p_c, &cfg);
            d_c.close();
        }
    });
    vbox.append(&btn);
    dialog.set_child(Some(&vbox));
    dialog.present();
}


// --- BUILD UI ---

fn build_ui(app: &Application) {
	    // 1. Подключаем стили для подсветки карточек
    let provider = CssProvider::new();
    provider.load_from_data("
        .file-card { 
            padding: 10px; 
            border-radius: 8px; 
        }
        .file-card:hover { 
            background-color: rgba(255, 255, 255, 0.08); 
        }
    ");

    // ИСПОЛЬЗУЕМ gdk ДЛЯ ПОЛУЧЕНИЯ ДИСПЛЕЯ
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
    

    let home = env::var("HOME").unwrap_or_default();
    let config_path = PathBuf::from(&home).join(".config/pinnacle-fm/config.toml");
    
    let current_config = Rc::new(RefCell::new(load_config(&config_path)));
    let current_dir = Rc::new(RefCell::new(if current_config.borrow().default_path.is_empty() { 
        PathBuf::from(&home) 
    } else { 
        PathBuf::from(&current_config.borrow().default_path) 
    }));

    let window = ApplicationWindow::builder().application(app).title("🏔️  Pinnacle FM").default_width(900).default_height(600).build();
    let last_selected = Rc::new(RefCell::new(String::new()));

    let main_vbox = Box::new(Orientation::Vertical, 0);
    let content_hbox = Box::new(Orientation::Horizontal, 0);
    window.set_child(Some(&main_vbox));

    let sidebar_scroll = ScrolledWindow::builder().width_request(200).build();
        let sidebar_list = ListBox::builder().build();
    // Достаем текущего пользователя для USB
    let user = std::env::var("USER").unwrap_or_else(|_| "default".to_string());
    
    let places = [
    ("\u{f015}  Домой", home.clone()), 
    ("\u{f019}  Загрузки", format!("{}/Downloads", home)), 
    ("\u{f15c}  Документы", format!("{}/Documents", home)), 
    ("\u{f001}  Музыка", format!("{}/Музыка", home)), 
    ("\u{f1c5}  Изображения", format!("{}/Изображения", home)), 
    ("---", String::new()), 
    ("\u{f17c}  Система", "/".to_string()), 
    ("---", String::new()), 
    ("\u{f0a0}  USB", format!("/run/media/{}/", user)) 
];


    for (name, path) in places {
        if name == "---" {
            // Настоящий разделитель вместо забора из подчёркиваний
            let sep = gtk::Separator::new(gtk::Orientation::Horizontal);
            sep.set_margin_top(10);
            sep.set_margin_bottom(10);
            sep.set_opacity(0.3);
            sidebar_list.append(&sep);
        } else {
            let lbl = Label::builder()
                .label(name)
                .xalign(0.0)
                .margin_start(12) // Отступ текста от левого края
                .build();
            lbl.set_widget_name(&path);
            sidebar_list.append(&lbl);
        }
    }
    sidebar_scroll.set_child(Some(&sidebar_list));


    let right_vbox = Box::new(Orientation::Vertical, 0);
    right_vbox.set_hexpand(true);

    let toast_label = Label::builder().build();
    let revealer = Revealer::builder().child(&toast_label).transition_type(gtk::RevealerTransitionType::SlideDown).build();
    right_vbox.append(&revealer); 

    let top_bar = Box::builder().orientation(Orientation::Horizontal).margin_end(8).spacing(6).build();
    let status_label = Label::builder().label("Готов").hexpand(true).xalign(0.0).margin_start(10).build();
    let menu_button = MenuButton::builder().icon_name("view-more-symbolic").build();
    
   // Найди создание списка
   // Возвращаем как было
    let file_list = ListBox::builder()
    .selection_mode(gtk::SelectionMode::Single)
    .build();


// Найди ScrolledWindow, в котором он лежит
    let file_scroll = ScrolledWindow::builder()
    .hexpand(true) // Важно: контейнер ТОЖЕ должен тянуться
    .vexpand(true)
    .child(&file_list)
    .build();


        let up_btn = Button::builder().icon_name("go-up-symbolic").build();
    let add_file_btn = Button::builder().icon_name("document-new-symbolic").build();
    let add_dir_btn = Button::builder().icon_name("folder-new-symbolic").build();

    // 1. Создаем кнопку "Вид"
   // Там, где создаете view_btn:
    let view_btn = Button::builder()
    .icon_name("view-list-bullet-symbolic") // Показываем иконку списка, так как мы УЖЕ в сетке
    .build();


// Переменная для отслеживания текущего вида (true = сетка, false = список)
    let is_grid = Rc::new(RefCell::new(true)); 
   // 1. Сначала создаем тот самый btn_c, который потерялся

    let btn_c = view_btn.clone(); 

    // 2. Создаем клоны специально для внутренностей замыкания (чтобы не "красть" оригиналы)
    let fl_for_btn = file_list.clone(); 
    let cd_for_btn = current_dir.clone();
    let cfg_for_btn = current_config.clone();
    let ls_for_btn = last_selected.clone();
    let is_grid_for_btn = is_grid.clone();
    let btn_handle = btn_c.clone(); // Для смены иконки внутри

    // 3. Вешаем событие (оно заберет клоны "for_btn")
    btn_c.connect_clicked(move |_| {
        let mut grid = is_grid_for_btn.borrow_mut();
        *grid = !*grid; 
        
        if *grid {
            btn_handle.set_icon_name("view-list-bullet-symbolic");
            fill_file_grid(&fl_for_btn, &cd_for_btn.borrow(), &cfg_for_btn.borrow(), ls_for_btn.clone());
        } else {
            btn_handle.set_icon_name("view-grid-symbolic");
            fill_file_list(&fl_for_btn, &cd_for_btn.borrow(), &cfg_for_btn.borrow());
           
        }
    });

    // 4. Отрисовка при запуске (используем ОРИГИНАЛЬНЫЕ переменные, они выжили!)
    fill_file_grid(&file_list, &current_dir.borrow(), &current_config.borrow(), last_selected.clone());

    // --- ИСПРАВЛЕННАЯ ШАПКА ---
let header_box = gtk::Grid::builder()
    .column_spacing(10)
    .margin_start(12)
    .margin_end(6)
    .opacity(0.5)
    .build();

// 0. Пустое место под иконку (ширина 30)
let h_pad_left = Box::builder().width_request(30).build();
header_box.attach(&h_pad_left, 0, 0, 1, 1);

// 1. Заголовок "Имя"
let h_name = Label::builder().label("Имя").hexpand(true).xalign(0.0).build();
header_box.attach(&h_name, 1, 0, 1, 1);

// 2. Заголовок "Размер"
let h_size = Label::builder().label("Размер").width_request(100).xalign(0.0).build();
header_box.attach(&h_size, 2, 0, 1, 1);

// 3. Пустое место под кнопку меню (ширина 40)
let h_pad_right = Box::builder().width_request(40).build();
header_box.attach(&h_pad_right, 3, 0, 1, 1);

right_vbox.append(&header_box);


    // Устанавливаем начальную видимость шапки из конфига
    header_box.set_visible(current_config.borrow().show_size);

    top_bar.append(&up_btn);
    top_bar.append(&view_btn); 
    top_bar.append(&add_file_btn);
    top_bar.append(&add_dir_btn);
    top_bar.append(&status_label);
    top_bar.append(&menu_button);
    
    right_vbox.append(&top_bar);
    right_vbox.append(&header_box); 
    right_vbox.append(&file_scroll);

    content_hbox.append(&sidebar_scroll);
    content_hbox.append(&right_vbox);
    main_vbox.append(&content_hbox);

   let btn_v = view_btn.clone();

// 1. Создаем персональный клон для ЭТОЙ кнопки
    let ls_for_v = last_selected.clone();
    let fl_v = file_list.clone();
    let cd_v = current_dir.clone();
    let cfg_v = current_config.clone();
    let is_grid_v = is_grid.clone();

btn_v.clone().connect_clicked(move |_| {
    let _grid_state = is_grid_v.borrow(); 
    // 2. Используем КЛОН ls_for_v вместо оригинала
    fill_file_grid(&fl_v, &cd_v.borrow(), &cfg_v.borrow(), ls_for_v.clone());
});

// 3. Теперь оригинальный last_selected ЖИВ для финального вызова в конце!
fill_file_grid(&file_list, &current_dir.borrow(), &current_config.borrow(), last_selected.clone());

// ПЕРВИЧНАЯ ОТРИСОВКА (используем оригиналы, они теперь свободны)
    fill_file_grid(&file_list, &current_dir.borrow(), &current_config.borrow(), last_selected.clone());
        // --- ОБРАБОТЧИКИ КЛАВИАТУРЫ ---
    let key_controller = gtk::EventControllerKey::new();

    let cd_k = current_dir.clone();
    let sl_k = status_label.clone();
    let fl_k = file_list.clone();
    let cfg_k = current_config.clone();

    key_controller.connect_key_pressed(move |_, key, _, _| {
        if key == gtk::gdk::Key::BackSpace {
            // 1. Пытаемся достать путь из истории (БД)
            let target_path = if let Some(mutex) = pinnacle_fm::DB_GLOBAL.get() {
                if let Ok(db) = mutex.lock() {
                    let db: &pinnacle_fm::DbEngine = &*db; 
                    let stmt = db.conn.prepare("SELECT path FROM history ORDER BY id DESC LIMIT 1 OFFSET 1").ok();
                    stmt.and_then(|mut s| s.query_row([], |row: &rusqlite::Row| row.get::<usize, String>(0)).ok())
                } else { None }
            } else { None };

            // 2. Если в базе есть путь "назад"
            if let Some(path_str) = target_path {
                let p_buf = std::path::PathBuf::from(&path_str);

                // Обновляем состояние (RefCell)
                {
                    let mut curr = cd_k.borrow_mut();
                    *curr = p_buf.clone();
                }

                // Обновляем UI
                sl_k.set_label(&path_str);
                fill_file_list(&fl_k, &p_buf, &cfg_k.borrow());

                // 3. Удаляем последнюю запись, чтобы история двигалась назад
                if let Some(mutex) = pinnacle_fm::DB_GLOBAL.get() {
                    if let Ok(db) = mutex.lock() {
                        let _ = db.conn.execute("DELETE FROM history WHERE id = (SELECT MAX(id) FROM history)", []);
                    }
                }
                
                // Останавливаем событие, чтобы оно не ушло в другие виджеты
                return gtk::glib::Propagation::Stop;
            }
        }
        gtk::glib::Propagation::Proceed
    });

    window.add_controller(key_controller);

    let _fl_s = file_list.clone(); let _cd_s = current_dir.clone(); let _cfg_s = current_config.clone(); let _sl_s = status_label.clone();
        
        let fl_s = file_list.clone(); 
    let cd_s = current_dir.clone(); 
    let cfg_s = current_config.clone(); 
    let ls_s = last_selected.clone(); 

    sidebar_list.connect_row_activated(move |_, row| {
        if let Some(widget) = row.child() {
            if widget.type_().name() == "GtkSeparator" { return; }

            if let Some(label) = widget.downcast_ref::<Label>() {
                let path_str = label.widget_name().to_string();
                if path_str.is_empty() { return; }
                
                let path = PathBuf::from(&path_str);
                *cd_s.borrow_mut() = path.clone();
                let list_clone = fl_s.clone();
                let p_clone = path.clone();
                let cfg_clone = cfg_s.borrow().clone();
                let ls_c = ls_s.clone();

                fill_file_grid(&list_clone, &p_clone, &cfg_clone, ls_c);
            }
        }
    });

    
    let fl_f = file_list.clone(); let cd_f = current_dir.clone(); let cfg_f = current_config.clone(); let sl_f = status_label.clone(); let ls_f = last_selected.clone();
    file_list.connect_row_activated(move |_, row| {
    
    let rb = row.child().unwrap().downcast::<gtk::Box>().expect("Ожидался GtkBox"); 
    
    let p_str = rb.widget_name().to_string();
    let path = PathBuf::from(&p_str);
    *ls_f.borrow_mut() = p_str.clone();

    if path.is_dir() {
        *cd_f.borrow_mut() = path.clone();
        sl_f.set_label(&path.to_string_lossy());
        fill_file_list(&fl_f, &path, &cfg_f.borrow());
    } else {
        
        let _ = gio::AppInfo::launch_default_for_uri(
            &format!("file://{}", path.display()), 
            None::<&gio::AppLaunchContext>
        );
    }
});

    
    let _fl_u = file_list.clone(); let _cd_u = current_dir.clone(); let _cfg_u = current_config.clone(); let sl_u = status_label.clone();
    let fl_u = file_list.clone(); 
    let cd_u = current_dir.clone(); 
    let cfg_u = current_config.clone(); 
    let ls_u = last_selected.clone(); // ДОБАВИЛИ КЛОН ДЛЯ СЕТКИ
    up_btn.connect_clicked(move |_| {
    let mut target_path: Option<String> = None;

    if let Some(mutex) = pinnacle_fm::DB_GLOBAL.get() {
        if let Ok(mut db_lock) = mutex.lock() {
            let db = &mut *db_lock; 
            
            // 1. Сначала находим ПРЕДЫДУЩИЙ путь (OFFSET 1)
            let query = "SELECT path FROM history ORDER BY id DESC LIMIT 1 OFFSET 1";
            if let Ok(path) = db.conn.query_row(query, [], |row| row.get::<_, String>(0)) {
                target_path = Some(path);
                
                // 2. УДАЛЯЕМ текущий (последний) путь, чтобы "Назад" действительно сдвинуло историю
                let _ = db.conn.execute("DELETE FROM history WHERE id = (SELECT max(id) FROM history)", []);
            }
        }
    }

    // 3. Если путь найден, отрисовываем СЕТКУ
    if let Some(path_str) = target_path {
        let pb = PathBuf::from(&path_str);
        *cd_u.borrow_mut() = pb.clone(); // Обновляем текущий путь в памяти
        
        // Вызываем отрисовку СЕТКИ (всегда по умолчанию)
        fill_file_grid(&fl_u, &pb, &cfg_u.borrow(), ls_u.clone());
        
        // Обновляем статусную строку (если есть sl_u)
        sl_u.set_label(&format!("Переход в: {}", path_str));
    }
});

    let win_af = window.clone(); let cd_af = current_dir.clone(); let fl_af = file_list.clone(); let cfg_af = current_config.clone();
    add_file_btn.connect_clicked(move |_| { show_create_dialog(&win_af, cd_af.borrow().clone(), false, fl_af.clone(), cfg_af.borrow().clone()); });

    let win_ad = window.clone(); let cd_ad = current_dir.clone(); let fl_ad = file_list.clone(); let cfg_ad = current_config.clone();
    add_dir_btn.connect_clicked(move |_| { show_create_dialog(&win_ad, cd_ad.borrow().clone(), true, fl_ad.clone(), cfg_ad.borrow().clone()); });

    // --- ACTIONS ---

    let menu_model = gio::Menu::new();
    menu_model.append(Some("⚙️ Ассоциации"), Some("win.edit-apps"));
    menu_model.append(Some("Копировать путь"), Some("win.copy-path"));
    menu_model.append(Some("🖊 Переименовать"), Some("win.rename-file"));
    menu_model.append(Some("🗑 Удалить файл"), Some("win.delete-file"));
    menu_button.set_menu_model(Some(&menu_model));

    let win_e = window.clone(); let cp_e = config_path.clone();
    let edit_act = gio::SimpleAction::new("edit-apps", None);
    edit_act.connect_activate(move |_, _| { show_associations_dialog(&win_e, &cp_e); });
    window.add_action(&edit_act);

    let ls_r = last_selected.clone(); let win_r = window.clone(); let fl_r = file_list.clone(); let cfg_r = current_config.clone();
    let rename_act = gio::SimpleAction::new("rename-file", None);
    rename_act.connect_activate(move |_, _| {
        let p = ls_r.borrow().clone();
        if !p.is_empty() { show_rename_dialog(&win_r, p, fl_r.clone(), cfg_r.borrow().clone()); }
    });
    window.add_action(&rename_act);

    let ls_c = last_selected.clone(); let rev_c = revealer.clone(); let tl_c = toast_label.clone();
    let copy_act = gio::SimpleAction::new("copy-path", None);
    copy_act.connect_activate(move |_, _| {
        let p = ls_c.borrow().clone();
        if !p.is_empty() {
            if let Some(d) = gtk::gdk::Display::default() {
                d.clipboard().set_text(&p);
                show_toast(&rev_c, &tl_c, "Скопировано", false);
            }
        }
    });
    window.add_action(&copy_act);

       let ls_d = last_selected.clone(); let rev_d = revealer.clone(); let tl_d = toast_label.clone();
    let _fl_d = file_list.clone(); let _cd_d = current_dir.clone(); let _cfg_d = current_config.clone();
    let del_act = gio::SimpleAction::new("delete-file", None);
    
    del_act.connect_activate(move |_, _| {
        let p = ls_d.borrow().clone();
        if !p.is_empty() {
            if fs::remove_file(&p).is_ok() || fs::remove_dir_all(&p).is_ok() {
                show_toast(&rev_d, &tl_d, "Удалено", false);
                
               
      fill_file_grid(&file_list, &current_dir.borrow(), &current_config.borrow(), last_selected.clone());

            }
        }
    });
    window.add_action(&del_act);
   

window.present();
    
    let _ = std::process::Command::new("pinnacle_preview_sql")
        .spawn();     
}
 "#),
     ("gui-apps/pinnacle-fm/files/pinnacle-fm/src/bin/pinnacle_preview_sql.rs", r#"use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{gio, glib, gdk, Application, Box, Image, Label, Orientation, Window};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("io.github.pinnacle_preview_sql")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::builder()
        .application(app)
        .decorated(false)
        .can_focus(false)
        .resizable(false)
        .default_width(320)
        .build();
        
    
    // Стили под Pinnacle Preview
    let provider = gtk::CssProvider::new();
    provider.load_from_data("
        window { 
            background-color: rgba(20, 20, 30, 0.95); 
            border: 1px solid rgba(255, 255, 255, 0.1); 
            border-radius: 12px; 
            padding: 15px;
        }
        .name { font-weight: bold; font-size: 14px; color: #ffffff; }
        .info { font-size: 11px; color: #a6adc8; margin-top: 4px; }
    ");
    
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Display Error"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let root_box = Box::new(Orientation::Horizontal, 15);
    let icon_img = Image::builder().pixel_size(48).build();
    let text_box = Box::new(Orientation::Vertical, 0);
    
    let name_label = Label::builder().xalign(0.0).css_classes(["name"]).build();
    let info_label = Label::builder().xalign(0.0).css_classes(["info"]).build();

    text_box.append(&name_label);
    text_box.append(&info_label);
    root_box.append(&icon_img);
    root_box.append(&text_box);
    window.set_child(Some(&root_box));

    // Храним последний путь, чтобы не спамить окном
    let last_path = Arc::new(Mutex::new(String::new()));
    
    let win_c = window.clone();
    let icon_c = icon_img.clone();
    let name_c = name_label.clone();
    let info_c = info_label.clone();

    // ТАЙМЕР: Проверка базы каждые 300мс
    glib::timeout_add_local(std::time::Duration::from_millis(300), move || {
        let _home = std::env::var("HOME").unwrap();
        let db_path = "/var/tmp/wm/history.db".to_string();

        // Открываем базу только для чтения
        if let Ok(conn) = Connection::open_with_flags(&db_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY) {
            let mut stmt = conn.prepare("SELECT path FROM history ORDER BY id DESC LIMIT 1").unwrap();
            let current_path: Result<String, _> = stmt.query_row([], |row| row.get(0));

            if let Ok(path) = current_path {
                let mut last = last_path.lock().unwrap();
                
                // Если путь изменился — это новый визит!
                if path != *last {
                    *last = path.clone();
                    
                    let file = gio::File::for_path(&path);
                    if let Ok(info) = file.query_info("standard::*", gio::FileQueryInfoFlags::NONE, None::<&gio::Cancellable>) {
                        
                        name_c.set_text(&info.display_name());
                        let size_mb = info.size() as f64 / 1024.0 / 1024.0;
                        info_c.set_text(&format!("{:.2} MB • {}", size_mb, path));
                        
                        if let Some(gicon) = info.icon() {
                            icon_c.set_from_gicon(&gicon);
                        }

                        // Показываем превью
                        win_c.set_visible(true);
                        win_c.present();

                        // Скрываем через 2 секунды
                        let w_hide = win_c.clone();
                        glib::timeout_add_local(std::time::Duration::from_millis(2000), move || {
                            w_hide.set_visible(false);
                            glib::ControlFlow::Break
                        });
                    }
                }
            }
        }
        glib::ControlFlow::Continue
    });
}
 "#),
     ("gui-apps/pinnacle-fm/files/pinnacle-fm/src/bin/copy.rs", r#"use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Проверка аргументов (0 - имя проги, 1 - откуда, 2 - куда)
    if args.len() < 3 {
        eprintln!(">>> ОШИБКА: Мало аргументов. Нужно: откуда куда");
        std::process::exit(1);
    }

    let src = PathBuf::from(&args[1]);
    let dst = PathBuf::from(&args[2]);

    // Если папка назначения не существует - создаем её родителя
    if let Some(parent) = dst.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let result = if src.is_dir() {
        copy_dir_all(&src, &dst)
    } else {
        fs::copy(&src, &dst).map(|_| ())
    };

    match result {
        Ok(_) => println!(">>> УСПЕХ: {:?} скопирован в {:?}", src, dst),
        Err(e) => {
            eprintln!(">>> ОШИБКА системы: {}", e);
            std::process::exit(1);
        }
    }
} "#),
      ("gui-apps/pinnacle-fm/files/pinnacle-fm/src/bin/man.rs", r#"use std::{thread, io::{self, Read, Write}, time::Duration};
use std::process::Command;
use colored::Colorize;

fn my_custom_logic() {
	println!("{}", "Pinnacle-fm — НЕБОЛЬШОЙ ОБОЗРЕВАТЕЛЬ ФАЙЛОВ".bold().blue());
        println!("\n{}", "КОМАНДЫ:".yellow());
        println!("pinnacle-fm: вызов");        
        println!("\n{}", "ОПИСАНИЕ:".green());
        println!("Для изменения внешнего вида и нужных вам разделов-");
        println!("меняйте пути в исходнике.");
         println!("\n{}", "НЕ СОВЕТУЮ ЗАПУСКАТЬ ОТ root!".red());      
        println!("\x1B[1;32m[SYSTEM OK]\x1B[0m Инициализация мониторинга pkgrs...");
    
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
}
       "#), 
       ("gui-apps/pinnacle-fm/files/pinnacle-fm/src/lib.rs", r#"use gtk4 as gtk;
use gtk::gio;
use gtk::prelude::*;
use rusqlite::{Connection, Result as SqlResult};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};

// Дальше идет РАЗДЕЛ 1...

// =========================================================================
// РАЗДЕЛ 1: ГЛОБАЛЬНЫЙ БУФЕР (Память для Copy-Paste)
// =========================================================================
static COPY_BUFFER: Mutex<Option<PathBuf>> = Mutex::new(None);

// =========================================================================
// РАЗДЕЛ 2: БАЗА ДАННЫХ (История переходов)
// =========================================================================
pub static DB_GLOBAL: OnceLock<Mutex<DbEngine>> = OnceLock::new();

pub struct DbEngine { pub conn: Connection }

impl DbEngine {
    pub fn setup() {
        let home = std::env::var("HOME").unwrap_or_default();
        let db_path = PathBuf::from(&home).join("/var/tmp/wm/history.db");
        
        // 1. Открываем соединение
        if let Ok(conn) = Connection::open(&db_path) {
            // 2. СРАЗУ создаем таблицу (без лишних проверок)
            let _ = conn.execute(
                "CREATE TABLE IF NOT EXISTS history (
                    id INTEGER PRIMARY KEY AUTOINCREMENT, 
                    path TEXT NOT NULL, 
                    visit_time DATETIME DEFAULT CURRENT_TIMESTAMP
                )", 
                []
            );
            
            // 3. Кладем в глобальную переменную
            let _ = DB_GLOBAL.set(Mutex::new(DbEngine { conn }));
            println!(">>> База данных инициализирована по адресу: {:?}", db_path);
        } else {
            eprintln!(">>> ОШИБКА: Не удалось открыть базу данных!");
        }
    }
pub fn seed_home_dirs() {
    let home = std::env::var("HOME").unwrap_or_default();
    let dirs = vec!["Downloads", "Documents", "Pictures", "Videos", "Music"];
    
    for dir in dirs {
        let full_path = format!("{}/{}", home, dir);
        Self::store_path(&full_path);
    }
}

    pub fn store_path(path: &str) {
    println!(">>> Пытаюсь записать путь: {}", path); // ТЕСТ: появится ли это в терминале?
    if let Some(mutex) = DB_GLOBAL.get() {
        match mutex.lock() {
            Ok(db) => {
                let res = db.conn.execute("INSERT INTO history (path) VALUES (?1)", [path]);
                println!(">>> Результат записи: {:?}", res);
            },
            Err(e) => println!(">>> ОШИБКА МЬЮТЕКСА: {:?}", e),
        }
    } else {
        println!(">>> ОШИБКА: База DB_GLOBAL не инициализирована!");
    }
}

}


// =========================================================================
// РАЗДЕЛ 3: САМОДОСТАТОЧНОЕ МЕНЮ (Интерфейс)
// =========================================================================
pub struct FileMenu;
impl FileMenu {
    pub fn create_for_path(src_path: String, target_dir: String) -> gtk::PopoverMenu {
        let src = PathBuf::from(&src_path);
        let current_folder = PathBuf::from(&target_dir);
        
        let menu_model = gio::Menu::new();
        menu_model.append(Some("Копировать в буфер"), Some("menu.copy"));
        menu_model.append(Some("Вставить сюда"), Some("menu.paste"));
        menu_model.append(Some("🗑 Удалить"), Some("menu.delete"));

        let popover = gtk::PopoverMenu::builder().menu_model(&menu_model).build();
        let action_group = gio::SimpleActionGroup::new();

        // --- ЛОГИКА: КОПИРОВАТЬ ---
        let p_src = src.clone();
        let copy_act = gio::SimpleAction::new("copy", None);
        copy_act.connect_activate(move |_, _| {
            let mut buffer = COPY_BUFFER.lock().unwrap();
            *buffer = Some(p_src.clone());
            println!(">>> Файл в буфере: {:?}", p_src);
        });

        // --- ЛОГИКА: ВСТАВИТЬ (Вызов бинарника) ---
        let p_dest_dir = if src.is_dir() { src.clone() } else { current_folder.clone() };
        let paste_act = gio::SimpleAction::new("paste", None);
        paste_act.connect_activate(move |_, _| {
            let buffer = COPY_BUFFER.lock().unwrap();
            if let Some(from_path) = buffer.as_ref() {
                let to_path = p_dest_dir.join(from_path.file_name().unwrap());
                println!(">>> ЗАПУСК copy.rs: {:?} -> {:?}", from_path, to_path);
                
                if let Ok(mut exe) = std::env::current_exe() {
                    exe.set_file_name("pinnacle-copy");
                    let _ = Command::new(exe)
                        .arg(from_path.to_string_lossy().to_string())
                        .arg(to_path.to_string_lossy().to_string())
                        .spawn();
                }
            }
        });

        // --- ЛОГИКА: УДАЛИТЬ ---
        let p_del = src.clone();
        let del_act = gio::SimpleAction::new("delete", None);
        del_act.connect_activate(move |_, _| {
            let _ = if p_del.is_dir() { fs::remove_dir_all(&p_del) } else { fs::remove_file(&p_del) };
            println!(">>> Удалено: {:?}", p_del);
        });

        action_group.add_action(&copy_act);
        action_group.add_action(&paste_act);
        action_group.add_action(&del_act);
        popover.insert_action_group("menu", Some(&action_group));
        popover
    }
}

// =========================================================================
// РАЗДЕЛ 4: ВСПОМОГАТЕЛЬНАЯ ЛОГИКА (Внутренние функции)
// =========================================================================
pub fn get_file_type(path: &Path) -> String {
    if path.is_dir() { "Папка".to_string() } else { "Файл".to_string() }
}
"#), ];
      
 for (rel_path, content) in text_files {
        let full_path = base_path.join(rel_path.trim());
        if let Some(parent) = full_path.parent() {
            
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, content)?;
    }        
    println!("Структура giu-apps/pinnacle-fm успешно создана ✔️");
    Ok(())
}
