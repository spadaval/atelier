---
created_at: "2026-06-18T16:33:37.559511280+00:00"
id: "atelier-ngb2"
issue_type: "epic"
labels:
- "epic"
- "session-pr-overhaul"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-4clo"
  - kind: "issue"
    id: "atelier-98mo"
  - kind: "issue"
    id: "atelier-cglp"
  - kind: "issue"
    id: "atelier-p7oa"
  children:
  - kind: "issue"
    id: "atelier-5k7k"
  - kind: "issue"
    id: "atelier-7ssp"
  - kind: "issue"
    id: "atelier-hzo7"
  - kind: "issue"
    id: "atelier-jdvz"
  - kind: "issue"
    id: "atelier-lvgo"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T18:21:09.349929150+00:00"
status: "done"
title: "Epic: Simplify session and PR attribution model"
updated_at: "2026-06-18T18:21:09.349929150+00:00"
---

## Description

Coordinate the corrective session and PR attribution overhaul inside mission `atelier-0v3f`. The new session-as-issue-events model wins over the older durable optional session model: sessions are derived worker/reviewer/validator attempts from canonical issue activity, not standalone workflow records or mutating workflow drivers. This epic owns the coherent branch/review boundary while child issues split documentation, activity metadata, CLI behavior, PR coordination, and validation work.

## Outcome

Atelier supports lightweight issue-scoped worker/reviewer/validator attempts derived from canonical issue activity, preserves PRs as optional issue-linked review artifacts, and exposes status/history/session/PR behavior that tells a coherent attribution story without making standalone session records or PRs mandatory for unrelated work.

## Evidence

Child issue proof plus validation issue `atelier-lvgo` demonstrate the docs, issue-event activity model, CLI behavior, PR coordination, and end-to-end validation all pass together under this epic.
