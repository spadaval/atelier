---
schema: atelier.item
schema_version: 3
id: atelier-u88z
title: Log what a Pi thread did to a file, so the comparison runs yield tool, context and stop data
status: in_review
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- ca01f4b0-4d65-48db-9ae6-ba19251fd343
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T00:54:03.543235147Z
---

## Goal

Write a passive, per-session Pi log so comparison runs can measure tool use,
context consumption, compaction, provider diagnostics and the state at each stop.

## Done when

- [x] A session writes JSONL records for tool timing and sizes (including nested calls), context usage, compaction outcomes, provider diagnostics and settled state.
- [x] Logging failures do not interrupt the agent or send it messages; logs do not contain tool arguments or results.
- [x] The log location and fields are documented, and automated tests cover the records and an unwritable destination.

## Scope

Pi extension logging and its tests and documentation; no completion gate or changes to agent decisions.

## Report

Implemented in 47567edc1. Added best-effort per-session JSONL logging with tool IDs and parent IDs, UTF-8 sizes, execution and wall timing, context snapshots, compaction outcomes, bounded provider diagnostics and settled Atelier state. Documented location, fields and privacy considerations in pi-extension/README.md.

Validation: `node --test pi-extension/test.mjs` passes all 13 tests; `git diff --check` passes. Tests use a stand-in Pi emitter rather than a live provider. Coverage includes nested calls, compaction success/failure, diagnostics, passive handler returns and an unwritable destination.
