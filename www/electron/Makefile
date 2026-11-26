BROKEN-i386=v8/src/compiler/turboshaft/int64-lowering-reducer.h:295:24: error:
# call to member function 'Word32Constant' is ambiguous
# (but will probably run out of memory linking under DPB even if fixed)
BROKEN-powerpc64=Check failed: false. OS architecture not handled. (powerpc64)

USE_WXNEEDED=		Yes

DPB_PROPERTIES=		parallel parallel2
.if ${MACHINE_ARCH:Mi386}
DPB_PROPERTIES+=	lonesome
.endif

COMMENT=		cross-platform desktop applications

# sync with electron.port.mk
ELECTRON_V=		43.8.10
ELECTRON_DIST=		electron/resources
REVISION=		0

# make update-patches happy
FIX_EXTRACT_PERMISSIONS=Yes

DIST_TUPLE=		github electron electron v${ELECTRON_V} .

CATEGORIES=		www

HOMEPAGE=		https://www.electronjs.org/

MAINTAINER=		Fabien Romano <fabien@openbsd.org>

# MIT
PERMIT_PACKAGE=		Yes

# make versions
# electron/v43.8.10/DEPS
CHROMIUM_V=	150.0.7871.250
NODE_V=		24.22.1
NAN_V=		675cefebca42410733da8a454c8d9391fcebfbc2
SQUIRREL_MAC_V=	0e5d146ba13101a1302d59ea6e6e0b3cace4ae38
REACTIVEOBJC_V=	74ab5baccc6f7202c8ac69a8d1e152c29dc1ea76
MANTLE_V=	2a8e2123a3931038179ee06105c9e6ec336b12ea

DIST_TUPLE+=	github nodejs node v${NODE_V} .
DIST_TUPLE+=	github nodejs nan ${NAN_V} .
DIST_TUPLE+=	github Squirrel Squirrel.Mac ${SQUIRREL_MAC_V} .
DIST_TUPLE+=	github ReactiveCocoa ReactiveObjC ${REACTIVEOBJC_V} .
DIST_TUPLE+=	github Mantle Mantle ${MANTLE_V} .
DIST_TUPLE+=	github rollup rollup v4.60.4 .

CHROMIUM_RELEASE=	tagattie/FreeBSD-Electron/releases/download/v43.5.0
SITES.chromium=		${SITES.github}${CHROMIUM_RELEASE}/
CHROMIUM_ARCHIVE=	chromium-${CHROMIUM_V}.tar.xz
DISTFILES.chromium=	${CHROMIUM_ARCHIVE}.0 ${CHROMIUM_ARCHIVE}.1 \
			${CHROMIUM_ARCHIVE}.2
BUILD_DEPENDS+=		archivers/xz
EXTRACT_CASES+=		${CHROMIUM_ARCHIVE}.[012]);;

# uses pledge()
WANTLIB += FLAC GL X11 X11-xcb Xcomposite Xdamage Xext Xfixes
WANTLIB += Xi Xrandr Xrender Xtst atk-1.0 atk-bridge-2.0
WANTLIB += atspi c cairo cairo-gobject cbor crc32c crypto
WANTLIB += cups dav1d dbus-1 double-conversion drm epoll-shim
WANTLIB += execinfo expat fido2 fontconfig freetype
WANTLIB += gbm gdk-3 gdk_pixbuf-2.0 gio-2.0 glib-2.0
WANTLIB += gmodule-2.0 gobject-2.0 gthread-2.0 gtk-3
WANTLIB += harfbuzz harfbuzz-subset intl jpeg kvm lcms2
WANTLIB += m nspr4 nss3 nssutil3 openh264 openjp2 opus
WANTLIB += pango-1.0 pangocairo-1.0 plc4 plds4 pthread
WANTLIB += secret-1 smime3 snappy sndio tiff udev usbhid
WANTLIB += util webp webpdemux webpmux xcb xcb-shm xkbcommon
WANTLIB += xml2 xshmfence xslt z zstd

# Apply the Electron series first, then OpenBSD Chromium, then local changes.
.if empty(_GEN_PATCHES)
# pre-patch electron (1-2) > ALL_PATCHFILES(3) > PATCH_LIST(4-6)
ALL_PATCHFILES=	${FILESDIR}/chromium/patch-*
PATCH_CASES+=	${FILESDIR}/chromium/patch-*) \
		    ${PATCH} ${PATCH_DIST_ARGS} \
		    -z .orig.chromium \
		    -d ${WRKSRC}/ <$$patchfile || exit 1 ;;
