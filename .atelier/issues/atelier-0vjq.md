---
created_at: "2026-06-13T02:52:25.920806260+00:00"
id: "atelier-0vjq"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
- "evidence"
- "ux"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-dv3d"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Define unified evidence recording command contract"
updated_at: "2026-06-13T04:19:42.539808852+00:00"
---

## Description

Define the command contract that replaces the mental split between `evidence add` and `evidence capture` with one evidence-recording workflow. Manual summaries and command transcripts are input modes for the same operator job: record proof and attach it to accountable work.

## Outcome

- The command contract exposes one normal evidence-recording surface for both manual/prose evidence and command transcript evidence.
- Target attachment syntax is simple and hard to misuse, preferably one argument such as `--target issue/<id>` or an equivalent low-friction form.
- Existing capture behavior for exit status and bounded output is preserved.
- Help and Agent Factory guidance teach one evidence recording workflow.

## Evidence

- Command contract docs or help snapshots define the unified evidence recording surface.
- Review artifact compares the current `evidence add` and `evidence capture` shapes and records the chosen replacement.
- `atelier lint`, `atelier export --check`, and docs whitespace check commands pass.
