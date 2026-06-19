---
created_at: "2026-06-19T03:57:12.353691710+00:00"
id: "atelier-1cwz"
issue_type: "epic"
labels:
- "review"
- "schema"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0jsk"
  - kind: "issue"
    id: "atelier-69g3"
  - kind: "issue"
    id: "atelier-8uys"
  - kind: "issue"
    id: "atelier-at7i"
  - kind: "issue"
    id: "atelier-kyi8"
  - kind: "issue"
    id: "atelier-oe7c"
  - kind: "issue"
    id: "atelier-onkp"
  - kind: "issue"
    id: "atelier-q199"
  - kind: "issue"
    id: "atelier-qdgh"
  - kind: "issue"
    id: "atelier-rb5b"
  - kind: "issue"
    id: "atelier-swxv"
  - kind: "issue"
    id: "atelier-unwz"
  children:
  - kind: "issue"
    id: "atelier-5d7i"
  - kind: "issue"
    id: "atelier-j1i1"
  - kind: "issue"
    id: "atelier-xuxl"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Add review field and room record schema"
updated_at: "2026-06-19T03:58:30.974421169+00:00"
---

## Description

Add durable schema support for structured review links and canonical review room
records. This epic owns parsing, validation, projection, lint, doctor, and
migration behavior for review state, not command workflows.

## Outcome

- Issue records support a structured `review` field for room and provider modes
  and reject child-local review fields when the nearest parent epic owns the
  review boundary.
- `.atelier/reviews/*.yaml` records parse and render deterministically using
  schema `atelier.review`, `schema_version: 1`, top-level metadata, and an
  ordered `events:` timeline.
- Legacy `pull_request` issue fields migrate to `review.kind: pull_request` and
  are rejected after migration.
- Rebuild, projection, lint, and doctor paths validate review links and review
  room records without storing duplicate mutable room snapshots.

## Evidence

- Unit and fixture tests cover review field parsing, inheritance, invalid child
  fields, old `pull_request` rejection, and deterministic room YAML rendering.
- Rebuild/lint tests cover valid and invalid `.atelier/reviews/*.yaml` records.
- `atelier lint atelier-1cwz`, focused tests, and `git diff --check` pass.