PATCH_LIST=	patch-*
.else # pre-patch only, see targets chromium-patches
ALL_PATCHFILES=
PATCH_LIST=
.endif

COMPILER=	ports-clang

MODULES=	devel/berry \
		devel/cargo \
		lang/clang \
		lang/python \
		x11/qt5 \
		x11/qt6

MODCARGO_CARGOTOML=	${WRKSRC}/rollup/rust/Cargo.toml
MODCARGO_TARGET_DIR=	${WRKSRC}/rollup/rust/target
MODCARGO_BUILD=		No
MODCARGO_INSTALL=	No
MODCARGO_TEST=		No

MODCLANG_VERSION=	22
MODQT5_DEPS=		No
MODQT6_DEPS=		No
MODPY_RUNDEP=		No

MODBERRY_MODULE_FILE=	${.CURDIR}/modules.inc
MODBERRY_TARGETS=	${WRKSRC}/electron
MODBERRY_ENV+=		SENTRYCLI_SKIP_DOWNLOAD=1

MODBERRY_ARGS_TEST=	run test -- --ci
MODBERRY_ENV_TEST=	ELECTRON_SPEC_KEEP_RUNNING_INSTANCES=1 \
			LD_LIBRARY_PATH=${BUILDDIR} \
			DISPLAY=:1 \
			ELECTRON_OUT_DIR=${BUILDTYPE} \
			ELECTRON_EXE_PATH=${WRKSRC}/out/${BUILDTYPE}/electron \
			XDG_RUNTIME_DIR=${PORTHOME}/.run

# Keep the release yarn.lock. The GN build does not run the lint tools.

# XXX chromium ?
# + sysutils/libudev-openbsd VS LIB_DEPENDS ?
# - devel/libnotify
RUN_DEPENDS+=	devel/desktop-file-utils \
		devel/libnotify \
		devel/xdg-utils \
		fonts/noto/fonts \
		graphics/libexif \
		x11/gtk+3,-main \
		x11/gtk+4,-guic \
		x11/gtk+3,-cups

BUILD_DEPENDS+=	archivers/bzip2 \
		audio/pulseaudio \
		devel/bison \
		electron-esbuild-0.28.2:devel/electron-esbuild \
		devel/git \
		devel/gperf \
		devel/ninja \
		devel/rust-bindgen \
		graphics/png \
		lang/node \
		lang/rust \
		lang/rust,-rustfmt \
		net/libcares \
		shells/bash \
		sysutils/flock \
		sysutils/pciutils \
		www/nghttp2 \
		x11/py-xcbgen \
		${MODQT5_LIB_DEPENDS} \
		${MODQT6_LIB_DEPENDS} \
		${MODGCC4_CPPDEP} \
		${RUN_DEPENDS}

LIB_DEPENDS+=	archivers/snappy \
		audio/flac \
		audio/opus \
		devel/crc32c \
		devel/epoll-shim \
		devel/pango \
		graphics/jpeg \
		graphics/lcms2 \
		graphics/libwebp \
		graphics/openjp2 \
		graphics/tiff \
		math/double-conversion \
		multimedia/dav1d \
		multimedia/openh264 \
		print/cups,-libs \
		security/nss \
		sysutils/libudev-openbsd \
		textproc/libxml \
		textproc/libxslt \
		x11/gnome/at-spi2-core \
		x11/gnome/libsecret \
		x11/gtk+3,-main \
		x11/xkbcommon \
		${MODGCC4_CPPLIBDEP}

TEST_DEPENDS+=	devel/py-dbusmock \
		x11/dbus-python \
		devel/py-gobject3 \
		devel/glib2

USE_GMAKE=	Yes

# needed in early stage, for porters only ?
# XXX remember to review strip_binary in .gn files
# (without DEBUG_PACKAGES, ports doesn't strip and the build should handle that)
DEBUG_PACKAGES=	${BUILD_PACKAGES}

# dev flavor enable component build + dcheck (+ symbol, default)
FLAVORS=		dev
FLAVOR?=

.include <bsd.own.mk>

SUBST_VARS+=	ELECTRON_V MODCLANG_VERSION WRKDIR WRKSRC

