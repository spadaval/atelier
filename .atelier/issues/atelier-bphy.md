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
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Keep the tail of captured command output in evidence"
updated_at: "2026-10-17T01:10:57.974341462+00:00"
---

## Goal

Command capture keeps only the first 4,096 bytes of stdout and stderr (`OUTPUT_SUMMARY_LIMIT_BYTES`, `bounded_stream` in crates/atelier-cli/src/commands/evidence.rs:18 and :727). Test runners report failures at the end, so a failing receipt stores only passing lines. Real receipt atelier-1qkb (`cargo nextest run --workspace --features test-hooks`, exit 100, 80,590 bytes of stderr) ends at `PASS [0.012s] ( 29/623) at`, and the failing test is in neither the record nor the output. A scratch repro with 600 PASS lines followed by a FAIL cut off at `PASS test_299`.

## Done when

- Each stream keeps its head and its tail within the byte limit (for example the first 1 KiB and last 3 KiB, joined by `… N bytes omitted …`), and UTF-8 boundaries stay safe as in the existing test at evidence.rs:843.
- A capture of a command that prints 8 KiB and then `FAIL x` keeps `FAIL x`.
- The stored record says which part was omitted. Existing receipts still parse.
