---
created_at: "2026-06-13T02:35:52.407547330+00:00"
id: "atelier-b2vi"
issue_type: "task"
labels:
- "assignee:root"
- "docs"
- "proof"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ovs0"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Document strong proof taxonomy"
updated_at: "2026-06-13T04:11:50.100985856+00:00"
---

## Description

Create the durable vocabulary for strong proof and weak proof. The wording should be understandable to future agents and humans and should fit the existing Evidence, Workflow validator, Mission, and Issue glossary terms.

## Outcome

- CONTEXT and validation guidance define strong proof and weak proof without relying on private mission history.
- The definition names required properties: claim-specific, reproducible, attached, classified, scoped, and independent when required by risk.
- Examples distinguish broad supporting checks from claim-level proof.

## Evidence

- File-change review of documentation shows the vocabulary and examples.
- `git diff --check`, `atelier export --check`, and `atelier lint` pass.
