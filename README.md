# Private OpenBSD ports

This overlay adds ports to an existing OpenBSD ports tree. Use a builder with
the same OpenBSD release and architecture as the installation host, and a
matching ports tree and package mirror.

| Port | Package | Architecture | Contents |
| --- | --- | --- | --- |
| [net/agave](#agave) | `agave-4.4.1p7` | amd64 | Solana CLI, key generation, local test validator, and optional Ledger support |

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

Select a port from the table and check its build notes for dependencies and
options. The following example builds Agave.

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

## Agave

### Build considerations

Agave builds the three binaries from pinned sources with Cargo's `--frozen`
option. To save time on a clean builder, install the build and library
dependencies from binary packages first. Run these commands in `net/agave` so
the LLVM version used by bindgen comes from the matching ports tree.

```sh
doas pkg_add rust cargo-generate-vendor gmake protobuf zstd libhidapi libusb1--
llvm_port=/usr/ports/devel/llvm/$(make show=MODCLANG_VERSION)
doas pkg_add "$(make -C "$llvm_port" show=FULLPKGNAME)"
```

`libusb1--` selects the standard flavor. To use four Cargo build jobs, for
example, use `make MAKE_JOBS=4 build` for the build step above. To check the
package's library dependencies after packaging, run
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
