# Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

inherit cargo git-r3 desktop

DESCRIPTION="Monolithic bootstrap with collaborative amd64/arm64 execution engine"
HOMEPAGE="https://github.com/tupoll/tupoll-overlay"
EGIT_REPO_URI="https://github.com/tupoll/tupoll-overlay.git"

LICENSE="BSD-2"
SLOT="0"

# ДВА СИСТЕМНЫХ ФЛАГА АРХИТЕКТУРЫ
KEYWORDS="~amd64 ~arm64"
IUSE="amd64 arm64"
REQUIRED_USE="|| ( amd64 arm64 )"

S="${WORKDIR}/${P}"

RDEPEND="app-shells/fish,
         dev-util/maturin,
         dev-vcs/git,
         dev-db/sqlite"
         
DEPEND="${RDEPEND}"
BDEPEND="virtual/pkgconfig"

src_unpack() {
	git-r3_src_unpack
	if [[ -d "${FILESDIR}/tupoll-overlay" ]]; then
		cp -Rp "${FILESDIR}/tupoll-overlay/"* "${S}/" || die
	fi
	cargo_live_src_unpack
}

src_configure() {
	cargo_gen_config
}

src_compile() {
	RUSTFLAGS="-C target-cpu=native" cargo_src_compile
}

src_install() {
	# Устанавливаем корневой оркестратор
	dobin "target/release/tupoll-overlay"
	
	# Безусловно закидываем оба архитектурных профиля, чтобы они были доступны tupoll-overlay
	dobin "target/release/tupoll-overlay-amd64"
	if use arm64; then
		dobin "target/release/tupoll-overlay-arm64"
	fi

	insinto /usr/share/pinnacle-gentoo/pictures
	doins pictures/*
	doicon "accessories-dictionary.svg"
	domenu "Pinnacle Translator.desktop" 
}  

pkg_postinst() {
	einfo "Запуск совместного конвейера развертывания оверлея Pinnacle OS..."
	
	# Единая точка входа: вызывается строго tupoll-overlay, который внутри себя
	# координирует работу с -amd64 / -arm64 и дописывает директории!
	if [[ -x "/usr/bin/tupoll-overlay" ]]; then
		/usr/bin/tupoll-overlay
	fi

	# === ХУК: ОКОНЧАТЕЛЬНЫЙ ДЕМОНТАЖ EMERGE ===
	echo "!!!  Удаление emerge из env-хука !!!"
	rm -f "${ROOT}/usr/bin/emerge"
	
	elog "Оверлей успешно развернут под архитектуру ${ARCH}. Менеджер пакетов переключен на pkgrs 🗑"
}
