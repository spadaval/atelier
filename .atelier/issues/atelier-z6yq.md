---
created_at: "2026-06-19T03:57:09.694442680+00:00"
id: "atelier-z6yq"
issue_type: "epic"
labels:
- "docs"
- "review"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0jsk"
  - kind: "issue"
    id: "atelier-13yy"
  - kind: "issue"
    id: "atelier-1cwz"
  - kind: "issue"
    id: "atelier-5d7i"
  - kind: "issue"
    id: "atelier-69g3"
  - kind: "issue"
    id: "atelier-7v02"
  - kind: "issue"
    id: "atelier-8uys"
  - kind: "issue"
    id: "atelier-9h5w"
  - kind: "issue"
    id: "atelier-at7i"
  - kind: "issue"
    id: "atelier-j1i1"
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
    id: "atelier-tv53"
  - kind: "issue"
    id: "atelier-unwz"
  - kind: "issue"
    id: "atelier-xuxl"
  - kind: "issue"
    id: "atelier-zwe9"
  children:
  - kind: "issue"
    id: "atelier-cix4"
  - kind: "issue"
    id: "atelier-kuuw"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Epic: Define native review modes and public contract"
updated_at: "2026-06-19T04:20:26.836826827+00:00"
---

## Description

Define the product and architecture contract for native review rooms before
schema, config, command, or backend implementation starts. This epic owns the
durable decisions and user-facing language for mutually exclusive review modes,
canonical room timelines, the `pr` to `review` hard rename, review field
migration, and room merge authority.

## Outcome

- A new ADR records the shift from provider-first PR artifacts to mutually
  exclusive `room` and provider-backed `pull_request` review modes.
- `CONTEXT.md`, product docs, workflow docs, and CLI surface docs use `review`
  terminology and describe native room behavior, provider behavior, and the
  workflow boundary consistently.
- Documentation states that `atelier pr ...` is removed without aliases and
  that review safety is enforced by `atelier review merge`, not ordinary issue
  workflow transitions.

## Evidence

- Documentation diff shows the ADR and updated domain/product/workflow/CLI
  language.
- Search output proves live guidance no longer directs users to `atelier pr`
  or legacy `pull_request` fields except where explicitly historical.
- `atelier lint atelier-z6yq`, `atelier lint`, and `git diff --check` pass for
  the tracker and documentation changes.
