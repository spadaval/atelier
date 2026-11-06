---
created_at: "2026-11-06T21:26:18.397443944+00:00"
id: "atelier-hwxm"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-5fus"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "A review finding carries its disposition in one record"
updated_at: "2026-11-06T21:26:18.397443944+00:00"
---

## Goal

A finding and what was done about it are one record: the disposition (fix_now with resolution commit, defer with follow-up, no_action with reason) becomes fields on the finding, with who set it and at which commit, instead of a separate immutable event. Existing repositories migrate with one atelier migrate step.

## Done when

Provisional: review files per finding drop to 1 in the lifecycle scenario; merge and land gates read dispositions from findings; the non-author rules for defer and no_action still hold; an atelier migrate step folds existing disposition events into their findings and the fixture repositories pass atelier doctor after it; an ADR records the change and what it supersedes.

## Unknowns

How a disposition changed later (fix_now then defer) stays auditable without a second record.
