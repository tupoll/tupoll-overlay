# Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

inherit cargo desktop

DESCRIPTION="Notify for pinnacle-wm"
HOMEPAGE="https://github.com"
LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64"

SRC_URI=""
RESTRICT="fetch"

S="${WORKDIR}/${P}/pinnacle-lock"

RDEPEND="    
	gui-wm/pinnacle-gentoo
	dev-rust/kanata	
"
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_unpack() {
    mkdir -p "${WORKDIR}/${P}" || die
    cp -Rp "${FILESDIR}/pinnacle-lock" "${WORKDIR}/${P}/" || die
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
	exeinto /etc/pam.d
    doexe "pinnacle-lock"
    mkdir -p /etc/kantata
    exeinto /etc/kantata
    doexe "keymaps-lock.kbd"
    exeinto /usr/sbin
    doexe "kanata-daemon-lock.sh"
    elog "Создайте записи в /etc/sudoers.d:
    Cmnd_Alias PROCESSES = /usr/bin/nice, ..процессы.., /usr/bin/auth-rs
    Cmnd_Alias	REBOOT = /sbin/halt, /sbin/reboot, /sbin/poweroff, /usr/sbin/kanata-daemon-lock.sh
    root ALL=(ALL) ALL
    <пользователь> ALL=(ALL) ALL, NOPASSWD: REBOOT, PROCESSES"     	
}  
