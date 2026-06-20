---
created_at: "2026-06-20T15:11:48.954007711+00:00"
id: "atelier-p1yz"
issue_type: "task"
labels:
- "architecture"
- "cutting-pass"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Split overloaded CLI modules after removed surfaces are gone"
updated_at: "2026-06-20T16:48:55.397169481+00:00"
---

## Description

After removed surfaces are deleted, split overloaded CLI modules along product or use-case boundaries. Current audit pressure points are root dispatch in `main.rs`, `commands/workflow.rs`, `commands/mission.rs`, and `commands/agent_factory.rs`.

## Outcome

Command dispatch and large command modules are easier to navigate without changing product behavior. New boundaries follow actual product surfaces rather than preserving old command families.

## Evidence

- `cargo build -p atelier-cli` passes after module extraction.
- Focused integration tests for workflow, mission, and agent-factory command surfaces pass.
- File/module names map to current product concepts documented in `docs/product/command-audit/`.
