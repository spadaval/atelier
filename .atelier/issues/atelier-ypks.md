---
created_at: "2026-10-17T18:07:05.623651933+00:00"
id: "atelier-ypks"
issue_type: "bug"
labels:
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Clear or confirm a reused Close Reason when a reopened item closes again"
updated_at: "2026-10-17T18:14:22.478074553+00:00"
---

## Goal

A record that leaves done keeps its Close Reason, and the close transition with no flag reuses it, so a reopened item closed again silently carries its old Outcome and report; `--reason` on supersede also keeps report parts written for a close. Found by review atelier-hmta on atelier-sfsq.

## Done when

A transition out of a done status clears Close Reason (or the close transition says which Close Reason it used), and `--reason` on supersede does not keep report parts written for a close, shown by transition tests that reopen and reclose an item and that supersede an item whose record holds a report.
