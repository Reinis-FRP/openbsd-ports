ONLY_FOR_ARCHS =	amd64

COMMENT-main =		desktop word processor with DOCX and PDF support
COMMENT-cli =		command-line document editor and headless MCP server
DIST_TUPLE =		github storytold wordcraft v0.18.1 . \
			github storytold craft-fonts \
			b076417b5880bd4154bd22f00695b1d28c6bea83 ../fonts \
			github storytold craft-i18n \
			1e515422afdda2f317af430c1f9d68a5f83935b4 ../i18n
MULTI_PACKAGES =	-main -cli
REVISION-cli =		0
CATEGORIES =		editors

# MIT or Apache-2.0; embedded fonts and proofing data have separate notices.
PERMIT_PACKAGE =	Yes

MODULES =		devel/cargo
WANTLIB =		${MODCARGO_WANTLIB} m
# wgpu and winit load the graphics backends at runtime.
RUN_DEPENDS-main =	devel/desktop-file-utils \
			graphics/vulkan-loader \
			wayland/wayland \
			x11/gtk+4,-guic \
			x11/xkbcommon

# Keep each Git source at its own locked revision, even within one repository.
GIT_SOURCES = \
	craft-libs 01315d9c92b5b0e69082d35ca56099461a1efd45 proof \
	craft-libs 6e03f65bf45181db77ba6229716e6624db9eb25b pdf \
	craft-libs 762a06a4c6b48b522b7d4de84a87917ef90265e6 font-store \
	craft-libs 92d392cff132eaf53e13afff77f38b5c11b73aa0 ui \
	craft-libs a3e0b5a9440d18f19422a9dca13516747b7d3181 print \
	craft-libs fd5e2feff6ea25bed371a99cc732b3968d621505 web \
	egui 6d518fc799681571963842f95b55a6b68431b15a egui \
	winit a517b3846ee8457d84ee4975110bfda6797c44dc winit \
	winprint.rs 417ac3c4d14d3da5485a99c7d5ce8f05664ec312 gdi \
	winprint.rs 5f80b9baf6b58937c401656634e42d33f00ddf49 winprint

GIT_CRATES.proof =	crates/proof
GIT_CRATES.pdf =	crates/pdf crates/update vendor/krilla
GIT_CRATES.font-store =	crates/atomic crates/font-catalog crates/font-store
GIT_CRATES.ui =		crates/fonts-build crates/i18n crates/ui crates/window
GIT_CRATES.print =	crates/print
GIT_CRATES.web =	crates/web-api crates/web-host
GIT_CRATES.egui =	crates/ecolor crates/eframe crates/egui \
			crates/egui-wgpu crates/egui-winit crates/egui_demo_lib \
			crates/egui_extras crates/egui_glow crates/egui_inspection \
			crates/egui_kittest crates/egui_system_font_provider \
			crates/emath crates/epaint crates/epaint_default_fonts
GIT_CRATES.winit =	.
GIT_CRATES.gdi =	crates/gdi
GIT_CRATES.winprint =	.

.for repo rev group in ${GIT_SOURCES}
DIST_TUPLE +=		github storytold ${repo} ${rev} ../${group}
.endfor

CONFIGURE_STYLE =	cargo
MODCARGO_BUILD_ARGS =	--frozen -p wordcraft-cli --bin wordcraft-cli
MODCARGO_TEST_ARGS =	--frozen -p wordcraft-cli
MODCARGO_ENV +=		CRAFT_FONTS_DIR=${WRKDIR}/fonts CRAFT_FONTS_REQUIRED=1 \
			CRAFT_I18N_DIR=${WRKDIR}/i18n CRAFT_I18N_REQUIRED=1 \
			WORDCRAFT_BUILD_SHA=079f478bd85df7bcc567e3bafa7d7219096f9c3c

DOCDIR =		${PREFIX}/share/doc/wordcraft-cli

post-configure:
.for repo rev group in ${GIT_SOURCES}
	cd ${WRKDIR} && sh ${FILESDIR}/vendor-git ${repo} ${rev} ${group} \
	    ${GIT_CRATES.${group}}
.endfor

post-build:
	${MODCARGO_CARGO_RUN} build --release --frozen -p wordcraft --bin wordcraft

post-test:
	${MODCARGO_CARGO_RUN} test --release --frozen -p wordcraft
	perl ${FILESDIR}/smoke-test ${MODCARGO_TARGET_DIR}/release/wordcraft-cli

do-install:
	${INSTALL_PROGRAM} ${MODCARGO_TARGET_DIR}/release/wordcraft ${PREFIX}/bin/
	${INSTALL_PROGRAM} ${MODCARGO_TARGET_DIR}/release/wordcraft-cli \
	    ${PREFIX}/bin/
	${INSTALL_DATA_DIR} ${DOCDIR} ${DOCDIR}/fonts ${DOCDIR}/language-data \
	    ${DOCDIR}/pdf-writer
	cd ${WRKSRC} && ${INSTALL_DATA} ATTRIBUTION.md LICENSE-APACHE \
	    LICENSE-MIT NOTICE ${DOCDIR}/
	${INSTALL_DATA} ${WRKSRC}/assets/fonts/OFL-*.txt ${DOCDIR}/fonts/
	${INSTALL_DATA} ${WRKDIR}/fonts/ATTRIBUTION.md ${DOCDIR}/fonts/
	for f in ${WRKDIR}/fonts/fonts/*/OFL.txt; do \
		family=$${f%/OFL.txt}; \
		${INSTALL_DATA} "$$f" ${DOCDIR}/fonts/OFL-$${family##*/}.txt; \
	done
	cd ${WRKDIR}/i18n && ${INSTALL_DATA} ATTRIBUTION.md LICENSE-MIT \
	    LICENSE-APACHE ${DOCDIR}/language-data/
.for locale in en-au en-ca en-gb en-us
	${INSTALL_DATA} ${WRKDIR}/i18n/language-data/${locale}/LICENCE-SCOWL.txt \
	    ${DOCDIR}/language-data/LICENCE-SCOWL-${locale}.txt
.endfor
	cd ${WRKDIR}/pdf && ${INSTALL_DATA} ATTRIBUTION.md NOTICE \
	    ${DOCDIR}/pdf-writer/
	${INSTALL_DATA} ${WRKDIR}/pdf/licenses/pdf-writer/* ${DOCDIR}/pdf-writer/

	# Each package carries the notices for the assets embedded in its binary.
	${INSTALL_DATA_DIR} ${PREFIX}/share/doc/wordcraft/egui-fonts
	cp -R ${DOCDIR}/. ${PREFIX}/share/doc/wordcraft/
	${INSTALL_DATA} ${WRKDIR}/egui/crates/epaint_default_fonts/fonts/*.txt \
	    ${PREFIX}/share/doc/wordcraft/egui-fonts/
	${INSTALL_DATA} ${WRKSRC}/modcargo-crates/icu_locale_core-2.3.0/LICENSE \
	    ${PREFIX}/share/doc/wordcraft/language-data/LICENSE-Unicode
	${INSTALL_DATA_DIR} ${PREFIX}/share/applications ${PREFIX}/share/icons
	${INSTALL_DATA} ${WRKSRC}/packaging/linux/ai.storyteller.wordcraft.desktop \
	    ${PREFIX}/share/applications/
	cd ${WRKSRC}/assets/app-icon && pax -rw hicolor ${PREFIX}/share/icons

.include "crates.inc"
.include <bsd.port.mk>
