# Private OpenBSD ports

## Agave 4.4.1 on amd64

Use an amd64 builder running the same OpenBSD release as the installation
host, with its matching ports tree and packages. Check out this overlay at
`/usr/ports/mystuff`. On a fresh builder with ports work and package
directories set up for your build user, run:

```sh
cd /usr/ports/mystuff/net/agave
make fetch
make checksum
make build
make test
make fake
make port-lib-depends-check
make package
make lib-depends-check
pkgfile=$(make show=PKGFILE)
pkg_info -D unsigned -f "$pkgfile"
tar -tzf "$pkgfile"
```

To install the inspected package and verify the installed programs:

```sh
doas env TRUSTED_PKG_PATH="${pkgfile%/*}/" PKG_PATH=installpath \
    pkg_add "${pkgfile##*/}"
sh files/smoke-test /usr/local/bin
```

The smoke test prints both versions, generates a temporary keypair, and
checks a signature with its public key. For a separate Ledger test, attach
and unlock the device, open its Solana app, and run
`solana-keygen pubkey usb://ledger` as the user who will use the device.

After an OS upgrade, repeat the build with matching ports and dependencies
and fresh work and package directories. For a same-version replacement, use
`pkg_add -r -D installed` with the same package directory and filename.

### Source inputs

`net/agave/Makefile` pins Anza's tag and the three Git revisions from its
workspace lockfile. `crates.inc` and `distinfo` list the registry crates
and distfile checksums. The two Cargo patches redirect the four Git package
entries to the extracted archives. Cargo builds both binaries with a private
Cargo home, the ports vendor directory, and `--frozen`.

Both binaries enable Agave's libusb remote-wallet feature. Their build
graph uses blstrs and crossbeam-epoch. Cargo also resolves rocksdb and
librocksdb-sys for the workspace lockfile. Building the two binaries
requires only their manifests from rust-rocksdb, so the RocksDB and Snappy
submodule archives are omitted.

### Build on the development snapshot

OpenBSD 8.0-current/amd64 GENERIC#156 with Rust 1.99.1 built and packaged
Agave 4.4.1 from a fresh ports fetch and an empty Cargo home. All 937
distfiles passed checksum verification. All 933 registry checksums also
matched the upstream lockfile. The ports smoke test, fake installation,
package creation, portcheck, and both library-dependency checks passed.

Both binaries extracted from the package reported 4.4.1 at commit
`5bb30402` and linked directly to libhidapi-libusb and libusb-1.0. The
packaged keygen created a temporary keypair and verified a signature.
The snapshot package passed a dry-run install with `pkg_add -n`.

The clean 8.0-release build and installed smoke test remain pending. Run
the release procedure at the top of this file with the matching 8.0 ports
tree and packages. The Ledger public-key read requires a physical device.

#### Prepare the fresh build disk once

After attaching the disk and booting the VM, identify it by capacity and
check which disk holds the root filesystem.

```sh
sysctl hw.disknames
dmesg | grep '^sd[0-9]'
df -h /
```

Replace `sdN` below with the **confirmed empty disk**. Before the reboot,
this VM used `sd0` for its system disk. The following steps overwrite the
selected disk's partition table and create a new filesystem.

```sh
doas fdisk -g sdN
doas disklabel -E sdN
```

Confirm the GPT write in `fdisk`. At the `disklabel` prompt, enter `a a`
to add partition `a`. Accept the default offset, enter `*` for all
available space, and choose `4.2BSD` for the filesystem type. Enter `p`
to check the layout, then `w` to write and `q` to quit.

```sh
doas disklabel sdN          # check partition a and record the duid
doas newfs /dev/rsdNa       # for example, /dev/rsd1a if the new disk is sd1
```

Place this entry after the existing `/tmp` entry in `/etc/fstab`, replacing
`<DUID>` with the new disk's 16-digit hex DUID.

```fstab
<DUID>.a /tmp/agave-build ffs rw,nodev,nosuid,noauto 0 2
```

On this snapshot, `/etc/rc` clears `/tmp` after the automatic local mounts.
Use `noauto` to mount the volume after boot. Cargo runs build scripts from
this volume. The effective port setting is `USE_WXNEEDED=No`. The last
two fields skip dump scheduling for scratch data and select the usual
non-root fsck pass.

```sh
doas mkdir -m 0755 /tmp/agave-build
doas mount /tmp/agave-build && doas chown reinism:reinism /tmp/agave-build
df -h /tmp/agave-build
```

After mounting the scratch volume at `/tmp/agave-build`, run the following
from this overlay's root on the development VM. A fresh run directory keeps
the build and distfiles outside the overlay and system ports tree.

```sh
overlay=$PWD
cd "$overlay/net/agave"
df -h /tmp/agave-build
buildroot=$(mktemp -d /tmp/agave-build/run.XXXXXXXX)
m() {
    make PORTSDIR_PATH="/usr/ports:$overlay" WRKOBJDIR="$buildroot/obj" \
        DISTDIR="$buildroot/dist" PACKAGE_REPOSITORY="$buildroot/packages" \
        PLIST_REPOSITORY="$buildroot/plist" "$@"
}
m -j6 fetch && m checksum && m build && m test && m fake &&
    m port-lib-depends-check && m package && m lib-depends-check
```

Once packaging succeeds, inspect and test the files from the actual archive
without installing on the development VM.

```sh
pkgfile=$(m show=PKGFILE)
pkg_info -D unsigned -f "$pkgfile"
tar -tzf "$pkgfile"
mkdir "$buildroot/payload"
tar -xzf "$pkgfile" -C "$buildroot/payload" bin/solana bin/solana-keygen
sh files/smoke-test "$buildroot/payload/bin"
```
