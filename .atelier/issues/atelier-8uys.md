---
created_at: "2026-06-19T03:58:44.193415172+00:00"
id: "atelier-8uys"
issue_type: "feature"
labels:
- "review"
- "room"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Implement room findings decisions and stale approval handling"
updated_at: "2026-06-19T03:58:44.193415172+00:00"
---

## Description

Implement room findings and review decisions. This issue owns approval,
changes-requested, blocking/non-blocking findings, finding IDs, resolution, and
stale approval behavior.

## Outcome

- `atelier review comment --blocking` and `--non-blocking` create findings with
  room-local IDs such as `F1` and `F2`.
- `atelier review approve` and `atelier review request-changes` record decisions
  tied to the current source-branch head commit.
- Any new source-branch commit after approval makes that approval non-current.
- `atelier review resolve <finding-id>` records a finding resolution event and
  rejects unknown or already-resolved findings with direct guidance.

## Evidence

- CLI tests cover blocking and non-blocking finding creation, deterministic
  finding IDs, approval, changes requested, stale approval, and resolve paths.
- Command output fixtures for `atelier review status` and
  `atelier review show` demonstrate current approval and open finding
  derivation from events.
- `atelier lint atelier-8uys` and focused decision/finding tests pass.
