---
created_at: "2026-11-06T22:08:25.195127217+00:00"
id: "atelier-rkxm"
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
status: "todo"
title: "Count how repositories use each status and issue type"
updated_at: "2026-11-06T22:08:25.195127217+00:00"
---

## Goal

Give the mission the facts that its after-merge model (atelier-n0qh) and the status and type cut (atelier-bqbz) are designed from: how often each workflow status and issue type is used across atelier, astromech, demiurge and cosplay-studio, including statuses that appear only on work branches and never at rest on the base.

## Done when

- A note on this issue gives, per repository, a table of each status and each issue type in that repository's `.atelier/workflow.yaml`: records currently in it on the base, and records that passed through it in git history on any branch (counted from the record files' history, `git log --all -p -- .atelier/issues`).
- The same note names, for each status and type, the gates or commands in Atelier's code that read it (file:line), or says none do.
- An evidence receipt on this issue records the command or script used, so the counts can be rerun; a script, if any, is committed under `scripts/` or kept in the receipt itself.

## Scope

Measurement only. No change to policies, records or product code. Repositories are read in place at ~/projects/{atelier,astromech,demiurge,cosplay-studio}; astromech-atelier and astromech-orch share astromech's history and are counted once.

## Context

Strategy: docs/strategies/lighter-workflow.md (Shape, "Statuses and types follow use"). Sumeet asked to measure before cutting: every type is believed to be in use today.
