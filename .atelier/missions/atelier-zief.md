---
created_at: "2026-06-19T03:57:07.073309689+00:00"
id: "atelier-zief"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1cwz"
    type: "advances"
  - kind: "issue"
    id: "atelier-7v02"
    type: "advances"
  - kind: "issue"
    id: "atelier-kyi8"
    type: "advances"
  - kind: "issue"
    id: "atelier-oe7c"
    type: "advances"
  - kind: "issue"
    id: "atelier-qdgh"
    type: "advances"
  - kind: "issue"
    id: "atelier-z6yq"
    type: "advances"
  - kind: "issue"
    id: "atelier-zwe9"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "ready"
title: "Native review rooms and review commands"
updated_at: "2026-06-19T04:11:54.209873463+00:00"
---

## Intent

Atelier supports exactly one configured review mode per project: native review rooms stored in canonical .atelier/reviews YAML, or provider-backed pull requests. The public PR command surface is hard-renamed to review, old pull_request issue fields are migrated to the new review shape, and review merge becomes the approval enforcement boundary without driving ordinary workflow transitions.

## Constraints

- No compatibility alias or old-command shim for atelier pr is introduced.
- Projects configure exactly one review mode: room or provider-backed pull_request.
- Room state is canonical YAML under .atelier/reviews/<id>.yaml with current state derived from metadata plus ordered events.
- Provider behavior remains Forgejo-only in this mission and moves under atelier review commands.
- Review merge enforces review safety but does not transition Atelier issue workflow.

## Risks

- Review rooms can become a second workflow state machine unless merge authority and workflow transitions remain separate.
- Command rename and field migration can leave stale docs, validators, or help text pointing at pr or pull_request.
- Room timelines can drift if projections store duplicate mutable snapshots instead of deriving state from events.

## Validation

- CONTEXT.md, product docs, CLI surface docs, workflow docs, and a new ADR define review modes, room YAML, command rename, issue field migration, and room merge authority.
- Canonical issue records use review.kind room or pull_request and reject legacy pull_request fields after migration.
- Room mode proves review open/status/show/comments/comment/approve/request-changes/resolve/merge behavior, stale approval invalidation, blocking finding enforcement, and merge without workflow transition.
- Provider mode proves Forgejo open/link/status/show/comment/approve/request-changes/merge parity through atelier review commands.
- Negative validation proves atelier pr is rejected and room-only/provider-only commands fail with direct guidance.
- Final checks include cargo fmt -- --check, focused tests, cargo nextest run, git diff --check, atelier lint, and atelier doctor.
