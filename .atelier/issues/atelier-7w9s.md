---
created_at: "2026-11-10T00:56:12.036820453+00:00"
id: "atelier-7w9s"
issue_type: "loose_end"
labels:
- "needs-human"
- "redesign"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-2mlh"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Convert the work host with the cutover runbook"
updated_at: "2026-11-10T00:56:12.036820453+00:00"
---

## Goal

Move Sumeet's work host, which agents on this host cannot reach, onto the new binary, skill and records.

## Done when

- [ ] Sumeet ran the runbook there and `atelier check` passes in each repository on that host.

## Context

The runbook is the Transition section of `docs/strategies/redesign-architecture.md`. Until this is done the work host keeps the old binary, skill and records, which still work together.
