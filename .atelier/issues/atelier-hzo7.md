---
created_at: "2026-06-18T16:20:35.192250064+00:00"
id: "atelier-hzo7"
issue_type: "feature"
labels:
- "activity"
- "session-pr-overhaul"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-cln0"
  - kind: "issue"
    id: "atelier-fdi4"
  - kind: "issue"
    id: "atelier-lvgo"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add issue-event attempt attribution model"
updated_at: "2026-06-18T16:48:58.919069823+00:00"
---

## Description

Add the canonical issue activity metadata needed to derive issue-scoped role attempts and PR attribution history. This work implements the data model chosen by `atelier-7ssp`; it does not preserve `.atelier/sessions` as live workflow truth.

## Outcome

Issue activity records can carry structured attempt and PR attribution metadata, attempts are derived by issue, role, and serial, and `.atelier/sessions` is no longer used as live session truth. Existing `.atelier/sessions` files are ignored rather than migrated.

## Evidence

Unit and CLI test output proves activity metadata round-trips, projection rebuilds preserve the metadata, and session projections are derived from issue activity records rather than standalone session records.
