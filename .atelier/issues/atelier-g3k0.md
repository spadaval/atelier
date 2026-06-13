---
created_at: "2026-06-13T04:01:43.142202423+00:00"
id: "atelier-g3k0"
issue_type: "task"
labels:
- "cli"
- "projection"
- "runtime"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T04:26:36.954904843+00:00"
status: "done"
title: "Harden projection rebuild contention and recovery"
updated_at: "2026-06-13T04:26:36.954904843+00:00"
---

## Description

Make projection rebuild and runtime-cache recovery reliable under parallel agent reads and writes. User-facing diagnostics should filter rebuild temp and journal artifacts, avoid false blockers where possible, and offer one clear recovery path.

## Outcome

- Projection rebuilds are safe under parallel agent reads and writes, with
  lock-aware behavior or clear retry semantics.
- User-facing health and lint diagnostics filter rebuild temp files, SQLite
  journal files, and other ignored runtime/cache artifacts.
- Runtime/projection failures produce one clear recovery path instead of a
  mixture of stale projection, malformed record, and temp-file noise.
- Read-only orientation commands remain usable or fail with actionable recovery
  guidance when projection rebuild contention occurs.

## Evidence

- Focused concurrency test or transcript covers parallel read/write or rebuild
  contention and records the expected recovery behavior.
- Regression test proves rebuild temp and journal artifacts are ignored by
  lint/health diagnostics.
- Transcript shows the recovery path from stale or missing projection state.
- `atelier lint`, `atelier export --check`, `atelier doctor`, and relevant
  projection/runtime tests pass.
