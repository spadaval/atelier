---
created_at: "2026-06-23T20:17:40.258977606+00:00"
id: "atelier-ih42"
issue_type: "validation"
labels:
- "cli"
- "validation"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "review"
title: "Validate explicit mission/epic domain and branch workflow"
updated_at: "2026-06-23T22:29:22.113434188+00:00"
---

## Description

Validate the fixed mission/epic domain model and explicit branch workflow end to end. Cover hierarchy rejection, mission advances links, standalone issue branches, child issue epic branches, explicit branch.prepare planning/execution, custom context-only links, bundle preview/apply behavior, and cleaned human output around branch state.

## Outcome

Focused tests and command transcripts prove invalid hierarchy shapes are rejected across issue commands, canonical lint/rebuild paths, and bundle preview/apply; valid standalone and epic-child work remains usable; bundle mission scope uses `advances` rather than parent hierarchy; branch.prepare is explicit and not injected; and normal status/issue output no longer exposes branch owner internals as routine workflow guidance.

## Evidence

- First-class evidence attaches the domain/workflow validation matrix to `atelier-c0qc` and references the child evidence for `atelier-59vp`, `atelier-ht4k`, `atelier-krt8`, and `atelier-pguu`.
- Transcript evidence covers invalid hierarchy rejection, mission `advances` scope, valid standalone branches, valid epic-child branches, explicit `branch.prepare` execution, custom context-only links, bundle preview/apply behavior, and cleaned branch-state output.
- `cargo fmt -- --check`, `git diff --check`, `target/debug/atelier lint`, and focused CLI tests pass.
