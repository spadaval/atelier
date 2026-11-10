---
created_at: "2026-11-10T19:23:14.887937319+00:00"
id: "atelier-rsfm"
issue_type: "task"
labels:
- "deferred-findings"
- "redesign"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-3iyd"
  - kind: "issue"
    id: "atelier-ceko"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-xtvh"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Bound merge's CI check under the lock and fix its flaky wait test: follow-ups to atelier-xtvh"
updated_at: "2026-11-10T21:23:33.142279536+00:00"
---

## Goal

Collect the follow-ups deferred from review findings on atelier-xtvh, one per finding, filed by `atelier review defer atelier-xtvh`, and the defect found in atelier-xtvh after it merged (atelier-ceko). They are fixed together on one branch, with one review and one merge.

## Done when

- [ ] Each child's Done when lines hold.
- [ ] The Merge and forge sections of `docs/strategies/redesign-architecture.md` still match the code.
