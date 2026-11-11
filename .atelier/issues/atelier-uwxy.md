---
created_at: "2026-11-11T02:17:25.926248278+00:00"
id: "atelier-uwxy"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1ie1"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Serve the mission page's data from the new core: the atelier-desk crate, its API types and the views it needs"
updated_at: "2026-11-11T02:17:25.926248278+00:00"
---

## Goal

Preliminary. Stand up `next/crates/atelier-desk` serving repositories and one mission's page data from a commit, with any view the first increment found missing added to `atelier-app`.

## Done when

- [ ] Provisional: `atelier-desk` serves the routes `docs/architecture/desk.md` gives for repositories and a mission, from converted scratch clones, with types generated for the app; a test compares each mission's title, status and item counts with `atelier show` on the same commit; a workspace test fails if another crate depends on `atelier-desk`.

## Unknowns

- Which views are missing and how large they are; settled by atelier-1ie1.
