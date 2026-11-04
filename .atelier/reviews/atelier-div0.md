---
schema: atelier.review
schema_version: 1
id: atelier-div0
title: approval on atelier-g8cn @95a48e1a
target: atelier-g8cn
identity: claude-reviewer-subagent
commit: 95a48e1a9d14286ce07a03ba3bdb0a1a96293a15
created_at: 2026-11-04T22:08:00.054030226Z
event:
  event: approval
---

Version pins: workspace Cargo.toml 0.3.1, all five crates use version.workspace, Cargo.lock updated for all five crates, cargo metadata --locked passes, git grep finds no other 0.3.0 pin outside CHANGELOG/records. CHANGELOG 0.3.1 entry matches git log v0.3.0..HEAD (musl targets 3c486736 + README a523e50c; svi6 unvendoring is repo-internal); its claim that the installer falls back to musl was checked in a locally generated installer (gnu archive dropped on failed check_glibc, then musl archive). dist plan --tag v0.3.1 announces 0.3.1 with gnu+musl x86_64/aarch64 Linux, both macOS archives, checksums, sha256.sum and shell installer. PR #16 CI green at this head: test ubuntu/macos, and the dist pr-run upload built all six targets including both musl. Local proof: diff --check, fmt, metadata --locked, nextest 947/947, nextest --features test-hooks 953/953.
