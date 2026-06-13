---
created_at: "2026-06-13T01:08:17.135459296+00:00"
id: "atelier-wvpb"
issue_type: "task"
labels:
- "validation"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-v9id"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Repair workflow validator expectation drift"
updated_at: "2026-06-13T01:08:17.135459296+00:00"
---

## Description

The default workflow-validator unit test still expects the mission close validator list from before command-surface freshness became part of the mission closeout contract. Repair the test so it matches the current public closeout validators without weakening any validator behavior.

## Outcome

- The default mission close validator expectation includes command-surface freshness when that validator is part of the actual closeout contract.
- The focused workflow validator unit test passes without removing any required mission closeout gate.
- Tracker lint and export checks remain clean for this repair record.

## Evidence

- Run `cargo nextest run default_validators_are_target_and_transition_aware`.
- Run `target/debug/atelier rebuild && target/debug/atelier lint <issue-id> && target/debug/atelier export --check`.
- Run `git diff --check`.
