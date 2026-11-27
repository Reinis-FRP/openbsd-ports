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
| [editors/wordcraft](#editorswordcraft) | `wordcraft-0.19.1`, `wordcraft-cli-0.19.1` | amd64 (release build pending) | Desktop word processor, document conversion and headless MCP server |
| [math/gridcraft](#mathgridcraft) | `gridcraft-0.19.1`, `gridcraft-cli-0.19.1` | amd64 | Desktop spreadsheet, workbook conversion and headless MCP server |
| [print/pdfcraft](#printpdfcraft) | `pdfcraft-0.22.0`, `pdfcraft-cli-0.22.0`, `pdfcraft-data-0.22.0` | amd64 (8.0-current tested) | Desktop PDF editor, headless editing and OCR, and shared model data |
| [productivity/deckcraft](#productivitydeckcraft) | `deckcraft-0.19.1`, `deckcraft-cli-0.19.1` | amd64 (8.0-current tested) | Desktop presentation editor, headless rendering and MCP |

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

The port builds two packages from WordCraft v0.19.1
(`5d658dc033f3139926839bffd6ae0eef558c210c`). Install `wordcraft`
for the desktop editor or `wordcraft-cli` for the command-line tools and
headless MCP server. The packages can be installed together. Both use the
release lockfile and checksummed sources, fonts and language data.

On OpenBSD 8.0-current/amd64 (GENERIC.MP#163, Rust 1.99.1), ports fetched
and verified all 684 distfiles, extracted the workspace, applied all six
patches and ran configure. All 676 registry archives matched the tagged
Cargo.lock checksums. Full `cargo metadata --frozen --offline --format-version 1`
with the generated Cargo configuration and an isolated Cargo home resolved
752 packages, including 44 from the single craft-libs revision. The lockfile
remained byte-identical to the tag. Cargo reported three unused upstream egui
patches. The install inputs matched both PLISTs, the patched desktop
entry passed `desktop-file-validate`, and ports derived both package names.
`portcheck` and `git diff --check` passed.

The first v0.19.1 build, tests, packages and X11 runs are pending on
8.0-release/amd64, including compilation of the native craft-font-store
dependency cap-primitives 4.0.3. The earlier release-desktop CLI and X11 GUI
results, including automatic Vulkan on the Radeon RX 7900 XT, apply to v0.18.1.

Use a matching release ports tree and binary packages. On a single-user
builder with a writable mount at `/tmp/ports-build`, run the following
blocks in the same shell. The build uses up to eight jobs.

```sh
cd /usr/ports/mystuff/editors/wordcraft
doas pkg_add rust cargo-generate-vendor libxkbcommon wayland vulkan-loader \
    desktop-file-utils gtk4-update-icon-cache

build=$(mktemp -d /tmp/ports-build/wc.XXXXXXXX)
mkdir "$build/t"
export TMPDIR="$build/t" PKG_TMPDIR="$build/t"
port() {
    make PORTS_PRIVSEP=No SUDO= MAKE_JOBS=8 WRKOBJDIR="$build/obj" \
        WRKOBJDIR_MFS="$build/obj" DISTDIR="$build/dist" \
        PACKAGE_REPOSITORY="$build/packages" PLIST_REPOSITORY="$build/plist" "$@"
}
(
    set -e
    port checksum
    port clean
    port FETCH_CMD=/usr/bin/false build test fake package
    for sub in -main -cli; do
        SUBPACKAGE="$sub" port FETCH_CMD=/usr/bin/false \
            port-lib-depends-check lib-depends-check
        archive=$(SUBPACKAGE="$sub" port show=PKGFILE)
        ls -l "$archive"
    done
)
```

### Install the signed CLI package

After `make package`, run these commands in `editors/wordcraft` on the builder
with the signing key from [Install a package](#install-a-package). Selecting
`-cli` gives the path to `wordcraft-cli-0.19.1.tgz`.

```sh
pkgfile=$(SUBPACKAGE=-cli port show=PKGFILE)
doas pkg_sign -s signify2 -s /etc/signify/my-pkg.sec -o "$(dirname "$pkgfile")" "$pkgfile"
```

On the same machine, keep `pkgfile` as set above. On another 8.0-release/amd64
host, copy the signed archive from the matching release builder, provision
`/etc/signify/my-pkg.pub` through a trusted channel, then run
`pkgfile="$PWD/wordcraft-cli-0.19.1.tgz"` in the download directory.
The following commands verify the signature, preview the installation and
install the CLI with its headless MCP server.

```sh
unset TRUSTED_PKG_PATH
signify -Vz -p /etc/signify/my-pkg.pub -x "$pkgfile" -m /dev/null &&
    pkg_add -n "$pkgfile" && doas pkg_add "$pkgfile" &&
    /usr/local/bin/wordcraft-cli --version
```

The version command prints `wordcraft-cli 0.19.1`. Run `pkg_info -I wordcraft-cli`
to see the installed package version.

### Smoke test

Run the test as a regular user against the staged executable.

```sh
perl files/smoke-test "$(port show=WRKINST)/usr/local/bin/wordcraft-cli"
```

The test converts text to DOCX, checks its text and counts, and exports a
strict PDF. It checks font discovery, spelling and hyphenation, then edits
a document over MCP stdio with a temporary home and automation roots. For an
installed package, pass `/usr/local/bin/wordcraft-cli` instead.

On an X11 desktop with a Vulkan driver, launch the staged sample with the
default backend, close it, then repeat with Vulkan selected explicitly.

```sh
stage="$(port show=WRKINST)/usr/local"
env -u WGPU_BACKEND "$stage/bin/wordcraft" --sample
WGPU_BACKEND=vulkan "$stage/bin/wordcraft" --sample
```

Both runs should render the two pages of “The Open Studio Handbook”,
including its ribbon, styled text and table.

To exercise the desktop entry and clipboard, open a disposable document from
the same staging directory.

```sh
d=$(mktemp -d "$build/clipboard.XXXXXXXX")
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

For the release compatibility check, set `fixture` to the absolute path of
a v0.18.1 DOCX containing formatted text and an image. In the first window,
add `Release round trip`, save and quit. In the second, verify the edit,
formatting and image, export `check.pdf` beside the copy, then quit. Open
the exported PDF and check that the formatting and image also survived export.

```sh
d=$(mktemp -d "$build/docx.XXXXXXXX")
cp "$fixture" "$d/check.docx"
"$stage/bin/wordcraft" "$d/check.docx"
"$stage/bin/wordcraft" "$d/check.docx"
"$stage/bin/wordcraft-cli" text "$d/check.docx" | grep -F 'Release round trip'
file "$d/check.pdf"
```

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

## print/pdfcraft

### Build considerations

PdfCraft v0.22.0 (`ac140f809d554981a5276d748e6193fcf4d4bdfa`) builds
two native frontends. Choose `pdfcraft` for the X11 desktop editor or
`pdfcraft-cli` for headless editing, rendering, OCR and stdio MCP. Both
can be installed together. Each pulls in `pdfcraft-data`, which owns the
two Ocrs ONNX models, their CC-BY-SA-4.0 licence and the shared asset
notices. Tesseract is an optional second OCR engine. The desktop package
carries the GUI-specific notices and PdfCraft's upstream icon. The port
omits the restricted ArtCraft marks from Home and About.

Use the matching 8.0-release/amd64 ports tree and package repository.
Install the build tools and desktop dependencies on the builder. Python
runs the staged smoke test.

```sh
doas pkg_add rust cargo-generate-vendor python \
    desktop-file-utils gtk4-update-icon-cache dbus libxkbcommon wayland vulkan-loader zenity
```

Run these blocks in the same shell. Cargo uses the release lockfile with
`--frozen`. The port supplies the separately pinned craft-fonts and
craft-i18n inputs, including the native Simplified Chinese face. Check
free space before extracting sources and before the second build. On a
configured privsep builder, `make build test fake package` uses the paths
from `/etc/mk.conf`. If the overlay is outside `/usr/ports/mystuff`,
make sure `_pbuild` also sees its `PORTSDIR_PATH` (for example, set it
in `/etc/mk.conf`) so `pkg_create` can resolve the data subpackage.
For an isolated build, give each ports account its
working directories and use a short, shared temporary directory for the
package metadata and Unix socket tests.

```sh
cd "$overlay/print/pdfcraft"
build=$(mktemp -d /tmp/ports-build/pdfcraft.XXXXXXXX)
chmod 755 "$build"
repo="$build/packages"
doas install -d -o _pfetch "$build/dist"
doas install -d -o _pbuild "$build/obj" "$repo" "$build/offline-packages" "$build/plist"
install -d -m 1777 "$build/tmp"
export TMPDIR="$build/tmp" PKG_TMPDIR="$build/tmp"
port() {
    make PORTS_PRIVSEP=Yes MAKE_JOBS=8 WRKOBJDIR="$build/obj" WRKOBJDIR_MFS="$build/obj" \
        DISTDIR="$build/dist" PACKAGE_REPOSITORY="$repo" PLIST_REPOSITORY="$build/plist" "$@"
}

port checksum && df -h "$build" && port build test fake package &&
    df -h "$build" && port clean &&
    repo="$build/offline-packages" &&
    port FETCH_CMD=/usr/bin/false checksum build test fake package
for sub in -main -cli -data; do
    SUBPACKAGE="$sub" port show=PKGFILE
    SUBPACKAGE="$sub" port port-lib-depends-check lib-depends-check || break
done
/usr/ports/infrastructure/bin/portcheck
stage="$(port show=WRKINST)/usr/local"
python3 files/smoke-test "$stage/bin/pdfcraft-cli"
```

On the 8.0-current/amd64 builder (GENERIC.MP#163, Rust 1.99.1), the
default wgpu renderer used llvmpipe through its GL backend. Explicit
OpenGL used eframe's glow renderer with Mesa 25.0.7. Both zoomed a
document, rotated its first page, saved a copy and reopened it. On plain
X11 with no desktop portal service, both Open and Save As used zenity.
The explicit Vulkan run found no adapter. The matching release build and
hardware Vulkan run still need verification.

All 201 upstream tests passed on the builder. Both executables link only
base libraries (`c`, `c++abi`, `m` and `pthread`). On X11, the default
renderer loaded EGL and the Vulkan loader, and glow loaded libGL. Both
loaded libxkbcommon and libdbus. The CLI archive requires only the
matching data package.

The smoke test creates a two-page PDF, extracts a page, verifies rendered
PNG data and saves an edit through `pdfcraft-cli mcp --root` over stdio.
It rasterizes a page, explicitly selects Ocrs and checks the saved PDF for
the recognized word. Run it against the staged or installed directory layout
so the executable can find `../share/pdfcraft/models`.

### Select and install a package

Select `sub=-cli` for the headless frontend or `sub=-main` for the GUI.
Sign both the selected frontend and its matching data archive.

```sh
sub=-cli
pkgfile=$(SUBPACKAGE="$sub" port show=PKGFILE)
data=$(SUBPACKAGE=-data port show=PKGFILE)
for archive in "$data" "$pkgfile"; do
    doas pkg_sign -s signify2 -s /etc/signify/my-pkg.sec \
        -o "$(dirname "$archive")" "$archive" || break
done
```

On the matching release host, provision the public key as described in
[Install a package](#install-a-package). Set `data` and `pkgfile` to the
two signed archives, then verify and preview the install.

```sh
unset TRUSTED_PKG_PATH
signify -Vz -p /etc/signify/my-pkg.pub -x "$data" -m /dev/null &&
    signify -Vz -p /etc/signify/my-pkg.pub -x "$pkgfile" -m /dev/null &&
    pkg_add -n "$data" "$pkgfile" && doas pkg_add "$data" "$pkgfile" &&
    pkg_info -I "$(basename "$pkgfile" .tgz)" &&
    "/usr/local/bin/$(basename "$pkgfile" -0.22.0.tgz)" --version
```

### Release desktop checks

Use the staged frontends from the release build or set `stage=/usr/local`
when both packages are installed. From an X11 terminal, create a private
test directory and a two-page fixture. Preserve the session's X authority
when changing home directories.

```sh
d=$(mktemp -d /tmp/ports-build/pdfcraft-x11.XXXXXXXX)
python3 files/smoke-test --fixture "$d/Two pages.pdf"
export XAUTHORITY="${XAUTHORITY:-$HOME/.Xauthority}"
export HOME="$d" TMPDIR="$d" XDG_CONFIG_HOME="$d/config" XDG_CACHE_HOME="$d/cache"
export XDG_DATA_HOME="$d/data" PATH="$stage/bin:$PATH"
export XDG_DATA_DIRS="$stage/share:${XDG_DATA_DIRS:-/usr/local/share:/usr/share}"
export RUST_LOG=info,egui_wgpu=debug,egui_glow=debug
ui() { pdfcraft-cli ui --control "$d/control" "$@"; }
```

Choose one renderer per run. Set `renderer=default`, `renderer=vulkan`
or `renderer=gl`. The private control file enables GUI automation for
that instance.

```sh
renderer=default
case "$renderer" in
    default) env -u PDFCRAFT_RENDERER -u WGPU_BACKEND pdfcraft --control "$d/control" ;;
    vulkan)  PDFCRAFT_RENDERER=wgpu WGPU_BACKEND=vulkan pdfcraft --control "$d/control" ;;
    gl)      PDFCRAFT_RENDERER=gl pdfcraft --control "$d/control" ;;
esac 2>"$d/$renderer.log" &
```

With the window open, run the following commands. The desktop launcher
should open the spaced filename and `ui state` should report two pages.

```sh
mkdir -p "$XDG_DATA_HOME/applications"
cp "$stage/share/applications/ai.storyteller.pdfcraft.desktop" "$XDG_DATA_HOME/applications/"
update-desktop-database "$XDG_DATA_HOME/applications"
gio mime application/pdf
gio launch "$stage/share/applications/ai.storyteller.pdfcraft.desktop" "$d/Two pages.pdf"
ui state
xprop WM_CLASS
ui set key=zoom value=125
ui command id=page.rotate
ui command id=file.save_as
```

`gio mime` should list `ai.storyteller.pdfcraft.desktop`. Click the window
for `xprop`. Both WM_CLASS strings should be `pdfcraft`. In Save As,
enter the full path printed by `echo "$d/$renderer-copy.pdf"`. Close the tab,
use Open to reopen that copy, and verify the first page is now 120 by 240
points.

```sh
ui command id=file.close
ui command id=file.open
ui state
pdfcraft-cli info "$d/$renderer-copy.pdf"
pdfcraft-cli text "$d/$renderer-copy.pdf"
ui quit
```

The text should still read ORCHARD and MEADOW. Repeat with the other
renderer settings. The Vulkan log should identify the hardware adapter
and Vulkan backend. The OpenGL log should identify glow and its driver.

```sh
grep -E 'renderer:|opengl renderer:|No suitable graphics adapter' \
    "$d/default.log" "$d/vulkan.log" "$d/gl.log"
```

## productivity/deckcraft

### Build considerations

DeckCraft v0.19.1 (`4a980c33ddf37eb64157777fcaeab6d19baa5878`) builds two
independent packages. `deckcraft` edits and presents slides on the desktop.
`deckcraft-cli` edits and renders presentations, exports PDF and serves MCP
over stdio with the display unset. Either package can be installed alone.

Cargo builds both native targets with `--frozen`. The port preserves the
eight craft-libs revisions and the egui and winit forks from the release
lockfile. The four OpenBSD wgpu 30.0.1 patches match WordCraft and
GridCraft. The tagged editor already selects X11 on OpenBSD. Both programs
embed the pinned craft-fonts input, including LXGW WenKai Lite from the
native Simplified Chinese manifest. DeckCraft reads `/etc/fonts/fonts.conf` for system fonts, including
`/usr/X11R6/lib/X11/fonts` and `/usr/local/share/fonts`. The tag supplies
its contributor credits and the media codecs through the sources already
listed in the port. Each package includes its own font and codec notices
and the PDF writer's CC0 ICC profile licence. The desktop package ships
DeckCraft's slide-board icon. The ArtCraft marks have separate terms in
`docs/brand/LICENSE-brand.txt` and are omitted from both packages.

Use matching OpenBSD 8.0-release/amd64 ports and binary packages on the
release builder. Preinstall the build tools, desktop dependencies and CLI
smoke-test tools.

```sh
doas pkg_add rust cargo-generate-vendor 'python%3' pngcheck poppler-utils \
    desktop-file-utils gtk4-update-icon-cache shared-mime-info dbus libxkbcommon \
    wayland vulkan-loader zenity
```

Run these blocks from the same shell on a configured privsep builder.
The `doas` rules for `_pfetch` and `_pbuild` must pass the ports build
environment to recursive makes, including `MAKEFLAGS` and `PORTSDIR_PATH`.
Use the account-specific setup in [Build a package](#build-a-package).
Keep the shared temporary path short for Unix sockets. The snapshot work
tree used 4.0 GiB, plus 284 MiB for cached distfiles and about 100 MiB
per pair of packages.

```sh
cd "$overlay/productivity/deckcraft"
export PORTSDIR_PATH="/usr/ports:$overlay"
df -h /tmp/ports-build
build=$(mktemp -d /tmp/ports-build/dc.XXXXXXXX)
chmod 755 "$build"
repo="$build/packages"
doas install -d -o _pfetch "$build/dist"
doas install -d -o _pbuild "$build/obj" "$repo" "$build/offline" "$build/plist"
install -d -m 1777 "$build/t"
export TMPDIR="$build/t" PKG_TMPDIR="$build/t"
port() {
    make PORTS_PRIVSEP=Yes MAKE_JOBS=8 WRKOBJDIR="$build/obj" \
        WRKOBJDIR_MFS="$build/obj" DISTDIR="$build/dist" \
        PACKAGE_REPOSITORY="$repo" PLIST_REPOSITORY="$build/plist" "$@"
}

port checksum && port build test fake package && port clean &&
    repo="$build/offline" &&
    port FETCH_CMD=/usr/bin/false checksum build test fake package

/usr/ports/infrastructure/bin/portcheck -p "$overlay"
for sub in -main -cli; do
    SUBPACKAGE="$sub" port port-lib-depends-check lib-depends-check || break
    archive=$(SUBPACKAGE="$sub" port show=PKGFILE)
    tar tzf "$archive"
done
stage="$(port show=WRKINST)/usr/local"
python3 files/smoke-test "$stage/bin/deckcraft-cli"

cli_archive=$(SUBPACKAGE=-cli port show=PKGFILE)
mkdir "$build/cli-only"
tar -xzf "$cli_archive" -C "$build/cli-only" bin share
python3 files/smoke-test "$build/cli-only/bin/deckcraft-cli"
```

The smoke test checks all eight 960×540 sample PNGs, adds a titled slide,
reopens both `.deckcraft` and `.pptx`, and checks the nine-page PDF. A
fresh stdio MCP process lists its tools and reads the sample deck.

### Snapshot verification

On OpenBSD 8.0-current/amd64 (GENERIC.MP#163, Rust 1.99.1), both the
initial build and a clean rebuild with fetching disabled produced the two
packages. Both rebuilt executables matched the originals by SHA-256. All 231
tests selected by the port passed. One loopback test stalled during the
parallel repeat. The port runs the test harness
serially, with compilation still using up to eight jobs.

`portcheck` and both library-dependency checks passed for each package.
The two archives matched their staged files and had no files in common.
The CLI archive has no package dependencies and passed the headless smoke
test with the GUI absent.

On Xvfb, the editor selected the wgpu GL backend and Mesa 25.0.7 llvmpipe.
Open and Save As both used zenity on plain X11. A new slide survived a save,
close and reopen. F5 started the slide show, Right advanced it and Escape
returned to editing.
The X11 window reported `deckcraft` for both WM_CLASS strings.
The process opened EGL, the Vulkan loader, libxkbcommon, libxkbcommon-x11
and libdbus. Both executables link only the base c, c++abi, m and pthread
libraries.

A live `font.rescan` opened DejaVu in the X11 tree and Noto in the local
fonts tree. The generated font table included the native Hani/Hans entry
for LXGW WenKai Lite. The font matched the pinned manifest's SHA-256.

The GUI archive opened a two-slide file through its desktop entry with the CLI
absent. The registered MIME type was `application/x-deckcraft`. A separate
Wayland connection probe loaded libwayland-client. The forced Vulkan run
found no adapter in this VM.

With an embedded H.264 video and AAC tone, the default run reported
`audio output unavailable (no audio output device)`. With both the default
and `DECKCRAFT_NO_AUDIO=1`, the playback clock advanced, the displayed
video changed from red to blue, and Right and Escape still worked. CPAL
0.16.0 selects its null host on OpenBSD, so this port has no audible
playback. An older sndio backend was closed unmerged and would need a
separate porting effort. [CPAL sndio implementation](https://github.com/RustAudio/cpal/pull/493)

The 8.0-release build and physical Radeon display checks remain pending.

### Select and install a package

On the release builder, choose `sub=-main` for the desktop or `sub=-cli`
for the CLI. Sign the chosen package using the key described in
[Install a package](#install-a-package).

```sh
sub=-main
pkgfile=$(SUBPACKAGE="$sub" port show=PKGFILE)
doas pkg_sign -s signify2 -s /etc/signify/my-pkg.sec -o "$(dirname "$pkgfile")" "$pkgfile"
```

On the matching release host, set `pkgfile` to the signed archive's path.

```sh
unset TRUSTED_PKG_PATH
signify -Vz -p /etc/signify/my-pkg.pub -x "$pkgfile" -m /dev/null &&
    pkg_add -n "$pkgfile" && doas pkg_add "$pkgfile"
```

### Release desktop checks

From an X11 terminal on the Radeon desktop, use the release build's
`stage` or set `stage=/usr/local` for installed packages. Preserve the
session's X authority when switching to a private test home.

```sh
d=$(mktemp -d /tmp/ports-build/dc-x11.XXXXXXXX)
export XAUTHORITY="${XAUTHORITY:-$HOME/.Xauthority}"
export HOME="$d" TMPDIR="$d" XDG_CONFIG_HOME="$d/config" XDG_CACHE_HOME="$d/cache"
export PATH="$stage/bin:$PATH" RUST_LOG=warn,egui_wgpu=debug,rfd=debug
cd "$d"
env -u WGPU_BACKEND deckcraft --sample 2>default.log
WGPU_BACKEND=vulkan deckcraft --sample 2>vulkan.log
grep -Ei 'wgpu adapter|software rasterizer|no suitable|backend:' default.log vulkan.log
```

In each run, insert a slide with a distinctive title, use Save As, close
the deck and reopen it through Open. Check the title, start the slide show
with F5, advance with Right and return with Escape. Record the adapter
name and backend from each log. The forced Vulkan run should identify the
Radeon adapter and `Vulkan`.

To reproduce the media check, use an MP4 containing moving video and a
known audible track, or generate the red/blue clip below with FFmpeg. The
fixture has four seconds of H.264 video and a 440 Hz AAC tone.

```sh
ffmpeg -f lavfi -i color=red:s=160x90:r=8:d=2 \
    -f lavfi -i color=blue:s=160x90:r=8:d=2 -f lavfi -i sine=frequency=440:duration=4 \
    -filter_complex '[0:v][1:v]concat=n=2:v=1:a=0[v]' -map '[v]' -map 2:a \
    -c:v libx264 -profile:v baseline -pix_fmt yuv420p -c:a aac clip.mp4
```

With both staged frontends on `PATH`, create a two-slide deck. Watch the
colors and listen on the first slide, advance to the second slide and
return to the editor. Repeat with audio disabled.

```sh
deckcraft-cli run --cmd 'insert.video={"path":"clip.mp4","rect":[100,80,760,380]}' \
    --cmd 'media.options={"autoplay":true,"loop":true}' \
    --cmd 'slide.new={"layout":"titleOnly","title":"After video"}' --save media.deckcraft
env -u DECKCRAFT_NO_AUDIO deckcraft --show media.deckcraft 2>audio-default.log
DECKCRAFT_NO_AUDIO=1 deckcraft --show media.deckcraft 2>audio-disabled.log
grep -i 'audio output' audio-default.log audio-disabled.log
```
