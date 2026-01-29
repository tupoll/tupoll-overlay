use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = "/var/db/repos/tupoll-overlay/profiles/targets/amd64/wayland/make.defaults";
    
    let content = r#"#COMMON_FLAGS="-march=core2 -ftree-vectorize -mtune=corei7-avx -pipe"
CFLAGS="-O2 -pipe -march=native"
CFLAGS="${COMMON_FLAGS}"
CXXFLAGS="${COMMON_FLAGS}"
FCFLAGS="${COMMON_FLAGS}"
FFLAGS="${COMMON_FLAGS}"

CHOST="x86_64-pc-linux-gnu"

#CPU_FLAGS_X86="sse sse2 avx avx2 sse3 sse4_1 sse4_2" 
# NOTE: This stage was built with the bindist USE flag enabled

# This sets the language of build output to English.
# Please keep this setting intact when reporting bugs.
LC_MESSAGES=C.utf8
USE="aac asm alsa a52 accessibility
    -berkdb bluetooth
     -clang cairo  corefonts -consolekit -cups -crypt -cjk
     dbus djvu dracut dist-kernel
     egl elogind -emacs extra 
     gtk3 gtk gstreamer -gles -gles1 gles2 gold
     harfbuzz h264
    jpeg jpeg2k
    id3tag idn icu -ipv6 -iwd
    flac -fltk fontconfig faad firmware ffmpeg fish
    kmod
    lua_targets_lua5-3 lua_single_target_lua5-4 -lua_single_target_lua5-1 lz4  lua_targets_luajit libzfs
    mp3 mp4 mpeg -mmal mpg123
    networkmanager
    opengl openh264 opus
    png pango pie -pipewire-alsa -pipewire
    ssh
    twolame -trash-panel-plugin
    qt6 -qt5 qml
    introspection
    udisks usb userland_GNU
    wayland wavpack
    X xwayland -xinerama -xvfb x264 xml xvid xft x265
    vulkan  v4l -vaapi  3dnow
    zip zstd -zsh"

PYTHON_TARGETS="python3_12 python3_13 python3_14"
PYTHON_SINGLE_TARGET="python3_12"
L10N="ru ru-RU"
MAKEOPTS="-j4 -l5"
#MAKEOPTS="-j2 -l3"
INPUT_DEVICES="libinput"
#GRUB_PLATFORMS="pc"
PORTDIR_OVERLAY="/var/db/repos/tupoll-overlay${PORTDIR_OVERLAY}"
ACCEPT_LICENSE="*"
#VIDEO_CARDS="radeon radeonsi"
VIDEO_CARDS="intel i915"
GENTOO_MIRRORS="http://mirror.yandex.ru/gentoo-distfiles/ https://mirrors.evowise.com/gentoo/ https://mirrors.lug.mtu.edu/gentoo/ http://distfiles.gentoo.org"


"#;

    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;

    println!("Успешно записан.");
    Ok(())
}
