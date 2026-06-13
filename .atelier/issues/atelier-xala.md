---
created_at: "2026-06-13T01:27:15.818383607+00:00"
id: "atelier-xala"
issue_type: "task"
labels:
- "evidence"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-v9id"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Block validation closeout on incomplete proof"
updated_at: "2026-06-13T01:37:29.195569742+00:00"
---

## Description

Independent validation found that validation item `atelier-v9id` can become close-ready when only broad passing evidence is attached. That is too weak for validation, epic-closeout, and mission-closeout items whose Evidence sections require line-by-line classifications or contract-audit proof. Repair the issue closeout and workflow evidence validator so high-risk work cannot close merely because any passing evidence is linked.

## Outcome

- Validation, closeout, epic, and mission-scoped issues require attached passing evidence whose summary or captured transcript demonstrates line-by-line or contract-audit proof before the evidence gate passes.
- Ordinary executable issues can still close with focused passing evidence as before.
- `atelier issue transition <id> --options`, `atelier workflow validate issue <id> --validator evidence_attached`, and `atelier issue close <id>` all report the high-risk proof gap consistently.
- A regression test reproduces the `atelier-v9id` class of failure with broad evidence and proves the gate passes only after line-by-line or contract-audit evidence is attached.

## Evidence

- Run the new regression test covering high-risk validation item evidence gates.
- Run existing issue closeout proof-gate tests for ordinary evidence behavior.
- Run `cargo nextest run --status-level fail --final-status-level fail --failure-output final --max-fail 1:immediate --cargo-quiet`.
- Run `cargo fmt -- --check`, `atelier lint <issue-id>`, `atelier export --check`, and `git diff --check`.
