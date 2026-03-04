HOMEPAGE="https://github.com"
EGIT_REPO_URI="https://github.com/tupoll/tupoll-overlay.git"

LICENSE="BSD-2"
SLOT="0"
KEYWORDS=" "

S="${WORKDIR}/${P}/tupoll-overlay"

RDEPEND=" "
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_unpack() {
    mkdir -p "${WORKDIR}/${P}" || die
    cp -Rp "${FILESDIR}/tupoll-overlay" "${WORKDIR}/${P}/" || die
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
	insinto /usr/share/pinnacle-gentoo
    doins "/pictures/*"
    doicon "accessories-dictionary.svg"
    domenu "Pinnacle Translator.desktop"
    domenu "Pinnacle Terminal.desktop"
}  

pkg_postinst() {
    /usr/bin/tupoll-overlay
    elog "Структура оверлея создана ✔️"
    /usr/bin/pinnacle-install
    elog "Конфигурация для PINNACLE создана ✔️"
    /usr/bin/translator-config
    elog "Структура для переводчика готова ✔️"
    
}

pkg_prerm() {
    rm -fv /usr/bin/tupoll-overlay
    rm -fv /usr/bin/pinnacle-install
    rm -fv /usr/bin/translator-config
     elog "Всё почищено 🗑"
}