# Collect all DISTS under 'src'
WRKDIST=	${WRKDIR}/src
WRKSRC=		${WRKDIR}/src

BUILD_TARGET=	electron \
		electron:node_headers \
		chromedriver \
		licenses \
		version

DIST_TARGET=	electron/${ELECTRON_V}

# GN_SYSTEM_LIBS : See ${WRKSRC}/build/linux/unbundle/README
# These libraries are used from the system and the build infrastructure
# removes them from the bundled third_party directory and replaces them
# with hooks to use them from the system.
# Some GN_SYSTEM_LIBS just change args, prefer GN_SYSTEM_LIBS to further reduce
# tarball content/size based on this list.
GN_SYSTEM_LIBS=	crc32c \
		dav1d \
		double-conversion \
		flac \
		fontconfig \
		freetype \
		libdrm \
		libjpeg \
		libpng \
		libsecret \
		libwebp \
		libxml \
		libxslt \
		openh264 \
		opus \
		snappy \
		zstd

# make gn_args-system, search ${WRKDIR}/gn_args-electron.list use_system/bundled
GN_ARGS=	use_system_cares=true \
		use_system_harfbuzz=true \
		use_system_nghttp2=true
# pdfium only
GN_ARGS+=	use_system_lcms2=true \
		use_system_libopenjpeg2=true \
		use_system_libpng=true \
		use_system_libtiff=true \
		use_system_zlib=true

# NOTE. GN_SYSTEM_LIBS & GN_ARGS use_system_* are hard to maintain and also time
# consuming. Some of the depends may work assuming their ports got updated but
# going through some updates (lot of reverse deps) would take times and also
# represent a risk of revert if something goes wrong. On the other end Electron
# and Chromium are moving verry fast with many changes month by month so this
# update requirement isn't possible to acheive atm.
# XXX Ideally we need a way to move those depends just to make sure nothing
# use it when we enable the system shim.
# XXX make an exhaustive list to maintain
# GN_SYSTEM_LIBS+=highway, ports update + last tag < chromium depends
# GN_SYSTEM_LIBS+=icu, check to do (may work)

# XXX drop highway on chromium 146.0.7680.166
# highway 84379d1c73de9681b54fbe1c035a23c7bd5d272d 25/09/2025
# highway 1.3.0 from 14/08/2025, diff to check
# devel/highway 1.2.0, ports to update
# third_party/blink/renderer/platform/wtf/text/string_impl.h:368:5: error:
# unknown type name 'HWY_LANES_CONSTEXPR'
# GN_SYSTEM_LIBS+=highway

# XXX to test again on chromium 144.0.7559.236
# break with ICU 78.1
# ld.lld: error: undefined symbol: icu::UnicodeString::doReplace(
#	int, int, std::__Cr::basic_string_view<char16_t,
#	std::__Cr::char_traits<char16_t>>)
# >>> referenced by unistr.h:2024 (.../include/unicode/unistr.h:2024)
# >>> obj/v8/v8_base_without_compiler/js-date-time-format.o:
# (v8::internal::JSDateTimeFormat::TimeZoneId(
#	v8::internal::Isolate*, icu::TimeZone const&)
# GN_SYSTEM_LIBS+=icu

# make gn_args-check, check www/chromium against ${WRKDIR}/gn_args-electron.list
EXTRA_CPPFLAGS=	-I${LOCALBASE}/include/libepoll-shim \
		-idirafter ${LOCALBASE}/include -idirafter ${X11BASE}/include
GN_ARGS+=	clang_use_chrome_plugins=false \
		disable_fieldtrial_testing_config=true \
		enable_backup_ref_ptr_support=false \
		enable_hangout_services_extension=true \
		enable_remoting=false \
		extra_cppflags=\"${EXTRA_CPPFLAGS}\" \
		fatal_linker_warnings=false \
		icu_use_data_file=false \
		treat_warnings_as_errors=false \
		use_allocator_shim=false \
		use_custom_libunwind=true \
		use_kerberos=false \
		use_pulseaudio=true \
		use_sndio=true \
		use_sysroot=false \
		use_system_libdrm=true \
		v8_enable_cet_ibt=true \
		use_qt5=true \
		use_qt6=true \
		moc_qt5_path=\"${MODQT5_MOC}\" \
		moc_qt6_path=\"${MODQT6_MOC}\"

