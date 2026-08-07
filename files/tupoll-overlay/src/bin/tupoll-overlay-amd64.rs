use std::fs;
use std::path::PathBuf;
use std::path::Path;
use fs::OpenOptions;
use std::io::BufReader;
use std::io::Write;
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let home_base = PathBuf::from("/var/db/repos/tupoll-overlay");
    let text_files = [   
        ("profiles/default/linux/amd64/23.0/desktop/wayland/wayland/eapi", r#"8"#),     
        ("media-video/soxbar/soxbar-9999.ebuild", r#"EAPI=8
inherit cargo git-r3 desktop

DESCRIPTION="Sox Bar - Sox Control Center 2026"
HOMEPAGE="https://github.com"
# Здесь ТОЛЬКО ссылка на GitHub
EGIT_REPO_URI="https://github.com/tupoll/soxbar.git"

LICENSE="BSD-2"
SLOT="0"
KEYWORDS=""
PROPERTIES="live"

# Зависимости для GTK4 и VTE
RDEPEND="
	gui-libs/gtk:4
	gui-libs/vte
	media-sound/sox	
"
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

# Фаза распаковки: скачиваем из GitHub, затем скачиваем зависимости Rust
src_unpack() {
	git-r3_src_unpack
	
	# Для live-ebuild 2026 года используем автоматическое скачивание крейтов
	# Это избавляет от необходимости прописывать сотни CRATES вручную
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
	
	# Установка дополнительных файлов из репозитория
	domenu "Sox Control Center 2026.desktop"
}   "#),
        ("metadata/layout.conf", r#"thin-manifests = true
sign-manifests = false
profile-formats = portage-2
cache-formats = md5-dict
masters = gentoo"#),
       ("profiles/targets/amd64/wayland/make.defaults", r#"# These settings were set by the catalyst build script that automatically
# built this stage.
#COMMON_FLAGS="-march=core2 -ftree-vectorize -mtune=corei7-avx -pipe"
COMMON_FLAGS="-O2 -pipe -march=native"
CFLAGS="${COMMON_FLAGS}"
CXXFLAGS="${COMMON_FLAGS}"
FCFLAGS="${COMMON_FLAGS}"
FFLAGS="${COMMON_FLAGS}"

LDFLAGS="${LDFLAGS} -fuse-ld=lld"

CHOST="x86_64-pc-linux-gnu"

#CPU_FLAGS_X86="sse sse2 avx avx2 sse3 sse4_1 sse4_2" 
# NOTE: This stage was built with the bindist USE flag enabled

# This sets the language of build output to English.
# Please keep this setting intact when reporting bugs.
LC_MESSAGES=C.utf8
USE="aac asm alsa a52 accessibility 
    -berkdb bluetooth
     cairo  corefonts -consolekit -cups -crypt -cjk
     dbus djvu dracut dist-kernel dev-rust  
     egl elogind -emacs extra efi 
     gtk gsm gtk+  gtk3 gstreamer -gles -gles1 gles2 gold
     harfbuzz h264
    jpeg jpeg2k jit
    idn icu -ipv6 -iwd
    firmware fishflac -fltk fontconfig faad ffmpeg fish
    kmod
    lua_targets_lua5-3 lua_single_target_lua5-4 -lua_single_target_lua5-1 lua_targets_luajit libzfs
    llvm lz4 lsms
    mp3 mp4 mpeg -mmal mpg123
    networkmanager
    opengl openh264 opus
    -pulseaudio -pipewire-alsa -pipewire png pango
    rust 
    ssh -su
    twolame -trash-panel-plugin
    qt6 -qt5 qml
    introspection initramfs
    udisks usb userland_GNU
     wayland wavpack
    -X xwayland -xinerama -xvfb x264 xml xvid xft x265
    vulkan  v4l -vaapi  3dnow
    zip zstd -zsh"

FEATURES="-network-sandbox userpriv"

PYTHON_TARGETS="python3_13 python3_14"
PYTHON_SINGLE_TARGET="python3_14"
L10N="ru ru-RU"
#MAKEOPTS="-j4 -l5"
#MAKEOPTS="-j2"
MAKEOPTS="-j8 -l16" 

INPUT_DEVICES="libinput"
#GRUB_PLATFORMS="pc"
PORTDIR_OVERLAY="/var/db/repos/tupoll-overlay${PORTDIR_OVERLAY}"
ACCEPT_LICENSE="*"
VIDEO_CARDS="amdgpu radeon radeonsi"
#VIDEO_CARDS="intel"
#VIDEO_CARDS="intel i915"
GENTOO_MIRRORS="http://mirror.yandex.ru/gentoo-distfiles/ https://mirrors.evowise.com/gentoo/ https://mirrors.lug.mtu.edu/gentoo/ http://distfiles.gentoo.org"
  "#), 
        ("profiles/default/linux/amd64/23.0/llvm/wayland/eapi", r#"8"#),    
        ("profiles/default/linux/amd64/23.0/llvm/wayland/wayland/parent", r#"gentoo:default/linux/amd64/23.0/llvm
../../../../../../../targets/amd64/wayland"#),
        ("profiles/default/linux/amd64/23.0/llvm/wayland/parent", r#"gentoo:default/linux/amd64/23.0/llvm
../../../../../../targets/amd64/wayland"#), 
       ("profiles/profiles.desc", r#" #arch		profile_directory				status
amd64           default/linux/amd64/23.0/llvm/wayland           exp
#arm64            default/linux/arm64/23.0/llvm/wayland           exp "#), 
       ("metadata/repoman/qa_data.yaml", r#"---
# This yaml syntax file holds various configuration data for
# the Quality-Assurance checks performed.

# no random drive-by commits please
# Please obtain authorization from the portage team
#
# Overlay maintainers override/add/negate checks at your discression
# but support for third party module will be limited to the plugin API
#

# Repoman API version (do not edit)
version: 1
# minimum
repoman_version: 2.3.3


allowed_filename_chars: "a-zA-Z0-9._-+:"
max_description_length: 80

# missingvars check: Mandatory (non-defaulted) ebuild variables
# list
missingvars:
    - KEYWORDS
    - LICENSE
    - DESCRIPTION
    - HOMEPAGE

# file.executable check, non executable files
# list
no_exec_files:
    - Manifest
    - ChangeLog
    - metadata.xml

# qawarnings: Non-fatal warnings,
#             all values in here MUST have a corresponding qahelp entry
# list
qawarnings:
    - changelog.missing
    - changelog.notadded
    - dependency.unknown
    - dependency.badmasked
    - dependency.badindev
    - dependency.badmaskedindev
    - dependency.badtilde
    - dependency.deprecated
    - dependency.equalsversion
    - dependency.missingslot
    - dependency.perlcore
    - DESCRIPTION.toolong
    - digest.assumed
    - digest.unused
    - EAPI.deprecated
    - ebuild.notadded
    - ebuild.nesteddie
    - ebuild.absdosym
    - ebuild.minorsyn
    - ebuild.badheader
    - ebuild.patches
    - file.empty
    - file.size
    - HOMEPAGE.virtual
    - inherit.unused
    - inherit.deprecated
    - IUSE.rubydeprecated
    - java.eclassesnotused
    - KEYWORDS.dropped
    - KEYWORDS.stupid
    - KEYWORDS.missing
    - KEYWORDS.unsorted
    - LICENSE.deprecated
    - LICENSE.virtual
    - metadata.warning
    - PDEPEND.suspect
    - portage.internal
    - RDEPEND.implicit
    - RDEPEND.suspect
    - repo.eapi-deprecated
    - RESTRICT.invalid
    - usage.obsolete
    - upstream.workaround
    - uri.https
    - virtual.suspect
    - wxwidgets.eclassnotused

# ruby_deprecated: Deprecated ruby targets
# list
ruby_deprecated:
    - ruby_targets_ruby18
    - ruby_targets_ruby19
    - ruby_targets_ruby20

# suspect_rdepend: Common build only Dependencies
#                  not usually run time dependencies
# list
suspect_rdepend:
  - app-arch/cabextract
  - app-arch/rpm2targz
  - app-doc/doxygen
  - dev-lang/nasm
  - dev-lang/swig
  - dev-lang/yasm
  - dev-perl/extutils-pkgconfig
  - dev-qt/linguist-tools
  - dev-util/byacc
  - dev-util/cmake
  - dev-util/ftjam
  - dev-util/gperf
  - dev-util/gtk-doc
  - dev-util/gtk-doc-am
  - dev-util/intltool
  - dev-util/jam
  - dev-util/pkg-config-lite
  - dev-util/pkgconf
  - dev-util/pkgconfig
  - dev-util/pkgconfig-openbsd
  - dev-util/scons
  - dev-util/unifdef
  - dev-util/yacc
  - media-gfx/ebdftopcf
  - sys-apps/help2man
  - sys-devel/autoconf
  - sys-devel/automake
  - sys-devel/bin86
  - sys-devel/bison
  - sys-devel/dev86
  - sys-devel/flex
  - sys-devel/m4
  - sys-devel/pmake
  - virtual/linux-sources
  - virtual/linuxtv-dvb-headers
  - virtual/os-headers
  - virtual/pkgconfig
  - x11-misc/bdftopcf
  - x11-misc/imake

# suspect_virtual: Dependencies that should usually be made to the virtual
#                  Not to the final target library
# dictionary
suspect_virtual:
  dev-libs/libusb: virtual/libusb
  dev-libs/libusb-compat: virtual/libusb
  dev-libs/libusbx: virtual/libusb
  dev-util/pkg-config-lite: virtual/pkgconfig
  dev-util/pkgconf: virtual/pkgconfig
  dev-util/pkgconfig: virtual/pkgconfig
  dev-util/pkgconfig-openbsd: virtual/pkgconfig

# valid_restrict: ???
# list
valid_restrict:
    - binchecks
    - bindist
    - fetch
    - installsources
    - mirror
    - preserve-libs
    - primaryuri
    - splitdebug
    - strip
    - test
    - userpriv        
   "#), 
      ("metadata/repoman/repository.yaml", r#"# Регистрационные данные оверлея tupoll-overlay
name: tupoll-overlay
description: "Personal Gentoo overlay for Rust-based system tools and GUI apps"
owner:
  - type: person
    email: "tupoll@example.com"
    name: "tupoll"
status: official        
   "#),  
      ("profiles/repo_name", r#"tupoll-overlay"#), 
      
     ("profiles/targets/amd64/wayland/eapi", r#"8"#),  
      ("profiles/targets/amd64/wayland/use.force", r#"wayland
lua_single_target_lua5-4
efistub"#), 
     ("profiles/targets/amd64/wayland/use.mask", r#"systemd
pulseaudio
-x264
-gnome-keyring
-fuse
 mmal
-efistub"#),

     ("profiles/targets/amd64/wayland/package.accept_keywords/custom", r#"gui-libs/gtk ~amd64
dev-scheme/xdgdirs ~amd64
gui-apps/pinnacle-lock ~amd64
gui-apps/pinnacle-screenshot ~amd64
gui-apps/pinnacle-wallpaper ~amd64
gui-apps/pinnacle-terminal ~amd64
gui-libs/gtk4-layer-shell ~amd64
sys-fs/zfs-kmod **
sys-fs/zfs **
net-wireless/hidclient ~amd64
dev-python/dasbus ~amd64
media-video/soxbar **


x11-themes/adapta-gtk-theme ~amd64
x11-themes/beautyline-icon-theme ~amd64
x11-themes/gnome-icon-theme-extras ~arm64
x11-themes/fluent-icon-theme ~amd64

x11-themes/yasis-icon-theme ~amd64

x11-themes/numix-gtk-theme ~amd64
x11-themes/numix-icon-theme ~amd64
app-misc/nwg-look ~amd64
x11-apps/xcur2png ~amd64
gui-apps/crucian ~amd64
 "#),
     ("profiles/targets/amd64/wayland/package.mask/custom", r#"sys-kernel/gentoo-kermel-bin
sys-kernel/gentoo-kernel
gnome-extra/zenity "#),
     ("profiles/targets/amd64/wayland/package.use/custom", r#"media-libs/freetype -harfbuzz
dev-python/docutils -python_targets_python3_11
dev-lang/python -bluetooth

sys-auth/seatd server

www-client/vivaldi -qt6
sys-fs/fuse suid
sys-libs/gdbm berkdb
net-wireless/wpa_supplicant tkip wps wep
sys-libs/ncurses -gpm
sys-apps/attr -nls
sys-libs/libcap -pam
dev-libs/icu abi_x86_32 

sys-libs/zlib minizip
net-misc/networkmanager -iwd dhcpcd -resolvconf -ppp 
dev-python/pillow webp
net-libs/libtorrent-rasterbar python
media-libs/freetype harfbuzz
dev-libs/boost python
app-crypt/pinentry -qt6

gnome-extra/nm-applet appindicator
net-wireless/wpa_supplicant -gui 

sys-kernel/installkernel dracut

sys-kernel/gentoo-kernel-bin -initramfs
sys-kernel/gentoo-kernel savedconfig -initramfs
net-wireless/bluez deprecated
 
x11-libs/libdrm video_cards_amdgpu

media-sound/sox alsa encode ffmpeg flac id3tag ogg 
media-libs/libaom -examples

media-video/ffmpeg v4l lcms udev drm threads -vpx
gnome-extra/zenity -webkit 
media-libs/mesa -wayland -egl -gles -gles1 -gles2
gui-libs/gtk aqua
media-libs/libglvnd X
sys-fs/zfs python_targets_python3_12
media-libs/vulkan-loader X wayland
media-libs/mesa X wayland -egl -gles1 -gles2 -llvm -vaapi -vdpau -zstd -xmlconfig gbm gallium
x11-libs/gtk+ X
x11-libs/cairo X aqua
media-libs/libepoxy X
gui-libs/gtk X
app-text/xmlto text
media-libs/gst-plugins-base X egl
dev-cpp/cairomm X
dev-python/pillow tiff
media-libs/babl lcms

media-libs/gegl lcms
dev-cpp/gtkmm X
media-plugins/gst-plugins-meta ogg "#),
     ("profiles/targets/amd64/wayland/package.use.force/custom", r#"##############media-video/vlc libav
#x11-libs/gtk+:3 -wayland
#x11-libs/gtk+ -wayland

media-libs/mesa X wayland -egl -gles1 -gles2 llvm -vaapi -vdpau -zstd -xmlconfig gbm gallium "#),
     ("profiles/targets/amd64/wayland/package.use.mask", r#"####замаскать полностью :media-video/ffmpeg
#media-sound/pulseaudio-daemon
#sys-boot/uefi-mkconfig "#),
    
 ]; 
     
     for (rel_path, content) in &text_files {
        let full_path = home_base.join(rel_path.trim_start_matches('/'));
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, content)?;
    }

    println!("Файлы оверлея созданы /var/db/repos/*.");
    
   
    
    let path_clone = "/etc/portage/repos.conf/tupoll-overlay.conf";    
    let content = r#"[tupoll-overlay]
location = /var/db/repos/tupoll-overlay
auto-sync = no
priority = 1
"#;

    if let Some(parent) = Path::new(path_clone).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path_clone, content)?;

    println!("Конфигурация оверлея записана в /etc/portage/repos.conf.");
    
    
    let path = Path::new("/etc/portage/repos.conf/eselect-repo.conf");
    let repo_header = "[guru]";
    let config_block = "\n[guru]\nlocation = /var/db/repos/guru\nsync-type = git\nsync-uri = https://github.com/gentoo-mirror/guru.git\n";

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let already_exists = if path.exists() {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        reader.lines().any(|line| line.unwrap_or_default().trim() == repo_header)
    } else {
        false
    };

    if !already_exists {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        
        file.write_all(config_block.as_bytes())?;
        println!("Репозиторий GURU успешно добавлен в {}", path.display());
    } else {
        println!("Конфигурация GURU уже присутствует в {}", path.display());
    }
    
    
    Ok(())
}
