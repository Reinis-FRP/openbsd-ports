# Private OpenBSD ports

## Agave 4.4.1 on amd64

Use an amd64 builder running the same OpenBSD release as the installation
host, with its matching ports tree and packages.

### Check out the overlay

Once the main ports tree is in place, create the overlay directory if
`/usr/ports/mystuff` does not yet exist. Run Git as your regular user with
SSH access to the private repository. The `022` umask makes the checked-out
port files readable by `_pbuild` and `_pfetch`.

```sh
doas mkdir -m 0755 /usr/ports/mystuff &&
    doas chown "$(id -un):$(id -gn)" /usr/ports/mystuff &&
    (umask 022 && git clone --branch main \
        git@github.com:Reinis-FRP/openbsd-ports.git /usr/ports/mystuff)
```

For an existing checkout of this repository, including one restored after
a ports tree refresh, update it as the same user:

```sh
(umask 022 && git -C /usr/ports/mystuff pull --ff-only)
```

Verify the checked-out commit and that the ports framework resolves the
overlay port as `net/agave`:

```sh
cd /usr/ports/mystuff/net/agave
git log -1 --oneline
make show=PKGPATH
```

### Release builder with ports privilege separation

Check the builder's effective settings before changing anything. From
`/usr/ports/mystuff/net/agave`, run:

```sh
make show=PORTS_PRIVSEP
make show=FETCH_USER
make show=BUILD_USER
make show=PKGFILE
```

The documented desktop setup gives `Yes`, `_pfetch`, and `_pbuild` for the
first three values, using these entries in `/etc/mk.conf`.

```make
PORTS_PRIVSEP=Yes
WRKOBJDIR=/usr/obj/ports
DISTDIR=/usr/distfiles
PACKAGE_REPOSITORY=/usr/packages
```

With these paths the package lands in `/usr/packages/amd64/all/`. The
following `/etc/doas.conf` rules from the desktop notes allow a build user
in `wheel` to run commands as the two service accounts.

```text
permit keepenv nopass :wheel as _pbuild
permit keepenv nopass :wheel as _pfetch
```

When first enabling privilege separation or moving the ports directories,
run `cd /usr/ports && doas make fix-permissions` to set their ownership.
The service accounts also need search permission on parent directories
and read access to the overlay. On an already configured builder, proceed
directly to the build as your regular user.

### Build the release package

On the builder, install the dependencies from the official OpenBSD 8.0/amd64
packages. These include Rust 1.98.1, the version pinned by Agave v4.4.1.
The trailing `--` selects the standard flavor of `libusb1`.

```sh
doas pkg_add rust cargo-generate-vendor zstd libhidapi libusb1--
```

The ports framework reuses matching installed packages. On a clean build,
Cargo compiles Agave and the Rust crates needed for the two binaries.
For example, it compiles the `hidapi` bindings and links to the installed
USB libraries. When updating the overlay to a new port revision on a builder
with an existing Agave work directory, run `make clean` in the port first
so the new patches are applied. The downloaded distfiles are reusable.

```sh
cd /usr/ports/mystuff/net/agave
sysctl -n hw.ncpuonline
make show=MAKE_JOBS
```

This port defaults to one Cargo job unless the builder overrides `MAKE_JOBS`.
The two commands above show the online CPU count and the configured job limit.
The Cargo module passes `MAKE_JOBS` to Cargo as `CARGO_BUILD_JOBS`. To use
the online CPUs, set the limit on the build command as shown below. Use a
smaller limit such as `MAKE_JOBS=4` to leave resources for other desktop work.
The parallel desktop build completed Cargo's optimized release phase in 1m 44s.

```sh
make fetch
make checksum
make MAKE_JOBS="$(sysctl -n hw.ncpuonline)" build
make test
make fake
make port-lib-depends-check
make package
make lib-depends-check
pkgfile=$(make show=PKGFILE)
pkg_info -D unsigned -f "$pkgfile"
tar -tzf "$pkgfile"
```

To build and package in one command, use
`make MAKE_JOBS="$(sysctl -n hw.ncpuonline)" package` from the port directory.

### Sign and install the release package

The ports build above produces an unsigned package. For distribution to
your release host, use the local package-signing key from your OpenBSD
desktop setup. If this is the builder's first signing key, generate the pair
once. `-n` creates a passphrase-free secret key. Keep it on the builder.

```sh
doas signify -Gns /etc/signify/my-pkg.sec -p /etc/signify/my-pkg.pub
```

From the same shell as the build, sign just the Agave archive into a sibling
`signed` directory. Verify it with the builder's public key.

```sh
signed_dir="${pkgfile%/*}/../signed"
doas pkg_sign -s signify2 -s /etc/signify/my-pkg.sec -o "$signed_dir" "$pkgfile"
signed_pkg="$signed_dir/${pkgfile##*/}"
signify -Vz -p /etc/signify/my-pkg.pub -x "$signed_pkg" -m /dev/null
```

On a separate release host, provision the builder's **public** key as
`/etc/signify/my-pkg.pub` through a trusted channel. Copy the signed archive
and set `signed_pkg` to its absolute path on that host. If the signing
builder is also the installation host, the key and variable are already in
place. Inspect, dry-run, then install with normal signature checks.

```sh
unset TRUSTED_PKG_PATH
pkg_info -f "$signed_pkg"
pkg_add -n "$signed_pkg"
doas pkg_add "$signed_pkg"
sh /usr/ports/mystuff/net/agave/files/smoke-test /usr/local/bin
```

On a host with an earlier Agave package installed, use `pkg_add -nr "$signed_pkg"`
for the dry run and `doas pkg_add -r "$signed_pkg"` for the replacement.
Check the installed revision with `pkg_info -e agave-4.4.1p1`.

