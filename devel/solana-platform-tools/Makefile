ONLY_FOR_ARCHS =	amd64
COMMENT =		Solana SBF cross compiler and target libraries
DISTNAME =		solana-platform-tools-1.57
CATEGORIES =		devel
HOMEPAGE =		https://github.com/anza-xyz/platform-tools
# Rust and Cargo: MIT/Apache-2.0; LLVM: Apache-2.0 with LLVM exception;
# newlib: various permissive licenses
PERMIT_PACKAGE =	Yes

RUST_REV =		ae660768afd467d131776918a1627e18920b230c
DIST_TUPLE =		github anza-xyz rust ${RUST_REV} . \
			github anza-xyz llvm-project a4135d044fb7a19f2b7e7d04d0373cd2ba190e6e src/llvm-project \
			github anza-xyz cargo bbbd3a76017677319ff85d096a7c9bee9813c3db ../cargo \
			github anza-xyz newlib f0f8003daa3f71a1a3e2f067f8d5bb46e2756514 ../newlib
SITES.rust =		https://static.rust-lang.org/dist/
DISTFILES.rust =	rustc-1.95.0-src.tar.xz
SITES.bootstrap =	https://github.com/semarie/openbsd-rustc-bootstrap/releases/download/openbsd/
DISTFILES.bootstrap =	rustc-bootstrap-amd64-1.94.0.tar.lz

# Anza pins an older cc crate in src/bootstrap/Cargo.lock.
SITES.crate =		https://static.crates.io/crates/cc/
DISTFILES.crate =	cargo/cc-1.1.23.tar.gz{cc-1.1.23.crate}
BUILD_DEPENDS +=	devel/cargo-generate-vendor
WRKDIST =		${WRKDIR}/rust-${RUST_REV}
MODULES =		lang/python
MODPY_RUNDEP =		No
BUILD_DEPENDS +=	archivers/libarchive devel/cmake/core devel/ninja \
			devel/gmake shells/bash lang/rust
LIB_DEPENDS +=		net/curl
WANTLIB +=		c c++ c++abi crypto curl execinfo m pthread ssl z
COMPILER =		base-clang

# The official source distribution supplies the locked crate vendor tree and
# upstream submodule sources. Extract only the parts used by this build.
RUST_DIST =		rustc-1.95.0-src
EXTRACT_CASES =		${RUST_DIST}.tar.xz) \
			bsdtar -xf ${FULLDISTDIR}/$$archive \
			    ${RUST_DIST}/vendor ${RUST_DIST}/.cargo \
			    ${RUST_DIST}/library/backtrace \
			    ${RUST_DIST}/src/tools/cargo \
			    ${RUST_DIST}/src/tools/rustc-perf ;;

MAKE_ENV +=		CARGO_NET_OFFLINE=true CARGO_HOME=${WRKDIR}/cargo-home \
			LIBGIT2_NO_PKG_CONFIG=1 LIBSSH2_NO_PKG_CONFIG=1 \
			OPENSSL_NO_VENDOR=1
SUBST_VARS +=		WRKDIR MAKE_JOBS
HOST_TRIPLE =		x86_64-unknown-openbsd
STAGE1 =		${WRKSRC}/build/${HOST_TRIPLE}/stage1
LLVM =			${WRKSRC}/build/${HOST_TRIPLE}/llvm
TOOLS =			${PREFIX}/libexec/solana-platform-tools

# Resolve rustc's per-target library hashes when pkg_create reads the list.
PKG_ARGS +=		`perl ${FILESDIR}/plist-hashes ${WRKINST}${TRUEPREFIX} ${PKGDIR}/PLIST`

post-extract:
	cd ${WRKDIR}/${RUST_DIST} && for d in vendor library/backtrace \
	    src/tools/cargo src/tools/rustc-perf; do \
		rmdir ${WRKSRC}/$$d 2>/dev/null || true; \
		mv $$d ${WRKSRC}/$$d || exit 1; \
	done
	mkdir -p ${WRKSRC}/.cargo
	cp ${WRKDIR}/${RUST_DIST}/.cargo/config.toml ${WRKSRC}/.cargo/
	mv ${WRKDIR}/cc-1.1.23 ${WRKSRC}/vendor/
	${LOCALBASE}/bin/cargo-generate-vendor -e ${FULLDISTDIR}/cargo/cc-1.1.23.tar.gz \
	    ${WRKSRC}/vendor/cc-1.1.23

post-patch:
	# Accept LibreSSL from the matching OpenBSD ports tree.
	sed -i -e "/ => ('.', '.'),/h" -e "/ => ('.', '.', '.'),/h" \
	    -e "/_ => version_error(),/{g; s/(.*) =>/_ =>/; }" \
	    ${WRKSRC}/vendor/openssl-sys-*/build/main.rs
	sed -i 's/"files":{[^}]*}/"files":{}/' \
	    ${WRKSRC}/vendor/openssl-sys-*/.cargo-checksum.json \
	    ${WRKSRC}/vendor/psm-0.1.26/.cargo-checksum.json

