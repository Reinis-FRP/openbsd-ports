ONLY_FOR_ARCHS =	amd64

COMMENT =		Cargo subcommands for Solana SBF programs
DISTNAME =		cargo-build-sbf-4.5.0
CATEGORIES =		devel
HOMEPAGE =		https://github.com/anza-xyz/cargo-build-sbf
SITES =			https://static.crates.io/crates/cargo-build-sbf/
DISTFILES =		${DISTNAME}.tar.gz{${DISTNAME}.crate}

# Apache-2.0
PERMIT_PACKAGE =	Yes
MODULES =		devel/cargo
WANTLIB +=		${MODCARGO_WANTLIB}
RUN_DEPENDS =		lang/rust solana-platform-tools-1.57:devel/solana-platform-tools
TEST_DEPENDS =		solana-platform-tools-1.57:devel/solana-platform-tools
CONFIGURE_STYLE =	cargo
MODCARGO_BUILD_ARGS =	--frozen --bins
MODCARGO_TEST_ARGS =	--frozen --lib
MODCARGO_ENV +=		SBF_PLATFORM_TOOLS=${TRUEPREFIX}/libexec/solana-platform-tools

do-install:
	${INSTALL_PROGRAM_DIR} ${PREFIX}/bin
	cd ${MODCARGO_TARGET_DIR}/release && \
	    ${INSTALL_PROGRAM} cargo-build-sbf cargo-test-sbf ${PREFIX}/bin/

# Run the compatibility fixture with the same compiler that users receive.
post-test:
	TMPDIR=${WRKDIR} sh ${FILESDIR}/smoke-test ${FILESDIR}/fixture \
	    ${MODCARGO_TARGET_DIR}/release ${MODCARGO_VENDOR_DIR} ${LOCALBASE}/bin

.include "crates.inc"
.include <bsd.port.mk>
