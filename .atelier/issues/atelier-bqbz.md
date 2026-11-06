---
created_at: "2026-11-06T21:26:18.892841983+00:00"
id: "atelier-bqbz"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Keep only the statuses and issue types repositories use"
updated_at: "2026-11-06T21:26:18.892841983+00:00"
---

## Goal

Each workflow status and issue type is kept only if records across atelier, astromech, demiurge and cosplay-studio use it, or a gate needs it; the after-merge state replaces what it makes redundant.

## Done when

Provisional: a note gives per-status and per-type counts across those repositories, including branch history for statuses that are transient on the base; the starter workflow policy drops what is unused; an atelier migrate step retypes existing records; existing repository policies keep working.

## Unknowns

How often in_progress, review and blocked are used on branches, which the base never shows at rest.
