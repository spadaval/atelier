---
created_at: "2026-06-23T20:16:14.569627762+00:00"
id: "atelier-vhxi"
issue_type: "epic"
labels:
- "cli"
- "domain-model"
- "workflow"
review:
  kind: pull_request
  number: 23
  provider: forgejo
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-59vp"
  - kind: "issue"
    id: "atelier-e96p"
  - kind: "issue"
    id: "atelier-ht4k"
  - kind: "issue"
    id: "atelier-ih42"
  - kind: "issue"
    id: "atelier-krt8"
  - kind: "issue"
    id: "atelier-pguu"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-c0qc"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-23T22:33:25.009678451+00:00"
status: "done"
title: "Epic: Make mission and epic explicit domain types"
updated_at: "2026-06-23T22:33:25.009678451+00:00"
---

## Description

Track the branch, hierarchy, and relationship cleanup for the CLI surface refresh. Missions and epics become explicit product domain types; branch setup becomes an explicit workflow action; custom issue links remain context-only unless a built-in semantic gives them workflow meaning.

## Outcome

Atelier has a fixed coordination model with configurable delivery workflows. Mission and epic rules are documented and enforced, branch actions are visible in workflow YAML, and normal status surfaces no longer expose ambient expected-branch policy as lifecycle state.
