use std::fs;
use std::path::Path; 

fn main() -> std::io::Result<()> {
    let base_path = Path::new("/var/db/repos/tupoll-overlay/");   
    let text_files = [
   ("www-client/servo/servo-9999.ebuild", r#"EAPI=8
inherit cargo git-r3

DESCRIPTION="servo (live git version)"
EGIT_REPO_URI="https://github.com/servo/servo.git"
LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64 ~arm64"

RDEPEND="media-libs/fontconfig
	media-libs/freetype
	dev-rust/clang
	media-libs/vulkan-loader
	x11-libs/libxkbcommon
	dev-rust/clang-sys-linkage
	dev-rust/link-cplusplus"
	
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_unpack() {
	# Честный фетч исходников из Git
	git-r3_src_unpack
	
}

src_configure() {
	# Убиваем дефолтное поведение eclass, которое подменяет источники на пустую папку
	# Заставляем Cargo думать, что он работает в обычном домашнем каталоге
	export CARGO_HOME="${T}/cargo_home"
	mkdir -p "${CARGO_HOME}" || die

	# Очищаем настройки, чтобы Cargo смотрел напрямую в интернет (crates.io)
	rm -f "${S}/.cargo/config.toml"
}
src_compile() {
	# 1. Ловушка путей для песочницы Portage (xargs/gxargs)
	mkdir -p "${T}/bin-bridge" || die
	ln -sf /usr/bin/xargs "${T}/bin-bridge/xargs"
	ln -sf /usr/bin/xargs "${T}/bin-bridge/gxargs"
	ln -sf /usr/bin/gawk "${T}/bin-bridge/awk"
	ln -sf /usr/bin/gmake "${T}/bin-bridge/gmake"
	
	export PATH="${T}/bin-bridge:/usr/bin:/bin:${PATH}"
	export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER="env PATH=${T}/bin-bridge:/usr/bin:/bin"

	# 2. Выравнивание слота под активный LLVM 22
	export LIBCLANG_PATH="/usr/lib/llvm/22/lib64"
	export BINDGEN_EXTRA_CLANG_ARGS="-stdlib=libc++"
	
	# 3. ТРОЙНОЙ СНАРЯД ДЛЯ ЛИНКЕРА:
	# Вызываем системный clang++, но принудительно заставляем его линковать 
	# рантайм GCC (libstdc++) для закрытия хэш-таблиц V8 (_M_next_bkt),
	# и докидываем libc++ вместе с libc++abi для совместимости с ICU 77!
	export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="clang++"
	export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C link-arg=-lstdc++ -C link-arg=-lc++ -C link-arg=-lc++abi"
	
	# 4. Сетевые переменные под Yota и Python
	export UV_PYTHON_DOWNLOADS=auto
	export UV_PYTHON=python3.11
	export CARGO_HOME="${T}/cargo_home"
	unset CARGO_FEATURE_FLAGS
	export CARGO_NET_GIT_FETCH_WITH_CLI=true

	# Понеслась с песней!
	cargo build --release || die "Cargo compilation failed"
}

src_install() {
	# ХИРУРГИЧЕСКАЯ УСТАНОВКА РУКАМИ В ОБХОД КАПРИЗНОГО ECLASS:
	# Просто берем наш готовый, честно выстраданный бинарник 
	# и прописываем его в системный /usr/bin/
	dobin "target/release/servoshell"
	
	# Если в корне есть иконки или ресурсы, их тоже забираем
	if [ -d "resources" ]; then
		insinto /usr/share/servo
		doins -r resources/*
	fi
	domenu "Servo.desktop"
}
  "#), 
      ("www-client/servo/files/Servo.desktop", r#"[Desktop Entry]
Version=1.0
Type=Application
Name=Servo Web Browser
Comment=Next-generation web browser engine
# Чистокровный, реактивный запуск напрямую через системный servoshell
Exec=servoshell %u
# Иконка из вашей системной темы (или можно подставить путь к кастомной)
Icon=web-browser
Terminal=false
Categories=Network;WebBrowser;
MimeType=text/html;text/xml;application/xhtml+xml;application/xml;
StartupNotify=true
# Явно заявляем, что браузер нативно поддерживает чистый Wayland
Env=WGPU_BACKEND=vulkan "#),
  
       ("www-client/servomenu/servomenu-9999.ebuild", r#"# Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

inherit cargo desktop

DESCRIPTION="Menu for the Servo web browser."
HOMEPAGE="https://github.com"
LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64 ~arm64"

SRC_URI=""
RESTRICT="fetch"

S="${WORKDIR}/${P}/servomenu"

RDEPEND="    	
	dev-rust/wayshot"
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_unpack() {
    mkdir -p "${WORKDIR}/${P}" || die
    cp -Rp "${FILESDIR}/servomenu" "${WORKDIR}/${P}/" || die
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
	domenu "Servomenu.desktop"	
}    
"#),
        ("www-client/servomenu/files/servomenu/Cargo.toml", r#"[package]
name = "servomenu"
version = "0.1.0"
edition = "2024"

[dependencies]
gtk4 = "0.11"
glib = "0.22"
gio = "0.22.8"

[[bin]]
name = "grab-helper"
path = "src/bin/grab.rs"

[[bin]]
name = "rename-url"
path = "src/bin/rename.rs" "#),
        ("www-client/servomenu/files/servomenu/Servomenu.desktop", r#"[Desktop Entry]
Version=1.0
Type=Application
Name=Pinnacle Speed Dial
Comment=Экспресс-панель быстрых закладок для Servo
# Вызываем бинарник servomenu напрямую (он сам подхватит Cairo и вызовет сателлиты)
Exec=servomenu
# Иконка сетки приложений или закладок из вашей GTK темы
Icon=view-grid-symbolic
Terminal=false
Categories=System;Utility;Network;
# Исключаем Vulkan-ругань на уровне запуска самого окна меню
Env=GSK_RENDERER=cairo
StartupNotify=true "#),
        ("www-client/servomenu/files/servomenu/src/main.rs", r#"use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, ListBox, ScrolledWindow, PolicyType, Image, Align, Entry, Box as GtkBox, Orientation, Label, PopoverMenu, MenuButton, Popover};
use std::process::Command;
use std::fs;
use std::{env, path::{Path, PathBuf}};

fn get_cache_dir() -> PathBuf {
    let home = env::var("HOME").expect("HOME not found");
    Path::new(&home).join(".config/servo/cache")
}

fn main() {
    unsafe {
        env::set_var("GSK_RENDERER", "cairo");
    }

    let app = Application::new(
        Some("org.pinnacle.servomenu"),
        Default::default(),
    );

    app.connect_activate(move |app| {
        let cache_dir = get_cache_dir();

        let window = ApplicationWindow::new(app);
        window.set_title(Some("Pinnacle Speed Dial"));
        window.set_default_size(450, 600);
        window.set_resizable(false); // Запрещаем ломать ширину в тайлинге

        // Главный вертикальный стек окна
        let main_box = GtkBox::new(Orientation::Vertical, 8);
        main_box.set_margin_top(10);
        main_box.set_margin_bottom(10);
        main_box.set_margin_start(10);
        main_box.set_margin_end(10);

        // 1. ВЕРХНИЙ ЭТАЖ ОКНА: Скролл со списком закладок (Занимает всё свободное место)
        let scrolled = ScrolledWindow::new();
        scrolled.set_policy(PolicyType::Automatic, PolicyType::Automatic);
        scrolled.set_vexpand(true); // Заставляем список растягиваться во всю высоту!
        scrolled.set_hexpand(true);

        let list_box = ListBox::new();
        list_box.set_selection_mode(gtk4::SelectionMode::None);

        // СОБИРАЕМ СКРИНШОТЫ ИЗ КЭША СЕРВО ДЛЯ ОТОБРАЖЕНИЯ СВЕРХУ
        let mut screenshots = Vec::new();
        if let Ok(entries) = fs::read_dir(&cache_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.to_str().map_or(false, |s| s.contains("-page.png")) {
                    screenshots.push(path);
                }
            }
        }
        
        screenshots.sort_by(|a, b| {
            let meta_a = fs::metadata(a).and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            let meta_b = fs::metadata(b).and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            meta_b.cmp(&meta_a)
        });

        // ОТРИСОВКА ГОРИЗОНТАЛЬНЫХ СТРОК ЗАКЛАДОК ВНУТРИ ВЕРХНЕГО СКРОЛЛА
        for i in 0..screenshots.len() {
            let img_path = screenshots[i].clone();
            let url_path = img_path.with_extension("url");

            let mut label_text = format!("Вкладка {}", i + 1);
            if url_path.exists() {
                if let Ok(raw_url) = fs::read_to_string(&url_path) {
                    let clean_url = raw_url.replace("https://", "").replace("http://", "");
                    if !clean_url.trim().is_empty() {
                        label_text = clean_url;
                    }
                }
            }

            let row_box = GtkBox::new(Orientation::Horizontal, 12);
            row_box.set_margin_start(6);
            row_box.set_margin_bottom(8);

            // Кликабельный эскиз слева (запускает servoshell по чесноку)
            let btn_img = Button::new();
            btn_img.set_size_request(48, 48);
            btn_img.set_halign(Align::Start);
            btn_img.add_css_class("flat");

            let img = Image::from_file(img_path.to_str().unwrap());
            img.set_pixel_size(44);
            btn_img.set_child(Some(&img));

            let u_path = url_path.clone();
            btn_img.connect_clicked(move |_| {
                let target_url = if u_path.exists() {
                    fs::read_to_string(&u_path).unwrap_or_else(|_| "https://servo.org".to_string())
                } else {
                    "https://servo.org".to_string()
                };

                println!("[Pinnacle] Клик по скриншоту! Запуск servoshell для: {}", target_url.trim());
                let _ = Command::new("fish")
                    .args(["-c", &format!("servoshell {}", target_url.trim())])
                    .spawn();
            });
            row_box.append(&btn_img);

            // Адрес строки по центру с жестким лимитом, чтобы не раздувало окно в сосиску!
            let label_url = Label::new(Some(&label_text));
            label_url.set_hexpand(true);
            label_url.set_halign(Align::Start);
            label_url.set_max_width_chars(25);
            label_url.set_ellipsize(gtk4::pango::EllipsizeMode::End);
            row_box.append(&label_url);

            // Контекстное меню трех точек справа
            let menu_model = gio::Menu::new();
            menu_model.append(Some("✏️ Переименовать адрес"), Some("row_menu.rename"));
            menu_model.append(Some("🗑 Удалить закладку"), Some("row_menu.delete"));

            let menu_btn = MenuButton::builder()
                .icon_name("view-more-symbolic")
                .css_classes(["flat"])
                .halign(Align::End)
                .build();

            let popover_menu = PopoverMenu::builder().menu_model(&menu_model).build();
            menu_btn.set_popover(Some(&popover_menu));
            row_box.append(&menu_btn);

            // Экшены GIO для удаления и вызова локального rename-url
            let action_group = gio::SimpleActionGroup::new();
            
            let img_p_del = img_path.clone();
            let url_p_del = url_path.clone();
            let list_box_clone = list_box.clone();
            let row_box_clone = row_box.clone();

            let del_act = gio::SimpleAction::new("delete", None);
            del_act.connect_activate(move |_, _| {
                println!(">>> [servomenu] Удаление файлов закладки: {:?}", img_p_del);
                let _ = fs::remove_file(&img_p_del);
                let _ = fs::remove_file(&url_p_del);
                list_box_clone.remove(&row_box_clone);

                let _ = Command::new("notify-send")
                    .arg("-u").arg("normal")
                    .arg("-i").arg("user-trash")
                    .arg("Pinnacle Speed Dial")
                    .arg("🗑️ Закладка успешно удалена.")
                    .spawn();
            });
            action_group.add_action(&del_act);

            let img_p_ren = img_path.clone();
            let rename_act = gio::SimpleAction::new("rename", None);
            rename_act.connect_activate(move |_, _| {
                if let Ok(mut exe) = std::env::current_exe() {
                    exe.set_file_name("rename-url");
                    let _ = Command::new(exe).arg(img_p_ren.to_string_lossy().to_string()).spawn();
                }
            });
            action_group.add_action(&rename_act);

            row_box.insert_action_group("row_menu", Some(&action_group));
            list_box.append(&row_box);
        }

        scrolled.set_child(Some(&list_box));
        main_box.append(&scrolled); // Добавили список наверх

        // 2. СТРОГО НИЖНИЙ ЭТАЖ ОКНА: Наша управляющая кнопка добавления Плюса "+"
        let row_add = GtkBox::new(Orientation::Horizontal, 12);
        row_add.set_margin_top(8);
        row_add.set_margin_start(6);

        let btn_add = Button::with_label("＋ Добавить новую закладку");
        btn_add.set_hexpand(true); // Кнопка растянется на всю ширину снизу окна!
        btn_add.set_halign(Align::Fill);

        let popover = Popover::new();
        let pop_box = GtkBox::new(Orientation::Vertical, 6);
        pop_box.set_margin_top(8); pop_box.set_margin_bottom(8);
        pop_box.set_margin_start(8); pop_box.set_margin_end(8);

        let pop_entry = Entry::new();
        pop_entry.set_placeholder_text(Some("Введи URL и нажми Enter"));
        pop_entry.set_width_request(260);
        pop_box.append(&pop_entry);
        popover.set_child(Some(&pop_box));
        popover.set_parent(&btn_add);

        let popover_click = popover.clone();
        btn_add.connect_clicked(move |_| {
            popover_click.popup();
        });

        let popover_close = popover.clone();
        let cache_dir_add = cache_dir.clone();
        pop_entry.connect_activate(move |entry| {
            let input_url = entry.text().to_string();
            let final_url = if input_url.trim().is_empty() {
                "https://servo.org".to_string()
            } else {
                input_url
            };

            println!("[Pinnacle] Ввод подтвержден! Запуск grab-helper...");
            popover_close.popdown();

            let _ = Command::new("fish")
                .args(["-c", "grab-helper"])
                .status();

            let mut latest_screenshot = None;
            let mut latest_time = std::time::SystemTime::UNIX_EPOCH;

            if let Ok(entries) = fs::read_dir(&cache_dir_add) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() && path.to_str().map_or(false, |s| s.contains("-page.png")) {
                        if let Ok(metadata) = fs::metadata(&path) {
                            if let Ok(modified) = metadata.modified() {
                                if modified > latest_time {
                                    latest_time = modified;
                                    latest_screenshot = Some(path);
                                }
                            }
                        }
                    }
                }
            }

            if let Some(fresh_png) = latest_screenshot {
                let url_path = fresh_png.with_extension("url");
                let _ = fs::write(url_path, &final_url);
                println!("[Pinnacle] Ссылка успешно привязана к свежему снимку!");
            }
        });

        row_add.append(&btn_add);
        main_box.append(&row_add); // Жёстко прибили Плюс в самый низ окна!
        
        window.set_child(Some(&main_box));
        window.present();
});app.run();} "#),
        ("www-client/servomenu/files/servomenu/src/bin/grab.rs", r#"use std::process::Command;

fn main() {
    let home = std::env::var("HOME").expect("HOME not found");
    // ИСПРАВЛЕНО БЕЗ СПЕШКИ: Точное попадание в официальный кэш Servo с каноничным слэшем
let target = format!("{}/.config/servo/cache/%Y:%d:%m-%H:%M-page", home);


    // Только wayshot -g и твой путь
    let _ = Command::new("wayshot")
        .arg("-g")
        .args(["--file-name-format", &target])
        .status();
}  "#),
        ("www-client/servomenu/files/servomenu/src/bin/rename.rs", r#"use gtk4 as gtk;
use gtk::prelude::*;
use std::fs;
use std::path::{ PathBuf};


fn main() -> gtk::glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id("org.pinnacle.servomenu.rename")
        .flags(gtk::gio::ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();

    app.connect_command_line(|app, app_cmd| {
        let args = app_cmd.arguments();
        
        if args.len() < 2 {
            eprintln!("!!! [rename-url] Ошибка: не передан путь к закладке.");
            return 1.into();
        }
        
        // Получаем путь к переданному файлу (например, slot_1.png или slot_1.url)
        let input_path = PathBuf::from(&args[1].to_string_lossy().to_string());
        
        // Гарантируем, что работаем именно с .url файлом конфигурации адреса
        let url_path = input_path.with_extension("url");
        
        // Читаем текущий адрес, если файла нет — ставим дефолт
        let current_url = fs::read_to_string(&url_path)
            .unwrap_or_else(|_| "https://servo.org".to_string())
            .trim()
            .to_string();

        // Настройка геометрии плавающего окна под тайлинг
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("Переименовать адрес")
            .modal(true)
            .default_width(320)
            .default_height(130)
            .resizable(false) 
            .build();

        window.set_decorated(true);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
        vbox.set_margin_end(12);
        vbox.set_margin_start(12);
        vbox.set_margin_top(12);
        vbox.set_margin_bottom(12);

        // Строка ввода, куда сразу подставляем ТЕКУЩИЙ URL-адрес
        let entry = gtk::Entry::builder()
            .text(&current_url)
            .activates_default(true)
            .build();
            
        // Выделяем весь адрес для мгновенного редактирования/ввода с нуля!
        entry.select_region(0, -1);
        vbox.append(&entry);

        let bbox = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        bbox.set_halign(gtk::Align::End);
        
        let btn_cancel = gtk::Button::builder().label("Отмена").build();
        let btn_ok = gtk::Button::builder().label("ОК").css_classes(["suggested-action"]).build();
        
        bbox.append(&btn_cancel);
        bbox.append(&btn_ok);
        vbox.append(&bbox);
        
        window.set_default_widget(Some(&btn_ok));

        let w_c = window.clone();
        let entry_c = entry.clone();
        let url_path_c = url_path.clone();
        let old_url = current_url.clone();

        // ЛОГИКА СОХРАНЕНИЯ НОВОГО АДРЕСА В ФАЙЛ
        btn_ok.connect_clicked(move |_| {
            let new_url = entry_c.text().to_string().trim().to_string();
            if !new_url.is_empty() && new_url != old_url {
                // Просто перезаписываем содержимое .url файла новым вбитым адресом!
                if fs::write(&url_path_c, new_url).is_ok() {
                    println!(">>> [rename-url] Адрес успешно обновлен в: {:?}", url_path_c);
                }
            }
            w_c.close();
        });

        let w_cancel = window.clone();
        btn_cancel.connect_clicked(move |_| {
            w_cancel.close();
        });

        window.set_child(Some(&vbox));
        window.present();

        0.into()
    });

    app.run()
} "#),
 ];
      
 for (rel_path, content) in text_files {
        let full_path = base_path.join(rel_path.trim());
        if let Some(parent) = full_path.parent() {
            
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, content)?;
    }        
    println!("Структура для браузере Servo успешно создана ✔️");
    Ok(())
}
