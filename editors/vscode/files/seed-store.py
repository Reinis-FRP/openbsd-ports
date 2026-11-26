"""Seed pnpm's offline store from the archives verified by make checksum."""

import json
from pathlib import Path
import subprocess
import sys
from tempfile import TemporaryDirectory

import yaml


def supports(values, target):
    if values is None:
        return True
    return target in values or ("!" + target not in values and
                                all(value.startswith("!") for value in values))


def registry_packages(source):
    packages = {}
    for workspace in (source, source / "build"):
        with (workspace / "pnpm-lock.yaml").open() as f:
            lock = list(yaml.safe_load_all(f))[-1]
        for key, metadata in lock["packages"].items():
            resolution = metadata["resolution"]
            if "integrity" not in resolution:
                continue
            name, version = key.rsplit("@", 1)
            filename = name.split("/")[-1] + "-" + version + ".tgz"
            if supports(metadata.get("os"), "openbsd") and supports(metadata.get("cpu"), "x64"):
                archive = Path(name, "-", filename)
                entry = (archive, resolution["integrity"])
                if key in packages and packages[key] != entry:
                    raise ValueError(f"Conflicting resolutions for {key}")
                packages[key] = entry
    return packages


def seed(batch, scratch):
    # Keep the registry identity, including the version, in the store index.
    # "store add file:..." records a different key and cannot serve an offline
    # registry install. Limit the number of open tarballs and WASI threads.
    deps = {f"archive-{i}": f"npm:{key}" for i, key in enumerate(batch)}
    importer = {alias: {"specifier": spec, "version": spec[4:]}
                for alias, spec in deps.items()}
    lock = {"lockfileVersion": "9.0",
            "settings": {"autoInstallPeers": True, "excludeLinksFromLockfile": False},
            "importers": {".": {"dependencies": importer}},
            "packages": {key: {"resolution": resolution} for key, resolution in batch.items()},
            "snapshots": {key: {} for key in batch}}
    with TemporaryDirectory(prefix="seed-", dir=scratch) as temp:
        project = Path(temp)
        (project / "package.json").write_text(json.dumps({"name": "seed", "dependencies": deps}))
        (project / "pnpm-lock.yaml").write_text(json.dumps(lock))
        subprocess.run(["pnpm", "install", "--offline", "--frozen-lockfile",
                        "--ignore-scripts"], cwd=project, check=True)


if __name__ == "__main__":
    if sys.argv[1] == "--distfiles":
        packages = registry_packages(Path(sys.argv[2]))
        print("# Registry archives from the desktop and build pnpm lockfiles.")
        continuation = " " + chr(92) + "\n\t"
        print("DISTFILES.npm =" + continuation + continuation.join(
            str(entry[0]) for key, entry in sorted(packages.items())))
        sys.exit(0)
    source, dist, scratch = map(Path, sys.argv[1:])
    packages = [(key, {"integrity": digest, "tarball": "file:" + str(dist / archive)})
                for key, (archive, digest) in registry_packages(source).items()]
    for start in range(0, len(packages), 32):
        seed(dict(packages[start:start + 32]), scratch)
