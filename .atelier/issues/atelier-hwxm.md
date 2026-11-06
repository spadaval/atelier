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
    id: "atelier-efnf"
    type: "related"
  - kind: "issue"
    id: "atelier-ma4a"
    type: "related"
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "A review finding carries its disposition in one record"
updated_at: "2026-11-06T22:08:52.398852821+00:00"
---

## Goal

A finding and what was done about it are one record: the disposition (fix_now with resolution commit, defer with follow-up, no_action with reason) becomes fields on the finding, with who set it and at which commit, instead of a separate immutable event. Existing repositories migrate with one atelier migrate step.

## Done when

Provisional: review files per finding drop to 1 in the lifecycle scenario; merge and land gates read dispositions from findings; the non-author rules for defer and no_action still hold, with who set the disposition traced per field rather than by the commit that added a separate file (today original_independent in crates/atelier-app/src/review_gate.rs); recording a disposition does not make an approval stale (today editing existing review records does, docs/product/reviews-and-merge.md); an atelier migrate step folds existing disposition events into their findings and the fixture repositories pass atelier doctor after it; an ADR records the change and the part of ADR 0019's append-only events it supersedes, and PRODUCT_INTENT.md, CONTEXT.md and the Agent Factory text describing two events (references/finding-disposition.md, references/atelier.md in ~/projects/agents-skills) are updated.

## Unknowns

How a disposition changed later (fix_now then defer) stays auditable without a second record. How atelier-efnf's readiness view and atelier-ma4a's review history read the new shape, if they land first.