# XXX disable by electron
# enable_cet_shadow_stack

# rust
MAKE_ENV+=	RUSTC_BOOTSTRAP=1
GN_ARGS+=	enable_rust=true \
		rust_sysroot_absolute=\"${LOCALBASE}\"

# The Chromium archive contains the codec sources matched to Electron's patches.
GN_ARGS+=	use_vaapi=true

_EXTRA_LDFLAGS=-L${LOCALBASE}/lib -L${X11BASE}/lib
.if ${MACHINE_ARCH} == "aarch64"
GN_ARGS+=	extra_ldflags=\"${_EXTRA_LDFLAGS}\"
PKG_ARGS+=	-Dswiftshader=0
FFMPEG_TARGET=	arm64
.elif ${MACHINE_ARCH} == "amd64"
BUILD_DEPENDS+=	devel/nasm
GN_ARGS+=	extra_ldflags=\"${_EXTRA_LDFLAGS}\"
PKG_ARGS+=	-Dswiftshader=1
FFMPEG_TARGET=	x64
.elif ${MACHINE_ARCH} == "i386"
BUILD_DEPENDS+=	devel/nasm
GN_ARGS+=	extra_ldflags=\"${_EXTRA_LDFLAGS} -Wl,--no-keep-memory\"
PKG_ARGS+=	-Dswiftshader=1
FFMPEG_TARGET=	ia32
.else
PKG_ARGS+=	-Dswiftshader=0
.endif

FFMPEG_BRAND=	Chrome
FFMPEG_SRC=	${WRKSRC}/third_party/ffmpeg
FFMPEG_CONFIG=	${FFMPEG_SRC}/chromium/config
FFMPEG_BDIR=	${FFMPEG_SRC}/build.${FFMPEG_TARGET}.openbsd/${FFMPEG_BRAND}
FFMPEG_CDIR=	${FFMPEG_CONFIG}/${FFMPEG_BRAND}/openbsd/${FFMPEG_TARGET}

GN_ARGS+=	ffmpeg_branding=\"${FFMPEG_BRAND}\" \
		proprietary_codecs=true

# XXX dwz doesn't work with chromium yet
# dwz: .debug/electron.dbg: Too many DIEs, not optimizing
DWZ = :

# always build with symbol, bsd.port.mk will strip
# XXX bsd.port.mk doesn't strip, it does only to split debug-package
GN_ARGS+=	symbol_level=1

# always link static into bin, no component
GN_ARGS+=	is_component_build=false \
		is_component_ffmpeg=false

# do not use lto, otherwise it needs -fno-ret-protector & -mno-retpoline
GN_ARGS+=	use_thin_lto=false \
		thin_lto_enable_optimizations=true

GN_ARGS+=	override_electron_version=\"${ELECTRON_V}\"

# XXX need chrome-linux-${CHROMIUM_V}-llvm<XX>.profdata${EXTRACT_SUFX}
GN_ARGS+=	chrome_pgo_phase=0 \
		v8_builtins_profiling_log_file=\"\"

.if ${FLAVOR:Mdev}
# dev only, enable dcheck_always_on
# electron exceed MAXTSIZE, build in component
# fix component build, hint ld to builddir
GN_ARGS+=	dcheck_always_on=true
GN_ARGS+=	is_component_build=true
MAKE_ENV+=	LD_LIBRARY_PATH=${BUILDDIR}
.endif

BUILDTYPE=	Release
BUILDDIR=	${WRKSRC}/out/${BUILDTYPE}
ELECTRON_GN=	//electron/build/args/release.gn
MAKE_ENV+=	ELECTRON_OUT_DIR=${BUILDTYPE}

DEVFRONT_T =	${WRKSRC}/third_party/devtools-frontend/src/

