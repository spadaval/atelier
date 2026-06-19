---
created_at: "2026-06-18T22:42:47.662168928+00:00"
id: "atelier-0wyy"
issue_type: "task"
labels:
- "architecture"
- "pr"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T23:42:58.167412451+00:00"
status: "done"
title: "Move PR artifact orchestration behind app use-case boundary"
updated_at: "2026-06-18T23:42:58.167412451+00:00"
---

## Description

The PR command module currently owns target inference, Forgejo calls, workflow
branch policy checks, canonical `RecordStore` mutation, projection refresh,
activity attribution, and terminal rendering. That makes PR workflow changes
harder to test without CLI command fixtures and keeps use-case behavior in the
CLI crate despite the layered architecture target.

## Outcome

- PR artifact operations have an `atelier-app` use-case boundary with request
  and outcome types independent of Clap and terminal rendering.
- CLI PR command handlers become thin argument parsing and rendering wrappers.
- Existing PR behavior, attribution, canonical writes, and projection refresh
  are preserved.

## Evidence

- Source review shows PR orchestration moved out of `crates/atelier-cli` into
  app-owned use-case code without adding compatibility aliases.
- Focused unit tests cover app-level PR open/link/comment/review/merge outcomes
  without parsing terminal output.
- Existing PR CLI tests continue to pass for rendered command behavior.
