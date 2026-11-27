---
schema: atelier.item
schema_version: 3
id: atelier-u88z
title: Log what a Pi thread did to a file, so the comparison runs yield tool, context and stop data
status: changes_requested
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

How this item was written, which a reviewer should know: the logging code and its test were written by the thread that holds the item. During a real-model check, a Sol session in Pi was given a one-line prompt asking for a single `codemode` call. It ignored the prompt, took the extension's start message as its assignment, and instead added `toolCallId` and byte sizes to the log rows, wrote the README section and this record's Goal, Done when and first Report paragraphs, committed and ran `atelier submit`. Its changes were read afterward and kept. Treat the whole diff as unreviewed.

Checked afterward against Pi 1.6.1 with `gpt-6.1-sol`: a `bash` call made from inside a `codemode` script is logged with the script's call as its `parent`; context size is logged at every turn; and the stop is logged with the item's status. Not seen in a real run: a compaction row or a diagnostics row, since no short session produces either.

