ONLY_FOR_ARCHS =	amd64

COMMENT-main =		desktop presentation editor and slide show
COMMENT-cli =		headless presentation editor, renderer and MCP server
V =			0.19.1
DISTNAME =		deckcraft-${V}
DECKCRAFT_REV =		4a980c33ddf37eb64157777fcaeab6d19baa5878
DIST_TUPLE =		github storytold deckcraft ${DECKCRAFT_REV} . \
			github storytold craft-fonts \
			37827adc41fd165f9a9234ac40f94dbad353aa69 ../fonts
WRKDIST =		${WRKDIR}/deckcraft-${DECKCRAFT_REV}
MULTI_PACKAGES =	-main -cli
CATEGORIES =		productivity x11

# MIT or Apache-2.0, with separate font, codec and profile licences.
PERMIT_PACKAGE =	Yes

HOMEPAGE =		https://github.com/storytold/deckcraft

MODULES =		devel/cargo
WANTLIB =		${MODCARGO_WANTLIB} m
RUN_DEPENDS-main =	devel/desktop-file-utils \
			graphics/vulkan-loader \
			misc/shared-mime-info \
			wayland/wayland \
			x11/dbus \
			x11/gnome/zenity \
			x11/gtk+4,-guic \
			x11/xkbcommon

# Different commits of craft-libs carry identically named crates.
GIT_SOURCES = \
	craft-libs 010702ee9b030844c852939095b1da0d0656a890 media \
	craft-libs 254e9a409b410dc93c0e06ff66cd1e767893ac32 format \
	craft-libs 63196e66f3033a6623eb629af95a104d26986725 ui \
	craft-libs 8048f8ef92da80fb3ce5a056d59ca591ee66faf1 pptx \
	craft-libs bc186ca719958fcce3b6ee24794277eabed14609 mcp \
	craft-libs d0193d7e94b5ec180ecc2a152ea72047d39ac632 rtf \
	craft-libs f428c3d36e0faf3924b76956a01334c051543527 metafile \
	craft-libs fd5e2feff6ea25bed371a99cc732b3968d621505 web \
	egui 6d518fc799681571963842f95b55a6b68431b15a egui \
	winit a517b3846ee8457d84ee4975110bfda6797c44dc winit

GIT_CRATES.media =	crates/audio crates/media crates/video
GIT_CRATES.format =	crates/codepage crates/deckcraft-format \
			crates/mime crates/zip
GIT_CRATES.ui =		crates/fonts-build crates/i18n crates/pdf crates/ui \
			crates/update vendor/krilla
GIT_CRATES.pptx =	crates/cfb crates/drawingml crates/opc crates/pptx \
			crates/xml crates/zip
GIT_CRATES.mcp =	crates/mcp-server
GIT_CRATES.rtf =	crates/codepage crates/rtf
GIT_CRATES.metafile =	crates/metafile
GIT_CRATES.web =	crates/web-api crates/web-host
GIT_CRATES.egui =	crates/ecolor crates/eframe crates/egui \
			crates/egui-wgpu crates/egui-winit \
			crates/egui_extras crates/egui_glow \
			crates/egui_demo_lib crates/egui_inspection \
			crates/egui_kittest crates/egui_system_font_provider \
			crates/emath crates/epaint \
			crates/epaint_default_fonts
GIT_CRATES.winit =	.

.for repo rev group in ${GIT_SOURCES}
DIST_TUPLE +=		github storytold ${repo} ${rev} ../${group}
.endfor

CONFIGURE_STYLE =	cargo
MODCARGO_BUILD_ARGS =	--frozen -p deckcraft -p deckcraft-cli --bins
# The loopback control-server tests can stall under parallel libtest.
MODCARGO_LIBTEST_ARGS =	--test-threads=1
MODCARGO_TEST_ARGS =	--frozen -p deckcraft -p deckcraft-cli \
			-p deckcraft-fonts -p deckcraft-media
MODCARGO_ENV +=		CRAFT_FONTS_DIR=${WRKDIR}/fonts CRAFT_FONTS_REQUIRED=1 \
			TMPDIR=${TMPDIR}

