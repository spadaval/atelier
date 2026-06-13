---
created_at: "2026-06-13T02:52:20.145617047+00:00"
id: "atelier-sv98"
issue_type: "epic"
labels:
- "cli"
- "product"
- "ux"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-88ud"
  - kind: "issue"
    id: "atelier-g3k0"
  - kind: "issue"
    id: "atelier-im60"
  - kind: "issue"
    id: "atelier-j6v4"
  - kind: "issue"
    id: "atelier-jxzc"
  - kind: "issue"
    id: "atelier-ktcm"
  - kind: "issue"
    id: "atelier-lme0"
  - kind: "issue"
    id: "atelier-oezx"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T16:09:18.038144101+00:00"
status: "done"
title: "Simplify core command surface around user workflows"
updated_at: "2026-06-13T16:09:18.038144101+00:00"
---

## Description

Run a broad command consolidation and simplification pass guided by real human and agent use. The goal is not fewer commands for its own sake; it is a smaller, clearer workflow where operators ask domain questions and receive safe next actions without knowing internal subsystems.
- The normal command surface is organized around user workflows: orient, choose work, inspect work, record proof, finish or close, inspect history, and recover health.
- Duplicate or competing commands are merged, hidden, or deleted unless they serve a distinct operator job.
- Advanced diagnostics remain available only where they are clearly labeled and absent from normal next actions.
- Command names, help, docs, and Agent Factory guidance optimize for low-mistake use by real humans and agents.
- The pass preserves necessary drill-down power without making every workflow start from a broad command reference.
- File-change review of product CLI docs records the command taxonomy and consolidation decisions.
- Transcript artifacts or focused tests cover representative human/agent workflows before and after consolidation.
- Validation evidence record classifies retained, merged, hidden, and deleted command surfaces with reasons.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
