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
    id: "atelier-n0qh"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
  - kind: "issue"
    id: "atelier-rkxm"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Keep only the statuses and issue types repositories use"
updated_at: "2026-11-06T22:08:52.775970858+00:00"
---

## Goal

Each workflow status and issue type is kept only if atelier-rkxm's counts show repositories use it, or a gate needs it; the after-merge state from atelier-n0qh replaces what it makes redundant.

## Done when

Provisional: the starter workflow policy drops what is unused, with each drop justified from atelier-rkxm's note; an atelier migrate step retypes existing records; existing repository policies keep working; the choice is recorded under this item.

## Unknowns

Whether any repository's own policy depends on a status the starter drops.
