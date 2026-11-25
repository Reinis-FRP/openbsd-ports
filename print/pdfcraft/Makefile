ONLY_FOR_ARCHS =	amd64

COMMENT-main =		desktop PDF editor with annotation and OCR
COMMENT-cli =		command-line PDF editor and headless MCP server
COMMENT-data =		shared OCR models and notices for PdfCraft
V =			0.22.0
DISTNAME =		pdfcraft-${V}
PDFCRAFT_REV =		ac140f809d554981a5276d748e6193fcf4d4bdfa
DIST_TUPLE =		github storytold pdfcraft ${PDFCRAFT_REV} . \
			github storytold craft-fonts \
			b076417b5880bd4154bd22f00695b1d28c6bea83 ../fonts \
			github storytold craft-i18n \
			866aff71dd63d8369eb100b9c432b5af1400c22a ../i18n
WRKDIST =		${WRKDIR}/pdfcraft-${PDFCRAFT_REV}
MULTI_PACKAGES =	-main -cli -data
CATEGORIES =		print

# MIT or Apache-2.0; fonts and OCR models have separate licences.
PERMIT_PACKAGE =	Yes

HOMEPAGE =		https://github.com/storytold/pdfcraft

MODULES =		devel/cargo
WANTLIB-main =		${MODCARGO_WANTLIB} m
WANTLIB-cli =		${MODCARGO_WANTLIB} m
RUN_DEPENDS-cli =	${FULLPKGNAME-data}:print/pdfcraft,-data
RUN_DEPENDS-main =	${FULLPKGNAME-data}:print/pdfcraft,-data \
			devel/desktop-file-utils \
			graphics/vulkan-loader \
			wayland/wayland \
			x11/dbus \
			x11/gnome/zenity \
			x11/gtk+4,-guic \
			x11/xkbcommon

