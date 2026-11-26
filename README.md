# Private OpenBSD ports

This overlay adds ports to an existing OpenBSD ports tree. Use a builder with
the same OpenBSD release and architecture as the installation host, and a
matching ports tree and package mirror.

Tested on OpenBSD 8.0-release/amd64.

| Port | Package | Architecture | Contents |
| --- | --- | --- | --- |
| [net/agave](#netagave) | `agave-4.4.1p7` | amd64 | Solana CLI, key generation, local test validator, and optional Ledger support |
| [devel/solana-platform-tools](#develsolana-platform-tools) | `solana-platform-tools-1.57` | amd64 | OpenBSD-hosted Rust cross compiler and Solana SBF target libraries |
| [devel/cargo-build-sbf](#develcargo-build-sbf) | `cargo-build-sbf-4.5.0` | amd64 | Cargo subcommands for building and testing Solana SBF programs |
| [devel/anchor-cli](#develanchor-cli) | `anchor-cli-1.2.2` | amd64 | Anchor CLI for building, testing and deploying Solana programs |
| [www/electron](#wwwelectron) | `electron-43.8.10p0` | amd64 | Electron runtime, tested on 8.0-current |
| [editors/vscode](#editorsvscode) | `vscode-1.148.0` | amd64 | Code - OSS desktop editor |

## Build a package

OpenBSD searches `/usr/ports/mystuff` for local ports by default. On a builder
where that directory does not yet exist, clone the overlay as your regular user.

```sh
overlay=/usr/ports/mystuff
doas mkdir -m 0755 "$overlay" &&
    doas chown "$(id -un):$(id -gn)" "$overlay" &&
    (umask 022 && git clone git@github.com:Reinis-FRP/openbsd-ports.git "$overlay")
```

If you already use `mystuff` for other ports, clone this repository elsewhere,
set `overlay` to its absolute path, and export
`PORTSDIR_PATH="/usr/ports:/usr/ports/mystuff:$overlay"`.

With `PORTS_PRIVSEP=Yes`, the fetch and build users (by default `_pfetch`
and `_pbuild`) need to read the checkout and traverse its parent directories.
For the `doas.conf` rules and the `fix-permissions` target, see
[bsd.port.mk(5), PORTS_PRIVSEP](https://man.openbsd.org/bsd.port.mk.5#PORTS_PRIVSEP).

The ports framework can build missing dependencies from source. To speed up a
clean build, you can preinstall any of the binary packages listed in the
port's section with `doas pkg_add`.

For eight parallel build jobs, set `MAKE_JOBS=8` in the builder’s `/etc/mk.conf`.
Choose the job count based on the online CPUs (`sysctl -n hw.ncpuonline`) and
available memory.

Check the port's build notes before starting. The following example builds Agave.

```sh
cd "$overlay/net/agave"
make build && make test && make package
```

The ports framework fetches and verifies the distfiles. When rebuilding after
an overlay update, run `make clean` in the port directory first so the new
sources and patches are used.

## Install a package

Sign the package on the builder, including when installing it there. If the
builder needs its first package-signing key pair, generate it once with
`doas signify -Gns /etc/signify/my-pkg.sec -p /etc/signify/my-pkg.pub`.
This creates a passphrase-free private key that stays on the builder.
From the port directory, sign the archive at its build path. [pkg_sign(1)](https://man.openbsd.org/pkg_sign.1)

```sh
pkgfile=$(make show=PKGFILE)
doas pkg_sign -s signify2 -s /etc/signify/my-pkg.sec -o "$(dirname "$pkgfile")" "$pkgfile"
```

On the builder, install the signed archive directly. For another host,
copy the signed archive there, set `pkgfile` to its path, and provision
`/etc/signify/my-pkg.pub` through a trusted channel. On either host,
verify the signature and preview the installation before installing.

```sh
unset TRUSTED_PKG_PATH
signify -Vz -p /etc/signify/my-pkg.pub -x "$pkgfile" -m /dev/null &&
    pkg_add -n "$pkgfile" && doas pkg_add "$pkgfile"
```

The same commands upgrade an older revision of the same port. If you publish
the package in a repository on `PKG_PATH`, users can also update by name
with `doas pkg_add -u agave`, for example.

## net/agave

### Build considerations

Agave builds the three binaries from pinned sources with Cargo's `--frozen`
option. To preinstall its dependencies, run this in `net/agave` so bindgen
gets the LLVM version selected by the ports tree. `libusb1--` selects the
standard flavor.

```sh
llvm_port=/usr/ports/devel/llvm/$(make show=MODCLANG_VERSION)
doas pkg_add rust cargo-generate-vendor gmake protobuf zstd libhidapi \
    libusb1-- "$(make -C "$llvm_port" show=FULLPKGNAME)"
```

To check the package's library dependencies after packaging, run
`make port-lib-depends-check` and `make lib-depends-check`.

### Runtime notes

- **Ledger access.** The packaged hotplug helper grants a dedicated group
  access to the Ledger's `uhid(4)` interface. The default is `ledgerusb`.
  To choose a different name, set `AGAVE_LEDGER_GROUP` in the builder's
  `/etc/mk.conf` before a clean build. If replacing an active group, unplug
  the device first and verify its node returns to `root:wheel` mode `0600`.
  Hook setup is documented in `/usr/local/share/doc/pkg-readmes/agave`.
  [Package README](net/agave/pkg/README)
- **Local validator.** RPC, WebSocket, and faucet ports (TCP 8899, 8900, and
  9900 by default) listen on all IPv4 interfaces even with
  `--bind-address 127.0.0.1`. Restrict inbound access with PF when testing
  locally. If startup reports an allocation failure, check the shell's soft
  data limit. A 2 GiB limit has been sufficient for startup.
- **Off-chain signing.** With the Nano X Solana app 1.12.1, Agave 4.4.1's
  `solana sign-offchain-message` can print 64 Base58 `1` characters after
  the device rejects the message. That all-zero signature indicates failure.

### Smoke tests

After installation, run these scripts as a regular user from the checkout.
On a host without the overlay, copy the scripts from `net/agave/files/` and
pass the installed binary directory as their first argument.

```sh
cd "$overlay/net/agave"
sh files/smoke-test /usr/local/bin
sh files/validator-smoke-test /usr/local/bin
```

The first test prints the CLI and keygen versions and verifies a signature
using a temporary keypair. The second starts a disposable validator, waits
for slots to advance, and checks that a local faucet airdrop credits exactly
1,000,000,000 lamports. It stops the validator and removes its ledger on exit.

For an optional hardware check, unlock the Ledger, open its Solana app, and
run `solana-keygen pubkey usb://ledger` from the account configured for Ledger
access. A successful read prints the wallet's public key.

## devel/solana-platform-tools

This port builds Anza's v1.57 SBF compiler and target libraries.
You can preinstall its build dependencies with this command.

```sh
doas pkg_add rust cargo-generate-vendor cmake-core ninja gmake bash \
    python%3 libarchive xz curl
```

## devel/cargo-build-sbf

You can preinstall the build dependencies for the two Cargo subcommands.

```sh
doas pkg_add rust cargo-generate-vendor
```

Install the signed [solana-platform-tools](#develsolana-platform-tools) package
before testing or using the subcommands. Running `make test` in
`devel/cargo-build-sbf` invokes `cargo test-sbf --tools-version v1.57 --arch v3`
on the bundled fixture. Its host-side test loads the generated `.so` and
executes it in the Solana bytecode interpreter.

After installation, build the Solana Foundation's native Hello Solana example
from a scratch directory.

```sh
git clone --depth 1 --revision=b8008ebc18d6523f3d4559f0a034766f13e72ddb \
    https://github.com/solana-foundation/program-examples.git
cd program-examples/basics/hello-solana/native
cargo build-sbf --tools-version v1.57 --arch v3 \
    --manifest-path program/Cargo.toml --sbf-out-dir out
```

A successful build produces `out/hello_solana_program.so`.

## devel/anchor-cli

### Build considerations

Anchor builds offline from the v1.2.2 workspace and its locked crates.
You can preinstall its build dependencies with this command.

```sh
doas pkg_add rust cargo-generate-vendor oniguruma zstd
```

### Runtime notes

The CLI uses cargo-build-sbf 4.5.0 with platform-tools 1.57 (architecture v3), Agave
4.4.1p7 for localnet, host Cargo for IDL generation, and Bash for test scripts.
Leave `RUSTUP_TOOLCHAIN` unset when using packaged Rust.
The debugger can resolve project sources. The packaged SBF compiler omits
the Rust standard-library sources.

New projects resolve their own Cargo and npm dependencies on first use.
Keep `anchor-lang`, `anchor-spl` and `@anchor-lang/core` in the manifests
of projects that use them. The default LiteSVM template uses Rust tests.

### Smoke tests

Both smoke tests use jq. The validator test also uses Node and npm.

Run each check as a regular user with several GiB free in `TMPDIR`. Both tests
use temporary wallets and workspaces. The legacy validator inherits the shell's
soft data limit (`ulimit -Sd`). Follow the [net/agave](#netagave) guidance on validator
startup memory and restricting inbound access with PF.

```sh
cd "$overlay/devel/anchor-cli"
export PATH="$(make show=WRKINST)/usr/local/bin:$PATH"
sh files/smoke-test litesvm
sh files/smoke-test validator
```

Both commands check the SBF program, IDL and TypeScript types. The first
runs the generated LiteSVM instruction test. The second starts Agave with
`anchor test --validator legacy`, deploys over loopback and checks that
the counter advances from zero to one.

## www/electron

Electron 43.8.10 was built, packaged and tested on OpenBSD 8.0-current/amd64
(`GENERIC.MP#163`, November 20, 2026). The installed package reported
Chromium 150.0.7871.250 and Node 24.22.1. A sandboxed BrowserWindow completed
a renderer-to-main IPC round trip under Xvfb and exited with status 0.

The port derives from Fabien Romano’s Electron 41.10.6 port and retains its
OpenBSD pledge and unveil integration. It streams the three FreeBSD Chromium
archive fragments during extraction. The historical OpenBSD Chromium 150 port
supplies the platform patches and the Cargo recipe for Rollup 4.60.4. The
shared BSD patches also draw on the FreeBSD Electron 43.8.10 port.
[Original port](https://github.com/opencrew-tech/openbsd-electron/tree/033b8dcdbba860fc0ea2d7c6bfd80a40b8cf3c04)
· [OpenBSD Chromium baseline](https://github.com/openbsd/ports/tree/6edefd9db6245d818bea82e7b0e32e7c16f1de81/www/chromium)
· [FreeBSD Electron sources and checksums](https://github.com/freebsd/freebsd-ports/tree/9d136dd8c7f497580f5db8dc54f3b90e0c4b937b/devel/electron43)

Use a matching snapshot package mirror and stock ports tree. The tested tree was
`d9fc0a1bf36574e20fc55a7dd34c0459c1440b78` (Go 1.27.3). Put the overlay
**first** so the build selects its esbuild 0.28.2p0 with the JavaScript module,
Berry 4.18.0 with the offline lockfile helper, and py-dbusmock. Both privsep
accounts need read access to both checkouts. Use the standard privsep
doas rules that preserve the environment for `_pbuild` and `_pfetch`.
With the exports below, `doas -u _pbuild make -C "$overlay/www/electron"
show=MODBERRY_GEN_SCRIPT` should print the helper path under the overlay.
The snapshot VM used a local wrapper to preserve its build environment
and scope unsigned dependency installs to its scratch package directory.

Run from the overlay root. On this VM the dedicated FFS scratch mount had
`nodev,nosuid`; Electron’s build tools also require `wxallowed` during the
build. The original mount flags can be restored after packaging. With eight
vCPUs and 15 GiB of RAM, the build finished at six parallel jobs. Scratch
usage rose from 25.8 GiB after resizing to 66.3 GiB during packaging.

```sh
overlay=$(pwd -P)
export PORTSDIR=/usr/ports    # path to the matching stock checkout
export PORTSDIR_PATH="$overlay:$PORTSDIR"
build=$(mktemp -d /tmp/ports-build/electron.XXXXXXXX)
chmod 755 "$build"
doas install -d -o _pfetch "$build/dist"
doas install -d -o _pbuild "$build/obj" "$build/packages" "$build/plist"
install -d -m 1777 "$build/t"
export TMPDIR="$build/t" PKG_TMPDIR="$build/t"
doas mount -u -o nodev,nosuid,wxallowed /tmp/ports-build

port() {
    runner=$1
    shift
    "$runner" -C "$overlay/www/electron" PORTSDIR="$PORTSDIR" PORTSDIR_PATH="$PORTSDIR_PATH" \
        PORTS_PRIVSEP=Yes MAKE_JOBS=6 \
        WRKOBJDIR="$build/obj" WRKOBJDIR_MFS="$build/obj" \
        BULK_COOKIES_DIR="$build/obj/bulk" UPDATE_COOKIES_DIR="$build/obj/update" \
        DISTDIR="$build/dist" PACKAGE_REPOSITORY="$build/packages" \
        PLIST_REPOSITORY="$build/plist" "$@"
}
port make checksum && port make package
port make show=PKGFILE
doas mount -u -o nodev,nosuid,nowxallowed /tmp/ports-build
```

All 1,304 distfiles passed checksum. For updates, `port make versions`
reads the tagged Electron DEPS. Run `port make chromium-patches` in a
fresh object tree to compare against the pinned OpenBSD revision. For
43.8.10, 1,587 patches applied directly, five needed merging, four were
specific to the Chromium port’s codec sources, and no target files were
missing.

Run the library checks as the build account so its dependency cache is
writable. The explicit make command keeps the checker’s recursive lookups
on the selected stock tree even when it lives outside /usr/ports.

```sh
build_make() {
    doas -u _pbuild env TMPDIR="$build/t" PORTSDIR_PATH="$PORTSDIR_PATH" make "$@"
}
check_make="make PORTSDIR=$PORTSDIR PORTS_PRIVSEP=No TMPDIR=$build/t"
port build_make PORTS_PRIVSEP=No \
    "_CHECK_LIB_DEPENDS=env PORTSDIR=$PORTSDIR MAKE='$check_make' $PORTSDIR/infrastructure/bin/check-lib-depends" \
    port-lib-depends-check lib-depends-check
for d in www/electron devel/berry devel/esbuild devel/py-dbusmock; do
    (cd "$overlay/$d" && "$PORTSDIR/infrastructure/bin/portcheck" -p "$overlay") || exit 1
done
```

Install the package using the [package installation instructions](#install-a-package).
For a local test of the unsigned archive on the builder, scope trust to its
package directory. The smoke test starts an authenticated Xvfb display with
a temporary home and a D-Bus session, then runs the installed launcher with
the normal OpenBSD sandbox settings. The preload checks that the renderer is
sandboxed. Success prints the three versions, a `pong/<uuid>` reply with the
renderer’s original nonce, and `IPC exit 0`. Logs stay under `$TMPDIR`.

```sh
pkgfile=$(port make show=PKGFILE)
doas env TRUSTED_PKG_PATH="$build/packages/amd64/all/" PKG_TMPDIR="$build/t" TMPDIR="$build/t" pkg_add -n "$pkgfile" &&
    doas env TRUSTED_PKG_PATH="$build/packages/amd64/all/" PKG_TMPDIR="$build/t" TMPDIR="$build/t" pkg_add "$pkgfile"
pkg_check -n -qq -I -x
sh "$overlay/www/electron/files/ipc-smoke/run-xvfb"
```

## editors/vscode

Code - OSS 1.148.0 uses the overlay's Electron 43.8.10 and its native headers.
The desktop port derives from Fabien Romano's 1.134.0 launcher, Open VSX
configuration, unveil profile, and BSD native module patches. The build uses
the desktop and build-tool pnpm workspaces from Microsoft's tag. It uses the
upstream pnpm 12.16.0 WebAssembly distribution with OpenBSD file-open flags.
All 2,357 inputs, including the registry archives and three debugger VSIX
files, are checksummed. pnpm installs from those archives offline. The editor
uses system ripgrep and the kqueue backend of Parcel's file watcher.

The built-in GitHub Copilot extension and Copilot CLI agent require a native
Copilot runtime unavailable for OpenBSD. Foundry Local on-device chat
dictation and the native OS-proxy diagnostic also lack OpenBSD binaries.
Open VSX supports portable extensions on this port.
[Microsoft sources](https://github.com/microsoft/vscode/tree/1.148.0)
· [Fabien Romano's original ports](https://github.com/opencrew-tech/openbsd-electron/tree/033b8dcdbba860fc0ea2d7c6bfd80a40b8cf3c04)
· [FreeBSD port](https://github.com/freebsd/freebsd-ports/tree/main/editors/vscode)

Install the matching Electron package first. From the overlay root, use the
same stock checkout, overlay-first search path and privsep environment as in
the Electron instructions. Give the editor its own scratch directory.

```sh
overlay=$(pwd -P)
export PORTSDIR=/tmp/ports-build/electron.hexOUBgx/current-ports
export PORTSDIR_PATH="$overlay:$PORTSDIR"
build=$(mktemp -d /tmp/ports-build/vsc.XXXXXXXX)
chmod 755 "$build"
doas install -d -o _pfetch "$build/dist"
doas install -d -o _pbuild "$build/obj" "$build/packages" "$build/plist"
install -d -m 1777 "$build/t"
export TMPDIR="$build/t" PKG_TMPDIR="$build/t"
# This VM uses Electron's privsep wrapper. Retarget only its scratch paths.
prior=${PORTSDIR%/*}
sed -e "s|$prior/t|$build/t|g" -e "s|$prior/packages|$build/packages|g" \
    "$prior/as-user" > "$build/as-user"
chmod 755 "$build/as-user"
port() {
    make -C "$overlay/editors/vscode" SUDO="$build/as-user" PORTSDIR="$PORTSDIR" \
        PORTSDIR_PATH="$PORTSDIR_PATH" PORTS_PRIVSEP=Yes MAKE_JOBS=8 \
        WRKOBJDIR="$build/obj" WRKOBJDIR_MFS="$build/obj" \
        BULK_COOKIES_DIR="$build/obj/bulk" UPDATE_COOKIES_DIR="$build/obj/update" \
        DISTDIR="$build/dist" PACKAGE_REPOSITORY="$build/packages" \
        PLIST_REPOSITORY="$build/plist" "$@"
}
port checksum && port package
pkgfile=$(port show=PKGFILE)
doas env TRUSTED_PKG_PATH="$build/packages/amd64/all/" PKG_TMPDIR="$build/t" \
    pkg_add "$pkgfile"
sh "$overlay/editors/vscode/files/desktop-test/run-xvfb"
```

The test uses Xvfb, D-Bus, xdotool, Python and Node. It creates a temporary
home, installs a local VSIX and exercises GUI editing, workspace search,
external changes, Git, TypeScript navigation and two integrated terminals.
Its output gives the test directory. To check gallery access independently,
use a second disposable profile and inspect the signature result.

```sh
testhome=$(mktemp -d "$build/t/gallery.XXXXXXXX")
HOME="$testhome" dbus-run-session -- vscode --user-data-dir "$testhome/profile" \
    --extensions-dir "$testhome/ext" --verbose --install-extension redhat.vscode-yaml
```

Verified on OpenBSD 8.0-current/amd64 (GENERIC.MP #163) with eight jobs
and 15 GiB of RAM. The installed package passed the Xvfb test with the
shipped sandbox. Physical desktop and OpenBSD 8.0-release testing remain.

To regenerate the registry manifest for a new tag, run
`python3 files/seed-store.py --distfiles /path/to/extracted/vscode`
from the port directory and review the output before updating `modules.inc`.
