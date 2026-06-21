---
created_at: '2026-06-19T03:57:07.073309689+00:00'
id: atelier-zief
issue_type: mission
labels:
- mission
priority: P2
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: issue
    id: atelier-1cwz
    type: advances
  - kind: issue
    id: atelier-7v02
    type: advances
  - kind: issue
    id: atelier-kyi8
    type: advances
  - kind: issue
    id: atelier-oe7c
    type: advances
  - kind: issue
    id: atelier-qdgh
    type: advances
  - kind: issue
    id: atelier-z6yq
    type: advances
  - kind: issue
    id: atelier-zwe9
    type: advances
schema: atelier.issue
schema_version: 1
closed_at: '2026-06-19T05:15:03.850098831+00:00'
status: closed
title: Native review rooms and review commands
updated_at: '2026-06-19T05:15:03.850098831+00:00'
---

## Description

Atelier supports exactly one configured review mode per project: native review rooms stored in canonical .atelier/reviews YAML, or provider-backed pull requests. The public PR command surface is hard-renamed to review, old pull_request issue fields are migrated to the new review shape, and review merge becomes the approval enforcement boundary without driving ordinary workflow transitions.

## Outcome

### Constraints

- No compatibility alias or old-command shim for atelier pr is introduced.
- Projects configure exactly one review mode: room or provider-backed pull_request.
- Room state is canonical YAML under .atelier/reviews/<id>.yaml with current state derived from metadata plus ordered events.
- Provider behavior remains Forgejo-only in this mission and moves under atelier review commands.
- Review merge enforces review safety but does not transition Atelier issue workflow.

### Risks

- Review rooms can become a second workflow state machine unless merge authority and workflow transitions remain separate.
- Command rename and field migration can leave stale docs, validators, or help text pointing at pr or pull_request.
- Room timelines can drift if projections store duplicate mutable snapshots instead of deriving state from events.

## Evidence

- Manual check: CONTEXT.md, product docs, CLI surface docs, workflow docs, and a new ADR define review modes, room YAML, command rename, issue field migration, and room merge authority.
- Manual check: Canonical issue records use review.kind room or pull_request and reject legacy pull_request fields after migration.
- Manual check: Room mode proves review open/status/show/comments/comment/approve/request-changes/resolve/merge behavior, stale approval invalidation, blocking finding enforcement, and merge without workflow transition.
- Manual check: Provider mode proves Forgejo open/link/status/show/comment/approve/request-changes/merge parity through atelier review commands.
- Manual check: Negative validation proves atelier pr is rejected and room-only/provider-only commands fail with direct guidance.
- Manual check: Final checks include cargo fmt -- --check, focused tests, cargo nextest run, git diff --check, atelier lint, and atelier doctor.

## Notes

### Terminal Notes

- Close reason: Native review rooms and review command migration completed and validated.

Migrated from `.atelier/missions/atelier-zief.md` as a declared mission objective issue.
