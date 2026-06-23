---
created_at: "2026-06-23T16:21:20.070365285+00:00"
id: "atelier-v9sy"
issue_type: "epic"
labels: []
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ckca"
  - kind: "issue"
    id: "atelier-hl1n"
  - kind: "issue"
    id: "atelier-x7lq"
  children:
  - kind: "issue"
    id: "atelier-95rr"
  - kind: "issue"
    id: "atelier-pxxj"
  - kind: "issue"
    id: "atelier-u327"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mska"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Persistence architecture contract"
updated_at: "2026-06-23T16:21:20.086417637+00:00"
---

## Description

Define the target persistence model before implementation: unified record-file storage, concrete domain types, domain-specific SQLite cache read models, lazy cache access, incremental changed-record reindexing, and cache terminology.

## Outcome

- The repository has a durable architecture contract for record files, domain models, the lazy SQLite cache, and changed-record incremental reindexing.
- The contract records the decision to keep SQLite as disposable cache for this mission rather than switching to a custom binary cache, Tantivy, graph database, or external search service.
- The target schema and migration stance are explicit enough for implementation issues to proceed without private chat context.

## Evidence

- File changes in architecture docs name the storage/domain/cache boundaries and explain which facts live in each layer.
- File changes in ADR or architecture docs record the SQLite cache decision and rejected alternatives.
- `atelier lint atelier-v9sy` passes after the architecture contract updates.
