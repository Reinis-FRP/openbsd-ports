ONLY_FOR_ARCHS =	amd64

COMMENT =		Microsoft's open source desktop code editor

V =			1.148.0
DISTNAME =		vscode-${V}
CATEGORIES =		editors
HOMEPAGE =		https://code.visualstudio.com/

# MIT
PERMIT_PACKAGE =	Yes

SITES =			https://codeload.github.com/microsoft/vscode/tar.gz/refs/tags/
DISTFILES =		${DISTNAME}.tar.gz{${V}}
DIST_SUBDIR =		vscode
PNPM_V =		12.16.0
SITES.pnpm =		https://registry.npmjs.org/@pnpm/wasm/-/
DISTFILES.pnpm =	wasm-${PNPM_V}.tgz
SITES.npm =		https://registry.npmjs.org/
.include "modules.inc"

SITES.vsix =		https://github.com/microsoft/
.for repo ext ver in js-debug js-debug 1.140.1 \
    js-debug-companion js-debug-companion 1.1.3 \
    js-profile-visualizer vscode-js-profile-table 1.0.11
DISTFILES.vsix += \
	{vscode-${repo}/releases/download/v${ver}/}ms-vscode.${ext}.${ver}.vsix
.endfor
VSIX_EXCLUDE.js-debug =	-x 'extension/src/*.win32-*.node'
EXTRACT_ONLY =		${DISTNAME}.tar.gz wasm-${PNPM_V}.tgz

MODULES =		lang/python www/electron
MODPY_RUNDEP =		No
MODELECTRON_TARGET =	code-oss
MODELECTRON_APP_NAME =	Code - OSS
MODELECTRON_UNVEIL =	Yes
MODELECTRON_BUILD =	No
MODELECTRON_INSTALL =	No

BUILD_DEPENDS +=	archivers/unzip devel/electron-esbuild devel/gmake \
			lang/node security/heimdal,-libs textproc/py-yaml
RUN_DEPENDS +=		devel/desktop-file-utils devel/git \
			security/heimdal,-libs textproc/ripgrep
COMPILER =		base-clang ports-gcc
WANTLIB +=		${COMPILER_LIBCXX} m util X11 xkbfile

MAKE_ENV +=		BUILD_SOURCEVERSION=661626d0e72f97806550790a98fb87116c9db825 \
			BUILD_SOURCE_DATE=2026-11-24T10:26:34Z \
			CI=true ESBUILD_BINARY_PATH=${LOCALBASE}/libexec/electron-esbuild/esbuild \
			MAKE_JOBS=${MAKE_JOBS} GOMAXPROCS=${MAKE_JOBS} \
			PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1 \
			pnpm_config_store_dir=${WRKDIR}/store \
			pnpm_config_trust_lockfile=true
NO_TEST =		Yes

post-patch:
	${SUBST_CMD} ${WRKSRC}/src/vs/base/node/ripgrep.ts
	cd ${WRKDIR}/package && /usr/bin/patch -s -V none -p0 \
	    < ${FILESDIR}/pnpm-openbsd.patch

do-configure:
	ln -sf ${WRKDIR}/package/dist/pnpm.mjs ${WRKDIR}/bin/pnpm
	ulimit -c 0 && ulimit -d $$(ulimit -Hd) && ulimit -n $$(ulimit -Hn) && \
	    ${SETENV} ${MAKE_ENV} ${MODPY_BIN} ${FILESDIR}/seed-store.py \
	    ${WRKSRC} ${FULLDISTDIR} ${WRKDIR}
.for subdir in . build
	cd ${WRKSRC}/${subdir} && ulimit -c 0 && \
	    ulimit -d $$(ulimit -Hd) && ulimit -n $$(ulimit -Hn) && \
	    ${SETENV} ${MAKE_ENV} RAYON_NUM_THREADS=2 TOKIO_WORKER_THREADS=2 \
	    pnpm_config_network_concurrency=2 pnpm install \
	    --offline --frozen-lockfile --ignore-scripts
.endfor

post-configure:
	cd ${WRKSRC} && sh ${FILESDIR}/prepare-native ${FILESDIR} ${LOCALBASE}

do-build:
	cd ${WRKSRC} && ${SETENV} ${MAKE_ENV} ${ELECTRON_REBUILD_ENV} \
	    sh ${FILESDIR}/build-editor ${MAKE_JOBS} ${LOCALBASE}

do-install:
	${MODELECTRON_UNVEIL_INSTALL}
	${INSTALL_DATA_DIR} ${PREFIX}/${MODELECTRON_DIST_T}
	cp -Rp ${WRKDIR}/VSCode-openbsd-x64/. ${PREFIX}/${MODELECTRON_DIST_T}/
	cd ${PREFIX}/${MODELECTRON_DIST_T}/extensions && \
	    rm -f microsoft-authentication/esbuild.mts${PATCHORIG}
	${SUBST_PROGRAM} ${FILESDIR}/vscode ${PREFIX}/bin/vscode
	${INSTALL_DATA_DIR} ${PREFIX}/share/applications ${PREFIX}/share/pixmaps
	${INSTALL_DATA_DIR} ${PREFIX}/${MODELECTRON_DIST_T}/resources/linux
	${INSTALL_DATA} ${WRKSRC}/resources/linux/code.png \
	    ${PREFIX}/${MODELECTRON_DIST_T}/resources/linux/code.png
	ln -s ../../${MODELECTRON_DIST_T}/resources/linux/code.png \
	    ${PREFIX}/share/pixmaps/code-oss.png
.for f in code.desktop code-url-handler.desktop
	sed -e 's/@@NAME_LONG@@/Code - OSS/g' -e 's/@@NAME@@/code-oss/g' \
	    -e 's/@@DESKTOP_NAME@@/Code - OSS/g' -e 's/@@ICON@@/code-oss/g' \
	    -e 's/@@URLPROTOCOL@@/code-oss/g' \
	    -e 's|@@EXEC@@|${TRUEPREFIX}/bin/vscode|g' \
	    ${WRKSRC}/resources/linux/${f} > ${WRKDIR}/${f}
	${INSTALL_DATA} ${WRKDIR}/${f} \
	    ${PREFIX}/share/applications/${f:S/code/code-oss/}
.endfor
.for repo ext ver in js-debug js-debug 1.140.1 \
    js-debug-companion js-debug-companion 1.1.3 \
    js-profile-visualizer vscode-js-profile-table 1.0.11
	mkdir -p ${WRKDIR}/vsix/${ext}
	unzip -oq ${FULLDISTDIR}/ms-vscode.${ext}.${ver}.vsix \
	    'extension/*' -d ${WRKDIR}/vsix/${ext} \
	    ${VSIX_EXCLUDE.${ext}}
	mv ${WRKDIR}/vsix/${ext}/extension \
	    ${PREFIX}/${MODELECTRON_DIST_T}/extensions/ms-vscode.${ext}
.endfor

.include <bsd.port.mk>
