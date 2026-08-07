use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let home_base = PathBuf::from("/var/db/repos/tupoll-overlay");
    let text_files = [   
            
        
       
       ("profiles/targets/arm64/wayland/make.defaults", r#"COMMON_FLAGS="-O2 -pipe -mcpu=cortex-a76+crc+crypto"
CFLAGS="${COMMON_FLAGS}"
CXXFLAGS="${COMMON_FLAGS}"
FCFLAGS="${COMMON_FLAGS}"
FFLAGS="${COMMON_FLAGS}"

# WARNING: Changing your CHOST is not something that should be done lightly.
# Please consult https://wiki.gentoo.org/wiki/Changing_the_CHOST_variable before changing.
CHOST="aarch64-unknown-linux-gnu"

# NOTE: This stage was built with the bindist USE flag enabled

# This sets the language of build output to English.
# Please keep this setting intact when reporting bugs.
LC_MESSAGES=C.utf8
# global USE flag overrides
USE="aac asm alsa a52 accessibility 
    -berkdb bluetooth
     -clang cairo  corefonts -consolekit -cups -crypt cjk
     dbus djvu 
     egl elogind -emacs extra
     gtk3 gtk gstreamer -gles -gles1 gles2 gold
     harfbuzz
    jpeg jpeg2k
    idn icu -ipv6
    flac -fltk fontconfig faad -firmware ffmpeg fish
    kmod
    lua_targets_lua5-3 lua_single_target_lua5-4 -lua_single_target_lua5-1 linguas_ru libinput -lirc  -lvm  libv4l -llvm
    lz4 lua_targets_luajit libzfs
    mp3 mp4 mpeg -mmal mpg123
    -networkmanager
    opengl opus
    png pango pie -pipewire-alsa -pipewire
    qt6 -qt5 qml 
    server -samba sox sdl svg sdl-image ssse3 -suid system-wlroots sound-server -su  
    -trash-panel-plugin truetype tiff twolame ttf
    introspection
    udisks usb userland_GNU
    wayland wavpack
    X xwayland -xinerama -xvfb x264 xml -xvid xft x265
    v4l  3dnow  
    zip zstd -zsh"
# less aggressive here
#MAKEOPTS="-j5 -l4"
MAKEOPTS="-j4 -l3"
#MAKEOPTS="-j2 -l3"
VIDEO_CARDS="v3d vc4"
INPUT_DEVICES="libinput"  ### synaptics"

# uncomment to build binary packages as a byproduct of each emerge
# (these are useful backups) in /usr/portage/packages
FEATURES="${FEATURES} -buildpkg -cross"

# uncomment to disribute emerges, where possible, using distcc
# (NB: distcc-pump mode is now effectively deprecated)
#FEATURES="${FEATURES} distcc"

# uncomment to use binary packages from PORTAGE_BINHOST, where available,
# (and build normally, where not)
FEATURES="${FEATURES} -getbinpkg"

#PKGDIR=/var/cache/binpkgs
DISTDIR=/var/cache/distfiles
L10N="ru ru-RU"
#LLVN_TARGETS="arm"
PYTHON_TARGETS="python3_13 python3_14"
PYTHON_SINGLE_TARGET="python3_14"
MAKE="gmake"
PORTDIR_OVERLAY="/var/db/repos/tupoll-overlay${PORTDIR_OVERLAY}"
ACCEPT_LICENSE="*"
  "#), 
        ("profiles/default/linux/arm64/23.0/llvm/wayland/eapi", r#"8"#),
        ("profiles/default/linux/arm64/23.0/llvm/wayland/wayland/eapi", r#"8"#),   
        ("profiles/default/linux/arm64/23.0/llvm/wayland/wayland/parent", r#"gentoo:default/linux/arm64/23.0/llvm
../../../../../../../targets/arm64/wayland "#),
        ("profiles/default/linux/arm64/23.0/llvm/wayland/parent", r#"gentoo:default/linux/arm64/23.0/llvm
../../../../../../targets/arm64/wayland"#), 
      
       
     

      
     ("profiles/targets/arm64/wayland/eapi", r#"8"#),  
      ("profiles/targets/arm64/wayland/use.force", r#" "#), 
     ("profiles/targets/arm64/wayland/use.mask", r#" "#),

     ("profiles/targets/arm64/wayland/package.accept_keywords/rpi5", r#"#dev-lang/rust-bin ~arm64
 media-libs/suil ~arm64
 media-libs/libmp3splt **
 media-libs/portsmf ~arm64
 media-libs/libsoundtouch ~arm64
media-sound/audacity ~arm64
app-arch/rar **
app-arch/unrar **
media-plugins/kodi-pvr-wmc **
media-tv/kodi **
media-plugins/kodi-pvr-iptvsimple **
x11-themes/lyra-icon-theme **
x11-themes/sweet-dark-kvantum-theme ** 
net-wireless/wpa_supplicant ~arm64
media-sound/bluez-alsa ~arm64
dev-cpp/abseil-cpp ~arm64
dev-cpp/expected-lite ~arm64
dev-cpp/cppgir ~arm64
sys-kernel/raspberrypi-sources ~arm64
sys-fs/zfs ~arm64
sys-fs/zfs-kmod ~arm64
gui-apps/waybar ~arm64
 dev-libs/date ~arm64
 dev-util/geany **
xfce-extra/thunar-archive-plugin ~arm64
 xfce-extra/thunar-media-tags-plugin **
dev-python/i3ipc **
sys-power/acpi ~arm64
dev-libs/light **
gui-apps/wofi **
www-client/vivaldi ~arm64
 app-misc/nwg-look **
x11-apps/xcur2png **
media-gfx/nsxiv **
gui-libs/xdg-desktop-portal-wlr ~arm64
www-client/brave-bin ~arm64
 dev-libs/libpthread-stubs **
app-misc/ansiweather **
gui-apps/grimshot **
x11-themes/xfwm4-themes **
x11-themes/mate-themes **
x11-misc/icon-naming-utils ~arm64
app-misc/color **
net-wireless/bluez-tools ~arm64
 net-wireless/hidclient **
sys-fs/zfs ~arm64
sys-fs/zfs-kmod ~arm64
sys-block/gparted ~arm64
sys-kernel/genkernel ~arm64
dev-python/ansi ~arm64
dev-python/ansicolor ~arm64
media-fonts/fira-mono ~arm64
media-fonts/gofont-mono **
media-fonts/monoid **
x11-misc/dex **
media-fonts/ubuntu-font-family ~arm64
app-misc/calendar ~arm64
 net-im/telegram-desktop ~arm64
dev-cpp/ada ~arm64
media-libs/libtgvoip ~arm64
 media-libs/rnnoise ~arm64
media-libs/tg_owt ~arm64
dev-libs/libdispatch ~arm64
media-gfx/gthumb ~arm64
gui-apps/qt6ct **
x11-themes/dracula-gtk-theme **
x11-themes/bibata-cursor-theme **
x11-themes/adapta-gtk-theme **
x11-themes/beautyline-icon-theme **
media-gfx/inkscape ~arm64
x11-themes/sweet-dark-gtk-theme **
x11-themes/sweet-folders-icon-theme **
gui-apps/nwg-displays **
gui-apps/wlr-randr ~arm64
dev-cpp/ms-gsl ~arm64
x11-themes/gnome-icon-theme-extras ~arm64
x11-themes/fluent-icon-theme **
x11-themes/candy-icon-theme **
net-p2p/deluge ~arm64
net-libs/libtorrent-rasterbar ~arm64
dev-python/rencode ~arm64
dev-build/just ~arm64
x11-themes/yasis-icon-theme **
x11-themes/oie-icon-theme **
dev-perl/Syntax-Highlight-Engine-Simple-Perl **
dev-perl/Syntax-Highlight-Engine-Simple ** "#),
     ("profiles/targets/arm64/wayland/package.mask/rpi5", r#"=gui-apps/wofi-9999
=media-gfx/nsxiv-9999
=x11-themes/lyra-icon-theme-9999
=x11-themes/oie-icon-theme-9999
=media-tv/kodi-19*
#=media-tv/kodi-9999
#=media-plugins/kodi-pvr-iptvsimple-9999
#net-im/telegram-desktop
sys-kernel/installkernel
sys-kernel/gentoo-sources "#),
     ("profiles/targets/arm64/wayland/package.use/rpi5", r#"media-libs/freetype -harfbuzz
dev-python/docutils -python_targets_python3_11
dev-lang/python -bluetooth
gui-apps/swaybg gdk-pixbuf
sys-auth/seatd server
gui-wm/sway  swaynag tray wallpapers
 dev-lua/luv -lua_single_target_luajit
gui-wm/sway filecaps swaybar swaymsg wallpapers
www-client/vivaldi -qt6
sys-fs/fuse suid
sys-libs/gdbm berkdb
net-wireless/wpa_supplicant tkip wps wep
sys-libs/ncurses -gpm
sys-apps/attr -nls
sys-libs/libcap -pam
gui-apps/waybar network tray wifi  
media-video/ffmpeg vpx
sys-libs/zlib minizip
net-misc/networkmanager dhcpcd resolvconf appindicator 
dev-python/pillow webp
net-libs/libtorrent-rasterbar python
media-libs/freetype harfbuzz
dev-libs/boost python
x11-libs/gtk+ X "#),
     
     ("profiles/targets/arm64/wayland/use.mask", r#"pulseaudio
     systemd "#),
    
 ]; 
     
     for (rel_path, content) in &text_files {
        let full_path = home_base.join(rel_path.trim_start_matches('/'));
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, content)?;
    }

    println!("Файлы оверлея созданы /var/db/repos/*.");
    
   
    
    Ok(())
}
