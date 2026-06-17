---
created_at: "2026-06-17T20:03:33.991961265+00:00"
id: "atelier-7kh1"
issue_type: "task"
labels:
- "artifact-update"
- "bundle"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-jmmn"
  - kind: "issue"
    id: "atelier-mrj5"
  - kind: "issue"
    id: "atelier-tkiw"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Document declarative bundle apply contract and semantics"
updated_at: "2026-06-17T21:28:55.166077888+00:00"
---

## Description

Write the declarative bundle apply contract before implementation changes. The
contract should settle the questions raised by comparing this feature to
CloudFormation/Terraform: file input, schema, resource set, identity resolution,
preview, apply behavior, conflict handling, workflow defaults, rollback/atomicity,
and unsupported operations.

Decision: use `bundle`, not `bulk` or `change`. `bulk` is too mechanical and
volume-based; `change` is too broad and implies this is the general mutation
path. A bundle is a temporary multi-record graph package that is applied into
canonical Markdown and then stops being relevant as tracker state.

## Outcome

- Docs specify that input is a real file path, not stdin.
- Docs specify `atelier bundle preview <file>` and
  `atelier bundle apply <file> --yes`.
- Docs specify v1 is create-only unless a different operation is explicitly
  accepted in the schema.
- Docs specify `atelier.bundle` schema identity/version, client ref rules,
  existing-record refs,
  relationship semantics, workflow status defaults, dry-run/validate-only output,
  idempotency expectations, and failure behavior.
- Docs specify that first-class plans and milestones are not legal v1 resources.
- Docs specify that bundle files are one-shot graph deltas, not durable source
  of truth; optional consume/archive behavior may run only after successful
  apply.

## Evidence

- Review or evidence record maps every open semantics question to an explicit
  contract line.
- `atelier lint` and `git diff --check` pass for the contract update.
