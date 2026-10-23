---
created_at: "2026-10-23T14:13:37.487607900+00:00"
id: "atelier-dg3h"
issue_type: "feature"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-dg3h
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-57wj"
  - kind: "issue"
    id: "atelier-auca"
  - kind: "issue"
    id: "atelier-bphy"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T14:35:48.747465060+00:00"
status: "done"
title: "Evidence receipts keep the failure tail and summarize passes"
updated_at: "2026-10-23T14:35:48.747465060+00:00"
---

## Goal

Evidence receipts keep what a reader needs and nothing else: a failing capture keeps the head and the tail of each stream with an omitted-bytes marker, a passing capture stores a one-line summary plus each stream's byte count and digest instead of the log, and records end without a trailing blank line.

## Done when

CLI tests: a failing capture whose last line is the failure keeps that line; a passing capture's record has no stream body, only the summary, byte counts and digests; every new record passes `git diff --check`; existing receipts in both old shapes (with `Truncated: yes`, and with no output section) still parse and render (a compatibility test over fixtures copied from `.atelier/evidence/`). Delivers S3's tail, passing-summary and trailing-line clauses. Author proof plus independent review. The CLI is reinstalled after merge because the record format changes.

## Scope

Children: atelier-bphy, atelier-57wj, atelier-auca. Existing receipts are not rewritten. Output views (`record`, `list`, `show`) belong to the next evidence increment except where a format change forces a renderer change.

## Context

Regroom 2026-10-23: capture in `crates/atelier-cli/src/commands/evidence.rs` (`bounded_stream` :843, `capture_record`, 4 KiB head-only cap); body render/parse in `crates/atelier-records/src/lib.rs` (`render_evidence_transcript` :2030, `render_evidence_body` :2013, stream sections :2075/:2181). Today 562 of 590 receipts are passes holding 2.1 MB of logs; 533 end with a blank line.

## Close Reason

### Outcome
Evidence receipts keep what a reader needs. A failing capture keeps each stream's first 1 KiB and last 3 KiB with a `… N bytes omitted …` marker, cut on UTF-8 boundaries, so a final failure line survives (atelier-bphy). A passing capture stores each stream's byte count, SHA-256 and a test-count line instead of the log (atelier-57wj). Every evidence file ends with exactly one newline (atelier-auca). Captured text is fenced with a backtick run longer than any inside it, so output cannot pose as a section or field. Delivers atelier-rc8j S3's failure-tail, passing-summary and trailing-line clauses; shown by CLI and records tests (receipts atelier-20sv, atelier-yfod, atelier-dpm6).

### Deviations
The one-newline rule lives in the shared evidence writer, so a receipt rewritten for another reason (such as attach) also loses an old trailing blank line. Failing streams also carry the digest and test-count line. A narrow `.gitattributes` exempts the copied old-shape fixtures from whitespace checks.

### Decisions
Decision notes on atelier-bphy, atelier-57wj and atelier-auca; existing receipts are not rewritten (atelier-rc8j decision).

### Surprises
Independent review found captured output could break out of its fence and be misattributed to another stream (atelier-yif8) and a wrong label on a blank failing stream (atelier-7yla); both fixed in 4cdaf39f.

### Open items
Record-format change: reinstall the CLI after merge. `evidence record`, `list` and `show` views are atelier-l7nz.
