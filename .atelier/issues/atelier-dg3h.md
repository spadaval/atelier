---
created_at: "2026-10-23T14:13:37.487607900+00:00"
id: "atelier-dg3h"
issue_type: "feature"
labels:
- "cli-output"
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
status: "todo"
title: "Evidence receipts keep the failure tail and summarize passes"
updated_at: "2026-10-23T14:13:47.492658520+00:00"
---

## Goal

Evidence receipts keep what a reader needs and nothing else: a failing capture keeps the head and the tail of each stream with an omitted-bytes marker, a passing capture stores a one-line summary plus each stream's byte count and digest instead of the log, and records end without a trailing blank line.

## Done when

CLI tests: a failing capture whose last line is the failure keeps that line; a passing capture's record has no stream body, only the summary, byte counts and digests; every new record passes `git diff --check`; existing receipts in both old shapes (with `Truncated: yes`, and with no output section) still parse and render (a compatibility test over fixtures copied from `.atelier/evidence/`). Delivers S3's tail, passing-summary and trailing-line clauses. Author proof plus independent review. The CLI is reinstalled after merge because the record format changes.

## Scope

Children: atelier-bphy, atelier-57wj, atelier-auca. Existing receipts are not rewritten. Output views (`record`, `list`, `show`) belong to the next evidence increment except where a format change forces a renderer change.

## Context

Regroom 2026-10-23: capture in `crates/atelier-cli/src/commands/evidence.rs` (`bounded_stream` :843, `capture_record`, 4 KiB head-only cap); body render/parse in `crates/atelier-records/src/lib.rs` (`render_evidence_transcript` :2030, `render_evidence_body` :2013, stream sections :2075/:2181). Today 562 of 590 receipts are passes holding 2.1 MB of logs; 533 end with a blank line.
