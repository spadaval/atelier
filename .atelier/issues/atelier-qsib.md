---
created_at: "2026-06-15T15:16:25.104361056+00:00"
id: "atelier-qsib"
issue_type: "task"
labels:
- "guardrail"
- "rewrite"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Add crate migration guardrails and warning-free closeout gate"
updated_at: "2026-06-15T16:40:22.829227589+00:00"
---

## Description

Add durable source checks or tests that fail if the root package, root lib/bin targets, old root module paths, or warning-producing workspace code return after the migration.

## Outcome

- Source checks or tests fail if the repository root becomes a package again, a root lib/bin target returns, or deleted root module paths are reintroduced.
- The closeout gate includes warning-free workspace compilation under `RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets`.
- Guardrails are documented in the relevant architecture or quality guidance so future migration work can run the same proof.

## Evidence

- Test transcript or review artifact shows the guard detects a representative root package/module regression.
- Passing transcript includes the guard check and `RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets`.
- Documentation or quality diff names the guard command and when it belongs in crate-migration closeout.