post-extract:
	# Stream the verified fragments directly into tar.
	set -o pipefail; cd ${FULLDISTDIR} && \
	    cat ${DISTFILES.chromium} | xz -dc -T${MAKE_JOBS} | \
	    tar -xf - -C ${WRKDIR}
	# setup distfiles into src
	mv ${WRKDIR}/chromium-${CHROMIUM_V} ${WRKSRC}
	mv ${WRKDIR}/electron-${ELECTRON_V} ${WRKSRC}/electron
	mv ${WRKDIR}/rollup-4.60.4 ${WRKSRC}/rollup
	mv ${WRKDIR}/node-${NODE_V} ${WRKSRC}/third_party/electron_node
	mv ${WRKDIR}/nan-${NAN_V} ${WRKSRC}/third_party/nan
	mv ${WRKDIR}/Squirrel.Mac-${SQUIRREL_MAC_V} \
		${WRKSRC}/third_party/squirrel.mac
	mkdir -p ${WRKSRC}/third_party/squirrel.mac/vendor
	mv ${WRKDIR}/ReactiveObjC-${REACTIVEOBJC_V} \
		${WRKSRC}/third_party/squirrel.mac/vendor/ReactiveObjC
	mv ${WRKDIR}/Mantle-${MANTLE_V} \
		${WRKSRC}/third_party/squirrel.mac/vendor/Mantle

pre-patch:
	${MODPY_BIN} ${FILESDIR}/apply-electron-patches ${WRKSRC}

.if empty(_GEN_PATCHES)
post-patch:
	# force agnostic version of system bin
	ln -sf ${MODPY_BIN} ${WRKDIR}/bin/python
	ln -sf ${LOCALBASE}/bin/llvm-ar-${MODCLANG_VERSION} ${WRKDIR}/bin/ar
	# hotfix node bin into third_party/node
	mkdir -p ${WRKSRC}/third_party/node/openbsd/node-openbsd/bin
	ln -sf ${TRUEPREFIX}/bin/node \
		${WRKSRC}/third_party/node/openbsd/node-openbsd/bin/node
	# hotfix esbuild modules into third_party/devtools-frontend
	rm -rf ${DEVFRONT_T}/node_modules/esbuild
	ln -sf ${MODBERRY_INSTALL_DIST}/electron-esbuild/node_modules/esbuild \
		${DEVFRONT_T}/node_modules/esbuild
	# hotfix esbuild bin
	mkdir -p ${DEVFRONT_T}/third_party/esbuild
	ln -sf ${LOCALBASE}/libexec/electron-esbuild/esbuild \
		${DEVFRONT_T}/third_party/esbuild/esbuild
	# replace system path & variables in patch using SUBST_CMD
.for f in \
    base/base_paths_posix.cc \
    base/process/process_handle_openbsd.cc \
    electron/script/yarn.js \
    build/gn_run_binary.py \
    build/linux/strip_binary.gni \
    build/rust/rust_bindgen.gni \
    build/rust/rust_bindgen_generator.gni \
    v8/tools/run.py \
    tools/protoc_wrapper/protoc_wrapper.py \
    third_party/blink/renderer/build/scripts/run_with_pythonpath.py \
    third_party/electron_node/src/env.cc \
    third_party/perfetto/src/base/utils.cc \
    third_party/vulkan-loader/src/loader/vk_loader_platform.h
	${SUBST_CMD} ${WRKSRC}/${f}
.endfor
	mkdir -p ${WRKSRC}/third_party/rust-toolchain/bin \
	    ${WRKSRC}/third_party/gperf/cipd/bin
	ln -sf ${LOCALBASE}/bin/rustc ${WRKSRC}/third_party/rust-toolchain/bin/rustc
	ln -sf ${LOCALBASE}/bin/gperf ${WRKSRC}/third_party/gperf/cipd/bin/gperf

.endif

pre-configure:
	# configure ffmpeg
	cd ${WRKSRC}/media/ffmpeg && \
		${MODPY_BIN} scripts/build_ffmpeg.py openbsd ${FFMPEG_TARGET} \
		--config-only --branding=${FFMPEG_BRAND}
	cd ${FFMPEG_BDIR} && ${GMAKE} ffversion.h
	${INSTALL_DATA_DIR} ${FFMPEG_CDIR}
.for f in config.h config.asm config_components.h config_components.asm \
    libavcodec libavformat libavutil
	cp -pR ${FFMPEG_BDIR}/${f} ${FFMPEG_CDIR}
.endfor

