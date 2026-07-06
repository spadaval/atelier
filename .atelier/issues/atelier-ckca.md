---
created_at: "2026-06-23T16:21:20.079723985+00:00"
id: "atelier-ckca"
issue_type: "epic"
labels: []
review:
  kind: pull_request
  number: 52
  provider: forgejo
fields:
  workflow_branch:
    branch_base: mission/atelier-mska
    integration_target: mission/atelier-mska
    merge_strategy: squash
    owner_issue_id: atelier-ckca
    owner_kind: epic
    review_target: mission/atelier-mska
    work_branch: epic/atelier-ckca
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-0p7e"
  - kind: "issue"
    id: "atelier-idwz"
  - kind: "issue"
    id: "atelier-nxq9"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mska"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "review"
title: "Epic: Rewrite domain-shaped cache schema"
updated_at: "2026-07-06T20:10:49.903381056+00:00"
---

## Description

Rewrite the SQLite cache schema and rebuild code to use domain-shaped tables instead of the current hybrid issue tables plus generic non-issue record tables.

## Outcome

- SQLite is a domain-shaped cache, not a mirror of the record-file storage abstraction.

## Evidence

- Evidence `atelier-eoec`, `atelier-nixw`, and `atelier-4uqh` prove the domain
  tables/source metadata, shared rebuild and incremental indexers, and command
  query cutover through focused SQLite, app, and CLI tests.
- Evidence `atelier-8szj` and `atelier-evxl` prove the completed hard cutover
  with 90/90 app tests and 546/546 full SQLite/app/CLI tests; independent review
  evidence `atelier-9isp` confirms generic tables, reverse export, and the
  standalone `ProjectionIndex` are removed.