SITES.models =		https://ocrs-models.s3-accelerate.amazonaws.com/
SITES.license =		https://creativecommons.org/licenses/by-sa/4.0/
MODEL_FILES =		text-detection.onnx text-recognition.onnx
.for model in ${MODEL_FILES}
DISTFILES.models +=	pdfcraft/${model}{${model}}
.endfor
DISTFILES.license =	pdfcraft/CC-BY-SA-4.0.txt{legalcode.txt}
EXTRACT_ONLY =		${ALL_DISTFILES:Npdfcraft/*}

GIT_SOURCES = \
	craft-libs 010702ee9b030844c852939095b1da0d0656a890 pptx \
	craft-libs 1546550b0d4b79b8156a1fefaac8b3b317a4aee0 xfdf \
	craft-libs 23acc704933a9a5619a5cd1cddc9bb1fb5a9b2d3 xfa \
	craft-libs 8503f24cb845268f47f97e829491114fe9ad1bfa cms \
	craft-libs 9eb02be84f21898e247daeb2af5827c9fe85a3a0 pdf \
	craft-libs a7c6c9d61c6b36f4aec55b044f257340083fd571 image \
	craft-libs b8eaa8e2f3561f9fa27fcf3b7f67ce915e93caa3 docx \
	craft-libs bc4aae37bb219049596327fca971046dab0b78be pkcs7 \
	craft-libs e2dc681a3ad8a47038da2f11ddd2c5e1c0454991 ui \
	craft-libs e7873217287876f76a045550369c90a541693c33 postscript \
	craft-libs fbb2d4b299d87191a79444c6e4e02d69d9325059 mcp \
	craft-libs fbb3f47f7ae712192586984af11ca8f6c0f64d33 markdown \
	craft-libs fd5e2feff6ea25bed371a99cc732b3968d621505 web \
	egui 725a39fca940f77149ffbb9c58eef405a16997bc egui \
	winit a517b3846ee8457d84ee4975110bfda6797c44dc winit \
	winprint.rs 5f80b9baf6b58937c401656634e42d33f00ddf49 winprint \
	winprint.rs 64e7afd4d1483512385012c340ecaaaf0f3beb7f gdi \
	winprint.rs 72951d7455ebcab569fbd9386c513d3d9f53fb02 print

GIT_CRATES.pptx =	crates/drawingml crates/opc crates/pptx crates/xml crates/zip
GIT_CRATES.xfdf =	crates/codepage crates/html crates/mime crates/xfdf crates/xml
GIT_CRATES.xfa =	crates/xfa crates/xml
GIT_CRATES.cms =	crates/cms
GIT_CRATES.pdf =	crates/pdf crates/stream-codecs vendor/hayro \
			vendor/hayro-cmap vendor/hayro-interpret vendor/hayro-jbig2 \
			vendor/hayro-syntax
GIT_CRATES.image =	crates/dds crates/image-codecs crates/jpeg \
			crates/metadata crates/tiff
GIT_CRATES.docx =	crates/cfb crates/codepage crates/docx crates/drawingml \
			crates/flow crates/math crates/metafile crates/mime \
			crates/offcrypto crates/opc crates/rtf crates/wordmodel crates/xml crates/zip
GIT_CRATES.pkcs7 =	crates/pkcs7
GIT_CRATES.ui =		crates/cms crates/csv crates/docpath crates/fonts-build \
			crates/i18n crates/jpeg crates/keymap crates/menu crates/prc \
			crates/print crates/quit crates/tiff crates/ui crates/update crates/zip
GIT_CRATES.postscript =	vendor/hayro-postscript
GIT_CRATES.mcp =	crates/jsonrpc crates/mcp-server
GIT_CRATES.markdown =	crates/flow crates/html crates/markdown crates/math \
			crates/wordmodel crates/xml
GIT_CRATES.web =	crates/web-api crates/web-host
GIT_CRATES.egui =	crates/ecolor crates/eframe crates/egui crates/egui-wgpu \
			crates/egui-winit crates/egui_demo_lib crates/egui_extras \
			crates/egui_glow crates/egui_inspection \
			crates/egui_system_font_provider crates/egui_kittest \
			crates/emath crates/epaint crates/epaint_default_fonts
GIT_CRATES.winit =	.
GIT_CRATES.winprint =	.
GIT_CRATES.gdi =	crates/gdi
GIT_CRATES.print =	.

# Cargo keys this source by the manifest's requested rev, including its length.
GIT_REV.ui =		e2dc681a3

.for repo rev group in ${GIT_SOURCES}
GIT_REV.${group} ?=	${rev}
DIST_TUPLE +=		github storytold ${repo} ${rev} ../${group}
.endfor

CONFIGURE_STYLE =	cargo
MODCARGO_BUILD_ARGS =	--frozen -p pdfcraft -p pdfcraft-cli --bins
MODCARGO_TEST_ARGS =	--frozen -p pdfcraft -p pdfcraft-cli
# Use the framework TMPDIR for Cargo and its single-instance socket tests.
MODCARGO_ENV +=		CRAFT_FONTS_DIR=${WRKDIR}/fonts CRAFT_FONTS_REQUIRED=1 \
			CRAFT_I18N_DIR=${WRKDIR}/i18n CRAFT_I18N_REQUIRED=1 \
			TMPDIR=${TMPDIR}

DOCDIR =		${PREFIX}/share/doc/pdfcraft
MODELDIR =		${PREFIX}/share/pdfcraft/models

FONT_FAMILIES =		biz-ud-mincho biz-ud-pgothic lxgw-wenkai-lite \
			nanum-gothic noto-sans-arabic noto-sans-devanagari \
			noto-sans-georgian noto-sans-hebrew noto-sans-symbols \
			noto-sans-symbols-2 noto-sans-thai shippori-mincho

post-configure:
.for repo rev group in ${GIT_SOURCES}
	cd ${WRKDIR} && sh ${FILESDIR}/vendor-git ${repo} \
	    ${GIT_REV.${group}} ${group} ${GIT_CRATES.${group}}
.endfor

do-install:
.for bin in pdfcraft pdfcraft-cli
	${INSTALL_PROGRAM} ${MODCARGO_TARGET_DIR}/release/${bin} ${PREFIX}/bin/
.endfor
	${INSTALL_DATA_DIR} ${MODELDIR} ${DOCDIR}/fonts ${DOCDIR}/i18n
	cd ${FULLDISTDIR}/pdfcraft && ${INSTALL_DATA} ${MODEL_FILES} \
	    CC-BY-SA-4.0.txt ${MODELDIR}/
	${INSTALL_DATA} ${FILESDIR}/models-attribution ${MODELDIR}/ATTRIBUTION.txt
	cd ${WRKSRC} && ${INSTALL_DATA} ATTRIBUTION.md ATTRIBUTION.toml \
	    LICENSE-APACHE LICENSE-MIT NOTICE ${DOCDIR}/
	cd ${WRKSRC} && pax -rw licenses ${DOCDIR}
	${INSTALL_DATA} ${WRKSRC}/assets/fonts/OFL-*.txt ${DOCDIR}/fonts/
	${INSTALL_DATA} ${WRKDIR}/fonts/ATTRIBUTION.md ${DOCDIR}/fonts/
.for family in ${FONT_FAMILIES}
	${INSTALL_DATA} ${WRKDIR}/fonts/fonts/${family}/OFL.txt \
	    ${DOCDIR}/fonts/OFL-${family}.txt
.endfor
	cd ${WRKDIR}/i18n && ${INSTALL_DATA} ATTRIBUTION.md LICENSE-MIT \
	    LICENSE-APACHE ${DOCDIR}/i18n/
.for lib in edit-session lopdf vello_common vello_cpu
	${INSTALL_DATA_DIR} ${DOCDIR}/licenses/${lib}
	${INSTALL_DATA} ${WRKSRC}/vendor/${lib}/LICENSE* ${DOCDIR}/licenses/${lib}/
.endfor
	${INSTALL_DATA} ${WRKSRC}/assets/app-icon/LICENSE.txt ${DOCDIR}/LICENSE-icon
	${INSTALL_DATA} ${WRKSRC}/assets/icons/LICENSE-lucide.txt ${DOCDIR}/
	${INSTALL_DATA_DIR} ${PREFIX}/share/applications ${PREFIX}/share/icons
	# winit uses the executable name for WM_CLASS on OpenBSD X11.
	sed 's/^StartupWMClass=.*/StartupWMClass=pdfcraft/' \
	    ${WRKSRC}/packaging/linux/ai.storyteller.pdfcraft.desktop \
	    > ${WRKDIR}/ai.storyteller.pdfcraft.desktop
	${INSTALL_DATA} ${WRKDIR}/ai.storyteller.pdfcraft.desktop \
	    ${PREFIX}/share/applications/
	cd ${WRKSRC}/assets/app-icon && pax -rw hicolor ${PREFIX}/share/icons

.include "crates.inc"
.include <bsd.port.mk>
