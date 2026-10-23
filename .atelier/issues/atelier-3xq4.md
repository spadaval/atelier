---
created_at: "2026-10-23T15:01:14.879470764+00:00"
id: "atelier-3xq4"
issue_type: "bug"
labels: []
priority: "P2"
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
status: "backlog"
title: "Rerun receipts from merge --sync nest the old receipt's whole body in their summary"
updated_at: "2026-10-23T15:01:24.167480796+00:00"
---

## Goal

`atelier merge <id> --sync` reruns current command-backed receipts with the summary `<old summary> (rerun after merging <base>)`, but the old summary it reads (`record.summary` in `crates/atelier-cli/src/commands/merge_sync.rs:316`) is the whole receipt body: Summary, Command, Exit status and stream sections. The rerun receipt then nests the full old body inside its own Summary, every later sync nests it again, and field-like lines such as `Test summary:` inside it can be read as the new receipt's fields.

## Done when

A CLI test syncs a branch with a current captured receipt and asserts the rerun receipt's Summary section is the old Summary text plus the rerun suffix, with no Command or stream lines.

## Context

Seen 2026-10-23 on feature/atelier-h1nc: rerun receipts atelier-7t38 and atelier-w253 (commit 80953e4b) carry atelier-8z2v's whole body in their summary. Check whether `EvidenceRecord.summary` holds the body by design (then take the Summary section) or the parser is wrong since atelier-dg3h.
