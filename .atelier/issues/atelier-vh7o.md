---
created_at: "2026-06-11T02:28:52.991144170+00:00"
id: "atelier-vh7o"
issue_type: "task"
labels:
- "assignee:root"
- "cli-output"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T02:32:54.218262026+00:00"
status: "done"
title: "Improve mission list human overview"
updated_at: "2026-06-11T02:32:54.218262026+00:00"
---

## Description

Scope: Upgrade human `atelier mission list` output to action-first grouped overview with status summary, linked work counts, blocker counts, evidence gap information, and next commands. Preserve `--json` shape and existing `--status` filtering.

## Outcome

human mission list sorts non-closed missions before closed missions by updated time descending; rows include updated date, work counts, blockers, and evidence state; empty filtered output includes next-command guidance; JSON mission list remains `{ data: [...] }`.

## Evidence

Evidence was not specified in the legacy issue record.
