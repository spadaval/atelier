---
created_at: "2026-06-13T02:35:55.611441511+00:00"
id: "atelier-s8z0"
issue_type: "task"
labels:
- "docs"
- "evidence"
- "schema"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-dv3d"
  - kind: "issue"
    id: "atelier-h2tq"
  - kind: "issue"
    id: "atelier-xmss"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Specify structured evidence schema and claim anchors"
updated_at: "2026-06-13T02:35:55.611441511+00:00"
---

## Description

Define the target structure for evidence records and how evidence maps to issue, epic, and mission claims. This is the contract item that implementation depends on.

## Outcome

- Evidence records have documented fields for target IDs, covered claims, kind, result, commands or artifacts, agent identity, independence level, residual risks, and follow-up IDs.
- Claim anchors or equivalent stable claim references are defined for Outcome and Validation bullets.
- Migration expectations for existing prose evidence are explicit.

## Evidence

- File-change review of schema or architecture documentation captures the evidence contract.
- Review artifact includes examples for one command transcript, one audit table, one failed validation, and one deferred result.
- `atelier lint`, `atelier export --check`, and relevant docs check commands pass.
