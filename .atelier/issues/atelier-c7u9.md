---
created_at: "2026-06-12T02:09:39.822101089+00:00"
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
closed_at: "2026-06-12T03:32:48.055172483+00:00"
status: "done"
title: "Specify mission lifecycle status contract"
updated_at: "2026-06-12T03:32:48.055172483+00:00"
---

## Description

Define the mission lifecycle statuses and their command semantics. Scope: replace the open plus data.active model with draft, ready, active, and closed as mission statuses; preserve the one-active-mission invariant; explain readiness as a status/transition contract rather than a computed health label; keep health/at-risk style summaries out of scope; update the root status, mission status, and mission show contract work as needed. Acceptance: docs or issue notes specify creation defaults, start/activation behavior, list/status filtering, closeout behavior, invalid transition handling, and direct migration expectations for existing records. Mission commands should reject obsolete status aliases instead of preserving compatibility behavior.
Outcome was not specified in the legacy issue record.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