# Configuration consists of three steps:
# 1. replace the specified bundled libraries with hooks to use the system libs
# 2. bootstrap gn, the tool to generate ninja files
# 3. run gn with the appropriate configuration arguments
do-configure:
	@${MODCARGO_configure}
	cd ${WRKSRC} && \
		${SETENV} ${MAKE_ENV} ${MODPY_BIN} \
		build/linux/unbundle/replace_gn_files.py \
		--system-libraries ${GN_SYSTEM_LIBS}
	cd ${WRKSRC}/tools/gn && \
		${SETENV} ${MAKE_ENV} LDFLAGS=-L${LOCALBASE}/lib ${MODPY_BIN} \
		bootstrap/bootstrap.py --no-clean -s --skip-generate-buildfiles
	ln -fs ${WRKSRC}/out/${BUILDTYPE}/gn ${WRKDIR}/bin/gn
	cd ${WRKSRC} && ${SETENV} ${CONFIGURE_ENV} \
		${WRKSRC}/out/${BUILDTYPE}/gn gen out/${BUILDTYPE} \
		--args="import(\"${ELECTRON_GN}\") ${GN_ARGS}"

do-build:
	@${MODCARGO_BUILD_TARGET}
	cd ${DEVFRONT_T}/node_modules/@rollup && \
	    mkdir -p rollup-openbsd && \
	    cp ${MODCARGO_TARGET_DIR}/release/libbindings_napi.so \
	        rollup-openbsd/rollup.openbsd.node && \
	    echo '{"main":"./rollup.openbsd.node"}' > rollup-openbsd/package.json
	cd ${WRKSRC} && ${SETENV} ${MAKE_ENV} \
		ninja -j ${MAKE_JOBS} -C out/${BUILDTYPE} ${BUILD_TARGET}

do-install:
	${INSTALL_DATA_DIR} ${PREFIX}/${DIST_TARGET}
.for f in electron mksnapshot v8_context_snapshot_generator chromedriver
	${INSTALL_PROGRAM} ${BUILDDIR}/${f} ${PREFIX}/${DIST_TARGET}
