ONLY_FOR_ARCHS =	amd64

COMMENT-main =		desktop spreadsheet with formulas and charts
COMMENT-cli =		command-line spreadsheet editor and headless MCP server
DIST_TUPLE =		github storytold gridcraft v0.19.1 . \
			github storytold craft-fonts \
			b076417b5880bd4154bd22f00695b1d28c6bea83 ../fonts
MULTI_PACKAGES =	-main -cli
CATEGORIES =		math x11

# MIT or Apache-2.0; embedded assets carry additional notices.
PERMIT_PACKAGE =	Yes

MODULES =		devel/cargo
WANTLIB =		${MODCARGO_WANTLIB} m
RUN_DEPENDS-main =	devel/desktop-file-utils \
			graphics/vulkan-loader \
			wayland/wayland \
			x11/gnome/zenity \
			x11/gtk+4,-guic \
			x11/xkbcommon

# Cargo distinguishes sources at different commits in the same repository.
GIT_SOURCES = \
	craft-libs 6e03f65bf45181db77ba6229716e6624db9eb25b ui \
	craft-libs fd5e2feff6ea25bed371a99cc732b3968d621505 web \
	egui 6d518fc799681571963842f95b55a6b68431b15a egui \
	winit a517b3846ee8457d84ee4975110bfda6797c44dc winit \
	winprint.rs 64e7afd4d1483512385012c340ecaaaf0f3beb7f gdi \
	winprint.rs bc06a7c48b768eb27a828c7986f6d2868b245ee0 winprint

GIT_CRATES.ui =		crates/fonts-build crates/i18n crates/print \
			crates/ui crates/update crates/window
GIT_CRATES.web =	crates/web-api crates/web-host
GIT_CRATES.egui =	crates/ecolor crates/eframe crates/egui \
			crates/egui-wgpu crates/egui-winit \
			crates/egui_demo_lib crates/egui_extras \
			crates/egui_glow crates/egui_inspection \
			crates/egui_kittest crates/egui_system_font_provider \
			crates/emath crates/epaint crates/epaint_default_fonts
GIT_CRATES.winit =	.
GIT_CRATES.gdi =	crates/gdi
GIT_CRATES.winprint =	.

.for repo rev group in ${GIT_SOURCES}
DIST_TUPLE +=		github storytold ${repo} ${rev} ../${group}
.endfor

CONFIGURE_STYLE =	cargo
MODCARGO_BUILD_ARGS =	--frozen -p gridcraft -p gridcraft-cli --bins
MODCARGO_TEST_ARGS =	--frozen -p gridcraft -p gridcraft-cli
MODCARGO_ENV +=		CRAFT_FONTS_DIR=${WRKDIR}/fonts CRAFT_FONTS_REQUIRED=1 \
			TMPDIR=${WRKDIR}

DOCDIR =		${PREFIX}/share/doc/gridcraft
FONT_FAMILIES =		biz-ud-mincho biz-ud-pgothic lxgw-wenkai-lite \
			nanum-gothic noto-sans-arabic noto-sans-devanagari \
			noto-sans-georgian noto-sans-hebrew noto-sans-symbols \
			noto-sans-symbols-2 noto-sans-thai shippori-mincho

post-configure:
.for repo rev group in ${GIT_SOURCES}
	cd ${WRKDIR} && sh ${FILESDIR}/vendor-git ${repo} ${rev} ${group} \
	    ${GIT_CRATES.${group}}
.endfor

post-test:
	TMPDIR=${WRKDIR} perl ${FILESDIR}/smoke-test \
	    ${MODCARGO_TARGET_DIR}/release/gridcraft-cli

do-install:
	cd ${WRKSRC}/crates/pdf/src && sed -n 's,^//! *,,p' metrics.rs symbol.rs \
	    > ${WRKDIR}/NOTICE-pdf
.for bin in gridcraft gridcraft-cli
	${INSTALL_PROGRAM} ${MODCARGO_TARGET_DIR}/release/${bin} ${PREFIX}/bin/
	${INSTALL_DATA_DIR} ${PREFIX}/share/doc/${bin}
	cd ${WRKSRC} && ${INSTALL_DATA} ATTRIBUTION.md LICENSE-APACHE \
	    LICENSE-MIT NOTICE ${PREFIX}/share/doc/${bin}/
	${INSTALL_DATA} ${WRKDIR}/NOTICE-pdf ${PREFIX}/share/doc/${bin}/
.endfor
	${INSTALL_DATA_DIR} ${DOCDIR}/fonts ${DOCDIR}/egui-fonts
	${INSTALL_DATA} ${WRKDIR}/fonts/ATTRIBUTION.md ${DOCDIR}/fonts/
.for family in ${FONT_FAMILIES}
	${INSTALL_DATA} ${WRKDIR}/fonts/fonts/${family}/OFL.txt \
	    ${DOCDIR}/fonts/OFL-${family}.txt
.endfor
	${INSTALL_DATA} ${WRKDIR}/egui/crates/epaint_default_fonts/fonts/*.txt \
	    ${DOCDIR}/egui-fonts/
	${INSTALL_DATA} ${WRKSRC}/assets/app-icon/LICENSE.txt \
	    ${DOCDIR}/LICENSE-icon
	${INSTALL_DATA} ${MODCARGO_VENDOR_DIR}/icu_locale_core-2.3.0/LICENSE \
	    ${DOCDIR}/LICENSE-Unicode
	${INSTALL_DATA_DIR} ${PREFIX}/share/applications ${PREFIX}/share/icons
	${INSTALL_DATA} ${WRKSRC}/packaging/linux/ai.storyteller.gridcraft.desktop \
	    ${PREFIX}/share/applications/
	cd ${WRKSRC}/assets/app-icon && pax -rw hicolor ${PREFIX}/share/icons

.include "crates.inc"
.include <bsd.port.mk>
