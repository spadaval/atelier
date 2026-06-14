---
created_at: "2026-06-13T20:37:14.944847400+00:00"
id: "atelier-d7lw"
issue_type: "task"
labels:
- "architecture"
- "assignee:root"
- "refactor"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Thin main command routing and oversized command handlers"
updated_at: "2026-06-14T00:14:02.492349849+00:00"
---

## Description

main.rs and large command modules contain command enums, help taxonomy, compatibility dispatch, storage access modes, evidence target parsing, issue lifecycle routing, and telemetry identity in one place. Simplify routing so public command contracts and domain handlers are easier to audit.

## Outcome

- CLI parsing remains declarative and thin, with group-specific dispatch moved to cohesive command modules.
- Storage access and projection freshness policy is explicit but not tangled with every command branch.
- Large command handlers such as agent_factory and mission are split only where it reduces real coupling or change amplification.

## Evidence

- Module-size and dependency review before/after identifies moved responsibilities.
- Focused command tests prove behavior did not drift during extraction.
- `cargo fmt -- --check` and relevant cargo nextest slices pass.
