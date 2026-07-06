---
created_at: "2026-06-29T17:46:53.131957004+00:00"
id: "atelier-qdp8"
issue_type: "epic"
labels:
- "reliability"
- "validation"
- "workflow"
review:
  kind: pull_request
  number: 30
  provider: forgejo
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-2uim"
  children:
  - kind: "issue"
    id: "atelier-dyp1"
  - kind: "issue"
    id: "atelier-i9bo"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1mga"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-30T15:05:29.974797719+00:00"
status: "done"
title: "Epic: Implement baseline and closeout reliability gates"
updated_at: "2026-06-30T15:05:29.974797719+00:00"
---

## Description

Mission workflow and closeout make repository health explicit in tool behavior, not just policy text. Work starts from a known baseline or an explicit blocker/waiver, and closeout treats failing default checks as owned facts rather than background noise.

## Outcome

Mission execution has enforced repository-health gates. Readiness and transition behavior prevent a mission from proceeding from draft/ready into real execution on an unexplained red baseline, with any waiver stored durably with failure, reason, approver, and owner. Mission closeout refuses to treat failing default checks as irrelevant without an owner, blocker, or human-approved waiver.
