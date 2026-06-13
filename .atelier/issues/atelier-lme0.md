---
created_at: "2026-06-13T02:52:22.538971656+00:00"
id: "atelier-lme0"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
- "docs"
- "ux"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-im60"
  - kind: "issue"
    id: "atelier-j6v4"
  - kind: "issue"
    id: "atelier-oezx"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Define simple operator command taxonomy"
updated_at: "2026-06-13T04:08:59.680155697+00:00"
---

## Description

Define the small set of operator jobs the CLI should optimize for before changing commands. The taxonomy should keep the tool simple enough that humans and agents can use it correctly under pressure.

## Outcome

- Product docs define the normal operator jobs and the command families that own them.
- The taxonomy distinguishes normal workflow commands from advanced diagnostics and destructive maintenance.
- The design includes a red-tape check: command consolidation must reduce mistakes and cognitive load, not add ritual.

## Evidence

- File-change review of product CLI docs shows the taxonomy and red-tape check.
- Review artifact includes representative workflows for a human operator and an Agent Factory worker.
- `atelier lint`, `atelier export --check`, and docs whitespace check commands pass.

## Notes

Initial command audit candidates:

- `evidence add` and `evidence capture` are two verbs for recording proof; the unified evidence command work should decide the normal spelling and attachment syntax.
- `mission status` and `mission audit` both answer readiness/proof questions; the mission operator contract should decide whether audit is a status mode, closeout mode, or advanced diagnostic.
- `workflow validate` remains discoverable as an advanced diagnostic while docs still describe broader repo-defined workflows; treat the repo-defined workflow mismatch as a known external bug and avoid relying on those unimplemented workflow claims in this mission.
- `issue create --template` and `issue create --issue-type` expose overlapping creation models and currently interact surprisingly with `--parent`.
- `start`, `finish`, `issue close`, `issue transition --options`, and `issue update --status` split lifecycle operations across field mutation and lifecycle command surfaces.
- `dep`, `link`, and `graph` expose relationship management through three nouns; classify which relationship jobs are normal workflow, drill-down, or advanced graph inspection.
- `lint`, `doctor`, `export --check`, and `rebuild` need a clear health/recovery split so normal users know when to check, when to repair, and which command output is authoritative.
- Wrong-kind record IDs are a known orientation hazard. The taxonomy should
  decide whether type-aware read commands, a generic record lookup, or better
  wrong-kind errors own this workflow.
- Installed binary/schema drift is a known mission hazard. The taxonomy should
  classify it as a health/recovery concern and keep the normal tracker command
  guidance aligned with Agent Factory.
