# Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

inherit cargo desktop

DESCRIPTION="Local overlay for Gentoo linux."
HOMEPAGE="https://github.com"
LICENSE="MIT"
SLOT="0"
KEYWORDS=" "

SRC_URI=""
RESTRICT="fetch"

S="${WORKDIR}/${P}/tupoll-overlay"

#ECARGO_VENDOR=""

RDEPEND=" "
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_unpack() {
    mkdir -p "${WORKDIR}/${P}" || die
    cp -Rp "${FILESDIR}/tupoll-overlay" "${WORKDIR}/${P}/" || die
    #git-r3_src_unpack
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
}  
 