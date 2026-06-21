---
created_at: "2026-06-21T16:37:30.760209666+00:00"
id: "atelier-7jma"
issue_type: "epic"
labels:
- "artifact-update"
- "mission-rework"
review:
  kind: pull_request
  number: 17
  provider: forgejo
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-7nrk"
  - kind: "issue"
    id: "atelier-zny7"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Epic: Define minimal mission contract"
updated_at: "2026-06-21T18:35:02.238827820+00:00"
---

## Description

Settle the target contract for missions before implementation. The contract must define which mission behavior is domain-specific, which behavior is workflow policy, and which command surfaces remain.

## Outcome

- Product and architecture docs name the smallest mission-specific constraints Atelier keeps.
- Docs state whether missions remain a first-class record kind or migrate to typed objective issues, and why.
- The target command map removes mission-only commands unless a distinct mission-only job is justified.
- Dependent implementation issues can cite this contract instead of private chat context.

## Evidence

- Diff shows updates to docs/product/zen.md-adjacent product docs, CONTEXT.md, command audit docs, or ADRs as needed.
- Validation transcript shows `atelier lint`, docs/help drift checks through lint, and targeted content searches for stale mission command guidance.