do-configure:
	${SUBST_CMD} -m 644 -c ${FILESDIR}/bootstrap.toml ${WRKSRC}/bootstrap.toml

do-build:
	cd ${WRKSRC} && ${SETENV} ${MAKE_ENV} ${MODPY_BIN} x.py build \
	    --stage 1 -j ${MAKE_JOBS} compiler/rustc library src/tools/rustdoc
	cd ${WRKDIR}/cargo && ${SETENV} ${MAKE_ENV} ${LOCALBASE}/bin/cargo \
	    --config ${WRKSRC}/.cargo/config.toml build --frozen --release \
	    --bin cargo -j ${MAKE_JOBS}
	${SETENV} ${MAKE_ENV} sh ${FILESDIR}/build-newlib ${WRKDIR}/newlib \
	    ${LLVM} ${WRKDIR}/newlib-out ${MAKE_JOBS}

do-test:
	printf '%s\n' '#[no_mangle] pub extern "C" fn entrypoint(_: *mut u8) -> u64 { 0 }' | \
	    ${STAGE1}/bin/rustc - --crate-name smoke --crate-type cdylib \
	    --target sbpfv3-solana-solana -O -o ${WRKDIR}/smoke.so
	${LLVM}/bin/llvm-readelf -h ${WRKDIR}/smoke.so

do-install:
	${INSTALL_PROGRAM_DIR} ${TOOLS}/rust/bin ${TOOLS}/rust/lib ${TOOLS}/llvm/bin ${TOOLS}/llvm/lib
	cd ${STAGE1}/bin && ${INSTALL_PROGRAM} rustc rustdoc \
	    ${WRKDIR}/cargo/target/release/cargo ${TOOLS}/rust/bin/
	${INSTALL_DATA} ${STAGE1}/lib/librustc_driver-*.so ${TOOLS}/rust/lib/
.for triple in ${HOST_TRIPLE} sbpf-solana-solana sbpfv1-solana-solana \
		sbpfv2-solana-solana sbpfv3-solana-solana
	${INSTALL_DATA_DIR} ${TOOLS}/rust/lib/rustlib/${triple}
	cd ${STAGE1}/lib/rustlib/${triple} && \
	    pax -rw lib ${TOOLS}/rust/lib/rustlib/${triple}/
.endfor
	${INSTALL_PROGRAM_DIR} ${TOOLS}/rust/lib/rustlib/${HOST_TRIPLE}/bin
	cd ${TOOLS}/rust/lib/rustlib/${HOST_TRIPLE}/bin && \
	    ln -s ../../../../../llvm/bin/lld rust-lld && \
	    ln -s ../../../../../llvm/bin/llvm-objcopy rust-objcopy
	cd ${LLVM}/bin && for t in clang lld llvm-ar llvm-objcopy llvm-objdump \
	    llvm-readobj; do ${INSTALL_PROGRAM} $$t ${TOOLS}/llvm/bin/$$t; done
	cd ${TOOLS}/llvm/bin && ln -s clang clang++ && ln -s lld ld.lld && \
	    ln -s llvm-ar llvm-ranlib && ln -s llvm-readobj llvm-readelf
	cd ${LLVM}/lib && pax -rw clang ${TOOLS}/llvm/lib/
.for arch in v0 v1 v2 v3
	${INSTALL_DATA_DIR} ${TOOLS}/llvm/sbpf${arch:S/v0//} ${TOOLS}/llvm/lib/sbpf${arch:S/v0//}
	cd ${WRKDIR}/newlib-out/${arch}/sbf-solana && \
	    pax -rw include ${TOOLS}/llvm/sbpf${arch:S/v0//} && \
	    ${INSTALL_DATA} lib/libc.a lib/libm.a ${TOOLS}/llvm/lib/sbpf${arch:S/v0//}/
.endfor
	cd ${TOOLS}/llvm && ln -s sbpf/include include && \
	    cd lib && ln -s sbpf/libc.a libc.a && ln -s sbpf/libm.a libm.a
	${INSTALL_DATA_DIR} ${TOOLS}/licenses
	cd ${WRKSRC} && ${INSTALL_DATA} COPYRIGHT LICENSE-APACHE LICENSE-MIT ${TOOLS}/licenses/
	${INSTALL_DATA} ${WRKDIR}/cargo/LICENSE-THIRD-PARTY ${TOOLS}/licenses/cargo-third-party
	${INSTALL_DATA} ${WRKSRC}/src/llvm-project/llvm/LICENSE.TXT ${TOOLS}/licenses/llvm
	${INSTALL_DATA} ${WRKDIR}/newlib/COPYING.NEWLIB ${TOOLS}/licenses/newlib

.include <bsd.port.mk>
