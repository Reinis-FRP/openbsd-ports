ONLY_FOR_ARCHS =	amd64

COMMENT =		CLI for developing Anchor programs on Solana
DIST_TUPLE =		github otter-sec anchor v1.2.2 .
PKGNAME =		anchor-cli-${DISTNAME:S/anchor-//}
CATEGORIES =		devel

# Apache-2.0
PERMIT_PACKAGE =	Yes

MODULES =		devel/cargo
WANTLIB +=		${MODCARGO_WANTLIB} m onig zstd
LIB_DEPENDS =		archivers/zstd textproc/oniguruma
RUN_DEPENDS =		agave-4.4.1p7:net/agave \
			cargo-build-sbf-4.5.0:devel/cargo-build-sbf \
			shells/bash

CONFIGURE_STYLE =	cargo
MODCARGO_BUILD_ARGS =	--frozen -p anchor-cli --bin anchor
MODCARGO_TEST_ARGS =	--frozen -p anchor-cli --lib debugger::

do-install:
	${INSTALL_PROGRAM} ${MODCARGO_TARGET_DIR}/release/anchor ${PREFIX}/bin/

.include "crates.inc"
.include <bsd.port.mk>
