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
| [editors/wordcraft](#editorswordcraft) | `wordcraft-0.18.1`, `wordcraft-cli-0.18.1p0` | amd64 | Desktop word processor, document conversion and headless MCP server |
| [math/gridcraft](#mathgridcraft) | `gridcraft-0.19.1`, `gridcraft-cli-0.19.1` | amd64 | Desktop spreadsheet, workbook conversion and headless MCP server |

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

## editors/wordcraft

### Build considerations

The port builds two packages from WordCraft v0.18.1
(`079f478bd85df7bcc567e3bafa7d7219096f9c3c`). Install `wordcraft`
for the desktop editor or `wordcraft-cli` for the command-line tools and
headless MCP server. The packages can be installed together. Both use the
release lockfile and checksummed sources, fonts and language data.

On X11, the desktop editor supports Vulkan and Mesa software OpenGL.
The CLI and the patched build-tree GUI have been exercised on
OpenBSD 8.0-release/amd64, the latter on X11 with a Radeon RX 7900 XT.

You can preinstall the build tools and desktop package dependencies on a
clean builder.

```sh
doas pkg_add rust cargo-generate-vendor libxkbcommon wayland vulkan-loader \
    desktop-file-utils gtk4-update-icon-cache
```

Verify the distfiles, then start a clean, frozen build with fetching disabled.

```sh
cd "$overlay/editors/wordcraft"
make checksum && make clean
make FETCH_CMD=/usr/bin/false MAKE_JOBS=8 build test package
env SUBPACKAGE=-main make show=PKGFILE
env SUBPACKAGE=-cli make show=PKGFILE
```

### Install the signed CLI package

After `make package`, run these commands in `editors/wordcraft` on the builder
with the signing key from [Install a package](#install-a-package). Selecting
`-cli` gives the path to `wordcraft-cli-0.18.1p0.tgz`.

```sh
pkgfile=$(env SUBPACKAGE=-cli make show=PKGFILE)
doas pkg_sign -s signify2 -s /etc/signify/my-pkg.sec -o "$(dirname "$pkgfile")" "$pkgfile"
```

On the same machine, keep `pkgfile` as set above. On another 8.0-release/amd64
host, copy the signed archive from the matching release builder, provision
`/etc/signify/my-pkg.pub` through a trusted channel, then run
`pkgfile="$PWD/wordcraft-cli-0.18.1p0.tgz"` in the download directory.
The following commands verify the signature, preview the installation and
install the CLI with its headless MCP server.

```sh
unset TRUSTED_PKG_PATH
signify -Vz -p /etc/signify/my-pkg.pub -x "$pkgfile" -m /dev/null &&
    pkg_add -n "$pkgfile" && doas pkg_add "$pkgfile" &&
    /usr/local/bin/wordcraft-cli --version
```

This also upgrades an existing `wordcraft-cli-0.18.1`. The version command
prints `wordcraft-cli 0.18.1`. `pkg_info -I wordcraft-cli` shows the installed
package revision `0.18.1p0`.

### Smoke test

Run the test as a regular user against the staged executable.

```sh
perl files/smoke-test "$(make show=WRKINST)/usr/local/bin/wordcraft-cli"
```

The test converts text to DOCX, checks its text and counts, and exports a
strict PDF. It checks font discovery, spelling and hyphenation, then edits
a document over MCP stdio with a temporary home and automation roots. For an
installed package, pass `/usr/local/bin/wordcraft-cli` instead.

On an X11 desktop with a Vulkan driver, launch the staged sample with the
default backend, close it, then repeat with Vulkan selected explicitly.

```sh
stage="$(make show=WRKINST)/usr/local"
env -u WGPU_BACKEND "$stage/bin/wordcraft" --sample
WGPU_BACKEND=vulkan "$stage/bin/wordcraft" --sample
```

Both runs should render the two pages of “The Open Studio Handbook”,
including its ribbon, styled text and table.

To exercise the desktop entry and clipboard, open a disposable document from
the same staging directory.

```sh
d=$(mktemp -d)
printf '%s' 'Original three words' > "$d/input.txt"
"$stage/bin/wordcraft-cli" convert "$d/input.txt" "$d/check.docx"
export PATH="$stage/bin:$PATH"
export XDG_DATA_DIRS="$stage/share:${XDG_DATA_DIRS:-/usr/local/share:/usr/share}"
gio launch "$stage/share/applications/ai.storyteller.wordcraft.desktop" "$d/check.docx"
```

Copy the paragraph into another application, replace it there with
`Edited clipboard text`, and paste it back over the original paragraph.
Save, close and repeat the `gio launch` command. The reopened paragraph
and `wordcraft-cli text "$d/check.docx"` should both read `Edited clipboard text`.

## math/gridcraft

### Build considerations

GridCraft v0.19.1 (`476448d7d9f3a5f8e93fcf7d98b47fd52293fe7f`) builds
two independently installable packages. Choose `gridcraft` for the desktop
spreadsheet and `gridcraft-cli` for workbook conversion, formulas and the
headless MCP server. Both can be installed on the same host.

The port uses the release lockfile and checksummed sources. The GUI embeds
the in-tree translations and the pinned craft-fonts faces, including LXGW
WenKai Lite. Both binaries use GridCraft's PDF writer. On OpenBSD,
`gridcraft-cli sandbox-check` reports unsupported and exits 2. Python cells
require the upstream helper sandbox. Ordinary formulas and `gridcraft-cli mcp`
work independently.

On the 8.0-current/amd64 snapshot (GENERIC.MP#163, Rust 1.99.1), the sales
sample rendered with Mesa llvmpipe on X11. Both charts, recalculation, the
clipboard and XLSX save/reopen worked. With the base X11 DejaVu fonts,
widen column G to see the full grand total in G11. On plain X11 without
a desktop portal, Open and Save As need zenity. The VM exposed no Vulkan adapter.
The release desktop still needs a matching build and Radeon Vulkan test.

Preinstall the build tools and runtime dependencies on a matching builder.

```sh
doas pkg_add rust cargo-generate-vendor libxkbcommon wayland vulkan-loader \
    desktop-file-utils gtk4-update-icon-cache zenity
```

On a single-user builder with a writable filesystem at `/tmp/ports-build`,
keep the distfiles, work tree, temporary files and packages together. Run
the following blocks in the same shell.

```sh
cd "$overlay/math/gridcraft"
build=$(mktemp -d /tmp/ports-build/gridcraft.XXXXXXXX)
export WRKOBJDIR="$build/obj" WRKOBJDIR_MFS="$build/obj" DISTDIR="$build/dist"
export PACKAGE_REPOSITORY="$build/packages" PLIST_REPOSITORY="$build/plist"
export TMPDIR="$build/tmp"
mkdir "$TMPDIR"
port() { make PORTS_PRIVSEP=No SUDO= MAKE_JOBS=8 "$@"; }

port checksum && df -h "$build" && port clean &&
    port FETCH_CMD=/usr/bin/false build test fake package
for sub in -main -cli; do
    SUBPACKAGE="$sub" port show=PKGFILE
    SUBPACKAGE="$sub" port port-lib-depends-check lib-depends-check
done
```

### Select and install a package

Select one output from the matching release build. Use `sub=-cli` for the
headless package or `sub=-main` for the desktop spreadsheet. Sign the selected
archive with the builder's package key.

```sh
sub=-cli
pkgfile=$(SUBPACKAGE="$sub" port show=PKGFILE)
doas pkg_sign -s signify2 -s /etc/signify/my-pkg.sec -o "$(dirname "$pkgfile")" "$pkgfile"
```

On another 8.0-release/amd64 host, copy the signed archive, set `pkgfile` to
its path and provision the public key as described in [Install a package](#install-a-package).
Verify, preview and install the selected archive.

```sh
pkgname=$(basename "$pkgfile" .tgz)
unset TRUSTED_PKG_PATH
signify -Vz -p /etc/signify/my-pkg.pub -x "$pkgfile" -m /dev/null &&
    pkg_add -n "$pkgfile" && doas pkg_add "$pkgfile" &&
    pkg_info -I "$pkgname" && "/usr/local/bin/${pkgname%-0.19.1}" --version
```

### Smoke tests

Run the workbook and headless MCP checks against the staged executable.
The script uses a temporary home, saves an XLSX with a SUM formula, verifies
the result after reopening, checks CSV and PDF export, and reads a saved
workbook through a fresh MCP process.

```sh
stage="$(port show=WRKINST)/usr/local"
perl files/smoke-test "$stage/bin/gridcraft-cli"
```

On the release X11 desktop, use the stage from its matching build, or set
`stage=/usr/local` after installation. Keep the session's X authority file
available when switching to a temporary home.

```sh
d=$(mktemp -d /tmp/ports-build/gridcraft-x11.XXXXXXXX)
export XAUTHORITY="${XAUTHORITY:-$HOME/.Xauthority}"
export HOME="$d" TMPDIR="$d"
export XDG_CONFIG_HOME="$d/config" XDG_CACHE_HOME="$d/cache"
export PATH="$stage/bin:$PATH"
export XDG_DATA_DIRS="$stage/share:${XDG_DATA_DIRS:-/usr/local/share:/usr/share}"
export RUST_LOG=warn,egui_wgpu=debug
env -u WGPU_BACKEND gridcraft --sample sales 2>"$d/default.log"
WGPU_BACKEND=vulkan gridcraft --sample sales 2>"$d/vulkan.log"
grep -E 'wgpu adapter:|Software rasterizer detected|No suitable graphics adapter' \
    "$d/default.log" "$d/vulkan.log"
```

The Vulkan log should report the Radeon adapter and `backend: Vulkan`.
For each launch, check the sales grid, the “Revenue by Region” column chart
and the “Share of Annual Total” pie chart. Change North's Q1 cell C5 from
48200 to 48201. G5 should change from 217420 to 217421. Copy C5 to another
X11 application, then paste its text into a blank cell. Use Save As to save
`$d/sales.xlsx`. Close it and reopen it through File → Open. Verify the saved
total and test opening two workbooks from the desktop entry.

```sh
gridcraft-cli cat "$d/sales.xlsx" --range G5
gridcraft-cli run --sample budget --out "$d/budget.xlsx"
gio launch "$stage/share/applications/ai.storyteller.gridcraft.desktop" \
    "$d/sales.xlsx" "$d/budget.xlsx"
xprop WM_CLASS
```

Click the GridCraft window for `xprop`. Both WM_CLASS strings should be
`gridcraft`. Check that both workbooks opened and that the saved sales
total displays as $217,421.
