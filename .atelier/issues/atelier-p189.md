---
created_at: "2026-10-17T19:00:36.350689381+00:00"
id: "atelier-p189"
issue_type: "feature"
labels:
- "merge"
- "retro-2026-10-17"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Retry merge when the base moved only by records-only commits"
updated_at: "2026-10-22T20:32:25.084090477+00:00"
---

## Goal

ydv0 chased a moving master three times in four minutes (07:13-07:17Z, 10-17) while o3lm landed records commits. When the only new base commits since the checked state touch `.atelier/` records that don't affect the item, `atelier merge` should re-check and proceed instead of refusing with `merge_diverged`.

## Done when

`atelier merge` given a base that moved only by records-only commits outside the item's own records re-runs its gates against the new base and completes without a manual catch-up, or names the commit that prevents it. A test covers the records-only case and a code-commit case that still refuses.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).