On a host that only installs Agave, `pkg_add` resolves its direct runtime
dependencies `zstd`, `libhidapi`, and `libusb1`. Rust and
`cargo-generate-vendor` are build-time tools.

The last command uses the overlay's smoke-test script. On a host without
the overlay, copy the script alongside the package and run it with the same
`/usr/local/bin` argument.

For an unsigned installation directly on the builder, the ports framework
supplies its local package directory as `TRUSTED_PKG_PATH` for `make install`.
That setting waives signature verification for packages in that directory.
Use the signed archive above when moving the package between hosts.

The smoke test prints both versions, generates a temporary keypair, and
checks a signature with its public key.

#### Ledger on OpenBSD

Starting with `agave-4.4.1p0`, both binaries look for the Ledger application
on native `/dev/uhidN` nodes. They check the USB vendor and product IDs,
the Ledger HID collection, and the 64-byte input and output report sizes
before sending a request. The existing libusb discovery path is also enabled.

Unlock the Ledger and open its Solana app. In the latest `dmesg` attachment,
find the Ledger's interface 0 and follow its `uhidev` parent to the
`uhidN` child. Unit numbers can change on reconnect. On the tested Nano X
this was `uhid12`. If that node is missing, substitute the current unit in
these commands. `MAKEDEV` creates it with root-only permissions.

```sh
(cd /dev && doas sh MAKEDEV uhid12)
doas usbhidctl -f /dev/uhid12 -r
```

The separate package hardware test reads a public key. Run it as a user
with read and write access to the current Ledger node. For a first test
with the default node permissions, use the second command.

```sh
/usr/local/bin/solana-keygen pubkey usb://ledger
doas /usr/local/bin/solana-keygen pubkey usb://ledger
```

On 8.0-release, the installed p0 keygen read a Nano X public key after
attachment and timed out on the next invocation. This occurred on two USB
controllers. A separate native probe retried the read-only configuration
request on the same open handle, received status `9000`, then received the
32-byte public key with status `9000`. The Solana app version was 1.12.1.

Revision p1 uses a five-second input wait during native discovery. It sends
the configuration request at most three times on the same handle. It checks
the public-key reply separately and ignores matching late configuration
replies within the number of retransmissions. Interactive operations keep
their original confirmation behavior.

To check reopening on the release host, leave the device connected and the
Solana app open across both invocations. With default node permissions, run:

```sh
for run in 1 2; do
    doas timeout -k 2 40 /usr/local/bin/solana-keygen pubkey usb://ledger || break
done
```

Both invocations should print the same public key.

After an OS upgrade, repeat the build with matching ports and dependencies
and fresh work and package directories. After signing the rebuilt package, use
`doas pkg_add -r -D installed "$signed_pkg"` for a same-version replacement.

### Source inputs

`net/agave/Makefile` pins Anza's tag and the three Git revisions from its
workspace lockfile. `crates.inc` and `distinfo` list the registry crates
and distfile checksums. The two Cargo patches redirect the four Git package
entries to the extracted archives. Cargo builds both binaries with a private
Cargo home, the ports vendor directory, and `--frozen`.

Both binaries enable Agave's libusb remote-wallet feature. The OpenBSD
Ledger code also links base libusbhid to inspect native report descriptors.
The probe uses the `cc` crate already pinned in the workspace lockfile.
The build graph uses blstrs and crossbeam-epoch. Cargo also resolves rocksdb and
librocksdb-sys for the workspace lockfile. Building the two binaries
requires only their manifests from rust-rocksdb, so the RocksDB and Snappy
submodule archives are omitted.

### Build on the development snapshot

OpenBSD 8.0-current/amd64 GENERIC#156 with Rust 1.99.1 built and packaged
the original agave-4.4.1 package from a fresh ports fetch and an empty Cargo home. All 937
distfiles passed checksum verification. All 933 registry checksums also
matched the upstream lockfile. The ports smoke test, fake installation,
package creation, portcheck, and both library-dependency checks passed.

Both binaries extracted from the package reported 4.4.1 at commit
`5bb30402` and linked directly to libhidapi-libusb and libusb-1.0. The
packaged keygen created a temporary keypair and verified a signature.
The unsigned snapshot package passed a dry-run install with its local
package directory in `TRUSTED_PKG_PATH`. Plain `pkg_add -n` rejected it.
A copy signed with a disposable key passed `signify -Vz` verification.

Revision p0 also built on GENERIC#156 from a fresh work directory and
empty Cargo home using the same 937 verified distfiles. The ports test,
fake, package, portcheck, library checks, and local package dry run passed.
Both extracted binaries reported 4.4.1 at commit `5bb30402` and linked
to base libusbhid as well as libhidapi-libusb and libusb. The extracted
keygen passed the disposable-keypair smoke test. A simulated native HID
exchange exercised app configuration and the default Solana public-key
request on the snapshot.

On the same snapshot, p1 passed the frozen build with an empty Cargo home,
the ports smoke test, fake, package creation, portcheck, both dependency
checks, and a local package installation dry run. Both binaries extracted
from the archive passed the version checks and keygen passed the disposable
keypair test. Simulated HID reports covered retries, delayed duplicates,
silent devices, legacy configuration, and a delayed signing confirmation.

To validate revision p1 on 8.0-release, rebuild from a clean work directory
with the matching release ports tree and packages, sign and install the new
archive, then run the installed smoke test and consecutive Ledger reads above.

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
from this overlay's root on the development VM. The tested snapshot reported
`No` for `make show=PORTS_PRIVSEP`. Its private `mktemp -d` directory is
accessible only to the build user. On a builder reporting `Yes`, use the
privilege-separated directories and release build commands above.

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
