---
created_at: "2026-06-20T16:47:51.096733394+00:00"
id: "atelier-4h62"
issue_type: "epic"
labels:
- "command-surface"
- "cutting-pass"
- "mission-collapse"
review:
  kind: pull_request
  number: 14
  provider: forgejo
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-19xa"
  - kind: "issue"
    id: "atelier-39um"
  - kind: "issue"
    id: "atelier-439j"
  - kind: "issue"
    id: "atelier-7qkf"
  - kind: "issue"
    id: "atelier-bruu"
  - kind: "issue"
    id: "atelier-db6z"
  - kind: "issue"
    id: "atelier-djoq"
  - kind: "issue"
    id: "atelier-e071"
  - kind: "issue"
    id: "atelier-ehit"
  - kind: "issue"
    id: "atelier-n25m"
  - kind: "issue"
    id: "atelier-v2o6"
  - kind: "issue"
    id: "atelier-ybz1"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T21:14:55.799057940+00:00"
status: "done"
title: "Epic: Collapse mission, issue, and graph command surfaces"
updated_at: "2026-06-20T21:14:55.799057940+00:00"
---

## Description

Consolidate command surfaces that answer relationship, objective, and
mission-control questions. The goal is not to flatten missions into ordinary
tasks, but to move mission behavior behind type-aware issue/workflow views
instead of preserving parallel mission and graph namespaces.

## Outcome

- Mission-shaped objectives can be created, displayed, transitioned, and
  closed through type-aware issue/workflow commands.
- The useful mission status view has a defensible home in the general issue or
  status command surface.
- Graph/tree/impact views are folded into stronger issue, objective status,
  and blocker views or explicitly removed.
- Mission focus/start is removed unless a concrete non-lifecycle use remains.

## Evidence

- Product docs and command audit describe the reduced mission/issue/graph
  command model.
- Focused tests prove replacement views answer the former mission and graph
  operator questions.
- Removed mission or graph commands fail as unknown commands after replacements
  exist.
