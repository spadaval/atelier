---
created_at: "2026-06-13T20:44:49.590075966+00:00"
id: "atelier-1vdl"
issue_type: "task"
labels:
- "readiness"
- "setup"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Document reproducible dev environment and env template"
updated_at: "2026-06-13T20:44:49.590075966+00:00"
---

## Description

Make local setup reproducible for fresh agents and humans. Audit whether this Rust CLI needs a .devcontainer, a setup script, an env template, or simply a documented no-env-required contract.

## Outcome

- Fresh setup docs identify required tools, Rust toolchain, cargo-nextest, optional integrations, and whether any environment variables are required.
- A .env.example exists if environment variables are required; otherwise docs explicitly state that no local secrets/env file is needed for normal development.
- A devcontainer or equivalent reproducible setup is added or explicitly deferred with rationale.

## Evidence

- File change adds .devcontainer/.env.example or updates setup documentation with no-env/devcontainer rationale.
- Fresh setup command transcript or documented `cargo`/`atelier doctor` transcript proves the path.
- `git diff --check`, `atelier lint`, and `atelier export --check` pass.