DOCDIR =		${PREFIX}/share/doc
LINUXDIR =		${WRKSRC}/packaging/linux
APP_ID =		ai.storyteller.deckcraft
FONT_FAMILIES =		biz-ud-mincho biz-ud-pgothic caladea carlito inter \
			lato liberation-mono liberation-sans \
			liberation-serif lxgw-wenkai-lite merriweather \
			montserrat nanum-gothic noto-sans-arabic \
			noto-sans-devanagari noto-sans-georgian \
			noto-sans-hebrew noto-sans-symbols \
			noto-sans-symbols-2 noto-sans-thai nunito-sans \
			open-sans playfair-display poppins roboto \
			shippori-mincho

post-configure:
.for repo rev group in ${GIT_SOURCES}
	cd ${WRKDIR} && sh ${FILESDIR}/vendor-git ${repo} ${rev} ${group} \
	    ${GIT_CRATES.${group}}
.endfor

do-install:
.for bin in deckcraft deckcraft-cli
	${INSTALL_PROGRAM} ${MODCARGO_TARGET_DIR}/release/${bin} ${PREFIX}/bin/
	${INSTALL_DATA_DIR} ${DOCDIR}/${bin}/fonts ${DOCDIR}/${bin}/egui-fonts \
	    ${DOCDIR}/${bin}/pdf-writer
	cd ${WRKSRC} && ${INSTALL_DATA} ATTRIBUTION.md LICENSE-APACHE \
	    LICENSE-MIT NOTICE ${DOCDIR}/${bin}/
	# The locked Symphonia crates reference the MPL. Ship its full text.
	${INSTALL_DATA} ${MODCARGO_VENDOR_DIR}/colored-2.2.0/LICENSE \
	    ${DOCDIR}/${bin}/MPL-2.0.txt
	${INSTALL_DATA} ${WRKDIR}/ui/licenses/pdf-writer/LICENSE* \
	    ${DOCDIR}/${bin}/pdf-writer/
	${INSTALL_DATA} ${WRKDIR}/fonts/ATTRIBUTION.md ${DOCDIR}/${bin}/fonts/
.for family in ${FONT_FAMILIES}
	${INSTALL_DATA} ${WRKDIR}/fonts/fonts/${family}/OFL.txt \
	    ${DOCDIR}/${bin}/fonts/OFL-${family}.txt
.endfor
	${INSTALL_DATA} ${WRKDIR}/egui/crates/epaint_default_fonts/fonts/*.txt \
	    ${DOCDIR}/${bin}/egui-fonts/
.endfor
	${INSTALL_DATA_DIR} ${PREFIX}/share/applications ${PREFIX}/share/icons \
	    ${PREFIX}/share/metainfo ${PREFIX}/share/mime/packages
	# winit uses the executable name for WM_CLASS on OpenBSD X11.
	sed -e 's/^StartupWMClass=.*/StartupWMClass=deckcraft/' \
	    -e 's/^Keywords=presentation;/Keywords=/' \
	    ${LINUXDIR}/${APP_ID}.desktop > ${WRKDIR}/${APP_ID}.desktop
	${INSTALL_DATA} ${WRKDIR}/${APP_ID}.desktop ${PREFIX}/share/applications/
	${INSTALL_DATA} ${LINUXDIR}/${APP_ID}.mime.xml \
	    ${PREFIX}/share/mime/packages/${APP_ID}.xml
	sed -e 's/@VERSION@/${V}/g' -e 's/@DATE@/2026-11-25/g' \
	    ${LINUXDIR}/${APP_ID}.metainfo.xml.in > ${WRKDIR}/metainfo.xml
	${INSTALL_DATA} ${WRKDIR}/metainfo.xml \
	    ${PREFIX}/share/metainfo/${APP_ID}.metainfo.xml
	cd ${WRKSRC}/assets/app-icon && pax -rw hicolor ${PREFIX}/share/icons

.include "crates.inc"
.include <bsd.port.mk>
