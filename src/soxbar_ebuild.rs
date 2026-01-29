use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = "/var/db/repos/tupoll-overlay/media-video/soxbar/soxbar-9999.ebuild";
    
    let content = r#"EAPI=8
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
}
"#;

    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;

    println!("Ebuild soxbar-9999 успешно записан.");
    Ok(())
}
