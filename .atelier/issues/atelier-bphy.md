---
created_at: "2026-10-17T01:10:57.974341462+00:00"
id: "atelier-bphy"
issue_type: "bug"
labels:
- "cli-output"
- "evidence"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T14:28:10.864853828+00:00"
status: "done"
title: "Keep the tail of captured command output in evidence"
updated_at: "2026-10-23T14:28:10.864853828+00:00"
---

## Goal

Command capture keeps only the first 4,096 bytes of stdout and stderr (`OUTPUT_SUMMARY_LIMIT_BYTES`, `bounded_stream` in crates/atelier-cli/src/commands/evidence.rs:18 and :727). Test runners report failures at the end, so a failing receipt stores only passing lines. Real receipt atelier-1qkb (`cargo nextest run --workspace --features test-hooks`, exit 100, 80,590 bytes of stderr) ends at `PASS [0.012s] ( 29/623) at`, and the failing test is in neither the record nor the output. A scratch repro with 600 PASS lines followed by a FAIL cut off at `PASS test_299`.

## Done when

- Each stream keeps its head and its tail within the byte limit (for example the first 1 KiB and last 3 KiB, joined by `… N bytes omitted …`), and UTF-8 boundaries stay safe as in the existing test at evidence.rs:843.
- A capture of a command that prints 8 KiB and then `FAIL x` keeps `FAIL x`.
- The stored record says which part was omitted. Existing receipts still parse.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed. bounded_stream (evidence.rs:843) keeps bytes[..4096] only; a failing last line is lost. Output lives only in the body Markdown (## Stdout, Bytes:, Truncated:, fenced block); keep 'Truncated: yes' parseable. Record-format change: reinstall after merge.
- 2026-10-23 [decision] A failing stream over 4,096 bytes keeps its first 1,024 and last 3,072 bytes, cut on UTF-8 character boundaries (not line boundaries), joined by a '… N bytes omitted …' line; kept text loses trailing whitespace at line ends.
  Reason: Snapping the tail to a line start can drop most of a long final failure line; trailing whitespace inside captured output would make the new record fail git diff --check.
  Rejected: Cutting at line boundaries; keeping trailing whitespace verbatim

## Close Reason

A failing or blocked capture now keeps each stream whole up to 4,096 bytes, otherwise its first 1,024 and last 3,072 bytes joined by a `… N bytes omitted …` line under `Truncated: yes`, so a command that prints 8 KiB and then `FAIL x` keeps `FAIL x` (commit fc115d5b). Receipts recorded before the change, including atelier-1qkb with `Truncated: yes`, still parse and render (evidence atelier-4dwk, full suite atelier-2c5x).

### Deviations
None. Record-format change: new failing stream sections add a `SHA-256:` line and optionally `Test summary:` after `Truncated:`; old receipts are not migrated. Reinstall the CLI after merge.

### Decisions
Cuts are on UTF-8 character boundaries, not line boundaries, and kept text loses trailing whitespace at line ends; see this item's decision note.

### Surprises
The parser read `Bytes:`/`Truncated:` from any line of the section, including captured text; fields are now read only before the fenced block.

### Open items
None.
