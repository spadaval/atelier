---
created_at: "2026-06-24T19:26:31.661899720+00:00"
id: "atelier-8c91"
issue_type: "epic"
labels: []
review:
  kind: pull_request
  number: 29
  provider: forgejo
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-z0ll"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "review"
title: "Validate command-surface rework and update agent guidance"
updated_at: "2026-06-25T01:51:21.865683691+00:00"
---

## Description

Prove the command-surface rework is coherent for humans and agents. Update product docs, command audit, CLI help, AGENTS.md, Agent Factory guidance, and tests/transcripts so they agree on the surviving surfaces. Validate that removed commands are rejected only after replacements preserve their capabilities. Add domain/app-level validation that workflow readiness, work rows, objective scope, evidence gates, relationship semantics, history scope, status decisions, bundle semantics, issue lifecycle mutation, and review lifecycle behavior are not owned by CLI renderers.

Constraints:
- This is closeout validation and guidance alignment, not implementation of replacement command surfaces.
- Start only after the command contract, replacement pipeline, removal, consolidation, drift-check, and focused proof-gap tasks are complete.
- Do not update Agent Factory guidance to teach future commands until the executable CLI and help output agree with the new surface.
- Validation must include both positive replacement behavior and negative removed-command behavior.

## Outcome

Documentation, help output, tracker guidance, Agent Factory instructions, and focused validation all agree on the surviving command surface. Tests or recorded transcripts prove the new commands preserve the useful behavior of the removed surfaces, removed commands fail intentionally once replacements are available, and domain/app tests prove product decisions are evaluated outside the CLI crate.

## Evidence

- Root help, focused help, man guidance, AGENTS.md, Agent Factory guidance, and product docs agree on the surviving command model and no longer teach deleted surfaces.
- Focused tests or recorded transcripts cover `status`, `work`, `issue show`, `issue transition`, `evidence`, `review`, `history`, `bundle`, `man`, and `check`, plus rejected removed commands.
- `atelier check`, `cargo fmt -- --check`, `cargo nextest run`, and `git diff --check` pass or have recorded, issue-linked exceptions.
