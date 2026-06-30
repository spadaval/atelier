---
created_at: "2026-06-29T20:10:40.564602241+00:00"
id: "atelier-kfey"
issue_type: "task"
labels:
- "cli"
- "tests"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ubf2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add regression coverage for transition output budget"
updated_at: "2026-06-29T20:10:40.564602241+00:00"
---

## Description

Add focused command-level coverage for the simplified transition output so future formatter or workflow changes do not reintroduce the validator/action firehose as the default view.

## Outcome

Tests or recorded CLI transcripts prove `atelier issue transition <id>` default output shows transition names, allowed/blocked state, and failed requirements only. The same coverage proves passing validators, long validator messages, action preflight details, dirty path dumps, transition descriptions, and full debug detail are absent from default output and available through the explicit verbose/detail path.
