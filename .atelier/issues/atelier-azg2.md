---
created_at: "2026-11-08T05:51:38.961142798+00:00"
id: "atelier-azg2"
issue_type: "loose_end"
labels:
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-stnv"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Add the declined status to every other Atelier repository's workflow once the new binary is installed"
updated_at: "2026-11-08T05:51:38.961142798+00:00"
---

## Goal

Let `decline` work in every Atelier repository, not only atelier, after atelier-stnv's binary is installed.

## Done when

- [ ] astromech, life-on-rails, cosplay-studio, demiurge and worldcrafter each have the `declined` status and `decline` transitions in `.atelier/workflow.yaml` as atelier-stnv defines them, committed on each base as a planning change, and `atelier check` passes in each.
