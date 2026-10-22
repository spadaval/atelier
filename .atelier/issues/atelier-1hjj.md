---
created_at: "2026-10-22T20:59:30.576687438+00:00"
id: "atelier-1hjj"
issue_type: "bug"
labels:
- "merge"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Print merge's report only from the attempt that returns"
updated_at: "2026-10-22T20:59:30.576687438+00:00"
---

## Goal

`atelier merge` checks whether the base moved by records-only commits once before printing its gate report (crates/atelier-cli/src/commands/merge.rs, around lines 530-541) and again after taking the mutation locks (around 586-589). If the base moves between the two, the report has already printed and the attempt still retries, so `--json` stdout holds two arrays. The window is now microseconds plus any lock wait. Found in review of atelier-c3kh (finding atelier-t9ew).

## Done when

A records-only base move at any point before integration yields exactly one printed report, from the attempt that returns: either take the locks before the pre-print check, or hold the report and print it only on return. A test that moves the base after the pre-print check, using a test hook, asserts stdout parses as one JSON document.
