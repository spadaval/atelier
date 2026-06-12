---
created_at: "2026-06-12T20:29:04.870070382+00:00"
id: "atelier-exz1"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
- "command-surface"
- "planning"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-64ea"
  - kind: "issue"
    id: "atelier-drfm"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Classify current issue subcommands against lifecycle contract"
updated_at: "2026-06-12T20:59:21.370723616+00:00"
---

## Description

Audit the current `atelier issue` command surface against the reduced lifecycle
contract captured in `atelier-9jbu` and `atelier-o2a4`. This is the decision
handoff for implementation: workers should not have to rediscover which issue
subcommands are kept, folded, moved, hidden, or removed.

## Outcome

- Every current `atelier issue` subcommand is inventoried from current help and
  source behavior.
- Each subcommand is classified as keep, fold into lifecycle options, move to a
  domain command group, hide as compatibility, or remove.
- The target home and replacement command are named for every moved or folded
  command.
- Any newly discovered command-surface gap becomes a follow-up issue before
  implementation starts.
- The active `atelier-efpk` epic points workers to the current classification
  and replacement commands directly.

## Evidence

- Current `atelier issue --help` transcript or equivalent inventory.
- Classification table linked from the issue, docs, or durable notes.
- Review note confirming the classification matches `atelier-9jbu` and
  `atelier-o2a4`.
