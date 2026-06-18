---
created_at: "2026-06-18T16:45:42.871295388+00:00"
id: "atelier-cln0"
issue_type: "feature"
labels:
- "activity"
- "pr"
- "session-pr-overhaul"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-cer4"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T18:01:55.598661343+00:00"
status: "done"
title: "Record PR actions as issue-event attribution"
updated_at: "2026-06-18T18:01:55.598661343+00:00"
---

## Description

Record local attribution for PR actions as canonical issue activity metadata. Scope includes PR open/comment/review/merge command paths after target resolution. Out of scope: target inference rules and remote merge semantics.

## Outcome

PR actions write structured issue activity that attributes the action to worker, reviewer, or validator attempts derived from issue events. Forgejo sudo authorship remains remote command authorship and does not replace Atelier issue-event attribution.

## Evidence

Focused tests prove PR open/comment/review/merge paths record issue activity metadata with role, action, PR identifier, and target issue or epic, and that history/status/session inspection can read the attribution without standalone session records.
