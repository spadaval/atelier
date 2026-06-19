---
created_at: "2026-06-19T22:42:56.451148542+00:00"
id: "atelier-vzrj"
issue_type: "task"
labels:
- "adr"
- "workflow-policy"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-h7n4"
  - kind: "issue"
    id: "atelier-jwvd"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-19T23:34:28.492167958+00:00"
status: "done"
title: "Document workflow actions and branching decision"
updated_at: "2026-06-19T23:34:28.492167958+00:00"
---

## Description

Write the architectural decision for workflow actions and branching. The ADR should explain why `actions` replaces `effects`, why `hooks` is rejected, why branch operations are transition actions, and what minimal workflow-engine behavior remains intrinsic.

## Outcome

- The branching model is understandable without `epic.branch.owner: self` style policy.
- The ADR names the exact built-in action families expected in v1.

## Evidence

- ADR accepted and linked from docs/architecture/index.md.
- Manual check of the docs/adr/ file change confirms rejected alternatives for generic capabilities, separate branch policy, mission-only branching, and arbitrary hooks.
