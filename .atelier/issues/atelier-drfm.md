---
created_at: "2026-06-12T20:29:16.997476280+00:00"
id: "atelier-drfm"
issue_type: "task"
labels:
- "cli"
- "command-surface"
- "implementation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-auqt"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Move non-lifecycle issue commands to explicit homes"
updated_at: "2026-06-12T20:29:16.997476280+00:00"
---

## Description

Move non-lifecycle issue commands out of the primary issue command group or hide
them from normal help. Dependency, link, search, hierarchy, activity, and
destructive maintenance behavior should have explicit homes outside ordinary
issue lifecycle work.

## Outcome

- Dependency operations no longer appear as normal `issue` lifecycle commands.
- Link, search, hierarchy/impact, activity/comment, delete, bulk-close, next,
  and tested behaviors are moved, hidden, or removed according to the command
  classification.
- Replacement commands exist for retained behavior and produce user-facing
  guidance when old commands remain as hidden compatibility.
- `atelier issue --help` scans as lifecycle-only.
- Tests stop preserving the old mixed issue surface as normal behavior.

## Evidence

- CLI transcript tests for moved dependency, link, search, hierarchy, activity,
  and destructive-maintenance workflows classified as retained.
- Help transcript proving non-lifecycle commands are absent from primary issue
  help.
- Negative tests for removed or hidden commands, including replacement guidance
  where compatibility remains.
