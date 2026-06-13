---
created_at: "2026-06-13T22:59:16.510809734+00:00"
id: "atelier-pa33"
issue_type: "task"
labels:
- "dependencies"
- "quality"
- "readiness"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Decide a repo-supported unused dependency scan"
updated_at: "2026-06-13T22:59:16.510809734+00:00"
---

## Description

`atelier-e723` classified unused-dependency scans as advisory because this
checkout does not have `cargo machete` or `cargo udeps` installed. Decide
whether Atelier should standardize one supported command, document the install
path, or explicitly defer unused-dependency scanning as a repo policy.

## Outcome

- The repo has a named unused-dependency scan path with its prerequisites and
  owner clearly documented, or an explicit durable defer/not-applicable
  decision.
- Workers no longer have to guess whether `cargo machete`, `cargo udeps`, or
  neither is the expected surface.
- Readiness docs and proof routing agree on how unavailable-tool results should
  be recorded.

## Evidence

- Documentation diff, config change, or tracker policy artifact names the
  selected unused-dependency scan path.
- Command transcripts show the supported command running, or evidence records
  show the explicit defer/unavailable classification and owner.
- `target/debug/atelier lint` and `target/debug/atelier export --check` pass
  after the policy update.