.endfor
	${SUBST_PROGRAM} ${FILESDIR}/electron \
		${PREFIX}/${DIST_TARGET}/electron.sh
	${SUBST_PROGRAM} ${FILESDIR}/electron_nosandbox \
		${PREFIX}/${DIST_TARGET}/electron_nosandbox.sh
	ln -s ${TRUEPREFIX}/${DIST_TARGET}/electron.sh \
		${PREFIX}/bin/electron
	# copy .pak
	${INSTALL_DATA} ${BUILDDIR}/*.pak ${PREFIX}/${DIST_TARGET}
	# copy locales & ressources
	${INSTALL_DATA_DIR} ${PREFIX}/${DIST_TARGET}/locales
	${INSTALL_DATA} ${BUILDDIR}/locales/* ${PREFIX}/${DIST_TARGET}/locales
	${INSTALL_DATA_DIR} ${PREFIX}/${DIST_TARGET}/resources
	cp -Rp ${BUILDDIR}/resources/* ${PREFIX}/${DIST_TARGET}/resources
	@chown -R ${SHAREOWN}:${SHAREGRP} ${PREFIX}/${DIST_TARGET}/resources
	# ANGLE, EGL, Vk, ...
.for f in libEGL libGLESv2 libvk_swiftshader libVkICD_mock_icd
	${INSTALL_DATA} ${BUILDDIR}/${f}.so ${PREFIX}/${DIST_TARGET}
.endfor
	${INSTALL_DATA} ${BUILDDIR}/libvulkan.so.1 \
		${PREFIX}/${DIST_TARGET}/libvulkan.so
.if ${MACHINE_ARCH} != "aarch64"
	# SwiftShader
	${INSTALL_DATA} ${BUILDDIR}/libvk_swiftshader.so \
		${PREFIX}/${DIST_TARGET}
.endif
	# sandbox, unveil & pledge
	${INSTALL_DATA_DIR} ${PREFIX}/share/examples/electron
	${INSTALL_DATA} ${FILESDIR}/unveil.cdm ${PREFIX}/share/examples/electron
	${INSTALL_DATA} ${FILESDIR}/unveil.gpu ${PREFIX}/share/examples/electron
.for f in main utility_audio utility_network utility_video
	${INSTALL_DATA} ${FILESDIR}/{pledge,unveil}.${f} \
		${PREFIX}/share/examples/electron
.endfor
	# data
.for f in LICENSE LICENSES.chromium.html protoc snapshot_blob.bin \
    v8_context_snapshot.bin version vk_swiftshader_icd.json
	${INSTALL_DATA} ${BUILDDIR}/${f} ${PREFIX}/${DIST_TARGET}
.endfor
	# electron
	${INSTALL_DATA_DIR} ${PREFIX}/${DIST_TARGET}/node_headers
	cp -Rp ${BUILDDIR}/gen/node_headers/* \
		${PREFIX}/${DIST_TARGET}/node_headers
	@chown -R ${SHAREOWN}:${SHAREGRP} ${PREFIX}/${DIST_TARGET}/node_headers
	${INSTALL_DATA_DIR} ${PREFIX}/${DIST_TARGET}/gen/electron/buildflags
	${INSTALL_DATA} ${BUILDDIR}/gen/electron/buildflags/* \
		${PREFIX}/${DIST_TARGET}/gen/electron/buildflags
	${INSTALL_DATA_DIR} ${PREFIX}/${DIST_TARGET}/gen/v8
	${INSTALL_DATA} ${BUILDDIR}/gen/v8/embedded.S \
		${PREFIX}/${DIST_TARGET}/gen/v8/embedded.S
	# ports' install location
	${INSTALL_DATA_DIR} ${PREFIX}/${ELECTRON_DIST}

# Tests requires:
# *** Custom pledge/unveil configurations
# mkdir -p /etc/electron
# cp files/unveil* files/pledge* /etc/electron
# echo "/\trwcx" >> /etc/electron/unveil.main
# echo "/\trwcx" >> /etc/electron/unveil.utility_network
# *** Fake display, env DISPLAY=:1
# Xvfb :1 &

# Notes :
# use a wrapper script, skip bin/electron check, electron/spec/api-app-spec.ts
# no crashReporter, electron/spec/disabled-tests.json
# depends on /etc/electron, need XDG_RUNTIME_DIR, set to ${PORTHOME}/.run
do-test:
	rm -rf ${PORTHOME}/.config
	mkdir -p ${PORTHOME}/.config
	mkdir -p -m 0700 ${PORTHOME}/.run
	ulimit -n `ulimit -Hn` ; \
	cd ${WRKSRC}/electron && \
	${MODBERRY_CMD_TEST} ${MODBERRY_ARGS_TEST}

### targets for port maintainer(s)

_compile_commands:
# run under _pbuild
	cd ${WRKSRC} && ${MODPY_BIN} tools/clang/scripts/generate_compdb.py \
		-p out/${BUILDTYPE} -o ${WRKSRC}/compile_commands.json \
		${BUILD_TARGET}
	mkdir -p ${WRKSRC}/.cache && chmod a+rwx ${WRKSRC}/.cache

compile_commands:
	${_PBUILD} make _compile_commands

# extract versions from upstream
_TMP?=		${TMPDIR}/electron-${ELECTRON_V}
versions:
	@mkdir -p "${_TMP}"
	@ftp -VMo "${_TMP}/DEPS" \
	    https://raw.githubusercontent.com/electron/electron/v${ELECTRON_V}/DEPS
	@${MODPY_BIN} ${FILESDIR}/versions "${_TMP}/DEPS"


# OpenBSD's last 150.0.7871 Chromium port, from 2026-07-27.
CHROMIUM_PORT_REV=	6edefd9db6245d818bea82e7b0e32e7c16f1de81
CHROMIUM_HISTORY?=	${_TMP}/openbsd-ports
CHROMIUM_PORT?=		${CHROMIUM_HISTORY}/www/chromium

chromium-patches:
	${MAKE} -D _GEN_PATCHES patch
.if ${CHROMIUM_PORT} == "${CHROMIUM_HISTORY}/www/chromium"
	@if [ ! -d "${CHROMIUM_HISTORY}/.git" ]; then \
		mkdir -p "${_TMP}" && git init -q "${CHROMIUM_HISTORY}" && \
		git -C "${CHROMIUM_HISTORY}" remote add origin \
		    https://github.com/openbsd/ports.git; fi
	git -C "${CHROMIUM_HISTORY}" fetch --depth=1 --filter=blob:none \
	    origin ${CHROMIUM_PORT_REV}
	git -C "${CHROMIUM_HISTORY}" sparse-checkout set www/chromium
	git -C "${CHROMIUM_HISTORY}" checkout --detach ${CHROMIUM_PORT_REV}
.endif
	${MODPY_BIN} ${FILESDIR}/compare-chromium \
	    "${WRKSRC}" "${CHROMIUM_PORT}" "${_TMP}"


_gn_args-electron-list:
# run under _pbuild
	# ${WRKDIR}/gn_args-electron.list
	@if [ ! -e "${WRKDIR}/gn_args-electron.list" ] ; then \
		cd ${WRKSRC} && ${SETENV} ${CONFIGURE_ENV} \
			gn args out/${BUILDTYPE} --list \
			| sed -E '/^[[:space:]]+From/s/\:[[:digit:]]+$$//' \
			> ${WRKDIR}/gn_args-electron.list ; \
	fi

_gn_args-chromium-list:
# run under _pbuild
	# ${WRKDIR}/gn_args-chromium.list
	@if [ ! -e "${WRKDIR}/gn_args-chromium.list" ] ; then \
		cd ${PORTSDIR}/www/chromium ; \
		_wrksrc=$$(make show=WRKSRC) ; \
		_gn=$$_wrksrc/out/${BUILDTYPE}/gn ; \
		cd $$_wrksrc && ${SETENV} ${CONFIGURE_ENV} \
			$$_gn args out/${BUILDTYPE} --list \
			| sed -E '/^[[:space:]]+From/s/\:[[:digit:]]+$$//' \
			> ${WRKDIR}/gn_args-chromium.list ; \
	fi

gn_args-diff:
	make configure
	${_PBUILD} make _gn_args-electron-list
	# configure www/chromium in order to compare
	cd ${PORTSDIR}/www/chromium && make configure ; cd -
	${_PBUILD} make _gn_args-chromium-list
	# ${WRKDIR}/gn_args.diff
	@${_PBUILD} sh -c \
	"diff -uN ${WRKDIR}/gn_args{-chromium,-electron}.list > \
		${WRKDIR}/gn_args.diff" || wc -l ${WRKDIR}/gn_args.diff

gn_args-check:
	make configure
	${_PBUILD} make _gn_args-electron-list
	# check GN_ARGS from www/chromium
	@cd ${PORTSDIR}/www/chromium && make show=GN_ARGS \
	    | perl -pe 's/\\"[^"]*\\"(*SKIP)(*F)|\h+/$&\n/g' \
	    | sort \
	    | while IFS=';' read _gn ; do \
		_arg=$${_gn%%=*} ; \
		_value=$${_gn##*=} ; \
		grep -A1 "^$${_arg}$$" ${WRKDIR}/gn_args-electron.list \
		    | grep -q "$$_value" || \
		echo "Wrong" $$( \
		    grep -A1 "^$${_arg}$$" ${WRKDIR}/gn_args-electron.list) \
		    "\nInstead of $$_arg = $$_value"; \
	done

gn-system:
	make configure
	${_PBUILD} make _gn_args-electron-list
	@_pkgs=$$(grep -C0 -e '^[^[:space:]]*use_system_.*' \
		${WRKDIR}/gn_args-electron.list) ; \
	echo "# use_system* !true\n" $$_pkgs | fmt ; \
	for _pkg in $$_pkgs; do \
		grep -A1 "^$$_pkg" ${WRKDIR}/gn_args-electron.list | \
			grep -q true || echo $$( \
			grep -A1 "^$$_pkg" ${WRKDIR}/gn_args-electron.list) \
		; \
	done
	@_pkgs=$$(grep -C0 -e '^[^[:space:]]*bundle_.*' \
		${WRKDIR}/gn_args-electron.list) ; \
	echo "# *bundle* !false\n" $$_pkgs | fmt ; \
	for _pkg in $$_pkgs; do \
		grep -A1 "^$$_pkg" ${WRKDIR}/gn_args-electron.list | \
			grep -q false || echo $$( \
			grep -A1 "^$$_pkg" ${WRKDIR}/gn_args-electron.list) \
		; \
	done
	@_pkgs=$$(ls ${WRKSRC}/build/linux/unbundle/ \
		| sed -n /\.gn$$/s/\.gn$$//p) ; \
	echo "# build/linux/unbundle !set\n" $$_pkgs | fmt ; \
	for _pkg in $$_pkgs; do \
		echo "${GN_SYSTEM_LIBS}" | \
			grep -q "$$_pkg" || echo $$_pkg ; \
	done

.include "crates.inc"
.include "modules.inc"

.include <bsd.port.mk>
