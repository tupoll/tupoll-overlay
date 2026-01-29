use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = "/var/db/repos/tupoll-overlay/app-portage/tupoll-overlay/tupoll-overlay-9999.ebuild";
    
    let content = r#"EAPI=8
inherit cargo git-r3 

DESCRIPTION="Local overlay for portage."

HOMEPAGE="https://github.com"
EGIT_REPO_URI="https://github.com/tupoll/tupoll-overlay.git"

LICENSE="BSD-2"
SLOT="0"
KEYWORDS=""
PROPERTIES="live"

RDEPEND=" "
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

    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;

    println!("Успешно записан.");
    Ok(())
}
