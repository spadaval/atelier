---
created_at: "2026-06-15T21:31:35.894790909+00:00"
id: "atelier-tpuc"
issue_type: "task"
labels:
- "docs"
- "tests"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-9p3t"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Update tests and docs for closeout removal"
updated_at: "2026-06-16T15:31:07.677541700+00:00"
---

## Description

Update tests, fixtures, command audits, target-state documentation, and Agent Factory skill guidance after closeout is removed from the live product model and evidence semantics are simplified.

## Outcome

- CLI integration tests no longer assert closeout issue type behavior, `--closeout` help, `Closeout` headings, or closeout-specific next commands.
- Product docs, specs, ADRs, command-audit docs, `AGENTFACTORY.md`, and `/root/.agents/skills/agent-factory` use the new terminal-check terminology consistently.
- Agent Factory validation guidance no longer tells validators to create closeout issues or treat evidence `pass/fail` result fields as proof semantics; it tells them to classify claims in the validation artifact and attach evidence that validates the accountable issue.
- Historical quality inventories remain clearly historical if they keep old wording.
- The new docs preserve the `docs/product/zen.md` principle that proof supports excellent working features rather than ceremony.

## Evidence

- `rg` over docs, tests, `AGENTFACTORY.md`, and `/root/.agents/skills/agent-factory` shows no live closeout guidance or obsolete evidence-result examples except intentionally historical files.
- `cargo fmt -- --check` and focused `cargo test` or `cargo nextest` runs covering changed tests pass.
- `atelier lint` and `git diff --check` pass.
