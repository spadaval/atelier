---
created_at: "2026-06-13T02:52:22.538971656+00:00"
id: "atelier-lme0"
issue_type: "task"
labels:
- "cli"
- "docs"
- "ux"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-im60"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Define simple operator command taxonomy"
updated_at: "2026-06-13T02:52:22.538971656+00:00"
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
