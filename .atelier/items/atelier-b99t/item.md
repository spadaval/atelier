---
schema: atelier.item
schema_version: 3
id: atelier-b99t
title: Point build.rs rerun-if-changed at the repository's .git, not crates/.git
status: draft
mission: null
labels:
- bug
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-28T01:52:31.022126706Z
---

## Goal

`crates/atelier-cli/build.rs` emits `cargo:rerun-if-changed=../.git/HEAD` and `../.git/refs/`, which resolve to `crates/.git`, a path that does not exist. A local `cargo build` can therefore stamp `atelier --version` with a stale commit or dirty flag. CI and release builds start fresh, so they are unaffected.

## Done when

- [ ] After a commit, `cargo build -p atelier-cli` with no other change reports the new short hash in `target/debug/atelier --version`. It must still build from a source tarball that has no `.git`.

## Context

Found in review of atelier-7v9q (finding R6).

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-7v9q (Publish tagged releases with dist: Linux and macOS archives, checksums, shell installer)
