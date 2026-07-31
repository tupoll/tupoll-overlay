# Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

inherit cargo git-r3 desktop

DESCRIPTION="Local overlay for Gentoo linux."

HOMEPAGE="https://github.com"
EGIT_REPO_URI="https://github.com/tupoll/tupoll-overlay.git"

LICENSE="BSD-2"
SLOT="0"
KEYWORDS="~amd64 ~arm64"

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
	insinto /usr/share/pinnacle-gentoo/pictures
    doins pictures/*
    doicon "accessories-dictionary.svg"
    domenu "Pinnacle Translator.desktop" 
}  

pkg_postinst() {
	 fish -c /usr/bin/tupoll-overlay
	 fish -c /usr/bin/pinnacle-install
	 fish -c /usr/bin/translator-config
     fish -c /usr/bin/pinnacle-terminal-config
     fish -c /usr/bin/pinnacle-wallpaper-config
     fish -c /usr/bin/pinnacle-notify-config
     fish -c /usr/bin/pinnacle-lock-config
     fish -c /usr/bin/pinnacle-screenshot-config
     fish -c /usr/bin/pinnacle-fm-config
     fish -c /usr/bin/pkgrs-install
     fish -c /usr/bin/crucian-config
     fish -c /usr/bin/repoman-config
     fish -c /usr/bin/servo-install
     fish -c /usr/bin/pkgrs-utils-config
     rm -fv /usr/bin/tupoll-overlay
     rm -fv /usr/bin/pinnacle-install
     rm -fv /usr/bin/translator-config
     rm -fv /usr/bin/pinnacle-terminal-config
     rm -fv /usr/bin/pinnacle-wallpaper-config
     rm -fv /usr/bin/pinnacle-notify-config
     rm -fv /usr/bin/pinnacle-lock-config
     rm -fv /usr/bin/pinnacle-screenshot-config
     rm -fv /usr/bin/pinnacle-fm-config
     rm -fv /usr/bin/pkgrs-install
     rm -fv /usr/bin/crucian-config
     rm -fv /usr/bin/repoman-config
     rm -fv /usr/bin/servo-install
     rm -fv /usr/bin/pkgrs-utils-config
     elog "Всё почищено 🗑"
}
