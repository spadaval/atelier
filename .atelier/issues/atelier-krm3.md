---
created_at: "2026-06-15T06:20:46.971423020+00:00"
id: "atelier-krm3"
issue_type: "bug"
labels:
- "validation"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-t35w"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Close workflow must allow freshly recorded evidence"
updated_at: "2026-06-15T06:20:46.971423020+00:00"
---

## Description

Validation of atelier-t35w exercised the documented worker flow: create/start, request review, request validation, record passing evidence, then close. `issue close` remained blocked because `closeout_clean` treated the newly created `.atelier/evidence/<id>.md` file as a dirty worktree entry, even though tracker-generated issue/activity entries were ignored. The normal flow should support recording evidence immediately before close without requiring an extra commit between the two commands, or the operator guidance and validator must clearly require that checkpoint.

## Outcome

- After `atelier evidence record --target issue/<id> --kind validation --result pass ...`, `atelier issue close <id> --reason ...` can close the issue when all other validators pass.
- `closeout_clean` handling is consistent for tracker-generated issue, activity, link, and evidence records.
- If an explicit pre-close commit is required instead, `atelier issue transition <id> --options`, `atelier man worker`, and tracker workflow docs say so before the operator records evidence.

## Evidence

- Disposable-repo transcript shows `issue create`, `start`, `request_review`, `request_validation`, `evidence record`, and `issue close` succeeding in sequence, or updated guidance plus tests prove the required checkpoint behavior.
- Add or update a focused integration test for evidence-record-then-close.
- `atelier lint`, `atelier export --check`, and `git diff --check` pass.
