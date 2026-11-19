ONLY_FOR_ARCHS =	amd64

COMMENT =		Solana command-line client and key generation tool

DIST_TUPLE =		github anza-xyz agave v4.4.1 . \
			github anza-xyz blstrs \
			a1faffa3420bdaf4102425c43f00cdd815d4f145 ../blstrs \
			github anza-xyz crossbeam \
			5ee1c6ec671d94db14ed9238eb99e59a84555aad ../crossbeam \
			github anza-xyz rust-rocksdb \
			4dba80dfe9434e595a971f5b2985ff9ba4fb31e7 ../rust-rocksdb
REVISION =		2
CATEGORIES =		net

# Apache-2.0
PERMIT_PACKAGE =	Yes

WANTLIB +=		${MODCARGO_WANTLIB} hidapi-libusb m usb-1.0 usbhid zstd

MODULES =		devel/cargo
LIB_DEPENDS =		archivers/zstd \
			comms/libhidapi>=0.15.0 \
			devel/libusb1

CONFIGURE_STYLE =	cargo
# Version output otherwise falls back to a random commit outside a git tree.
MAKE_ENV +=		CI_COMMIT=5bb304027fe1a166b02cb26dc8c161593f416fca
MODCARGO_BUILD_ARGS =	--frozen -p solana-cli -p solana-keygen \
			--bin solana --bin solana-keygen
# Ledger discovery on the libusb backend cannot rely on HID usage pages.
MODCARGO_NO_DEFAULT_FEATURES = Yes
MODCARGO_FEATURES =	solana-cli/remote-wallet-libusb \
			solana-keygen/remote-wallet-libusb

.include "crates.inc"

post-patch:
	${INSTALL_DATA} ${FILESDIR}/openbsd_ledger.rs ${WRKSRC}/remote-wallet/src/
	${INSTALL_DATA} ${FILESDIR}/ledger_uhid.c ${WRKSRC}/remote-wallet/
	${INSTALL_DATA} ${FILESDIR}/remote-wallet-build.rs \
	    ${WRKSRC}/remote-wallet/build.rs

do-install:
	${INSTALL_PROGRAM} ${MODCARGO_TARGET_DIR}/release/solana \
		${MODCARGO_TARGET_DIR}/release/solana-keygen ${PREFIX}/bin/

# CLI dev-dependencies pull in the validator. Exercise the shipped tools.
do-test:
	sh ${FILESDIR}/smoke-test ${MODCARGO_TARGET_DIR}/release

.include <bsd.port.mk>
