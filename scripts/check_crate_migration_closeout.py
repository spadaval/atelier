#!/usr/bin/env python3
"""Check crate-migration closeout invariants.

The live check is intended for the root-deletion closeout, after the repository
root has become a virtual Cargo workspace. Use --self-test before then to prove
the guard detects representative regressions without requiring the migration to
already be complete.
"""

from __future__ import annotations

import argparse
import tempfile
from pathlib import Path


EXPECTED_MEMBERS = {
    "crates/atelier-core",
    "crates/atelier-workflow",
    "crates/atelier-records",
    "crates/atelier-sqlite",
    "crates/atelier-app",
    "crates/atelier-cli",
}

ROOT_MODULE_PATHS = (
    "src/lib.rs",
    "src/main.rs",
    "src/commands",
    "src/db",
    "src/record_store",
)


def load_root_manifest_text(root: Path) -> str:
    manifest = root / "Cargo.toml"
    if not manifest.exists():
        raise AssertionError(f"missing root Cargo.toml: {manifest}")
    return manifest.read_text(encoding="utf-8")


def has_section(manifest: str, section: str) -> bool:
    return any(line.strip() == section for line in manifest.splitlines())


def workspace_members(manifest: str) -> set[str]:
    members: set[str] = set()
    in_members = False
    for line in manifest.splitlines():
        stripped = line.strip()
        if stripped.startswith("members") and "[" in stripped:
            in_members = True
            after_bracket = stripped.split("[", 1)[1]
            if "]" in after_bracket:
                in_members = False
        elif in_members and "]" in stripped:
            in_members = False

        if in_members or stripped.startswith("members"):
            for part in stripped.split(","):
                value = part.strip().strip("[]").strip()
                if value.startswith('"') and value.endswith('"'):
                    members.add(value.strip('"'))
    return members


def check_root(root: Path) -> list[str]:
    manifest = load_root_manifest_text(root)
    failures: list[str] = []

    if has_section(manifest, "[package]"):
        failures.append("root Cargo.toml must be a virtual workspace, but [package] is present")
    if has_section(manifest, "[lib]"):
        failures.append("root Cargo.toml must not define a [lib] target")
    if has_section(manifest, "[[bin]]"):
        failures.append("root Cargo.toml must not define root [[bin]] targets")
    if not has_section(manifest, "[workspace]"):
        failures.append("root Cargo.toml must define [workspace]")

    members = workspace_members(manifest)
    missing = sorted(EXPECTED_MEMBERS - members)
    if missing:
        failures.append(f"workspace is missing expected members: {', '.join(missing)}")

    for rel_path in ROOT_MODULE_PATHS:
        if (root / rel_path).exists():
            failures.append(f"root package module path must be deleted: {rel_path}")

    return failures


def write_fixture(root: Path, cargo_toml: str, root_paths: tuple[str, ...] = ()) -> None:
    (root / "Cargo.toml").write_text(cargo_toml, encoding="utf-8")
    for member in EXPECTED_MEMBERS:
        (root / member).mkdir(parents=True, exist_ok=True)
    for rel_path in root_paths:
        path = root / rel_path
        if rel_path.endswith(".rs"):
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text("// regression fixture\n", encoding="utf-8")
        else:
            path.mkdir(parents=True, exist_ok=True)


def run_self_test() -> None:
    good_manifest = """
[workspace]
members = [
  "crates/atelier-core",
  "crates/atelier-workflow",
  "crates/atelier-records",
  "crates/atelier-sqlite",
  "crates/atelier-app",
  "crates/atelier-cli",
]
"""
    bad_manifest = """
[package]
name = "atelier-tracker"
version = "0.2.0"
edition = "2021"

[workspace]
members = [
  "crates/atelier-core",
  "crates/atelier-workflow",
  "crates/atelier-records",
  "crates/atelier-sqlite",
  "crates/atelier-app",
  "crates/atelier-cli",
]

[lib]
name = "atelier"
path = "src/lib.rs"

[[bin]]
name = "atelier"
path = "src/main.rs"
"""

    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        write_fixture(root, good_manifest)
        good_failures = check_root(root)
        if good_failures:
            raise AssertionError(f"expected good fixture to pass, got: {good_failures}")

    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        write_fixture(root, bad_manifest, ("src/lib.rs", "src/main.rs", "src/commands"))
        bad_failures = check_root(root)
        expected_fragments = (
            "[package] is present",
            "[lib] target",
            "[[bin]] targets",
            "src/lib.rs",
            "src/main.rs",
            "src/commands",
        )
        missing = [fragment for fragment in expected_fragments if not any(fragment in failure for failure in bad_failures)]
        if missing:
            raise AssertionError(f"bad fixture did not report expected failures {missing}: {bad_failures}")


def main() -> int:
    parser = argparse.ArgumentParser(description="Check crate migration closeout invariants")
    parser.add_argument("--root", type=Path, default=Path.cwd(), help="repository root to check")
    parser.add_argument("--self-test", action="store_true", help="run built-in fixture tests")
    args = parser.parse_args()

    if args.self_test:
        run_self_test()
        print("crate migration closeout guard self-test passed")
        return 0

    failures = check_root(args.root)
    if failures:
        print("crate migration closeout guard failed:")
        for failure in failures:
            print(f"- {failure}")
        return 1

    print("crate migration closeout guard passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
