# Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

inherit cargo git-r3

DESCRIPTION="Local overlay for Gentoo linux."
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
	insinto /usr/share/pinnacle-gentoo/pictures
    doins pictures/*
domenu "Pinnacle-translator.desktop"
}  
