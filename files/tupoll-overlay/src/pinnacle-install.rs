use std::fs;
use std::path::Path; // Добавлено для работы с путями

fn main() -> std::io::Result<()> {
    // Используем Path для корректной работы метода .join()
    let base_path = Path::new("/var/db/repos/tupoll-overlay/");
   
    let text_files = [
        ("gui-wm/pinnacle/pinnacle-9999.ebuild", r#"EAPI=8
inherit cargo git-r3 desktop

DESCRIPTION="Window manager on rust using composer Smithay"
HOMEPAGE="https://github.com"
# Здесь ТОЛЬКО ссылка на GitHub
EGIT_REPO_URI="https://github.com/pinnacle-comp/pinnacle"

LICENSE="BSD-2"
SLOT="0"
KEYWORDS=""
PROPERTIES="live"

# Зависимости для GTK4 и VTE
RDEPEND="    
    gui-apps/foot
    sys-apps/kbd
    gui-apps/mako
    gui-apps/swaybg
    gui-apps/swaylock
    dev-libs/wayland
    x11-base/xwayland
	dev-libs/wayland
	>=dev-libs/wayland-protocols-1.26
	x11-libs/cairo
	media-sound/sox	
"
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"


src_unpack() {
	git-r3_src_unpack
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
}
"#),

     ("gui-wm/pinnacle-gentoo/pinnacle-gentoo-9999.ebuild", r#"# Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

inherit cargo desktop

DESCRIPTION="Pinnacle Wayland compositor for Gentoo"
HOMEPAGE="https://github.com"
LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64"

SRC_URI=""
RESTRICT="fetch"

S="${WORKDIR}/${P}/pinnacle-gentoo"

#ECARGO_VENDOR=""

RDEPEND="    
    gui-apps/foot
    sys-apps/kbd
    gui-apps/mako
    gui-apps/swaybg
    gui-apps/swaylock
    dev-libs/wayland
    gui-apps/ironbar
    x11-base/xwayland
	dev-libs/wayland
	>=dev-libs/wayland-protocols-1.26
	x11-libs/cairo
	media-libs/opusfile
	media-sound/sox
	media-video/soxbar
	gui-wm/pinnacle	
"
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_unpack() {
    mkdir -p "${WORKDIR}/${P}" || die
    cp -Rp "${FILESDIR}/pinnacle-gentoo" "${WORKDIR}/${P}/" || die
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

      ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/Cargo.toml", r#"[package]
name = "pinnacle-gentoo"
version = "0.1.0"
edition = "2024"
authors = ["tupoll"]
description = "Window manager for Gentoo Linux"


# Основные зависимости
[dependencies]
wallpaper = "3.2.0"
clap = { version = "4.4", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }  
serde_json = "1.0"
chrono = "0.4"
anyhow = "1.0"
nix = { version = "0.27", features = ["user"] }

[[bin]]
name = "pinnacle-gentoo"
path = "src/wm.rs"

[[bin]]
name = "pinnacle-screenshot"
path = "src/pinnacle-screenshot.rs"

#Смена обоев
[[bin]]
name = "wp-updater"
path = "src/switcher.rs"

#Инициализация
[[bin]]
name = "start_wallpaper"
path = "src/init.rs"

[[bin]]
name = "fstab-tmpfs-config"
path = "src/write_fstab.rs"

[[bin]]
name = "play_next"
path = "src/play_next.rs"

[[bin]]
name = "play_sox"
path = "src/play_sox.rs"

[[bin]]
name = "prev_track"
path = "src/prev_track.rs"

#[[bin]]
#name = "setup_playlist"
#path = "src/setup_playlist.rs"

#[[bin]]
#name = "stop_sox"
#path = "src/stop_sox.rs"

[[bin]]
name = "bottom_bar"
path = "src/bottom_bar.rs"

[[bin]]
name = "cpu_oval"
path = "src/cpu_oval.rs"

[[bin]]
name = "cpu_temp_oval"
path = "src/cpu_temp_oval.rs"

[[bin]]
name = "fish-conf"
path = "src/fish-conf.rs"

[[bin]]
name = "helper_kbd"
path = "src/helper_kbd.rs"

[[bin]]
name = "ironbar-config"
path = "src/ironbar-config.rs"

[[bin]]
name = "kbd-rs"
path = "src/kbd-rs.rs"

[[bin]]
name = "left_bar"
path = "src/left_bar.rs"

[[bin]]
name = "mako-conf"
path = "src/mako-conf.rs"

[[bin]]
name = "memory_oval"
path = "src/memory_oval.rs"

[[bin]]
name = "netgraph"
path = "src/netgraph.rs"

[[bin]]
name = "rxgraph"
path = "src/rxgraph.rs"

[[bin]]
name = "tagactive"
path = "src/tagactive.rs"


[[bin]]
name = "thunar-conf"
path = "src/thunar-conf.rs"

[[bin]]
name = "txgraph"
path = "src/txgraph.rs"

[[bin]]
name = "vol_info"
path = "src/vol_info.rs"

[[bin]]
name = "vol_widget"
path = "src/vol_widget.rs"
#
[profile.release]
strip = true      
lto = true        
opt-level = "z"   

  "#),
  
    ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/bottom_bar.rs", r##"use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() -> std::io::Result<()> {
	let path = "/var/tmp/wm/bar.lua";
    let mut output = File::create(path)?;
    write!(output, "#!/usr/bin/lua
cmd = string.format('left_bar')    
os.execute(cmd)")?;
	
	let _output = Command::new("pkill")
        .arg("-f")
        .arg("ironbar")
        .output().unwrap_or_else(|e| {
            panic!("Ошибка выполнения процесса {}", e)
    }); 
    // Создаем файл для записи
    let mut file = File::create(".config/ironbar/config.corn")?;

    // Строка, содержащая кавычки, которые нужно экранировать
 let text_with_quotes = "let {   
   $focused = {name = \"foco\" type = \"focused\"  show_icon = true icon_size = 22 truncate.mode = \"end\" truncate.max_length = 20 }
   $launcher = {name = \"launcher\" type = \"launcher\" show_titles =true show_icon = true icon_size = 22 truncate.mode = \"end\" truncate.max_length = 20 }
   $clock = {type = \"clock\" format = \"%a, %d %b %H:%M\" }
   $battery = {type = \"upower\" icon_size = 14 format = \"{percentage}%\"} 
   $memgraph = {class = \"cpu-menu\" name=\"cpu-menu\" type = \"custom\" bar =[ { type = \"box\" orientation = \"vertical\" widgets = [
   { type = \"button\" class=\"power-btn\" label = \"{{2000:memory_oval}}\" tooltip = \"{{echo \'MEMORY\'}}\" orientation = \"horizontal\"}]}]}
   $cpugraph = {class = \"cpu-menu\" name=\"cpu-menu\" type = \"custom\" bar =[ { type = \"box\" orientation = \"vertical\" widgets = [
   { type = \"button\" class=\"power-btn\" label = \"{{2000:cpu_oval}}\" tooltip = \"{{echo \'CPU\'}}\" orientation = \"horizontal\"}]}]}
   $cpu_tempgraph = {class = \"cpu-menu\" name=\"cpu-menu\" type = \"custom\" bar =[ { type = \"box\" orientation = \"vertical\" widgets = [
   { type = \"button\" class=\"power-btn\" label = \"{{2000:cpu_temp_oval}}\" tooltip = \"{{echo \'TEMPERATURE\'}}\" orientation = \"horizontal\"}]}]}
   $netgraph = {class = \"cpu-menu\" name=\"cpu-menu\" type = \"custom\" bar =[ { type = \"box\" orientation = \"vertical\" widgets = [
   { type = \"button\" class=\"power-btn\" label = \"{{2000:netgraph}}\" tooltip = \"{{echo \'Rx   Tx\'}}\" orientation = \"horizontal\"}]}]}
   $power ={ class = \"power-menu\" name=\"power-menu\" type = \"custom\"
        bar = [ { label = \"exit\" type = \"button\"  widgets = [{src=\"icon:system-shutdown\" type=\"image\" size=16}] on_click = \"popup:toggle\" }]    
        popup =[ { name=\"popup-box\" type = \"box\" orientation = \"horizontal\"    
           widgets = [
               { type = \"button\" class=\"power-btn\" label = \"\" widgets = [{src=\"icon:system-shutdown\" type=\"image\" size=48}] on_click = \"!fish -c poweroff\" }
               { type = \"button\" class=\"power-btn\" label = \"\" widgets = [{src=\"icon:system-reboot\" type=\"image\" size=48}] on_click = \"!fish -c reboot\" }
               { type = \"button\" class=\"power-btn\" label = \"\" widgets = [{src=\"icon:system-lock-screen\" type=\"image\" size=48}] on_click = \"!fish -c swaylock -f -C ~/.config/swaylock/config\" } ] } ] }\n
   $sox ={ class = \"sox-menu\" name=\"sox-menu\" type = \"custom\" bar = [ { label = \"play\" type = \"button\" widgets = [{src=\"icon:multimedia-audio-player\" type=\"image\" size=16}] on_click = \"popup:toggle\" }]   
      popup =[{ name=\"popup-box\" type = \"box\" orientation = \"horizontal\"  widgets = [
   { type = \"button\" class=\"play-btn\" label = \"\" widgets = [{type = \"script\" name = \"sox\" mode = \"poll\" cmd = \"echo \'📂\'\"}] on_click = \"!setup_playlist\" }
   { type = \"button\" class=\"play-btn\" label = \"\" widgets = [{type = \"script\" name = \"sox\" mode = \"poll\" cmd = \"echo \'📄\'\"}] on_click = \"!list_sox\" }
   { type = \"button\" class=\"play-btn\" label = \"\" widgets = [{type = \"script\" name = \"sox\" mode = \"poll\" cmd = \"echo \'⏪\'\"}] on_click = \"!prev_track\" } 
   { type = \"button\" class=\"play-btn\" label = \"\" widgets = [{type = \"script\" name = \"sox\" mode = \"poll\" cmd = \"echo \'⏹️\'\"}] on_click = \"!stop_sox\" } 
   { type = \"button\" class=\"play-btn\" label = \"\" widgets = [{type = \"script\" name = \"sox\" mode = \"poll\" cmd = \"echo \'▶️\'\"}] on_click = \"!play_sox\" }
   { type = \"button\" class=\"play-btn\" label = \"\" widgets = [{type = \"script\" name = \"sox\" mode = \"poll\" cmd = \"echo \'⏩\'\"}] on_click = \"!play_next\" }]}]}
   $tray = { type = \"tray\" direction = \"h\" icon_size = 14 }
   $keymaps = {type = \"script\" name = \"kbd\" mode = \"poll\" interval = 3000 cmd = \"kbd-rs\" on_click = \"!sudo /usr/local/bin/helper_kbd\"}
   $network = {type = \"network_manager\" icon_size = 14}   
   $tags = {class = \"tags-menu\" name=\"power-menu\" type = \"custom\"  bar = [{label = \"tags\" type = \"button\" widgets = [{ type = \"script\" name = \"tagactive\" mode = \"poll\" interval = 1000 cmd = \"tagactive\" }]
   widgets = [{ type = \"script\" name = \"tagactive\" mode = \"poll\" interval = 1000 cmd = \"tagactive\" }]
   on_click = \"popup:toggle\"}] popup =[{name=\"popup-box\" type = \"box\" orientation = \"vertical\"  widgets = [
   { type = \"button\" class=\"tags-btn\" label = \"\" widgets = [{type = \"script\" name = \"tagactive\" mode = \"poll\" cmd = \"echo '1️⃣  fm'\"}] on_click = \"!1_tag\" }
   { type = \"button\" class=\"tags-btn\" label = \"\" widgets = [{type = \"script\" name = \"tagactive\" mode = \"poll\" cmd = \"echo '2️⃣ browser'\"}] on_click = \"!2_tag\" }
   { type = \"button\" class=\"tags-btn\" label = \"\" widgets = [{type = \"script\" name = \"tagactive\" mode = \"poll\" cmd = \"echo '3️⃣ work'\"}] on_click = \"!3_tag\" }
   { type = \"button\" class=\"tags-btn\" label = \"\" widgets = [{type = \"script\" name = \"tagactive\" mode = \"poll\" cmd = \"echo '4️⃣ editor'\"}] on_click = \"!4_tag\" }
   { type = \"button\" class=\"tags-btn\" label = \"\" widgets = [{type = \"script\" name = \"tagactive\" mode = \"poll\" cmd = \"echo '5️⃣ ssh'\"}] on_click = \"!5_tag\" }
   { type = \"button\" class=\"tags-btn\" label = \"\" widgets = [{type = \"script\" name = \"tagactive\" mode = \"poll\" cmd = \"echo '6️⃣ media'\"}] on_click = \"!6_tag\" }]}]}\n
   $screenshot ={class = \"screenshot\" name=\"screenshot\" type = \"custom\" bar =[{type = \"box\" orientatron = \"horizontal\" widgets = [
           { type = \"button\" class=\"power-btn\" label = \"\" widgets = [{src=\"icon:applets-screenshooter\" type=\"image\" size=16}] on_click_left = \"pinnacle-screenshot --mode region\" on_click_right = \"pinnacle-screenshot --mode output\" }]}]}
   $menu ={class = \"wofi-menu\" name=\"wofi-menu\" type = \"custom\" bar =[{type = \"box\" orientatron = \"horizontal\" widgets = [
           { type = \"button\" class=\"menu-btn\" label = \"\" widgets = [{src=\"icon:distributor-logo-gentoo\" type=\"image\" size=16}] on_click_left = \"wofi -c ~/.config/wofi/config -I\"  on_click_right = \"killall wofi\"}]}]}
   $volume = {class = \"cpu-menu\" name=\"cpu-menu\" type = \"custom\" bar =[ { type = \"box\" orientation = \"vertical\" widgets = [
   { type = \"button\" class=\"power-btn\" label = \"{{1000:vol_widget}}\" tooltip = \"{{1000:vol_info}}\" orientation = \"horizontal\"
   on_scroll_down = \"amixer sset PCM 5%-\" on_scroll_up = \"amixer sset PCM 5%+\" on_click_left = \"amixer sset PCM 0\" on_click_right = \"amixer sset PCM 80\"}]}]}                           
   $left = [ $menu $tags $launcher $focused ]        
   $center = [ $clock ]        
   $right = [ $tray $volume $screenshot  $sox $power $keymaps $netgraph $cpu_tempgraph $cpugraph $memgraph]}\n
   in {
   monitors.HDMI-A-1 = { position = \"bottom\" height = 30 icon_theme = \"BeautyLine\" start = $left center = $center end = $right } } ";          
    // Записываем строку в файл
    writeln!(file, "{}", text_with_quotes)?;    
    println!("Данные успешно записаны в config.corn");
    let _output = Command::new("fish")
        .arg("-c")
        .arg("ironbar")
        .output().unwrap_or_else(|e| {
            panic!("Ошибка выполнения процесса {}", e)
    });
    
    
     
    Ok(())
} "##), 

      ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/left_bar.rs", r##"use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() -> std::io::Result<()> {
	let path = "/var/tmp/wm/bar.lua";
    let mut output = File::create(path)?;
    write!(output, "#!/usr/bin/lua
cmd = string.format('bottom_bar')    
os.execute(cmd)")?;
	let _output = Command::new("pkill")
        .arg("-f")
        .arg("ironbar")
        .output().unwrap_or_else(|e| {
            panic!("Ошибка выполнения процесса {}", e)
    }); 
    // Создаем файл для записи
    let mut file = File::create(".config/ironbar/config.corn")?;

    // Строка, содержащая кавычки, которые нужно экранировать
 let text_with_quotes = "let {   
   $focused = {name = \"foco\" type = \"focused\"  show_icon = true icon_size = 22 truncate.mode = \"end\" truncate.max_length = 20 orientation = \"horizontal\"}
   $clock = {type = \"clock\" format = \"%H:%M\" orientation = \"horizontal\"}
   $battery = {type = \"upower\" icon_size = 14 format = \"{percentage}%\"} 
   $memgraph = {class = \"cpu-menu\" name=\"cpu-menu\" type = \"custom\" bar =[ { type = \"box\" orientation = \"vertical\" widgets = [
   { type = \"button\" class=\"power-btn\" label = \"{{1000:memory_oval}}\" tooltip = \"{{echo \'MEMORY\'}}\" orientation = \"horizontal\"}]}]}
   $cpugraph = {class = \"cpu-menu\" name=\"cpu-menu\" type = \"custom\" bar =[ { type = \"box\" orientation = \"vertical\" widgets = [
   { type = \"button\" class=\"power-btn\" label = \"{{1000:cpu_oval}}\" tooltip = \"{{echo \'CPU\'}}\" orientation = \"horizontal\"}]}]}
   $cpu_tempgraph = {class = \"cpu-menu\" name=\"cpu-menu\" type = \"custom\" bar =[ { type = \"box\" orientation = \"vertical\" widgets = [
   { type = \"button\" class=\"power-btn\" label = \"{{2000:cpu_temp_oval}}\" tooltip = \"{{echo \'TEMPERATURE\'}}\" orientation = \"horizontal\"}]}]}  
   $power ={ class = \"power-menu\" name=\"power-menu\" type = \"custom\"
        bar = [ { label = \"exit\" type = \"button\"  widgets = [{src=\"icon:system-shutdown\" type=\"image\" size=16}] on_click = \"popup:toggle\" }]    
        popup =[ { name=\"popup-box\" type = \"box\" orientation = \"horizontal\"    
           widgets = [
               { type = \"button\" class=\"power-btn\" label = \"\" widgets = [{src=\"icon:system-shutdown\" type=\"image\" size=48}] on_click = \"!fish -c poweroff\" }
               { type = \"button\" class=\"power-btn\" label = \"\" widgets = [{src=\"icon:system-reboot\" type=\"image\" size=48}] on_click = \"!fish -c reboot\" }
               { type = \"button\" class=\"power-btn\" label = \"\" widgets = [{src=\"icon:system-lock-screen\" type=\"image\" size=48}] on_click = \"!fish -c swaylock -f -C ~/.config/swaylock/config\" } ] } ] }\n
   $sox ={ class = \"sox-menu\" name=\"sox-menu\" type = \"custom\" bar = [ { label = \"play\" type = \"button\" widgets = [{src=\"icon:multimedia-audio-player\" type=\"image\" size=16}] on_click = \"popup:toggle\" }]   
      popup =[{ name=\"popup-box\" type = \"box\" orientation = \"horizontal\"  widgets = [
   { type = \"button\" class=\"play-btn\" label = \"\" widgets = [{type = \"script\" name = \"sox\" mode = \"poll\" cmd = \"echo \'📂\'\"}] on_click = \"!setup_playlist\" }
   { type = \"button\" class=\"play-btn\" label = \"\" widgets = [{type = \"script\" name = \"sox\" mode = \"poll\" cmd = \"echo \'📄\'\"}] on_click = \"!list_sox\" }
   { type = \"button\" class=\"play-btn\" label = \"\" widgets = [{type = \"script\" name = \"sox\" mode = \"poll\" cmd = \"echo \'⏫\'\"}] on_click = \"!prev_track\" } 
   { type = \"button\" class=\"play-btn\" label = \"\" widgets = [{type = \"script\" name = \"sox\" mode = \"poll\" cmd = \"echo \'⏹️\'\"}] on_click = \"!stop_sox\" } 
   { type = \"button\" class=\"play-btn\" label = \"\" widgets = [{type = \"script\" name = \"sox\" mode = \"poll\" cmd = \"echo \'🔽\'\"}] on_click = \"!play_sox\" }
   { type = \"button\" class=\"play-btn\" label = \"\" widgets = [{type = \"script\" name = \"sox\" mode = \"poll\" cmd = \"echo \'⏬\'\"}] on_click = \"!play_next\" }]}]}
   $tray = { type = \"tray\" direction = \"h\" icon_size = 14 }
   $keymaps = {class = \"cpu-menu\" name=\"cpu-menu\" type = \"custom\" bar =[ { type = \"box\" orientation = \"vertical\" widgets = [
   { type = \"button\" class=\"power-btn\" label = \"{{3000:kbd-rs}}\" tooltip = \"{{3000:kbd-rs}}\" on_click = \"!sudo /usr/local/bin/helper_kbd\" orientation = \"horizontal\"}]}]}
   $network = {type = \"network_manager\" icon_size = 14}
   $txgraph = {class = \"cpu-menu\" name=\"cpu-menu\" type = \"custom\" bar =[ { type = \"box\" orientation = \"vertical\" widgets = [
   { type = \"button\" class=\"power-btn\" label = \"{{2000:txgraph}}\" tooltip = \"{{echo \'Tx\'}}\" orientation = \"horizontal\"}]}]}
   $rxgraph = {class = \"cpu-menu\" name=\"cpu-menu\" type = \"custom\" bar =[ { type = \"box\" orientation = \"vertical\" widgets = [
   { type = \"button\" class=\"power-btn\" label = \"{{2000:rxgraph}}\" tooltip = \"{{echo \'Rx\'}}\" orientation = \"horizontal\"}]}]}
   $tags = {class = \"cpu-menu\" name=\"cpu-menu\" type = \"custom\" bar =[ { type = \"box\" widgets = [
   { type = \"button\" class=\"power-btn\" label = \"{{3000:tagactive}}\" orientation = \"horizontal\" }]}]}
   $screenshot ={class = \"screenshot\" name=\"screenshot\" type = \"custom\" bar =[{type = \"box\" orientatron = \"horizontal\" widgets = [
           { type = \"button\" class=\"power-btn\" label = \"\" widgets = [{src=\"icon:applets-screenshooter\" type=\"image\" size=16}] on_click_left = \"pinnacle-screenshot --mode region\"  on_click_right = \"pinnacle-screenshot --mode output\"}]}]}
   $menu ={class = \"wofi-menu\" name=\"wofi-menu\" type = \"custom\" bar =[{type = \"box\" orientatron = \"horizontal\" widgets = [
           { type = \"button\" class=\"menu-btn\" label = \"\" widgets = [{src=\"icon:distributor-logo-gentoo\" type=\"image\" size=16}] on_click_left = \"wofi -c ~/.config/wofi/config -I\"  on_click_right = \"killall wofi\"}]}]}
   $volume = {class = \"cpu-menu\" name=\"cpu-menu\" type = \"custom\" bar =[ { type = \"box\" orientation = \"vertical\" widgets = [
   { type = \"button\" class=\"power-btn\" label = \"{{1000:vol_widget}}\" tooltip = \"{{1000:vol_info}}\" orientation = \"horizontal\"
   on_scroll_down = \"amixer sset PCM 5%-\" on_scroll_up = \"amixer sset PCM 5%+\" on_click_left = \"amixer sset PCM 0\" on_click_right = \"amixer sset PCM 80\"}]}]}                           
   $top = [ $menu $clock ]        
   $middle = [ $tags $keymaps $tray  $focused $volume ]        
   $bottom = [ $screenshot  $sox $txgraph $rxgraph $cpu_tempgraph $cpugraph $memgraph $power]}\n
   in {
   monitors.HDMI-A-1 = { position = \"left\" height = 30 icon_theme = \"BeautyLine\" start = $top center = $middle end = $bottom } } ";          
    // Записываем строку в файл
    writeln!(file, "{}", text_with_quotes)?;    
    println!("Данные успешно записаны в config.corn");
    let _output = Command::new("fish")
        .arg("-c")
        .arg("ironbar")
        .output().unwrap_or_else(|e| {
            panic!("Ошибка выполнения процесса {}", e)
    }); 
    Ok(())
}      
"##),      
      ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/cpu_oval.rs", r##"use std::fs;
use std::io::Write;

fn main() {
    // --- Настройки ---
    const WIDTH: usize = 8;
    const FONT_SIZE: &str = "9pt";
    const BARS: [&str; 8] = [" ", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
    const LEFT_CAP: &str = "";
    const RIGHT_CAP: &str = "";
    const COLOR_LOW: &str = "#00ff00";
    const COLOR_MID: &str = "#ffff00";
    const COLOR_HIGH: &str = "#ff0000";
    const TMP_PATH: &str = "/tmp/ironbar_cpu_state";

    // 1. Получение текущих данных CPU
    let (curr_w, curr_t) = get_cpu();

    // 2. Чтение предыдущего состояния
    let mut prev_w: u64 = 0;
    let mut prev_t: u64 = 0;
    if let Ok(content) = fs::read_to_string(TMP_PATH) {
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() == 2 {
            prev_w = parts[0].parse().unwrap_or(0);
            prev_t = parts[1].parse().unwrap_or(0);
        }
    }

    // 3. Расчет нагрузки
    let mut load = 0;
    if prev_t > 0 {
        let diff_w = curr_w.saturating_sub(prev_w);
        let diff_t = curr_t.saturating_sub(prev_t);
        if diff_t > 0 {
            load = (diff_w as f64 / diff_t as f64 * 100.0) as u32;
        }
    }
    let load = load.min(100);

    // 4. Сохранение состояния для следующего запуска
    if let Ok(mut f) = fs::File::create(TMP_PATH) {
        let _ = write!(f, "{} {}", curr_w, curr_t);
    }

    // 5. Логика визуализации
    let color = if load > 80 {
        COLOR_HIGH
    } else if load > 40 {
        COLOR_MID
    } else {
        COLOR_LOW
    };

    // Форматирование метки (минимум 4 символа с пробелами слева)
    let mut label = format!("{}%", load);
    while label.chars().count() < 4 {
        label.insert(0, ' ');
    }
    let label_chars: Vec<char> = label.chars().collect();

    // Центрирование текста внутри WIDTH
    let text_start = (WIDTH.saturating_sub(label_chars.len())) / 2;

    // Сборка Pango строки
    let mut output = format!("<span size='{}' color='{}' face='monospace'>{}", FONT_SIZE, color, LEFT_CAP);

    let bar_idx = ((load as f32 / 100.0) * (BARS.len() - 1) as f32).round() as usize;
    let bar_char = BARS[bar_idx];

    for i in 0..WIDTH {
        if i >= text_start && i < text_start + label_chars.len() {
            output.push(label_chars[i - text_start]);
        } else {
            output.push_str(bar_char);
        }
    }

    output.push_str(RIGHT_CAP);
    output.push_str("</span>");

    println!("{}", output);
}

fn get_cpu() -> (u64, u64) {
    if let Ok(stat) = fs::read_to_string("/proc/stat") {
        if let Some(line) = stat.lines().next() {
            let parts: Vec<u64> = line
                .split_whitespace()
                .skip(1)
                .filter_map(|s| s.parse().ok())
                .collect();

            if parts.len() >= 4 {
                let work = parts[0] + parts[1] + parts[2]; // user + nice + system
                let total = work + parts[3];             // work + idle
                return (work, total);
            }
        }
    }
    (0, 0)
}
      
  "##),
  
     ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/cpu_temp_oval.rs", r##"use std::fs;
use std::path::Path;

fn main() {
    // --- Настройки стиля ---
    const WIDTH: usize = 8;
    const FONT_SIZE: &str = "9pt";
    const BARS: [&str; 8] = [" ", "▂", "▃", "▄", "▅", "▆", "▇", "█"];

    // Символы скругления (Nerd Fonts)
    const LEFT_CAP: &str = "";
    const RIGHT_CAP: &str = "";

    // Цветовая схема
    const COLOR_HOT: &str = "#FF4500";  // > 75°C
    const COLOR_MID: &str = "#FFFFFF";  // 45-75°C
    const COLOR_COLD: &str = "#00BFFF"; // < 45°C

    // 1. Получение температуры
    let temp = get_cpu_temp();

    // 2. Выбор цвета
    let color = if temp > 75 {
        COLOR_HOT
    } else if temp < 45 {
        COLOR_COLD
    } else {
        COLOR_MID
    };

    // 3. Форматирование метки (центрирование)
    let label = format!("{}°", temp);
    let mut label_padded = label.clone();
    while label_padded.chars().count() < 4 {
        label_padded.insert(0, ' ');
    }
    let label_chars: Vec<char> = label_padded.chars().collect();
    let text_start = (WIDTH.saturating_sub(label_chars.len())) / 2;

    // 4. Расчет индекса полоски (шкала 30°C - 90°C)
    let factor = (temp as f32 - 30.0) / (90.0 - 30.0);
    let factor = factor.clamp(0.0, 1.0);
    let bar_idx = (factor * (BARS.len() - 1) as f32).round() as usize;
    let bar_char = BARS[bar_idx];

    // 5. Сборка Pango-строки
    let mut output = format!("<span size='{}' color='{}' face='monospace'>{}", FONT_SIZE, color, LEFT_CAP);

    for i in 0..WIDTH {
        if i >= text_start && i < text_start + label_chars.len() {
            output.push(label_chars[i - text_start]);
        } else {
            output.push_str(bar_char);
        }
    }

    output.push_str(RIGHT_CAP);
    output.push_str("</span>");

    println!("{}", output);
}

fn get_cpu_temp() -> i32 {
    // Список возможных путей к датчикам температуры
    let zones = ["/sys/class/thermal/thermal_zone0/temp", "/sys/class/thermal/thermal_zone1/temp"];

    for zone in zones {
        if let Ok(content) = fs::read_to_string(Path::new(zone)) {
            if let Ok(temp_raw) = content.trim().parse::<i32>() {
                return temp_raw / 1000; // Конвертация из миллиградусов
            }
        }
    }
    0
}      
  "##),
  
        ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/fish-conf.rs", r##"use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    // 1. Получаем путь к домашней директории
    let home = env::var("HOME").expect("Переменная окружения HOME не найдена");
    let mut config_path = PathBuf::from(home);
    config_path.push(".config/fish");

    // 2. Создаем директорию (аналог mkdir -p)
    fs::create_dir_all(&config_path)?;

    // 3. Содержимое config.fish
    let config_fish= r#"if status is-interactive
    # Commands to run in interactive sessions can go here
end
  
set LANGUAGE ru_RU.UTF-8
set LANG ru_RU.UTF-8
set LC_MESSAGES ru_RU.UTF-8
set LC_CTYPE ru_RU.UTF-8
set LC_NUMERIC ru_RU.UTF-8
set LC_TIME ru_RU.UTF-8
set LC_COLLATE ru_RU.UTF-8
set LC_MONETARY ru_RU.UTF-8
set LC_MESSAGES ru_RU.UTF-8
set LC_PAPER ru_RU.UTF-8
set LC_NAME ru_RU.UTF-8
set LC_ADDRESS ru_RU.UTF-8
set LC_TELEPHONE ru_RU.UTF-8
set LC_MEASUREMENT ru_RU.UTF-8
set LC_IDENTIFICATION ru_RU.UTF-8
set LC_ALL ru_RU.UTF-8

set PATH $HOME/.cargo/bin:/sbin:/usr/sbin:/usr/local/sbin:$HOME/.local/bin:/usr/local/bin:$HOME/.local/bin:$HOME/.local/bin:$HOME/.local/bin:/usr/local/bin:/usr/local/bin:/usr/bin:/bin:/usr/local/games:/usr/games

set XDG_SESSION_TYPE pinnacle --session
set XDG_SESSION_DESKTOP pinnacle --session
set XDG_CURRENT_DESKTOP pinnacle --session

## Default programs:
set EDITOR "nano"
set TERMINAL "foot"
set BROWSER "vivaldi"
set FMGR "thunar"
#set MESA_GLSL_VERSION_OVERRIDE 320
#set MESA_GL_VERSION_OVERRIDE 3.2COMPAT
set QT_QPA_PLATFORMTHEME qt6ct

###Должно быть в конце файла
# Start X(wayland) at login
#if status is-login
#    if test -z "$DISPLAY" -a "$XDG_VTNR" = 1
#        exec sway -- -keeptty
#    end
#end

#[ ! "$UID" = "0" ] && neofetch
#[  "$UID" = "0" ] && neofetch


if status is-login
    if test -z "$DISPLAY" -a "$XDG_VTNR" = 1
     [ ! "$UID" = "0" ] && pinnacle --session 
    end
end"#;

   
    fs::write(config_path.join("config.fish"), config_fish)?;
    
    println!("Конфигурация успешно сохранена в {:?}", config_path);
    Ok(())
}
"##),    
      
     ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/helper_kbd.rs", r#"use std::fs;
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let base_path = Path::new("/sys/class/leds");
    let pattern = "::capslock";

    // 1. Ищем первую подходящую директорию
    let entries = fs::read_dir(base_path)?;
    
    let target_dir = entries
        .filter_map(|e| e.ok())
        .find(|e| {
            e.file_name()
                .to_str()
                .map(|s| s.contains(pattern))
                .unwrap_or(false)
        });

    if let Some(entry) = target_dir {
        // 2. Формируем путь к файлу brightness внутри найденной папки
        let brightness_path = entry.path().join("brightness");
        println!("Попытка записи в: {:?}", brightness_path);

        // 3. Открываем файл на запись и пишем "1"
        // Файлы в /sys не поддерживают перезапись через создание нового файла,
        // поэтому используем OpenOptions или обычный Write.
        let mut file = fs::File::create(&brightness_path)?;
        file.write_all(b"1")?;

        println!("Успешно: Caps Lock включен (записано '1' в brightness)");
    } else {
        println!("Устройство с паттерном '{}' не найдено.", pattern);
    }

    Ok(())
}
       "#), 
       
      ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/init.rs", r#"use std::fs::{self, File};
use std::io::{Write};
use std::path::Path;
use wallpaper;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let src_dir = "/usr/share/pinnacle-gentoo/pictures";
    let base_dir = "/var/tmp/wm";
    let target_dir = format!("{}/pictures", base_dir);
    
    // 2. Создание директорий
    fs::create_dir_all(&target_dir)?;
 
    // 3. Копирование файлов
    if Path::new(src_dir).exists() {
        for entry in fs::read_dir(src_dir)? {
            let entry = entry?;
            let target_path = Path::new(&target_dir).join(entry.file_name());
            
            fs::copy(entry.path(), &target_path)?;         
        }
    }

    // 4. Установка обоев
   let initial_wp = format!("{}/tag1.jpg", target_dir);
    if Path::new(&initial_wp).exists() {
        wallpaper::set_from_path(&initial_wp)?;
    }

    // 5. Файл тега
    let tag_path = format!("{}/tag", base_dir);
    {
        let mut output = File::create(&tag_path)?;
        write!(output, "𝟏")?;
    }
    println!("Синхронизация завершена");
    Ok(())
}
      
  "#), 
    ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/ironbar-config.rs", r##"use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    // 1. Получаем путь к домашней директории
    let home = env::var("HOME").expect("Переменная окружения HOME не найдена");
    let mut config_path = PathBuf::from(home);
    config_path.push(".config/ironbar");

    // 2. Создаем директорию (аналог mkdir -p)
    fs::create_dir_all(&config_path)?;

    // 3. Содержимое srcery_css.css
    let srcery_css = r#"@define-color srcery-black #1C1B19;
@define-color srcery-red #EF2F27;
@define-color srcery-green #519F50;
@define-color srcery-yellow #FBB829;
@define-color srcery-blue #2C78BF;
@define-color srcery-magenta #E02C6D;
@define-color srcery-cyan #0AAEB3;
@define-color srcery-white #BAA67F;
@define-color srcery-bright-black #918175;
@define-color srcery-bright-red #F75341;
@define-color srcery-bright-green #98BC37;
@define-color srcery-bright-yellow #FED06E;
@define-color srcery-bright-blue #68A8E4;
@define-color srcery-bright-magenta #FF5C8F;
@define-color srcery-bright-cyan #2BE4D0;
@define-color srcery-bright-white #FCE8C3;
@define-color srcery-orange #FF5F00;
@define-color srcery-bright-orange #FF8700;
@define-color srcery-teal #008080;
@define-color srcery-hard-black #121212;
@define-color srcery-xgray1 #262626;
@define-color srcery-xgray2 #303030;
@define-color srcery-xgray3 #3A3A3A;
@define-color srcery-xgray4 #444444;
@define-color srcery-xgray5 #4E4E4E;
@define-color srcery-xgray6 #585858;
@define-color srcery-xgray7 #626262;
@define-color srcery-xgray8 #6C6C6C;
@define-color srcery-xgray9 #767676;
@define-color srcery-xgray10 #808080;
@define-color srcery-xgray11 #8A8A8A;
@define-color srcery-xgray12 #949494;"#;

    // 4. Содержимое style.css
    let style_css = r#"@import 'srcery_css.css';

* {
    font-family: MesloLGSDZ Nerd Font;         
    font-size: 14px;
    box-shadow: none;
    border: none;
    border-radius: 0px;
    border-image: none;
    background-image: none;
    outline: none;
    text-shadow: none;
    margin: 0;
    padding: 0;
}

button {
    box-shadow: none;
    border: none;
    background: none;
    outline: none;
    text-shadow: none;
}

.background {
    background-color: rgba(0,0,0,0);
}

scale slider {
  margin: -4px;
  min-width: 12px;
  min-height: 12px;
}

@keyframes blinker {
  50% {
    color: @srcery-red;
  }
}

#bar {
    /* background-color: @srcery-black; */
    /* box-shadow: inset 0 4 0 0 @srcery-xgray2; */
    min-height: 1.93em;
}

/* base */
.tray,
.notifications,
.button,
.widget,
.clock,
.menu,
.upower,
.system,
#script
#popup,
.keymaps,
#exit,
#net,
#memory,
#cpu,
#cpu_temp,
.tags
.foco,
#wifi,
.volume

#system {
    margin-left: 4px;
    transition: all 0.5s;
    transition-timing-function: ease;
}

.volume {
    padding: 0px 10px;
    margin-right: 1px;
    margin-left: 10px;
    border-radius: 10px;
    color: #D8D8AA;
    background: #1a1b26;
    opacity: 0.8;
}

.popup-volume {
    padding: 20px 20px 20px 20px;
    border-radius: 5px;
    color: #c0caf5;
    background-color: rgba(26, 27, 38, 0.8);
    border: 2px solid #bb9af7;
}

/*  -------- MUSIC ---------  */
.tags{
    background-color: @srcery-black;
    border-radius: 0 0 .4em .4em;
    padding-left: 4px;
    padding-right:4px;
    padding-top: 5px;
}

.tags:hover{
    color: @srcery-black;
}

.popup-music {
    background-color: @srcery-black;
    box-shadow: inset 0px 4px 0px 0px @srcery-cyan;
    margin-top: 3px;
    padding: 20px 20px 20px 20px;
    border-radius: 0 0 .5em .5em;
}

.popup-music .title .icon-box .text-icon,
.popup-music .artist .icon-box .text-icon,
.popup-music .volume .icon,
.popup-music .album .icon-box .text-icon {
    font-family: JetBrainsMono NFP;
    color: @srcery-cyan;
} 

.popup-music .progress scale trough,
.popup-music .volume scale trough {
    background-color: @srcery-bright-black;
}

.popup-music .progress scale trough highlight,
.popup-music .volume scale trough highlight{
    background-color: @srcery-cyan;
}

/*  -------- FOCUSED ---------  */
.foco .label{
    background-color: @srcery-black;
    box-shadow: inset 0px 4px 0px 0px @srcery-blue;
    padding-left: 4px;
    padding-right:4px;
    padding-top: 5px;
    border-radius: 0 0 .4em .4em;
}

/*  -------- NETWORK ---------  */
.network_manager {
    background-color: @srcery-black;
    box-shadow: inset 0px 4px 0px 0px @srcery-magenta;
    padding-left: 4px;
    padding-right:4px;
    padding-top: 5px;
    border-radius: 0 0 .4em .4em;
}

/*  -------- VOLUME ---------  */
#som {
    background-color: @srcery-black;
    border-radius: 0 0 .4em .4em;
    box-shadow: inset 0px 4px 0px 0px @srcery-red;
    padding-left: 4px;
    padding-right:4px;
    padding-top: 5px;
}

#som:hover {
    color: @srcery-black;
    box-shadow: inset 0px 32px 0px 0px @srcery-red;
}

/*  -------- BATTERY ---------  */
.upower {
    background-color: @srcery-black;
    border-radius: 0 0 .4em .4em;
    box-shadow: inset 0px 4px 0px 0px @srcery-orange;
    padding-left: 4px;
    padding-right:4px;
    padding-top: 5px;
}

.upower:hover {
    color: @srcery-black;
    box-shadow: inset 0px 32px 0px 0px @srcery-orange;
}

#cpu, #memory, #net, #cpu_temp {
    padding-bottom: 5px;
    min-width: 5em;
    font-family: IosevkaTermNerdFontPropo;
    font-style: italic;
    font-weight: 500;
    font-size: 12pt;
    text-shadow: @srcery-hard-black 4px 1px 2px;
    color: @srcery-white;
}

#keymaps {
    padding-bottom: 5px;
    min-width: 5em;
    margin-right: 1px;
    margin-left: 15px;
    color: @srcery-white;
}

/*  -------- NOTIFICATIONS ---------  */
.notifications button  {
    margin-left: 4px;
    font-family: IosevkaTermNerdFontMono;
    background-color: @srcery-black;
    padding-top: 5px;
    box-shadow: inset 0px 4px 0px 0px @srcery-blue;
    border-radius: 0 0 .4em .4em;
}

.notifications .count {
    font-family: IosevkaTermNerdFontMono;
    font-size: .6rem;
    background-color: @srcery-red;
    color: @srcery-bright-white;
    border-radius: 100%;
}

/*  -------- CLOCK ---------  */
.clock {
    padding-left:4px;
    padding-right:4px;
    padding-top: 5px;
    border-radius: .4em;
    font-family: IosevkaTermNerdFontPropo;
    font-style: italic;
    font-weight: 500;
    font-size: 12pt;
    color: @srcery-white;
}

.clock:hover {
    box-shadow: inset 0px 35px 0px 0px @srcery-white;
    color: @srcery-black;
}

.popup-clock .calendar-clock {
    color: @srcery-white;
    font-size: 2.2em;
    margin-top: 3px;
    margin-bottom: 7px;
    padding: 0px 2em;
    border-radius: 7px;
    background-color: @srcery-black;
}

.popup-clock .calendar {
    background-color: @srcery-black;
    color: @srcery-bright-white;
    border-radius: .4em;
}

/*  -------- TRAY ---------  */
.tray {
    padding-top: 5px;
}

.tray .item {
    padding-left:4px;
    padding-right:4px;
}

/*  -------- TAGS ---------  */
.tags {
    padding-left:4px;
    padding-right:4px;
    padding-top: 5px;
    border-radius: 0 0 .4em .4em;
    background-color: @srcery-black;
    box-shadow: inset 0px 4px 0px 0px @srcery-white;
}

.tags:hover {
    box-shadow: inset 0px 35px 0px 0px @srcery-white;
    color: @srcery-black;
}
"#;

    // 5. Запись в файлы
    fs::write(config_path.join("srcery_css.css"), srcery_css)?;
    fs::write(config_path.join("style.css"), style_css)?;

    println!("Конфигурация успешно сохранена в {:?}", config_path);
    Ok(())
}
     
  "##),
  
     ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/kbd-rs.rs", r#"use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let base_path = Path::new("/sys/class/leds");
    let pattern = "::capslock";

    // 1. Ищем директорию светодиода Caps Lock
    let entries = fs::read_dir(base_path)?;
    
    let target_dir = entries
        .filter_map(|e| e.ok())
        .find(|e| {
            e.file_name()
                .to_str()
                .map(|s| s.contains(pattern))
                .unwrap_or(false)
        });

    if let Some(entry) = target_dir {
        // 2. Формируем путь к файлу яркости
        let brightness_path = entry.path().join("brightness");

        // 3. Читаем содержимое файла (состояние LED)
        let content = fs::read_to_string(brightness_path)?;
        let status = content.trim(); // Убираем лишние пробелы/переносы

        // 4. Логика вывода флагов (аналог Lua gmatch)
        if status == "0" {
            println!("🇺🇸");
        } else if status == "1" {
            println!("🇷🇺");
        }
    } else {
        // Fallback если устройство не найдено
        eprintln!("Caps Lock LED not found");
    }

    Ok(())
}
      
  "#),
      
      ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/mako-conf.rs", r##"use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    // 1. Получаем путь к домашней директории
    let home = env::var("HOME").expect("Переменная окружения HOME не найдена");
    let mut config_path = PathBuf::from(home);
    config_path.push(".config/mako");

    // 2. Создаем директорию (аналог mkdir -p)
    fs::create_dir_all(&config_path)?;

    // 3. Содержимое config
    let config= r#"font=UbuntuMono 13.5
background-color=#080815
text-color=#BAA67F
width=400
height=300
padding=10
margin=10
progress-color=over #BAA67F
icon-path=/usr/share/icons/Yaru-blue
max-icon-size=70
layer=overlay
border-size=1
border-radius=10
border-color=#3B1515
default-timeout=8000
[urgency=high]
ignore-timeout=1"#;

    // 5. Запись в файлы
    fs::write(config_path.join("config"), config)?;
    
    println!("Конфигурация успешно сохранена в {:?}", config_path);
    Ok(())
}
      
  "##),
  
     ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/tagactive.rs", r#"use std::fs::File;
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let mut content = String::new();
    
    if let Ok(mut file) = File::open("/var/tmp/wm/tag") {
        file.read_to_string(&mut content)?;
        let tag = content.trim();

        // font_weight='900' дает максимально возможную жирность
        // Цвет #CDA434 (Satin Sheen Gold) — насыщенный темно-желтый
       // println!("<span color='#CDA434' font_weight='900'>{}</span>", tag);
       println!("<span color='#BAA67F' font_weight='900' size='large'>{}</span>", tag);

    }

    Ok(())
}      
  "#),
     
     ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/memory_oval.rs", r##"use std::fs;
use std::io::BufRead;

fn main() {
    // --- Настройки стиля ---
    const WIDTH: usize = 8;
    const FONT_SIZE: &str = "9pt";
    const BARS: [&str; 8] = [" ", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
    const LEFT_CAP: &str = "";
    const RIGHT_CAP: &str = "";

    // Цветовая схема
    const COLOR_HIGH: &str = "#8B4513"; // Коричневый (> 80%)
    const COLOR_MID: &str = "#FFFFFF";  // Белый (40-80%)
    const COLOR_LOW: &str = "#808080";  // Серый (< 40%)

    // 1. Получение данных о памяти
    let load = get_ram_usage();

    // 2. Выбор цвета
    let color = if load > 80 {
        COLOR_HIGH
    } else if load > 40 {
        COLOR_MID
    } else {
        COLOR_LOW
    };

    // 3. Форматирование метки (центрирование)
    let label = format!("{}%", load);
    let mut label_padded = label.clone();
    while label_padded.chars().count() < 4 {
        label_padded.insert(0, ' ');
    }
    let label_chars: Vec<char> = label_padded.chars().collect();
    let text_start = (WIDTH.saturating_sub(label_chars.len())) / 2;

    // 4. Сборка Pango-строки
    let mut output = format!(
        "<span size='{}' color='{}' face='monospace'>{}",
        FONT_SIZE, color, LEFT_CAP
    );

    // Расчет индекса полоски (0-100%)
    let bar_idx = ((load as f32 / 100.0) * (BARS.len() - 1) as f32).round() as usize;
    let bar_char = BARS[bar_idx.min(BARS.len() - 1)];

    for i in 0..WIDTH {
        if i >= text_start && i < text_start + label_chars.len() {
            output.push(label_chars[i - text_start]);
        } else {
            output.push_str(bar_char);
        }
    }

    output.push_str(RIGHT_CAP);
    output.push_str("</span>");

    println!("{}", output);
}

fn get_ram_usage() -> u32 {
    let mut total = 0;
    let mut available = 0;

    if let Ok(file) = fs::File::open("/proc/meminfo") {
        let reader = std::io::BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                match parts[0] {
                    "MemTotal:" => total = parts[1].parse::<u64>().unwrap_or(0),
                    "MemAvailable:" => available = parts[1].parse::<u64>().unwrap_or(0),
                    _ => {}
                }
            }
            if total > 0 && available > 0 { break; }
        }
    }

    if total > 0 {
        let used = total.saturating_sub(available);
        return ((used as f64 / total as f64) * 100.0) as u32;
    }
    0
}
      
  "##),
  
       ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/thunar-conf.rs", r##"use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    // 1. Получаем путь к домашней директории
    let home = env::var("HOME").expect("Переменная окружения HOME не найдена");
    let mut config_path = PathBuf::from(home);
    config_path.push(".config/Thunar");

    // 2. Создаем директорию (аналог mkdir -p)
    fs::create_dir_all(&config_path)?;

    // 3. Содержимое accels.scm
    let accels_scm= r#"; thunar GtkAccelMap rc-file         -*- scheme -*-
; this file is an automated accelerator map dump
;
; (gtk_accel_path "<Actions>/ThunarStandardView/sort-by-type" "")
; (gtk_accel_path "<Actions>/ThunarStatusBar/toggle-last-modified" "")
; (gtk_accel_path "<Actions>/Thunarwindow/menu" "")
; (gtk_accel_path "<Actions>/ThunarActionManager/cut" "<Primary>x")
; (gtk_accel_path "<Actions>/ThunarStandardView/sort-by-size" "")
; (gtk_accel_path "<Actions>/ThunarWindow/file-menu" "")
; (gtk_accel_path "<Actions>/ThunarWindow/close-tab" "<Primary>w")
; (gtk_accel_path "<Actions>/ThunarWindow/switch-previous-tab-alt" "<Primary><Shift>ISO_Left_Tab")
; (gtk_accel_path "<Actions>/ThunarStatusBar/toggle-size" "")
; (gtk_accel_path "<Actions>/ThunarWindow/new-window" "<Primary>n")
; (gtk_accel_path "<Actions>/ThunarWindow/clear-directory-specific-settings" "")
; (gtk_accel_path "<Actions>/ThunarWindow/close-window" "<Primary>q")
; (gtk_accel_path "<Actions>/ThunarWindow/open-parent" "<Alt>Up")
; (gtk_accel_path "<Actions>/ThunarWindow/view-side-pane-menu" "")
; (gtk_accel_path "<Actions>/ThunarStatusBar/toggle-size-in-bytes" "")
; (gtk_accel_path "<Actions>/ThunarWindow/switch-previous-tab" "<Primary>Page_Up")
; (gtk_accel_path "<Actions>/ThunarActionManager/open" "<Primary>o")
; (gtk_accel_path "<Actions>/ThunarStandardView/sort-ascending" "")
; (gtk_accel_path "<Actions>/ThunarWindow/toggle-split-view" "F3")
; (gtk_accel_path "<Actions>/ThunarActionManager/copy-2" "<Primary>Insert")
; (gtk_accel_path "<Actions>/ThunarActionManager/trash-delete" "Delete")
; (gtk_accel_path "<Actions>/ThunarWindow/open-recent" "")
; (gtk_accel_path "<Actions>/ThunarWindow/view-configure-toolbar" "")
; (gtk_accel_path "<Actions>/ThunarStandardView/forward" "<Alt>Right")
; (gtk_accel_path "<Actions>/ThunarActionManager/restore" "")
; (gtk_accel_path "<Actions>/ThunarWindow/open-location-alt" "<Alt>d")
; (gtk_accel_path "<Actions>/ThunarWindow/zoom-out-alt" "<Primary>KP_Subtract")
; (gtk_accel_path "<Actions>/ThunarStandardView/select-by-pattern" "<Primary>s")
; (gtk_accel_path "<Actions>/ThunarWindow/open-file-menu" "F10")
; (gtk_accel_path "<Actions>/ThunarWindow/contents" "F1")
; (gtk_accel_path "<Actions>/ThunarWindow/show-highlight" "")
; (gtk_accel_path "<Actions>/ThunarStandardView/sort-descending" "")
; (gtk_accel_path "<Actions>/ThunarStandardView/sort-by-name" "")
; (gtk_accel_path "<Actions>/ThunarStandardView/select-all-files" "<Primary>a")
; (gtk_accel_path "<Actions>/ThunarActionManager/execute" "")
; (gtk_accel_path "<Actions>/ThunarStandardView/properties" "<Alt>Return")
; (gtk_accel_path "<Actions>/ThunarActionManager/cut-2" "")
; (gtk_accel_path "<Actions>/ThunarStandardView/sort-by-dtime" "")
; (gtk_accel_path "<Actions>/ThunarBookmarks/e57e4df3a77c7742240de9671131bdd4" "")
; (gtk_accel_path "<Actions>/ThunarWindow/switch-next-tab" "<Primary>Page_Down")
; (gtk_accel_path "<Actions>/ThunarActionManager/paste-2" "<Shift>Insert")
; (gtk_accel_path "<Actions>/ThunarWindow/open-templates" "")
; (gtk_accel_path "<Actions>/ThunarStatusBar/toggle-filetype" "")
; (gtk_accel_path "<Actions>/ThunarWindow/close-all-windows" "<Primary><Shift>w")
; (gtk_accel_path "<Actions>/ThunarStandardView/create-document" "")
; (gtk_accel_path "<Actions>/ThunarWindow/detach-tab" "")
; (gtk_accel_path "<Actions>/ThunarWindow/cancel-search" "Escape")
; (gtk_accel_path "<Actions>/ThunarWindow/zoom-in-alt2" "<Primary>equal")
; (gtk_accel_path "<Actions>/ThunarStatusBar/toggle-hidden-count" "")
; (gtk_accel_path "<Actions>/ThunarShortcutsPane/sendto-shortcuts" "<Primary>d")
; (gtk_accel_path "<Actions>/ThunarActionManager/undo" "<Primary>z")
; (gtk_accel_path "<Actions>/ThunarStandardView/toggle-sort-order" "")
; (gtk_accel_path "<Actions>/ThunarWindow/view-location-selector-entry" "")
; (gtk_accel_path "<Actions>/ThunarActionManager/paste" "<Primary>v")
; (gtk_accel_path "<Actions>/ThunarWindow/zoom-in-alt1" "<Primary>KP_Add")
; (gtk_accel_path "<Actions>/ThunarWindow/view-menubar" "<Primary>m")
; (gtk_accel_path "<Actions>/ThunarStandardView/back" "<Alt>Left")
; (gtk_accel_path "<Actions>/ThunarWindow/open-desktop" "")
; (gtk_accel_path "<Actions>/ThunarWindow/view-as-detailed-list" "<Primary>2")
; (gtk_accel_path "<Actions>/ThunarActionManager/restore-show" "")
; (gtk_accel_path "<Actions>/ThunarWindow/sendto-menu" "")
; (gtk_accel_path "<Actions>/ThunarStatusBar/toggle-display-name" "")
; (gtk_accel_path "<Actions>/ThunarWindow/go-menu" "")
; (gtk_accel_path "<Actions>/ThunarWindow/zoom-out" "<Primary>minus")
; (gtk_accel_path "<Actions>/ThunarWindow/remove-from-recent" "")
; (gtk_accel_path "<Actions>/ThunarActionManager/open-with-other" "")
; (gtk_accel_path "<Actions>/ThunarStandardView/invert-selection" "<Primary><Shift>i")
; (gtk_accel_path "<Actions>/ThunarWindow/view-side-pane-shortcuts" "<Primary>b")
; (gtk_accel_path "<Actions>/ThunarWindow/reload-alt-2" "Reload")
; (gtk_accel_path "<Actions>/ThunarWindow/view-location-selector-menu" "")
; (gtk_accel_path "<Actions>/ThunarWindow/reload" "<Primary>r")
; (gtk_accel_path "<Actions>/ThunarWindow/edit-menu" "")
; (gtk_accel_path "<Actions>/ThunarActionManager/copy" "<Primary>c")
; (gtk_accel_path "<Actions>/ThunarWindow/bookmarks-menu" "")
; (gtk_accel_path "<Actions>/ThunarStandardView/forward-alt" "Forward")
; (gtk_accel_path "<Actions>/ThunarActionManager/move-to-trash" "")
; (gtk_accel_path "<Actions>/ThunarWindow/reload-alt-1" "F5")
; (gtk_accel_path "<Actions>/ThunarActionManager/delete-3" "<Shift>KP_Delete")
; (gtk_accel_path "<Actions>/ThunarStandardView/unselect-all-files" "Escape")
; (gtk_accel_path "<Actions>/ThunarWindow/contents/help-menu" "")
; (gtk_accel_path "<Actions>/ThunarStandardView/arrange-items-menu" "")
; (gtk_accel_path "<Actions>/ThunarStandardView/sort-by-mtime" "")
; (gtk_accel_path "<Actions>/ThunarWindow/open-computer" "")
; (gtk_accel_path "<Actions>/ThunarWindow/toggle-image-preview" "")
; (gtk_accel_path "<Actions>/ThunarWindow/toggle-side-pane" "F9")
; (gtk_accel_path "<Actions>/ThunarWindow/view-as-icons" "<Primary>1")
; (gtk_accel_path "<Actions>/ThunarActionManager/delete-2" "<Shift>Delete")
; (gtk_accel_path "<Actions>/ThunarWindow/zoom-in" "<Primary>plus")
; (gtk_accel_path "<Actions>/ThunarStandardView/rename" "F2")
; (gtk_accel_path "<Actions>/ThunarWindow/open-location" "<Primary>l")
; (gtk_accel_path "<Actions>/ThunarWindow/view-as-compact-list" "<Primary>3")
; (gtk_accel_path "<Actions>/ThunarWindow/view-menu" "")
; (gtk_accel_path "<Actions>/ThunarWindow/search" "<Primary>f")
; (gtk_accel_path "<Actions>/ThunarWindow/new-tab" "<Primary>t")
; (gtk_accel_path "<Actions>/ThunarWindow/zoom-reset" "<Primary>0")
; (gtk_accel_path "<Actions>/ThunarStandardView/back-alt2" "Back")
; (gtk_accel_path "<Actions>/ThunarActionManager/open-in-new-tab" "<Primary><Shift>p")
; (gtk_accel_path "<Actions>/ThunarBookmarks/069a39d9d85b380d00efff37b8f4a6a3" "")
; (gtk_accel_path "<Actions>/ThunarWindow/view-location-selector-buttons" "")
; (gtk_accel_path "<Actions>/ThunarActionManager/redo" "<Primary><Shift>z")
; (gtk_accel_path "<Actions>/ThunarWindow/open-trash" "")
; (gtk_accel_path "<Actions>/ThunarActionManager/open-in-new-window" "<Primary><Shift>o")
; (gtk_accel_path "<Actions>/ThunarWindow/view-statusbar" "")
; (gtk_accel_path "<Actions>/ThunarActionManager/open-location" "")
; (gtk_accel_path "<Actions>/ThunarStandardView/duplicate" "")
; (gtk_accel_path "<Actions>/ThunarActionManager/trash-delete-2" "KP_Delete")
; (gtk_accel_path "<Actions>/ThunarStandardView/back-alt1" "BackSpace")
; (gtk_accel_path "<Actions>/ThunarStandardView/create-folder" "<Primary><Shift>n")
; (gtk_accel_path "<Actions>/ThunarWindow/open-home" "<Alt>Home")
; (gtk_accel_path "<Actions>/ThunarWindow/switch-focused-split-view-pane" "")
; (gtk_accel_path "<Actions>/ThunarWindow/show-hidden" "<Primary>h")
; (gtk_accel_path "<Actions>/ThunarStandardView/set-default-app" "")
; (gtk_accel_path "<Actions>/ThunarWindow/empty-trash" "")
; (gtk_accel_path "<Actions>/ThunarWindow/preferences" "")
; (gtk_accel_path "<Actions>/ThunarActionManager/delete" "")
; (gtk_accel_path "<Actions>/ThunarWindow/open-network" "")
; (gtk_accel_path "<Actions>/ThunarWindow/view-side-pane-tree" "<Primary>e")
; (gtk_accel_path "<Actions>/ThunarWindow/open-file-system" "")
; (gtk_accel_path "<Actions>/ThunarWindow/search-alt" "Search")
; (gtk_accel_path "<Actions>/ThunarWindow/switch-next-tab-alt" "<Primary>Tab")
; (gtk_accel_path "<Actions>/ThunarActionManager/sendto-desktop" "")
; (gtk_accel_path "<Actions>/ThunarStandardView/make-link" "")
; (gtk_accel_path "<Actions>/ThunarWindow/zoom-reset-alt" "<Primary>KP_0")
; (gtk_accel_path "<Actions>/ThunarWindow/about" "")"#;

    // 4. Содержимое uca.xml
    let uca_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<actions>
<action>
	<icon></icon>
	<name>Playlist </name>
	<submenu></submenu>
	<unique-id>1716535410376310-1</unique-id>
	<command>echo -n %f | /usr/bin/awk &apos;{print $0}&apos;&gt;&gt;~/tmp/playlist</command>
	<description>Воспроизведение</description>
	<range>*</range>
	<patterns>*</patterns>
	<directories/>
	<audio-files/>
	<video-files/>
</action>
<action>
	<icon>utilities-terminal</icon>
	<name>Open Terminal Here</name>
	<submenu></submenu>
	<unique-id>1714545928336620-1</unique-id>
	<command>exo-open --working-directory %f --launch TerminalEmulator</command>
	<description>Example for a custom action</description>
	<range></range>
	<patterns>*</patterns>
	<startup-notify/>
	<directories/>
</action>
</actions>
"#;

    // 5. Запись в файлы
    fs::write(config_path.join("accels.scm"), accels_scm)?;
    fs::write(config_path.join("uca.xml"), uca_xml)?;

    println!("Конфигурация успешно сохранена в {:?}", config_path);
    Ok(())
}      
  "##),
       
      ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/netgraph.rs", r##"use std::fs;
use std::io::Write;

fn main() {
    // --- Настройки интерфейса ---
    const INTERFACE: &str = "eth0";
    const MAX_SPEED_KB: f64 = 1024.0;
    const INTERVAL: f64 = 2.0;
    const STATE_FILE: &str = "/tmp/ironbar_net_dual_state";

    // --- Настройки стиля ---
    const WIDTH_EACH: usize = 8;
    const FONT_SIZE: &str = "9pt";
    const BARS: [&str; 8] = [" ", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
    const LEFT_CAP: &str = "";
    const RIGHT_CAP: &str = "";

    const COLOR_LOW: &str = "#B38181";
    const COLOR_MID: &str = "#E3BFBF";
    const COLOR_HIGH: &str = "#EDE5E5";

    // 1. Получение текущих данных байт
    let (curr_rx, curr_tx) = get_net_bytes(INTERFACE);

    // 2. Чтение предыдущего состояния
    let mut prev_rx: u64 = 0;
    let mut prev_tx: u64 = 0;
    if let Ok(content) = fs::read_to_string(STATE_FILE) {
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() == 2 {
            prev_rx = parts[0].parse().unwrap_or(0);
            prev_tx = parts[1].parse().unwrap_or(0);
        }
    }

    // 3. Сохранение текущего состояния
    if let Ok(mut f) = fs::File::create(STATE_FILE) {
        let _ = write!(f, "{} {}", curr_rx, curr_tx);
    }

    // 4. Расчет скоростей (КБ/с)
    let speed_rx = if curr_rx >= prev_rx { (curr_rx - prev_rx) as f64 / 1024.0 / INTERVAL } else { 0.0 };
    let speed_tx = if curr_tx >= prev_tx { (curr_tx - prev_tx) as f64 / 1024.0 / INTERVAL } else { 0.0 };

    // 5. Функция генерации сегмента (RX или TX)
    let gen_segment = |speed: f64| -> String {
        let load_pct = (speed / MAX_SPEED_KB * 100.0).min(100.0);
        let color = if load_pct > 80.0 { COLOR_HIGH } else if load_pct > 40.0 { COLOR_MID } else { COLOR_LOW };

        let label = if speed >= 1000.0 {
            format!("{:.1}M", speed / 1024.0)
        } else {
            format!("{}K", speed.floor() as u32)
        };

        let mut label_padded = label.clone();
        while label_padded.chars().count() < 4 {
            label_padded.insert(0, ' ');
        }
        let label_chars: Vec<char> = label_padded.chars().collect();
        let text_start = (WIDTH_EACH.saturating_sub(label_chars.len())) / 2;

        let bar_idx = ((load_pct / 100.0) * (BARS.len() - 1) as f64).round() as usize;
        let bar_char = BARS[bar_idx];

        let mut segment = format!("<span color='{}'>", color);
        for i in 0..WIDTH_EACH {
            if i >= text_start && i < text_start + label_chars.len() {
                segment.push(label_chars[i - text_start]);
            } else {
                segment.push_str(bar_char);
            }
        }
        segment.push_str("</span>");
        segment
    };

    // 6. Сборка итоговой строки
    println!(
        "<span size='{}' face='monospace'><span color='{}'>{}</span>{} {} {}<span color='{}'>{}</span></span>",
        FONT_SIZE, COLOR_LOW, LEFT_CAP, gen_segment(speed_rx), " ", gen_segment(speed_tx), COLOR_LOW, RIGHT_CAP
    );
}

fn get_net_bytes(interface: &str) -> (u64, u64) {
    let read_val = |file: &str| -> u64 {
        let path = format!("/sys/class/net/{}/statistics/{}", interface, file);
        fs::read_to_string(path)
            .ok()
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0)
    };
    (read_val("rx_bytes"), read_val("tx_bytes"))
}      
  "##), 
       
     ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/txgraph.rs", r##"use std::fs;
use std::io::Write;

fn main() {
    // --- Настройки интерфейса ---
    const INTERFACE: &str = "eth0";
    const MAX_SPEED_KB: f64 = 1024.0;
    const INTERVAL: f64 = 2.0;
    const STATE_FILE: &str = "/tmp/ironbar_net_tx_state";

    // --- Настройки стиля ---
    const WIDTH: usize = 6;
    const FONT_SIZE: &str = "9pt";
    const BARS: [&str; 8] = [" ", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
    const LEFT_CAP: &str = "";
    const RIGHT_CAP: &str = "";

    // Цвета
    const COLOR_LOW: &str = "#B38181";
    const COLOR_MID: &str = "#E3BFBF";
    const COLOR_HIGH: &str = "#EDE5E5";

    // 1. Получение текущих байт TX
    let curr_tx = get_tx_bytes(INTERFACE);

    // 2. Чтение предыдущего состояния
    let prev_tx: u64 = fs::read_to_string(STATE_FILE)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(curr_tx);

    // 3. Сохранение текущего состояния
    if let Ok(mut f) = fs::File::create(STATE_FILE) {
        let _ = write!(f, "{}", curr_tx);
    }

    // 4. Расчет скорости (кБ/с)
    let diff_tx = if curr_tx >= prev_tx { curr_tx - prev_tx } else { 0 };
    let speed_kb = diff_tx as f64 / 1024.0 / INTERVAL;

    // 5. Расчет визуальных параметров
    let load_pct = (speed_kb / MAX_SPEED_KB * 100.0).min(100.0);
    
    let color = if load_pct > 80.0 {
        COLOR_HIGH
    } else if load_pct > 40.0 {
        COLOR_MID
    } else {
        COLOR_LOW
    };

    // Формирование метки скорости
    let label = if speed_kb >= 1000.0 {
        format!("{:.1}M", speed_kb / 1024.0)
    } else {
        format!("{}K", speed_kb.floor() as u32)
    };

    // Центрирование (подгонка под 4 символа)
    let mut label_padded = label.clone();
    while label_padded.chars().count() < 4 {
        label_padded.insert(0, ' ');
    }
    let label_chars: Vec<char> = label_padded.chars().collect();
    let text_start = (WIDTH.saturating_sub(label_chars.len())) / 2;

    // 6. Сборка Pango-строки
    let mut output = format!("<span size='{}' color='{}' face='monospace'>{}", FONT_SIZE, color, LEFT_CAP);

    let bar_idx = ((load_pct / 100.0) * (BARS.len() - 1) as f64).round() as usize;
    let bar_char = BARS[bar_idx];

    for i in 0..WIDTH {
        if i >= text_start && i < text_start + label_chars.len() {
            output.push(label_chars[i - text_start]);
        } else {
            output.push_str(bar_char);
        }
    }

    output.push_str(RIGHT_CAP);
    output.push_str("</span>");

    println!("{}", output);
}

fn get_tx_bytes(interface: &str) -> u64 {
    let path = format!("/sys/class/net/{}/statistics/tx_bytes", interface);
    fs::read_to_string(path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}
     
  "##),      
     
     ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/pinnacle-screenshot.rs", r#"use anyhow::{anyhow, Context, Result};
use clap::{Parser, ValueEnum};
use serde::Deserialize;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum Mode {
    Output,
    Window,
    Region,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Pinnacle screenshot utility for Sway")]
struct Args {
    #[arg(short, long, value_enum)]
    mode: Mode,

    /// Путь к папке (по умолчанию ~/Изображения/screenshots или $PINNACLE_SCREENSHOT_DIR)
    #[arg(short, long)]
    output_folder: Option<PathBuf>,

    /// Имя файла (по умолчанию дата-время)
    #[arg(short, long)]
    filename: Option<String>,

    #[arg(long)]
    clipboard_only: bool,

    #[arg(short, long)]
    debug: bool,

    #[arg(short, long)]
    silent: bool,

    #[arg(last = true)]
    command: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct SwayNode {
    rect: Rect,
    name: Option<String>,
    visible: Option<bool>,
    pid: Option<u64>,
    #[serde(default)]
    nodes: Vec<SwayNode>,
    #[serde(default)]
    floating_nodes: Vec<SwayNode>,
}

#[derive(Deserialize, Debug)]
struct Rect { x: i32, y: i32, width: i32, height: i32 }

fn main() -> Result<()> {
    let args = Args::parse();

    // 1. Определяем путь сохранения (логика из Bash-скрипта)
    let save_dir = if let Some(ref path) = args.output_folder {
        path
    } else if let Ok(env_path) = std::env::var("PINNACLE_SCREENSHOT_DIR") {
        &PathBuf::from(env_path)
    } else {
        let home = std::env::var("HOME").context("Не удалось найти $HOME")?;
        &PathBuf::from(format!("{}/Изображения/screenshots", home))
    };

    // 2. Определяем имя файла
    let filename = args.filename.clone().unwrap_or_else(|| {
        chrono::Local::now().format("%Y-%m-%d-%H%M%S_pinnacle-screenshot.png").to_string()
    });

    let full_path = save_dir.join(&filename);

    // 3. Выделение мышью (Grab geometry)
    let geometry = match args.mode {
        Mode::Output => grab_output()?,
        Mode::Region => grab_region()?, // Здесь вызывается slurp для выделения области
        Mode::Window => grab_window(args.debug)?, // Здесь slurp для выбора окна
    };

    if args.debug {
        println!("Selected Geometry: {}", geometry);
    }

    if args.clipboard_only {
        copy_to_clipboard_only(&geometry)?;
    } else {
        save_and_notify(&geometry, &full_path, &args)?;
        
        if !args.command.is_empty() {
            Command::new(&args.command[0])
                .args(&args.command[1..])
                .arg(&full_path)
                .spawn()
                .context("Ошибка при запуске внешней команды")?;
        }
    }

    Ok(())
}

fn grab_output() -> Result<String> {
    let output = Command::new("slurp").arg("-or").output()?;
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn grab_region() -> Result<String> {
    // slurp -d позволяет выделить область мышью
    let output = Command::new("slurp").arg("-d").output()
        .context("Убедитесь, что 'slurp' установлен")?;
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn grab_window(debug: bool) -> Result<String> {
    let sway_raw = Command::new("swaymsg").args(["-t", "get_tree"]).output()?.stdout;
    let tree: SwayNode = serde_json::from_slice(&sway_raw)?;
    let mut boxes = String::new();
    
    fn extract(node: &SwayNode, acc: &mut String) {
        if node.visible == Some(true) && node.pid.is_some() {
            acc.push_str(&format!("{},{} {}x{} {}\n", 
                node.rect.x, node.rect.y, node.rect.width, node.rect.height, 
                node.name.as_deref().unwrap_or("unknown")));
        }
        for n in &node.nodes { extract(n, acc); }
        for n in &node.floating_nodes { extract(n, acc); }
    }
    extract(&tree, &mut boxes);

    if debug { println!("Window Boxes:\n{}", boxes); }

    let mut child = Command::new("slurp")
        .arg("-r")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(boxes.as_bytes())?;
    drop(stdin);

    let output = child.wait_with_output()?;
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn save_and_notify(geometry: &str, path: &Path, args: &Args) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // grim делает скриншот выбранной области
    let status = Command::new("grim").args(["-g", geometry]).arg(path).status()?;
    if !status.success() { return Err(anyhow!("grim завершился с ошибкой")); }

    // convert для обрезки (если нужно)
    let _ = Command::new("convert")
        .args([path.to_str().unwrap(), "-trim", "+repage", path.to_str().unwrap()])
        .status();

    // Копирование в буфер обмена
    let img_data = fs::read(path)?;
    let mut child = Command::new("wl-copy").stdin(Stdio::piped()).spawn()?;
    child.stdin.as_mut().unwrap().write_all(&img_data)?;
    child.wait()?;

    if !args.silent {
        Command::new("notify-send")
            .arg("Скриншот сохранен")
            .arg(format!("Файл: {}\nСкопировано в буфер.", path.display()))
            .args(["-i", path.to_str().unwrap()])
            .status()?;
    }

    println!("Сохранено: {}", path.display());
    Ok(())
}

fn copy_to_clipboard_only(geometry: &str) -> Result<()> {
    let grim = Command::new("grim").args(["-g", geometry, "-"]).stdout(Stdio::piped()).spawn()?;
    let convert = Command::new("convert").args(["-", "-trim", "+repage", "-"])
        .stdin(grim.stdout.unwrap()).stdout(Stdio::piped()).spawn()?;
    let mut wl_copy = Command::new("wl-copy").stdin(convert.stdout.unwrap()).spawn()?;
    wl_copy.wait()?;
    Ok(())
}
     
  "#),
     
      ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/vol_info.rs", r#"use std::process::Command;

fn main() {
    // --- Настройки ---
    const BAR_LENGTH: usize = 15;
    const CHANNEL: &str = "PCM"; // Измените на "Master", если PCM не находит

    // 1. Вызываем amixer
    let output = Command::new("amixer")
        .args(["sget", CHANNEL])
        .output();

    match output {
        Ok(out) => {
            let result = String::from_utf8_lossy(&out.stdout);
            
            // 2. Ищем значение громкости (например, [50%])
            if let Some(start) = result.find('[') {
                if let Some(end) = result[start..].find('%') {
                    let vol_str = &result[start + 1..start + end];
                    
                    if let Ok(vol_num) = vol_str.parse::<u32>() {
                        // 3. Определяем иконку и текстовый статус
                        let (icon, status) = if vol_num > 80 {
                            ("🔊", "Высокая")
                        } else if vol_num > 30 {
                            ("🔉", "Средняя")
                        } else if vol_num > 0 {
                            ("🔈", "Низкая")
                        } else {
                            ("🔇", "Выключено")
                        };

                        // 4. Генерируем полоску (Progress Bar)
                        let full_chars = (vol_num as usize * BAR_LENGTH) / 100;
                        let empty_chars = BAR_LENGTH.saturating_sub(full_chars);
                        let bar = format!("{}{}", "█".repeat(full_chars), "░".repeat(empty_chars));

                        // 5. Вывод финальной строки
                        println!(
                            "{} {} громкость: {} {}%",
                            icon, status, bar, vol_num
                        );
                        return;
                    }
                }
            }
            println!("Ошибка: Не удалось получить данные от amixer");
        }
        Err(_) => {
            println!("Ошибка: Команда amixer не найдена");
        }
    }
}

      
  "#),
      ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/play_next.rs", r#"use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command};

fn main() -> std::io::Result<()> {
    // 1. Настройка путей
    let home = env::var("HOME").expect("Переменная HOME не найдена");
    let tmp_path = PathBuf::from(&home).join("tmp");
    let sox_path = tmp_path.join("sox");
    
    let playlist_main = tmp_path.join("playlist");
    let playlist_sox = sox_path.join("playlist");
    let name_file = sox_path.join("name");

    // 2. Остановка текущего воспроизведения (ОБЯЗАТЕЛЬНО для листания)
    // Убиваем процесс 'play', чтобы начать новый трек немедленно
    let _ = Command::new("pkill")
        .arg("-x")
        .arg("play")
        .status();

    // Создаем директорию если её нет
    fs::create_dir_all(&sox_path)?;

    // 3. Читаем основной плейлист
    if !playlist_main.exists() {
        eprintln!("Плейлист пуст или не найден!");
        return Ok(());
    }

    let file = File::open(&playlist_main)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    if lines.is_empty() {
        Command::new("notify-send").arg("Sox").arg("Плейлист окончен").spawn()?;
        return Ok(());
    }

    // 4. ЛИСТАНИЕ: Берем первый трек, удаляем его из списка
    let current_track = lines.remove(0);

    // Сохраняем копию плейлиста в sox (для истории или бэкапа)
    fs::copy(&playlist_main, &playlist_sox)?;

    // Обновляем основной плейлист (уже без первого трека)
    fs::write(&playlist_main, lines.join("\n") + "\n")?;

    // Записываем имя текущего трека в файл name для виджетов
    fs::write(&name_file, &current_track)?;

    // 5. ЗАПУСК
    // Запускаем через fish ваш скрипт play_sox или напрямую play
    // Рекомендуется напрямую, чтобы избежать гонки процессов
    Command::new("fish")
        .arg("-c")
        .arg("play_sox") // Ваш существующий скрипт с параметрами Sox
        .spawn()?;

    println!("Переключено на: {}", current_track);
    Ok(())
}
     
  "#),
     ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/vol_widget.rs", r#"use std::process::Command;

fn main() {
    // 1. Выполняем команду amixer
    let output = Command::new("amixer")
        .args(["sget", "PCM"])
        .output();

    match output {
        Ok(out) => {
            let result = String::from_utf8_lossy(&out.stdout);
            
            // 2. Ищем процент громкости в выводе с помощью регулярного выражения или поиска по подстроке
            // Пример строки: [50%]
            if let Some(start) = result.find('[') {
                if let Some(end) = result[start..].find('%') {
                    let vol_str = &result[start + 1..start + end];
                    
                    if let Ok(vol_num) = vol_str.parse::<u32>() {
                        // 3. Определяем иконку
                        let icon = if vol_num > 80 {
                            "🔊"
                        } else if vol_num > 30 {
                            "🔉"
                        } else if vol_num > 0 {
                            "🔈"
                        } else {
                            "🔇"
                        };

                        // 4. Печатаем результат 
                        println!("{}  {}%", icon, vol_num);
                        return;
                    }
                }
            }
            println!("Ошибка: Не удалось распознать уровень громкости");
        }
        Err(_) => {
            println!("Ошибка: amixer не установлен или не найден");
        }
    }
}      
  "#),
      ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/play_sox.rs", r#"use std::process::{Command, Stdio};
use std::io::{Write, Error, BufRead, BufReader};
use std::fs::{self, File, OpenOptions};
use std::env;

fn main() -> Result<(), Error> {
    // 1. Проверка: если 'play' уже запущен, выходим
    let is_playing = Command::new("pgrep")
        .arg("-x")
        .arg("play")
        .stdout(Stdio::null())
        .status();

    if let Ok(status) = is_playing {
        if status.success() {
            println!("Sox уже играет.");
            return Ok(());
        }
    }

    // 2. Определяем пути
    let home = env::var("HOME").expect("HOME not found");
    let tmp_path = format!("{}/tmp", home);
    let playlist_file = format!("{}/playlist", tmp_path);
    let name_file = format!("{}/sox/name", tmp_path);
    let history_file = format!("{}/playlist1", tmp_path);
    let wm_playlist = "/var/tmp/wm/playlist";

    // Создаем директорию для имени файла, если её нет
    fs::create_dir_all(format!("{}/sox", tmp_path))?;

    // 3. Читаем первую строку плейлиста
    let file = File::open(&playlist_file)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    if lines.is_empty() {
        println!("Плейлист пуст.");
        return Ok(());
    }

    let current_track = &lines[0]; // Первая строка (путь к файлу)
    let remaining_tracks = &lines[1..]; // Остальные строки

    // 4. Логика перемещения строк (аналог sed '1d' и head -n1)
    // Обновляем основной плейлист (удаляем первую строку)
    fs::write(&playlist_file, remaining_tracks.join("\n") + "\n")?;

    // Сохраняем имя текущего трека
    fs::write(&name_file, current_track)?;

    // Добавляем в историю (playlist1)
    let mut history = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&history_file)?;
    writeln!(history, "{}", current_track)?;

    // Обновляем плейлист для виджета (cut -c27-100)
    let short_list: Vec<String> = remaining_tracks
        .iter()
        .map(|s| if s.len() > 26 { s[26..].to_string() } else { s.clone() })
        .collect();
    fs::write(wm_playlist, short_list.join("\n") + "\n")?;

    // 5. ЗАПУСК PLAY НАПРЯМУЮ
    println!("Запуск: {}", current_track);
    Command::new("play")
        .arg(current_track)
        .arg("rate").arg("-v").arg("48000")
        .arg("bass").arg("+15")
        .arg("treble").arg("+12")
        .spawn()?;

    // 6. Рекурсивный вызов следующего трека
    // Чтобы после завершения play запустился следующий, используем цепочку:
    // В 2026 году лучше запустить фоновый процесс, который дождется завершения
    let current_exe = env::current_exe()?;
    Command::new("sh")
        .arg("-c")
        .arg(format!("sleep 1 && while pgrep -x play > /dev/null; do sleep 2; done; {} ", current_exe.display()))
        .spawn()?;

    Ok(())
}     
  "#),    


     ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/switcher.rs", r#"use std::fs;
use std::path::Path;
use wallpaper;

fn main() {
    let status_file = "/var/tmp/wm/tag";
    let wp_dir = "/var/tmp/wm/pictures";

    match fs::read_to_string(status_file) {
        Ok(content) => {
            let tag_num: String = content
                .chars()
                .filter_map(|c| match c {
                    '𝟏' | '1' => Some('1'),
                    '𝟐' | '2' => Some('2'),
                    '𝟑' | '3' => Some('3'),
                    '𝟒' | '4' => Some('4'),
                    '𝟓' | '5' => Some('5'),
                    '𝟔' | '6' => Some('6'),
                    _ => None,
                })
                .take(1)
                .collect();

            if tag_num.is_empty() { return; }
            let wp_path = format!("{}/tag{}.jpg", wp_dir, tag_num);

            if Path::new(&wp_path).exists() {
                let _ = wallpaper::set_from_path(&wp_path);
            }
        }
        Err(_) => (),
    }
}
       
    
  "#),  
        ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/rxgraph.rs", r##"use std::fs;
use std::io::Write;

fn main() {
    // --- Настройки интерфейса ---
    const INTERFACE: &str = "eth0";
    const MAX_SPEED_KB: f64 = 1024.0;
    const INTERVAL: f64 = 2.0;
    const STATE_FILE: &str = "/tmp/ironbar_net_rx_state";

    // --- Настройки стиля ---
    const WIDTH: usize = 7;
    const FONT_SIZE: &str = "9pt";
    const BARS: [&str; 8] = [" ", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
    const LEFT_CAP: &str = "";
    const RIGHT_CAP: &str = "";

    // Цветовая схема
    const COLOR_LOW: &str = "#8C74A1";
    const COLOR_MID: &str = "#E3BFBF";
    const COLOR_HIGH: &str = "#EDE5E5";

    // 1. Получение текущих байт RX
    let curr_rx = get_rx_bytes(INTERFACE);

    // 2. Чтение предыдущего состояния
    let prev_rx: u64 = fs::read_to_string(STATE_FILE)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(curr_rx);

    // 3. Сохранение текущего состояния
    if let Ok(mut f) = fs::File::create(STATE_FILE) {
        let _ = write!(f, "{}", curr_rx);
    }

    // 4. Расчет скорости (кБ/с)
    let diff_rx = if curr_rx >= prev_rx { curr_rx - prev_rx } else { 0 };
    let speed_kb = diff_rx as f64 / 1024.0 / INTERVAL;

    // 5. Расчет визуальных параметров
    let load_pct = (speed_kb / MAX_SPEED_KB * 100.0).min(100.0);
    
    let color = if load_pct > 80.0 {
        COLOR_HIGH
    } else if load_pct > 40.0 {
        COLOR_MID
    } else {
        COLOR_LOW
    };

    // Формирование метки скорости (М или К)
    let label = if speed_kb >= 1000.0 {
        format!("{:.1}M", speed_kb / 1024.0)
    } else {
        format!("{}K", speed_kb.floor() as u32)
    };

    // Центрирование (подгонка под 4 символа)
    let mut label_padded = label.clone();
    while label_padded.chars().count() < 4 {
        label_padded.insert(0, ' ');
    }
    let label_chars: Vec<char> = label_padded.chars().collect();
    let text_start = (WIDTH.saturating_sub(label_chars.len())) / 2;

    // 6. Сборка Pango-строки
    let mut output = format!("<span size='{}' color='{}' face='monospace'>{}", FONT_SIZE, color, LEFT_CAP);

    let bar_idx = ((load_pct / 100.0) * (BARS.len() - 1) as f64).round() as usize;
    let bar_char = BARS[bar_idx];

    for i in 0..WIDTH {
        if i >= text_start && i < text_start + label_chars.len() {
            output.push(label_chars[i - text_start]);
        } else {
            output.push_str(bar_char);
        }
    }

    output.push_str(RIGHT_CAP);
    output.push_str("</span>");

    println!("{}", output);
}

fn get_rx_bytes(interface: &str) -> u64 {
    let path = format!("/sys/class/net/{}/statistics/rx_bytes", interface);
    fs::read_to_string(path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}
       
         "##),
         
         ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/write_fstab.rs", r#"use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;
use std::env;

fn main() -> io::Result<()> {
    // 1. Пути
    let fstab_path = "/etc/fstab";
    let target_dir = "/var/tmp/wm";
    let line_to_add = "tmpfs  /var/tmp/wm  tmpfs size=10M  0 0";

    // 2. Добавление записи в /etc/fstab
    let mut already_exists = false;
    if let Ok(file) = fs::File::open(fstab_path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(l) = line {
                if l.contains("/var/tmp/wm") {
                    already_exists = true;
                    break;
                }
            }
        }
    }

    if !already_exists {
        match OpenOptions::new().append(true).open(fstab_path) {
            Ok(mut file) => {
                writeln!(file, "{}", line_to_add)?;
                println!("✅ Запись добавлена в {}", fstab_path);
            }
            Err(e) => {
                eprintln!("❌ Ошибка доступа к {}: {}. Запустите программу от root.", fstab_path, e);
                std::process::exit(1);
            }
        }
    } else {
        println!("ℹ️ Запись для {} уже есть в fstab.", target_dir);
    }

    // 3. Создание директории
    if !fs::metadata(target_dir).is_ok() {
        fs::create_dir_all(target_dir)?;
        println!("✅ Директория {} создана.", target_dir);
    }

    // 4. Передача прав текущему пользователю (не root)
    // Если программа запущена через sudo, переменная SUDO_USER содержит имя обычного юзера
    let user_name = env::var("SUDO_USER").unwrap_or_else(|_| {
        env::var("USER").expect("Не удалось определить пользователя")
    });

    let status = Command::new("chown")
        .arg(format!("{}:{}", user_name, user_name))
        .arg(target_dir)
        .status()?;

    if status.success() {
        println!("✅ Права на {} переданы пользователю {}", target_dir, user_name);
    } else {
        eprintln!("❌ Не удалось изменить владельца директории.");
    }

    Ok(())
}
        
        "#),
        
        ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/prev_track.rs", r#"use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;

fn main() -> std::io::Result<()> {
    // 1. Пути к файлам
    let home = env::var("HOME").expect("HOME not found");
    let tmp_path = PathBuf::from(&home).join("tmp");
    let playlist_main = tmp_path.join("playlist");   // Основной (будущее)
    let playlist_hist = tmp_path.join("playlist1");  // История (прошлое)
    let name_file = tmp_path.join("sox/name");

    // 2. Останавливаем текущую музыку
    let _ = Command::new("pkill").arg("-x").arg("play").status();

    // 3. Читаем историю
    if !playlist_hist.exists() {
        println!("История не найдена.");
        return Ok(());
    }

    let file = File::open(&playlist_hist)?;
    let reader = BufReader::new(file);
    let mut hist_lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    if hist_lines.is_empty() {
        println!("История пуста.");
        return Ok(());
    }

    // 4. ЛОГИКА: Забираем последний трек из истории
    let target_track = hist_lines.pop().unwrap(); // Извлекаем последний элемент

    // 5. Обновляем файлы
    // Перезаписываем историю (без извлеченного трека)
    fs::write(&playlist_hist, hist_lines.join("\n") + "\n")?;

    // Читаем текущий основной плейлист
    let current_main = fs::read_to_string(&playlist_main).unwrap_or_default();
    
    // Вставляем извлеченный трек в самое начало основного плейлиста
    let mut new_main = target_track.clone();
    new_main.push('\n');
    new_main.push_str(&current_main);
    fs::write(&playlist_main, new_main)?;

    // Обновляем имя для виджета
    fs::write(&name_file, &target_track)?;

    // 6. ЗАПУСК
    // Вызываем ваш fish-скрипт для старта воспроизведения
    Command::new("fish")
        .arg("-c")
        .arg("play_sox")
        .spawn()?;

    println!("Вернулись к треку: {}", target_track);
    
    // Если нужно уведомление, сколько треков осталось в истории
    if !hist_lines.is_empty() {
        println!("В истории осталось треков: {}", hist_lines.len());
    } else {
        println!("Вы дошли до начала истории.");
    }

    Ok(())
}
     
      "#), 
         
     
     
     ("gui-wm/pinnacle-gentoo/files/pinnacle-gentoo/src/wm.rs", r##"use std::fs;
use std::path::{PathBuf};
use std::env;

fn main() -> std::io::Result<()> {
    let home = env::var("HOME").map(PathBuf::from).expect("HOME не найдена");
    let _current_dir = env::current_dir()?;
    
    // 1. Настройка текстовых файлов (путь, контент)
        // 1. Настройка текстовых файлов (путь, контент)
    let text_files = [
        // ... (ваши остальные файлы: main.rs, Cargo.toml и т.д.) ...
        (".config/pinnacle/src/main.rs", r#"use std::sync::Arc;
use std::sync::Mutex;
use pinnacle_api::input;
use pinnacle_api::input::XkbConfig;
use pinnacle_api::input::Bind;
use pinnacle_api::input::Keysym;
use pinnacle_api::input::{Mod, MouseButton};
use pinnacle_api::layout;
use pinnacle_api::layout::generators::Corner;
use pinnacle_api::layout::generators::CornerLocation;
use pinnacle_api::layout::generators::Cycle;
use pinnacle_api::layout::generators::Dwindle;
use pinnacle_api::layout::generators::Fair;
use pinnacle_api::layout::generators::MasterSide;
use pinnacle_api::layout::generators::MasterStack;
use pinnacle_api::layout::generators::Spiral;
use pinnacle_api::layout::LayoutGenerator;
use pinnacle_api::layout::LayoutNode;
use pinnacle_api::output;
use pinnacle_api::pinnacle;
use pinnacle_api::pinnacle::Backend;
use pinnacle_api::process::Command;
use pinnacle_api::signal::WindowSignal;
use pinnacle_api::tag;
use pinnacle_api::util::{Axis, Batch};
use pinnacle_api::window;

async fn config() {
    // Change the mod key to `Alt` when running as a nested window.
    let mod_key = match pinnacle::backend() {
        Backend::Tty => Mod::SUPER,
        Backend::Window => Mod::ALT,
    };

    let terminal = "foot";
  
    input::set_xkb_config(XkbConfig::new()
     .with_layout("us,ru")
     .with_options("compose:ralt,grp_led:caps,grp:caps_toggle"));
    
    //------------------------
    // Mousebinds            |
    //------------------------

    // `mod_key + left click` starts moving a window
    input::mousebind(mod_key, MouseButton::Left)
        .on_press(|| {
            window::begin_move(MouseButton::Left);
        })
        .group("Mouse")
        .description("Start an interactive window move");

    // `mod_key + right click` starts resizing a window
    input::mousebind(mod_key, MouseButton::Right)
        .on_press(|| {
            window::begin_resize(MouseButton::Right);
        })
        .group("Mouse")
        .description("Start an interactive window resize");
        
      
    //------------------------
    // Keybinds              |
    //------------------------
        
    // `mod_key + s` shows the bindings overlay
    #[cfg(feature = "snowcap")]
    input::keybind(mod_key, 's')
        .on_press(|| {
            pinnacle_api::snowcap::BindOverlay::new().show();
        })
        .group("Compositor")
        .description("Show the bindings overlay");

    // `mod_key + shift + q` quits Pinnacle
    #[cfg(not(feature = "snowcap"))]
    input::keybind(mod_key | Mod::SHIFT, 'q')
        .set_as_quit()
        .group("Compositor")
        .description("Quit Pinnacle");

    #[cfg(feature = "snowcap")]
    {
        // `mod_key + shift + q` shows the quit prompt
        input::keybind(mod_key | Mod::SHIFT, 'q')
            .on_press(|| {
                pinnacle_api::snowcap::QuitPrompt::new().show();
            })
            .group("Compositor")
            .description("Show quit prompt");

        // `mod_key + ctrl + shift + q` for the hard shutdown
        input::keybind(mod_key | Mod::CTRL | Mod::SHIFT, 'q')
            .set_as_quit()
            .group("Compositor")
            .description("Quit Pinnacle without prompt");
    }

    // `mod_key + ctrl + r` reloads the config
    input::keybind(mod_key | Mod::CTRL, 'r')
        .set_as_reload_config()
        .group("Compositor")
        .description("Reload the config");

    // `mod_key + k ` closes the focused window
    input::keybind(mod_key, 'k')
        .on_press(|| {
            if let Some(window) = window::get_focused() {
                window.close();
            }
        })
        .group("Window")
        .description("Close the focused window");

    // `mod_key + ctrl + space` toggles floating
    input::keybind(mod_key | Mod::CTRL, Keysym::space)
        .on_press(|| {
            if let Some(window) = window::get_focused() {
                window.toggle_floating();
                window.raise();
            }
        })
        .group("Window")
        .description("Toggle floating on the focused window");

    // `mod_key + f` toggles fullscreen
    input::keybind(mod_key, 'f')
        .on_press(|| {
            if let Some(window) = window::get_focused() {
                window.toggle_fullscreen();
                window.raise();
            }
        })
        .group("Window")
        .description("Toggle fullscreen on the focused window");

    // `mod_key + m` toggles maximized
    input::keybind(mod_key, 'm')
        .on_press(|| {
            if let Some(window) = window::get_focused() {
                window.toggle_maximized();
                window.raise();
            }
        })
        .group("Window")
        .description("Toggle maximized on the focused window");
        
          ///////BAR/////////
     input::keybind(Mod::CTRL, Keysym::space)
        .on_press(move || {
        Command::with_shell(["lua"], "/var/tmp/wm/bar.lua").spawn();
        })
        .group("Process")
        .description("Bar bottom,left");             
     
     //          Apps
    // `mod_key + Return` spawns a terminal
    input::keybind(mod_key, Keysym::Return)
        .on_press(move || {
        Command::new(terminal).spawn();       
        })
        .group("Process")
        .description("Spawn a terminal");
        
     input::keybind(mod_key, "p")
        .on_press(move || {
        Command::with_shell(["fish", "-c"], "poweroff").spawn();
        })
        .group("Process")
        .description("Poweroff");                    
     
     input::keybind(mod_key, "r")
        .on_press(move || {
        Command::with_shell(["fish", "-c"], "reboot").spawn();
        })
        .group("Process")
        .description("Reboot");        

      input::keybind(mod_key, "l")
        .on_press(move || {
        Command::with_shell(["fish", "-c"], "swaylock -f -C ~/.config/swaylock/config").spawn();
        })
        .group("Process")
        .description("Lock pinnacle");
               
        input::keybind(mod_key, "q")
        .on_press(move || {
        Command::with_shell(["fish", "-c"], "wofi -c ~/.config/wofi/config -I").spawn();
        })
        .group("Process")
        .description("Run menu wofi"); 
//  Screenshot              
       input::keybind(mod_key, "x")
        .on_press(move || {
        Command::new("$HOME/.local/bin/pinnacle-screenshot").spawn();
        })
        .group("Process")
        .description("Screenshot");
//Translator
	  input::keybind (mod_key, "t")
        .on_press(move || {
        Command::new("pinnacle-translator").spawn();
        })
        .group("Tranalate")
        .description("Rust Translate");       
// SOX        
    input::keybind (mod_key, "n")
        .on_press(move || {
        Command::new("soxbar").spawn();
        })
        .group("Process")
        .description("Play sox");
        
    input::keybind (Mod::ALT, "s")
        .on_press(move || {
        Command::with_shell(["fish", "-c"], "pkill -f play").spawn();
        })
        .group("Process")
        .description("Stop sox");    
     
    //------------------------
    // Layouts               |
    //------------------------

    // Pinnacle supports a tree-based layout system built on layout nodes.
    //
    // To determine the tree used to layout windows, Pinnacle requests your config for a tree data structure
    // with nodes containing gaps, directions, etc. There are a few provided utilities for creating
    // a layout, known as layout generators.
    //
    // ### Layout generators ###
    // A layout generator is a table that holds some state as well as
    // the `layout` function, which takes in a window count and computes
    // a tree of layout nodes that determines how windows are laid out.
    //
    // There are currently six built-in layout generators, one of which delegates to other
    // generators as shown below.

    fn into_box<'a, T: LayoutGenerator + Send + 'a>(
        generator: T,
    ) -> Box<dyn LayoutGenerator + Send + 'a> {
        Box::new(generator) as _
    }

    // Create a cycling layout generator that can cycle between layouts on different tags.
    let cycler = Arc::new(Mutex::new(Cycle::new([
        into_box(MasterStack::default()),
        into_box(MasterStack {
            master_side: MasterSide::Right,
            ..Default::default()
        }),
        into_box(MasterStack {
            master_side: MasterSide::Top,
            ..Default::default()
        }),
        into_box(MasterStack {
            master_side: MasterSide::Bottom,
            ..Default::default()
        }),
        into_box(Dwindle::default()),
        into_box(Spiral::default()),
        into_box(Corner::default()),
        into_box(Corner {
            corner_loc: CornerLocation::TopRight,
            ..Default::default()
        }),
        into_box(Corner {
            corner_loc: CornerLocation::BottomLeft,
            ..Default::default()
        }),
        into_box(Corner {
            corner_loc: CornerLocation::BottomRight,
            ..Default::default()
        }),
        into_box(Fair::default()),
        into_box(Fair {
            axis: Axis::Horizontal,
            ..Default::default()
        }),
    ])));

    // Use the cycling layout generator to manage layout requests.
    // This returns a layout requester that allows you to request layouts manually.
    let layout_requester = layout::manage({
        let cycler = cycler.clone();
        move |args| {
            let Some(tag) = args.tags.first() else {
                return LayoutNode::new();
            };
            let mut generator = cycler.lock().unwrap();
            generator.set_current_tag(tag.clone());
            generator.layout(args.window_count)
        }
    });

    // `mod_key + space` cycles to the next layout
    input::keybind(mod_key, Keysym::space)
        .on_press({
            let cycler = cycler.clone();
            let requester = layout_requester.clone();
            move || {
                let Some(focused_op) = output::get_focused() else {
                    return;
                };
                let Some(first_active_tag) = focused_op
                    .tags()
                    .batch_find(|tag| Box::pin(tag.active_async()), |active| *active)
                else {
                    return;
                };

                cycler
                    .lock()
                    .unwrap()
                    .cycle_layout_forward(&first_active_tag);
                requester.request_layout_on_output(&focused_op);
            }
        })
        .group("Layout")
        .description("Cycle the layout forward");

    // `mod_key + shift + space` cycles to the previous layout
    input::keybind(mod_key | Mod::SHIFT, Keysym::space)
        .on_press(move || {
            let Some(focused_op) = output::get_focused() else {
                return;
            };
            let Some(first_active_tag) = focused_op
                .tags()
                .batch_find(|tag| Box::pin(tag.active_async()), |active| *active)
            else {
                return;
            };

            cycler
                .lock()
                .unwrap()
                .cycle_layout_backward(&first_active_tag);
            layout_requester.request_layout_on_output(&focused_op);
        })
        .group("Layout")
        .description("Cycle the layout backward");
   //------------------------
   //  Window rules 
   //------------------------
   
   window::add_window_rule(|window| {
    if window.app_id() == "foot" {
        window.set_floating(false)
    }
    });
   window::add_window_rule(|window| {
    if window.title() == "Действия над файлами" {
        window.set_floating(true)
    }
    });
    window::add_window_rule(|window| {
    if window.title() == "Переименовать..." {
        window.set_floating(true)
    }
    });


//----------------------------------
// Tags & Keybinds & Wallpaper     |
//----------------------------------

// --- Обои по номеру tag_name ---
fn sync_wallpaper() {
    let status_file = "/var/tmp/wm/tag";
    let wp_dir = "/var/tmp/wm/pictures";

    if let Ok(content) = std::fs::read_to_string(status_file) {
        let tag_num: String = content
            .chars()
            .filter_map(|c| match c {
                '𝟏' | '1' => Some('1'),
                '𝟐' | '2' => Some('2'),
                '𝟑' | '3' => Some('3'),
                '𝟒' | '4' => Some('4'),
                '𝟓' | '5' => Some('5'),
                '𝟔' | '6' => Some('6'),
                _ => None,
            })
            .take(1)
            .collect();

        if !tag_num.is_empty() {
            let wp_path = format!("{}/tag{}.jpg", wp_dir, tag_num);
            if std::path::Path::new(&wp_path).exists() {
                let _ = wallpaper::set_from_path(&wp_path);
            }
        }
    }
}

let tag_names = ["1", "2", "3", "4", "5", "6"];

output::for_each_output(move |output| {
    let mut tags = tag::add(output, tag_names);
    tags.next().unwrap().set_active(true);
});

for name in tag_names {
    let n = name.to_string();

    input::keybind(mod_key, name)
        .on_press({
            let n = n.clone();
            move || {
                if let Some(tag) = tag::get(&n) {
                    tag.switch_to();
                }

                let display = match n.as_str() {
                    "1" => " 𝟏 ", "2" => " 𝟐 ", "3" => " 𝟑 ",
                    "4" => " 𝟒 ", "5" => " 𝟓 ", "6" => " 𝟔 ",
                    _ => &n,
                };
                let _ = std::fs::write("/var/tmp/wm/tag", display);
                
                // ВСТАВЛЕНО: Синхронизируем обои после записи в файл
                sync_wallpaper();
            }
        })
        .group("Tag")
        .description(format!("Switch to tag {}", name));

    // Mod + Ctrl (Toggle) - Обои обычно не меняем при простом скрытии тега
    input::keybind(mod_key | Mod::CTRL, name)
        .on_press({
            let n = n.clone();
            move || {
                if let Some(tag) = tag::get(&n) {
                    tag.toggle_active();
                }
            }
        })
        .group("Tag")
        .description(format!("Toggle tag {}", name));

    // Mod + Shift (Move window)
    input::keybind(mod_key | Mod::SHIFT, name)
        .on_press({
            let n = n.clone();
            move || {
                if let (Some(tag), Some(win)) = (tag::get(&n), window::get_focused()) {
                    win.move_to_tag(&tag);
                }
            }
        })
        .group("Tag")
        .description(format!("Move window to tag {}", name));
}

// СПЕЦИАЛЬНЫЕ БИНДЫ ДЛЯ ПРИЛОЖЕНИЙ
let apps = [
    (Mod::CTRL, "f", "thunar", "1", " 𝟏 "),
    (Mod::CTRL, "w", "vivaldi", "2", " 𝟐 "),
    (Mod::CTRL, "o", "libreoffice-bin", "4", " 𝟒 "),
    (mod_key, "b", "soxbar", "6", " 𝟔 "),
];

for (m, key, cmd, t_name, display) in apps {
    input::keybind(m, key)
        .on_press(move || {
            let _ = std::process::Command::new(cmd).spawn();
            let _ = std::fs::write("/var/tmp/wm/tag", display);
            if let Some(tag) = tag::get(t_name) {
                tag.switch_to();
            }
            
            // ВСТАВЛЕНО: Синхронизируем обои при запуске приложения
            sync_wallpaper();
        })
        .group("Apps")
        .description(format!("Launch {} on tag {}", cmd, t_name));
}



    input::libinput::for_each_device(|device| {
        // Enable natural scroll for touchpads
        if device.device_type().is_touchpad() {
            device.set_natural_scroll(true);
        }
    });

    // There are no server-side decorations yet, so request all clients use client-side decorations.
    window::add_window_rule(|window| {
        window.set_decoration_mode(window::DecorationMode::ClientSide);
    });

    // Enable sloppy focus
    window::connect_signal(WindowSignal::PointerEnter(Box::new(|win| {
        win.set_focused(true);
    })));

//    #[cfg(feature = "snowcap")]
//    if let Some(error) = pinnacle_api::pinnacle::take_last_error() {
        // Show previous crash messages
//        pinnacle_api::snowcap::ConfigCrashedMessage::new().show(error);
//    } else {
        // Or show the bind overlay on startup
//        pinnacle_api::snowcap::BindOverlay::new().show();
//    }

    Command::with_shell(["fish", "-c"],  "wlr-randr --output HDMI-A-1 --mode  1920x1080px 60.000000Hz").once().spawn();    
    Command::new("start_wallpaper").once().spawn();
    Command::with_shell(["fish", "-c"], "dbus-daemon --session --address=unix:path=$XDG_RUNTIME_DIR/bus &").once().spawn();
    Command::new("bottom_bar").once().spawn();
    Command::new("nm-applet").once().spawn();
    Command::new("mako").once().spawn();
}

pinnacle_api::main!(config); "#),

        
         
        (".config/pinnacle/Cargo.toml", r#"[package]
name = "pinnacle-config"
version = "0.1.0"
edition = "2021"

[dependencies]
pinnacle-api = { git = "http://github.com/pinnacle-comp/pinnacle", default-features = false, tag = "v0.1.0" }
wallpaper = "3.2"

[features]
default = ["snowcap"]
snowcap = ["pinnacle-api/snowcap"] "#),

        (".config/pinnacle/pinnacle.toml", r#"# This pinnacle.toml file dictates what config Pinnacle will run.
#
# When running Pinnacle, the compositor will look in the following directories for a pinnacle.toml file,
# in order from top to bottom:
#     $PINNACLE_CONFIG_DIR
#     $XDG_CONFIG_HOME/pinnacle/
#     ~/.config/pinnacle/
#
# When Pinnacle finds a pinnacle.toml file, it will execute the command provided to `run`.
# To use a Rust config, this should be changed to something like ["cargo", "run"].

# The command Pinnacle will run on startup and when you reload your config.
# Paths are relative to the directory this file is in.
# This must be an array.
run = ["cargo", "run"]

### Socket directory ###
# Pinnacle will open a Unix socket at `$XDG_RUNTIME_DIR` by default, falling back to `/tmp` if it doesn't exist.
# If you want/need to change this, use the `socket_dir` setting set to the directory of your choosing.
#
# socket_dir = "/your/dir/here/"

### Environment Variables ###
# If you need to spawn your config with any environment variables, list them here.
[envs]
# key = "value"

### No xwayland ###
# If you want to stop xwayland from starting, uncomment the field below.
# 
# no_xwayland = true

### No config ###
# If you want to stop a config from starting, uncomment the field below.
# 
# no_config = true; "#),
 
        (".config/ironbar/config.corn", r#"let {   
   $focused = {name = "foco" type = "focused"  show_icon = true icon_size = 22 truncate.mode = "end" truncate.max_length = 20 }
   $launcher = {name = "launcher" type = "launcher" show_titles =true show_icon = true icon_size = 22 truncate.mode = "end" truncate.max_length = 20 }
   $clock = {type = "clock" format = "%a, %d %b %H:%M" }
   $battery = {type = "upower" icon_size = 14 format = "{percentage}%"} 
   $memgraph = {class = "cpu-menu" name="cpu-menu" type = "custom" bar =[ { type = "box" orientation = "vertical" widgets = [
   { type = "button" class="power-btn" label = "{{2000:memory_oval}}" tooltip = "{{echo 'MEMORY'}}" orientation = "horizontal"}]}]}
   $cpugraph = {class = "cpu-menu" name="cpu-menu" type = "custom" bar =[ { type = "box" orientation = "vertical" widgets = [
   { type = "button" class="power-btn" label = "{{2000:cpu_oval}}" tooltip = "{{echo 'CPU'}}" orientation = "horizontal"}]}]}
   $cpu_tempgraph = {class = "cpu-menu" name="cpu-menu" type = "custom" bar =[ { type = "box" orientation = "vertical" widgets = [
   { type = "button" class="power-btn" label = "{{2000:cpu_temp_oval}}" tooltip = "{{echo 'TEMPERATURE'}}" orientation = "horizontal"}]}]}
   $netgraph = {class = "cpu-menu" name="cpu-menu" type = "custom" bar =[ { type = "box" orientation = "vertical" widgets = [
   { type = "button" class="power-btn" label = "{{2000:netgraph}}" tooltip = "{{echo 'Rx   Tx'}}" orientation = "horizontal"}]}]}
   $power ={ class = "power-menu" name="power-menu" type = "custom"
        bar = [ { label = "exit" type = "button"  widgets = [{src="icon:system-shutdown" type="image" size=16}] on_click = "popup:toggle" }]    
        popup =[ { name="popup-box" type = "box" orientation = "horizontal"    
           widgets = [
               { type = "button" class="power-btn" label = "" widgets = [{src="icon:system-shutdown" type="image" size=48}] on_click = "!fish -c poweroff" }
               { type = "button" class="power-btn" label = "" widgets = [{src="icon:system-reboot" type="image" size=48}] on_click = "!fish -c reboot" }
               { type = "button" class="power-btn" label = "" widgets = [{src="icon:system-lock-screen" type="image" size=48}] on_click = "!fish -c swaylock -f -C ~/.config/swaylock/config" } ] } ] }

   $sox ={ class = "sox-menu" name="sox-menu" type = "custom" bar = [ { label = "play" type = "button" widgets = [{src="icon:multimedia-audio-player" type="image" size=16}] on_click = "popup:toggle" }]   
      popup =[{ name="popup-box" type = "box" orientation = "horizontal"  widgets = [
   { type = "button" class="play-btn" label = "" widgets = [{type = "script" name = "sox" mode = "poll" cmd = "echo '📂'"}] on_click = "!setup_playlist" }
   { type = "button" class="play-btn" label = "" widgets = [{type = "script" name = "sox" mode = "poll" cmd = "echo '📄'"}] on_click = "!list_sox" }
   { type = "button" class="play-btn" label = "" widgets = [{type = "script" name = "sox" mode = "poll" cmd = "echo '⏪'"}] on_click = "!prev_track" } 
   { type = "button" class="play-btn" label = "" widgets = [{type = "script" name = "sox" mode = "poll" cmd = "echo '⏹️'"}] on_click = "!stop_sox" } 
   { type = "button" class="play-btn" label = "" widgets = [{type = "script" name = "sox" mode = "poll" cmd = "echo '▶️'"}] on_click = "!play_sox" }
   { type = "button" class="play-btn" label = "" widgets = [{type = "script" name = "sox" mode = "poll" cmd = "echo '⏩'"}] on_click = "!play_next" }]}]}
   $tray = { type = "tray" direction = "h" icon_size = 14 }
   $keymaps = {type = "script" name = "kbd" mode = "poll" interval = 3000 cmd = "kbd-rs" on_click = "!sudo /usr/local/bin/helper_kbd"}
   $network = {type = "network_manager" icon_size = 14}   
   $tags = {class = "tags-menu" name="power-menu" type = "custom"  bar = [{label = "tags" type = "button" widgets = [{ type = "script" name = "tagactive" mode = "poll" interval = 1000 cmd = "tagactive" }]
   widgets = [{ type = "script" name = "tagactive" mode = "poll" interval = 1000 cmd = "tagactive" }]
   on_click = "popup:toggle"}] popup =[{name="popup-box" type = "box" orientation = "vertical"  widgets = [
   { type = "button" class="tags-btn" label = "" widgets = [{type = "script" name = "tagactive" mode = "poll" cmd = "echo '1️⃣  fm'"}] on_click = "!1_tag" }
   { type = "button" class="tags-btn" label = "" widgets = [{type = "script" name = "tagactive" mode = "poll" cmd = "echo '2️⃣ browser'"}] on_click = "!2_tag" }
   { type = "button" class="tags-btn" label = "" widgets = [{type = "script" name = "tagactive" mode = "poll" cmd = "echo '3️⃣ work'"}] on_click = "!3_tag" }
   { type = "button" class="tags-btn" label = "" widgets = [{type = "script" name = "tagactive" mode = "poll" cmd = "echo '4️⃣ editor'"}] on_click = "!4_tag" }
   { type = "button" class="tags-btn" label = "" widgets = [{type = "script" name = "tagactive" mode = "poll" cmd = "echo '5️⃣ ssh'"}] on_click = "!5_tag" }
   { type = "button" class="tags-btn" label = "" widgets = [{type = "script" name = "tagactive" mode = "poll" cmd = "echo '6️⃣ media'"}] on_click = "!6_tag" }]}]}

   $screenshot ={class = "screenshot" name="screenshot" type = "custom" bar =[{type = "box" orientatron = "horizontal" widgets = [
           { type = "button" class="power-btn" label = "" widgets = [{src="icon:applets-screenshooter" type="image" size=16}] on_click = "!pinnacle-screenshot --mode region" }]}]}
   $menu ={class = "wofi-menu" name="wofi-menu" type = "custom" bar =[{type = "box" orientatron = "horizontal" widgets = [
           { type = "button" class="menu-btn" label = "" widgets = [{src="icon:distributor-logo-gentoo" type="image" size=16}] on_click = "!wofi -c ~/.config/wofi/config -I"  on_click_right = "killall wofi"}]}]}
   $volume = {class = "cpu-menu" name="cpu-menu" type = "custom" bar =[ { type = "box" orientation = "vertical" widgets = [
   { type = "button" class="power-btn" label = "{{1000:vol_widget}}" tooltip = "{{1000:vol_info}}" orientation = "horizontal"
   on_scroll_down = "amixer sset PCM 5%-" on_scroll_up = "amixer sset PCM 5%+" on_click_left = "amixer sset PCM 0" on_click_right = "amixer sset PCM 80"}]}]}                           
   $left = [ $menu $tags $launcher $focused ]        
   $center = [ $clock ]        
   $right = [ $tray $volume $screenshot  $sox $power $keymaps $netgraph $cpu_tempgraph $cpugraph $memgraph]}

   in {
   monitors.HDMI-A-1 = { position = "bottom" height = 30 icon_theme = "BeautyLine" start = $left center = $center end = $right } }
"#),
     (".config/ironbar/srcery_css.css", r#"@define-color srcery-black #1C1B19;
@define-color srcery-red #EF2F27;
@define-color srcery-green #519F50;
@define-color srcery-yellow #FBB829;
@define-color srcery-blue #2C78BF;
@define-color srcery-magenta #E02C6D;
@define-color srcery-cyan #0AAEB3;
@define-color srcery-white #BAA67F;
@define-color srcery-bright-black #918175;
@define-color srcery-bright-red #F75341;
@define-color srcery-bright-green #98BC37;
@define-color srcery-bright-yellow #FED06E;
@define-color srcery-bright-blue #68A8E4;
@define-color srcery-bright-magenta #FF5C8F;
@define-color srcery-bright-cyan #2BE4D0;
@define-color srcery-bright-white #FCE8C3;
@define-color srcery-orange #FF5F00;
@define-color srcery-bright-orange #FF8700;
@define-color srcery-teal #008080;
@define-color srcery-hard-black #121212;
@define-color srcery-xgray1 #262626;
@define-color srcery-xgray2 #303030;
@define-color srcery-xgray3 #3A3A3A;
@define-color srcery-xgray4 #444444;
@define-color srcery-xgray5 #4E4E4E;
@define-color srcery-xgray6 #585858;
@define-color srcery-xgray7 #626262;
@define-color srcery-xgray8 #6C6C6C;
@define-color srcery-xgray9 #767676;
@define-color srcery-xgray10 #808080;
@define-color srcery-xgray11 #8A8A8A;
@define-color srcery-xgray12 #949494; "#),  
    

     (".config/ironbar/style.css", r#"@import 'srcery_css.css';

* {
    font-family: MesloLGSDZ Nerd Font;         
    font-size: 14px;
    box-shadow: none;
    border: none;
    border-radius: 0px;
    border-image: none;
    background-image: none;
    outline: none;
    text-shadow: none;
    margin: 0;
    padding: 0;
}

button {
    box-shadow: none;
    border: none;
    background: none;
    outline: none;
    text-shadow: none;
}

.background {
    background-color: rgba(0,0,0,0);
}

scale slider {
  margin: -4px;
  min-width: 12px;
  min-height: 12px;
}

@keyframes blinker {
  50% {
    color: @srcery-red;
  }
}

#bar {
    /* background-color: @srcery-black; */
    /* box-shadow: inset 0 4 0 0 @srcery-xgray2; */
    min-height: 1.93em;
}

/* base */
.tray,
.notifications,
.button,
.widget,
.clock,
.menu,
.upower,
.system,
#script
#popup,
.keymaps,
#exit,
#net,
#memory,
#cpu,
#cpu_temp,
.tags
.foco,
#wifi,
.volume

#system {
    margin-left: 4px;
    transition: all 0.5s;
    transition-timing-function: ease;
}

.volume {
    padding: 0px 10px;
    margin-right: 1px;
    margin-left: 10px;
    border-radius: 10px;
    color: #D8D8AA;
    background: #1a1b26;
    opacity: 0.8;
}

.popup-volume {
    padding: 20px 20px 20px 20px;
    border-radius: 5px;
    color: #c0caf5;
    background-color: rgba(26, 27, 38, 0.8);
    border: 2px solid #bb9af7;
}

/*  -------- MUSIC ---------  */
.tags{
    background-color: @srcery-black;
    border-radius: 0 0 .4em .4em;
    padding-left: 4px;
    padding-right:4px;
    padding-top: 5px;
}

.tags:hover{
    color: @srcery-black;
}

.popup-music {
    background-color: @srcery-black;
    box-shadow: inset 0px 4px 0px 0px @srcery-cyan;
    margin-top: 3px;
    padding: 20px 20px 20px 20px;
    border-radius: 0 0 .5em .5em;
}

.popup-music .title .icon-box .text-icon,
.popup-music .artist .icon-box .text-icon,
.popup-music .volume .icon,
.popup-music .album .icon-box .text-icon {
    font-family: JetBrainsMono NFP;
    color: @srcery-cyan;
} 

.popup-music .progress scale trough,
.popup-music .volume scale trough {
    background-color: @srcery-bright-black;
}

.popup-music .progress scale trough highlight,
.popup-music .volume scale trough highlight{
    background-color: @srcery-cyan;
}

/*  -------- FOCUSED ---------  */
.foco .label{
    background-color: @srcery-black;
    box-shadow: inset 0px 4px 0px 0px @srcery-blue;
    padding-left: 4px;
    padding-right:4px;
    padding-top: 5px;
    border-radius: 0 0 .4em .4em;
}

/*  -------- NETWORK ---------  */
.network_manager {
    background-color: @srcery-black;
    box-shadow: inset 0px 4px 0px 0px @srcery-magenta;
    padding-left: 4px;
    padding-right:4px;
    padding-top: 5px;
    border-radius: 0 0 .4em .4em;
}

/*  -------- VOLUME ---------  */
#som {
    background-color: @srcery-black;
    border-radius: 0 0 .4em .4em;
    box-shadow: inset 0px 4px 0px 0px @srcery-red;
    padding-left: 4px;
    padding-right:4px;
    padding-top: 5px;
}

#som:hover {
    color: @srcery-black;
    box-shadow: inset 0px 32px 0px 0px @srcery-red;
}

/*  -------- BATTERY ---------  */
.upower {
    background-color: @srcery-black;
    border-radius: 0 0 .4em .4em;
    box-shadow: inset 0px 4px 0px 0px @srcery-orange;
    padding-left: 4px;
    padding-right:4px;
    padding-top: 5px;
}

.upower:hover {
    color: @srcery-black;
    box-shadow: inset 0px 32px 0px 0px @srcery-orange;
}

#cpu, #memory, #net, #cpu_temp {
    padding-bottom: 5px;
    min-width: 5em;
    font-family: IosevkaTermNerdFontPropo;
    font-style: italic;
    font-weight: 500;
    font-size: 12pt;
    text-shadow: @srcery-hard-black 4px 1px 2px;
    color: @srcery-white;
}

#keymaps {
    padding-bottom: 5px;
    min-width: 5em;
    margin-right: 1px;
    margin-left: 15px;
    color: @srcery-white;
}

/*  -------- NOTIFICATIONS ---------  */
.notifications button  {
    margin-left: 4px;
    font-family: IosevkaTermNerdFontMono;
    background-color: @srcery-black;
    padding-top: 5px;
    box-shadow: inset 0px 4px 0px 0px @srcery-blue;
    border-radius: 0 0 .4em .4em;
}

.notifications .count {
    font-family: IosevkaTermNerdFontMono;
    font-size: .6rem;
    background-color: @srcery-red;
    color: @srcery-bright-white;
    border-radius: 100%;
}

/*  -------- CLOCK ---------  */
.clock {
    padding-left:4px;
    padding-right:4px;
    padding-top: 5px;
    border-radius: .4em;
    font-family: IosevkaTermNerdFontPropo;
    font-style: italic;
    font-weight: 500;
    font-size: 12pt;
    color: @srcery-white;
}

.clock:hover {
    box-shadow: inset 0px 35px 0px 0px @srcery-white;
    color: @srcery-black;
}

.popup-clock .calendar-clock {
    color: @srcery-white;
    font-size: 2.2em;
    margin-top: 3px;
    margin-bottom: 7px;
    padding: 0px 2em;
    border-radius: 7px;
    background-color: @srcery-black;
}

.popup-clock .calendar {
    background-color: @srcery-black;
    color: @srcery-bright-white;
    border-radius: .4em;
}

/*  -------- TRAY ---------  */
.tray {
    padding-top: 5px;
}

.tray .item {
    padding-left:4px;
    padding-right:4px;
}

/*  -------- TAGS ---------  */
.tags {
    padding-left:4px;
    padding-right:4px;
    padding-top: 5px;
    border-radius: 0 0 .4em .4em;
    background-color: @srcery-black;
    box-shadow: inset 0px 4px 0px 0px @srcery-white;
}

.tags:hover {
    box-shadow: inset 0px 35px 0px 0px @srcery-white;
    color: @srcery-black;
}     "#), ];  


    for (rel_path, content) in text_files {
        let full_path = home.join(rel_path.trim()); // trim() убирает случайные \n в путях
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, content)?;
    }
        
    // 2. Перенос обоев 
/*    let src_dir = current_dir.join("pinnacle-gentoo/pictures"); 
    let dest_dir = home.join(".config/pinnacle/pictures");

    if src_dir.exists() {
        fs::create_dir_all(&dest_dir)?;

        for entry in fs::read_dir(src_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let file_name = path.file_name().unwrap();
                let dest_path = dest_dir.join(file_name);
                fs::copy(&path, &dest_path)?;
            }
        }
        println!("Изображения успешно скопированы в {:?}", dest_dir);
    } else {
        println!("Предупреждение: Исходная папка {:?} не найдена. Проверьте путь.", src_dir);
    }
*/
    println!("Настройка структуры завершена.");
    Ok(())
}
  
       
       "##),  
  ];
  

    for (rel_path, content) in text_files {
        let full_path = base_path.join(rel_path.trim());
        if let Some(parent) = full_path.parent() {
            // Создаем директории, если их нет
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, content)?;
    }
        
    println!("Структура оверлея успешно создана.");
    Ok(())
}
