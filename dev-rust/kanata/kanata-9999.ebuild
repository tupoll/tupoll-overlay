#writer-git
EAPI=8
inherit cargo git-r3

DESCRIPTION="kanata (live git version)"
EGIT_REPO_URI="https://github.com/jtroo/kanata.git"
LICENSE="MIT"
SLOT="0"
KEYWORDS=""

RDEPEND=""
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_prepare() {
    default
    if [[ -f "Cargo.toml" ]]; then
        einfo "Внимательный патч Cargo.toml для kanata..."
        # 1. Если секция [lib] есть, вставляем типы под неё
        if grep -q "\[lib\]" Cargo.toml; then
            sed -i '/\[lib\]/a crate-type = ["cdylib", "rlib"]' Cargo.toml || die
        else
            # 2. Если секции нет, создаем её с именем пакета
            echo -e "\n[lib]\nname = \"kanata\"\ncrate-type = [\"cdylib\", \"rlib\"]" >> Cargo.toml || die
        fi
    fi
}

src_compile() {
    # Собираем всё: бинарники и динамические библиотеки
    cargo build --release --no-default-features --features "cmd" || die
}

src_install() {
    # Ставим исполняемые файлы
    find target/release -maxdepth 1 -executable -type f ! -name "*.so*" ! -name "*.a" -exec dobin {} +
    # Ставим динамические библиотеки .so
    find target/release -maxdepth 1 -name "*.so*" -exec dolib.so {} +
}
