---
created_at: "2026-06-19T03:57:17.837729185+00:00"
id: "atelier-kyi8"
issue_type: "epic"
labels:
- "review"
- "room"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0jsk"
  - kind: "issue"
    id: "atelier-oe7c"
  - kind: "issue"
    id: "atelier-q199"
  - kind: "issue"
    id: "atelier-swxv"
  children:
  - kind: "issue"
    id: "atelier-8uys"
  - kind: "issue"
    id: "atelier-at7i"
  - kind: "issue"
    id: "atelier-onkp"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Implement native review room backend"
updated_at: "2026-06-19T03:58:46.845763074+00:00"
---

## Description

Implement native review room behavior on top of the review schema and config
contract. This epic owns room commands and local merge authority, not provider
mode behavior.

## Outcome

- `atelier review open/status/show/comments/comment` operate on room-mode
  projects and write deterministic room YAML events.
- `atelier review approve`, `request-changes`, and `resolve <finding-id>`
  record room decisions and finding resolution tied to the source branch head.
- New commits after approval invalidate that approval.
- `atelier review merge` enforces current approval, no open blocking findings,
  branch policy, and clean local integration preconditions, then performs the
  configured local merge/squash behavior without transitioning issue workflow.

## Evidence

- CLI tests cover the room command surface and deterministic YAML output.
- Tests prove stale approval invalidation, blocking finding merge prevention,
  successful local merge, and no workflow transition side effect.
- `atelier lint atelier-kyi8`, focused room command tests, and `git diff --check`
  pass.
