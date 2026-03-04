use std::fs;
use std::path::Path; 

fn main() -> std::io::Result<()> {
    let base_path = Path::new("/var/db/repos/tupoll-overlay/");   
    let text_files = [
   ("gui-apps/pinnacle-wallpaper/pinnacle-wallpaper-9999.ebuild", r#"# Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

inherit cargo desktop

DESCRIPTION="Wallpaper-daemon for pinnacle-wm"
HOMEPAGE="https://github.com"
LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64"

SRC_URI=""
RESTRICT="fetch"

S="${WORKDIR}/${P}/pinnacle-wallpaper"

RDEPEND="    
	gui-wm/pinnacle-gentoo	
"
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_unpack() {
    mkdir -p "${WORKDIR}/${P}" || die
    cp -Rp "${FILESDIR}/pinnacle-wallpaper" "${WORKDIR}/${P}/" || die
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
	exeinto /usr/sbin
    doexe "WayWall"
     exeinto /usr/sbin
    doexe "randpaper.py"	
}  "#), 
      ("gui-apps/pinnacle-wallpaper/files/pinnacle-wallpaper/Cargo.toml", r#"[package]
name = "pinnacle-wallpaper"
version = "0.1.0"
edition = "2024"

[dependencies]
dirs = "5.0"  "#),
  
    ("gui-apps/pinnacle-wallpaper/files/pinnacle-wallpaper/randpaper.py", r#"#!/usr/bin/python

'''randpaper is a script to download photos with a specified keyword from Pexels
website (https://www.pexels.com/). To use it you must obtain an API KEY from
Pexels (https://www.pexels.com/api/). Put it into a file named as "api.key" near
randpaper.py script or insert below using an API_KEY variable.'''

import requests
import json
import secrets
import glob
import os
import shutil
import sys
from optparse import OptionParser

# PARAMS
with open(os.path.join(sys.path[0], 'api.key')) as f:
    API_KEY = f.read().strip()
# or comment the above lines and uncomment the following one and specify your key
# API_KEY = ''

min_width = 2560
min_height = 1440

keywords = ['landscape', 'city', 'nature', 'mountains', 'sea', 'ocean', 'pattern',
            'night', 'summer', 'winter', 'travel', 'beach', 'abstract', 'universe',
            'snow', 'road', 'river', 'sky', 'blur', 'stars', 'streets', 'sunset',
            'forest', 'rain', 'light', 'abstract', 'macro', 'art', 'design']


parser = OptionParser(usage="Usage: %prog [options]")
parser.add_option("-a", dest="popular", action='store_true', help="search within popular photos only (optional)", default=False)
parser.add_option("-k", dest="keyword", type='string', help="keyword (default - choose one from a predefined list)", default=None)
parser.add_option("-n", dest="number", type='int', help="number of photos to download (default = 1)", default=1)
parser.add_option("-p", dest="path", type='string', help="path to wallpaper dir", default=None)

options, args = parser.parse_args()

if not options.path:
    parser.error('No path supplied')
    sys.exit(1)

keyword = keywords[secrets.choice(range(0, len(keywords)))] if not options.keyword else options.keyword
path = options.path
path = os.path.join(path, '')
popular = options.popular
photos_num = options.number
photos_local = glob.glob1(path, '*.*')

def download_photo(url, path):
    filename = url[url.rfind('/') + 1:]
    r = requests.get(url, stream=True)

    if r.status_code == 200:
        with open(path + filename, 'wb') as f:
            r.raw.decode_content = True
            shutil.copyfileobj(r.raw, f)
        return filename
    return False

def find_pic(path, keyword, popular, photos_num):

    # Basic url
    if popular:
        url = 'https://api.pexels.com/v1/popular?per_page=40&page='
    else:
        url = 'https://api.pexels.com/v1/search?query=' + keyword + '&per_page=40&page='

    # Preloading first page
    page_first = url + '1'
    headers = {'Authorization': API_KEY}
    photos = []

    try:
        pre = requests.get(page_first, headers=headers)
        response = json.loads(pre.content)
        # Calculating overall page number
        if not popular:
            pages = int(response['total_results'] / 40) + 2

        # Iterating over 5 random pages until a photo with specified dimensions is found
        for _ in range(5):
            if popular:
                page_next = response['next_page']
            else:
                page_next = url + str(secrets.choice(range(1, pages)))

            # Loading and parsing a page
            result = requests.get(page_next, headers=headers)
            response = json.loads(result.content)
            page_photos_num = len(response['photos'])

            # Iterating over all photos on page
            for _ in range(page_photos_num):

                p = response['photos'][secrets.choice(range(0, page_photos_num))]

                # If a photo doesn't match specified dimensions, continue iterating
                if p['width'] < min_width or p['height'] < min_height:
                    continue

                # Photo is found, download it and return filename
                photo_url = p['src']['original']
                photo_name = photo_url[photo_url.rfind('/') + 1:]

                if photo_name not in photos_local:
                    filename = download_photo(photo_url, path)
                    if filename:
                        photos.append(os.path.join(path, filename))

                    if len(photos) == photos_num:
                        return photos

        return False
    except:
        raise Exception("Error occurred during photo download")

photos = find_pic(path, keyword, popular, photos_num)

if photos:
    print(' '.join(photos))
elif photos_local:
    photos = []
    while len(photos) < photos_num:
        random_index = secrets.choice(range(1, len(photos_local)))
        photos.append(os.path.join(path, photos_local[random_index]))
    print(' '.join(photos))
  "#),
  ("gui-apps/pinnacle-wallpaper/files/pinnacle-wallpaper/WayWall", r#"#!/usr/bin/env python

import gi
import os
import unicodedata

# Сначала СТРОГО объявляем версии
gi.require_version('Gtk', '3.0')
gi.require_version('GtkLayerShell', '0.1')

# Только ПОТОМ импортируем из репозитория
from gi.repository import Gtk, GtkLayerShell, GdkPixbuf, GLib, Gio

# ... остальной код класса WayWall ...


class WayWall(Gtk.Window):
    def __init__(self):
        super().__init__(type=Gtk.WindowType.TOPLEVEL)
        
        # 1. Настройка слоя (Pinnacle/Smithay native)
        GtkLayerShell.init_for_window(self)
        GtkLayerShell.set_layer(self, GtkLayerShell.Layer.BACKGROUND)
        
        # Растягиваем на весь экран (скромные 1920x1080)
        for edge in [GtkLayerShell.Edge.TOP, GtkLayerShell.Edge.BOTTOM, 
                     GtkLayerShell.Edge.LEFT, GtkLayerShell.Edge.RIGHT]:
            GtkLayerShell.set_anchor(self, edge, True)
            
        GtkLayerShell.set_exclusive_zone(self, -1)

        self.tag_file = "/var/tmp/wm/tag"
        self.pics_dir = "/var/tmp/wm/pictures"
        
        self.image_widget = Gtk.Image()
        self.add(self.image_widget)
        
        # 2. Мониторинг файла тега (Inotify через Gio)
        self.setup_monitor()
        
        # 3. Первый запуск
        self.refresh_wallpaper()
        self.show_all()
        print("🚀 WayWall запущен: мониторинг /var/tmp/wm/tag активен")

    def setup_monitor(self):
        # Если файла нет, Gio дождется его появления
        file_obj = Gio.File.new_for_path(self.tag_file)
        self.monitor = file_obj.monitor_file(Gio.FileMonitorFlags.NONE, None)
        self.monitor.connect("changed", self.on_tag_changed)

    def on_tag_changed(self, monitor, file, other_file, event_type):
        # Реагируем на запись файла
        if event_type in [Gio.FileMonitorEvent.CHANGES_DONE_HINT, Gio.FileMonitorEvent.CREATED]:
            self.refresh_wallpaper()

    def refresh_wallpaper(self):
        try:
            if not os.path.exists(self.tag_file):
                return

            with open(self.tag_file, 'r', encoding='utf-8') as f:
                content = f.read().strip()
            
            if not content:
                return

            # ХАК для Unicode (𝟏 𝟐 𝟑 -> 1 2 3)
            normalized = unicodedata.normalize('NFKC', content)
            tag = "".join(filter(str.isdigit, normalized))
            
            if not tag:
                return

            # Строгая логика: tag1.jpg, tag2.jpg...
            path = os.path.join(self.pics_dir, f"tag{tag}.jpg")
            
            if os.path.exists(path):
                # Качественный скейлинг под 1920x1080
                pixbuf = GdkPixbuf.Pixbuf.new_from_file_at_scale(path, 1920, 1080, True)
                GLib.idle_add(self.image_widget.set_from_pixbuf, pixbuf)
                print(f"✅ Установлен фон: {path}")
            else:
                print(f"⚠️ Файл не найден: {path}")

        except Exception as e:
            print(f"❌ Ошибка: {e}")

if __name__ == "__main__":
    # Убеждаемся, что папки на месте
    os.makedirs("/var/tmp/wm/pictures", exist_ok=True)
    
    app = WayWall()
    Gtk.main()
  "#),  
       ("gui-apps/pinnacle-wallpaper/files/pinnacle-wallpaper/src/main.rs", r#"use std::process::Command;
use std::fs;
use std::env;
use std::path::PathBuf;

fn main() {
   
    let _ = Command::new("pkill").args(["-f", "WayWall"]).status();
    println!("🚀 Запускаю WayWall...");
    
    let script_path = format!("/usr/sbin/WayWall");
    let status = Command::new("python")
        .arg(script_path)
        .spawn();

    match status {
        Ok(_) => println!("✔ Демон запущен!"),
        Err(e) => eprintln!("❌ Не удалось запустить питон: {}", e),
    }
}
"#), ];
      
 for (rel_path, content) in text_files {
        let full_path = base_path.join(rel_path.trim());
        if let Some(parent) = full_path.parent() {
            
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, content)?;
    }        
    println!("Структура giu-apps/pinnacle-wallpaper успешно создана ✔️");
    Ok(())
}
