use std::fs;

fn main() -> std::io::Result<()> {
    let path = "/var/db/repos/tupoll-overlay/dev-lang/emerge-rs/emerge-rs-1.0.0.ebuild";

    let ebuild_content = r#"# Copyright 2024-2026 Gentoo Authors
EAPI=8

inherit cargo

DESCRIPTION="Rust implementation of python-exec wrapper"
HOMEPAGE="https://github.com"

# Если файлы лежат локально в files/
LICENSE="BSD-2"
SLOT="0"
KEYWORDS="amd64 arm64"

IUSE=""

# Обязательно для генерации конфига Cargo из переменной CRATES
CRATES="
	libc-0.2.169
"

RDEPEND="dev-lang/python-exec-conf"
DEPEND="${RDEPEND}"

# Важный этап для исправления вашей ошибки
src_configure() {
	cargo_gen_config
}

src_unpack() {
	# Создаем рабочую директорию
	mkdir -p "${S}" || die
	
	# Копируем исходники из локального оверлея
	# Предполагается, что Cargo.toml и src/ лежат в ${FILESDIR}
	cp -r "${FILESDIR}/src" "${S}/" || die
	cp "${FILESDIR}/Cargo.toml" "${S}/" || die
	
	# Распаковываем зависимости (crates)
	cargo_src_unpack
}

src_compile() {
	cargo_src_compile
}

src_install() {
	cargo_src_install
}
"#;

    // Создаем директорию, если она еще не существует
    if let Some(parent) = std::path::Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, ebuild_content)?;

    println!("Ebuild записан успешно: {}", path);
    Ok(())
}
