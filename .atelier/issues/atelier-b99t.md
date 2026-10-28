---
created_at: "2026-10-28T01:52:31.022126706+00:00"
id: "atelier-b99t"
issue_type: "bug"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-7v9q"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Point build.rs rerun-if-changed at the repository's .git, not crates/.git"
updated_at: "2026-10-28T01:52:31.022126706+00:00"
---

## Goal

`crates/atelier-cli/build.rs` emits `cargo:rerun-if-changed=../.git/HEAD` and `../.git/refs/`, which resolve to `crates/.git`, a path that does not exist. A local `cargo build` can therefore stamp `atelier --version` with a stale commit or dirty flag. CI and release builds start fresh, so they are unaffected.

## Done when

After a commit, `cargo build -p atelier-cli` with no other change reports the new short hash in `target/debug/atelier --version`. It must still build from a source tarball that has no `.git`.

## Context

Found in review of atelier-7v9q (finding R6).
