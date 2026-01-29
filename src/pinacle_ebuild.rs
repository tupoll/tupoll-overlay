use std::fs;

fn main() -> std::io::Result<()> {
    let path = "/var/db/repos/tupoll-overlay/gui-wm/pinnacle/pinnacle-9999.ebuild";

    let content = r#"




EAPI=8
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
"#;

    // Создаем директорию, если её нет
    if let Some(parent) = std::path::Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;

    println!("Live-ebuild pinnacle-9999 успешно создан.");
    Ok(())
}
