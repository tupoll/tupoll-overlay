#writer-bin
EAPI=8
inherit cargo

DESCRIPTION="wayshot"
HOMEPAGE="https://crates.io/wayshot"
SRC_URI=""
S="${WORKDIR}/wayshot-1.4.5"
LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64"
RDEPEND="
	virtual/pkgconfig
	dev-libs/glib
	dev-libs/wayland
	x11-libs/libdrm"
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_unpack() { cp -Rp "/var/db/repos/tupoll-overlay/dev-rust/wayshot/sources-1.4.5/"* "${WORKDIR}/" || die; }

src_compile() {
    export CARGO_HOME="${T}/cargo_home"
    mkdir -p .cargo || die
    echo "[source.crates-io]
replace-with = 'vendored-sources'
[source.vendored-sources]
directory = 'vendor'" > .cargo/config.toml || die
    RUSTFLAGS='-C target-cpu=native' cargo build --release --offline --all-features || die
}

src_install() {
    find target/release -maxdepth 1 -executable -type f ! -name "*.so" -exec dobin {} +
}
