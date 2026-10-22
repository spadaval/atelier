---
created_at: "2026-10-17T19:00:36.347834302+00:00"
id: "atelier-iaas"
issue_type: "bug"
labels:
- "merge"
- "retro-2026-10-17"
- "review"
priority: "P1"
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
status: "backlog"
title: "Let a follow-up be linked to its mission without staling the branch approval"
updated_at: "2026-10-17T19:00:36.347834302+00:00"
---

## Goal

Linking a follow-up filed on an approved branch to its mission stales the approval (5rs0, o3lm 06:31Z). Linking it from the base fails with 'Issue atelier-jx7g was not found' because the issue exists only on the branch (07:18Z). There is no clean way to file a follow-up during review.

## Done when

Either creating a new issue record and its links on an approved branch leaves the approval current (the review gate ignores records that are new and not the item's own), or a base-side link may name an issue on an unmerged branch. `docs/product` describes the rule; tests cover filing a follow-up on an approved branch and merging it.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). The memory rule 'link follow-ups on base' records the workaround.
