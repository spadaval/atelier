---
created_at: "2026-06-17T19:37:43.104869767+00:00"
id: "atelier-nm00"
issue_type: "task"
labels:
- "app-layer"
- "audit"
- "docs"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-uro5"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Record app-layer contract audit and closeout criteria"
updated_at: "2026-06-17T23:27:50.781571605+00:00"
---

## Description

Record the audit finding that closed app-layer tracker work overstates the
current live-code boundary. The artifact should make future closeout proof
stricter without reopening closed epics.

## Outcome

- A durable note, architecture-quality entry, or linked evidence record explains
  why the previous app/CLI split closeout was insufficient.
- The artifact states the searchable proof required for future app-layer
  closeout claims.
- The artifact points workers to the follow-up epic and child issues that own
  the live-code reconciliation.

## Evidence

- Documentation diff, issue note, or evidence record quotes the current live
  contradiction and the new closeout criteria.
- Search transcript backs the contradiction with current file locations.
- `atelier lint` and `git diff --check` pass.
