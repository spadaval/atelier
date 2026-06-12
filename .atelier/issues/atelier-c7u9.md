---
acceptance: []
created_at: "2026-06-12T02:09:39.822101089+00:00"
evidence_required: []
id: "atelier-c7u9"
issue_type: "task"
labels:
- "cli"
- "mission"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-cv3p"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Specify mission lifecycle status contract"
updated_at: "2026-06-12T02:09:39.822101089+00:00"
---

Define the mission lifecycle statuses and their command semantics. Scope: replace the open plus data.active model with draft, ready, active, and closed as mission statuses; preserve the one-active-mission invariant; explain readiness as a status/transition contract rather than a computed health label; keep health/at-risk style summaries out of scope; update the root status, mission status, and mission show contract work as needed. Acceptance: docs or issue notes specify creation defaults, start/activation behavior, list/status filtering, closeout behavior, invalid transition handling, and migration expectations for existing open/active records.
