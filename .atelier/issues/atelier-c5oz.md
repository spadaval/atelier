---
created_at: "2026-06-17T17:59:03.456277421+00:00"
id: "atelier-c5oz"
issue_type: "epic"
labels:
- "forgejo"
- "implementation"
- "pr"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-495r"
  - kind: "issue"
    id: "atelier-4clo"
  - kind: "issue"
    id: "atelier-98mo"
  - kind: "issue"
    id: "atelier-cglp"
  - kind: "issue"
    id: "atelier-hw9t"
  - kind: "issue"
    id: "atelier-jhzk"
  - kind: "issue"
    id: "atelier-onie"
  - kind: "issue"
    id: "atelier-p7oa"
  children:
  - kind: "issue"
    id: "atelier-e7oj"
  - kind: "issue"
    id: "atelier-mpah"
  - kind: "issue"
    id: "atelier-udny"
  - kind: "issue"
    id: "atelier-vg25"
  - kind: "issue"
    id: "atelier-yrwm"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Epic: Add Forgejo PR integration"
updated_at: "2026-06-18T00:47:32.076010820+00:00"
---

## Description

Add Forgejo pull request commands and integration around Atelier's epic or
standalone issue review boundary.

## Outcome

- Forgejo connection and role sudo-user mapping are read from tracked project
  config.
- `atelier pr` commands infer the active PR from session target, issue target,
  or owner branch.
- The active PR is persisted in the owning issue's `forge_pr` typed field.
- Inline comments are queried live from Forgejo rather than mirrored into
  canonical records.

## Evidence

- Mocked Forgejo CLI tests cover PR open/status/show/comments/comment/review,
  sudo authorship, inference, and PR field persistence.
- Command transcript shows targeted Forgejo PR tests pass.
