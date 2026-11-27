ONLY_FOR_ARCHS =	amd64

COMMENT-main =		desktop word processor with DOCX and PDF support
COMMENT-cli =		command-line document editor and headless MCP server
DIST_TUPLE =		github storytold wordcraft v0.19.1 . \
			github storytold craft-fonts \
			b076417b5880bd4154bd22f00695b1d28c6bea83 ../fonts \
			github storytold craft-i18n \
			e56d26244bbac099bfd42e23fb12f78a4be418a7 ../i18n
MULTI_PACKAGES =	-main -cli
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
	craft-libs 93dac78c93a7f1fd66e2022edeb089233b1dc45a craft-libs \
	egui 6d518fc799681571963842f95b55a6b68431b15a egui \
	winit a517b3846ee8457d84ee4975110bfda6797c44dc winit \
	winprint.rs 64e7afd4d1483512385012c340ecaaaf0f3beb7f gdi \
	winprint.rs 8ae097ef941b432b0e7671b1ec3a506800e65e57 winprint

GIT_CRATES.craft-libs =	crates/atomic crates/cfb crates/codepage crates/dds \
			crates/doc-binary crates/docx crates/drawingml crates/flow \
			crates/font-catalog crates/font-store crates/fonts-build \
			crates/html crates/i18n crates/image-codecs crates/jpeg \
			crates/latex crates/markdown crates/math crates/mcp-server \
			crates/metadata crates/metafile crates/mime crates/odf \
			crates/ods crates/offcrypto crates/opc crates/pdf \
			crates/print crates/proof crates/rtf crates/sheet \
			crates/ssip crates/tiff crates/ui crates/update \
			crates/web-api crates/web-host crates/window crates/wordmodel \
			crates/xls crates/xlsx crates/xml crates/zip vendor/krilla
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
			WORDCRAFT_BUILD_SHA=5d658dc033f3139926839bffd6ae0eef558c210c

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
	cd ${WRKDIR}/craft-libs && ${INSTALL_DATA} ATTRIBUTION.md NOTICE \
	    ${DOCDIR}/pdf-writer/
	${INSTALL_DATA} ${WRKDIR}/craft-libs/licenses/pdf-writer/* \
	    ${DOCDIR}/pdf-writer/

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
