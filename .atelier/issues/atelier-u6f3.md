---
created_at: "2026-10-17T19:00:36.347242257+00:00"
id: "atelier-u6f3"
issue_type: "task"
labels:
- "retro-2026-10-17"
- "review"
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
title: "Batch low-priority review follow-ups into one branch and one review"
updated_at: "2026-10-22T23:21:07.658155216+00:00"
---

## Goal

o3lm pulled 10 deferred review follow-ups back into the mission and ran each through the full lifecycle: about 370-640 record lines each for 30-180 source lines (2hh8, jx7g, ytxg, ...). That doubled the mission's scope for low-priority polish.

## Done when

The Agent Factory orchestrate guidance says to batch low-priority deferred follow-ups into one owner branch (a parent with children) reviewed once, and atelier supports closing several children on one branch with one approval. Shown by the skill text and a test of one approval covering the children.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).

## Notes

- 2026-10-22 [decision] No Atelier change: children already start onto their parent's branch and one approval on the parent covers every child's code and done claim through land, merge and land on the base; this item adds the orchestrate guidance and a regression test of that flow
  Reason: a_task_parent_owns_its_children_through_start_land_and_merge and the scope rule in review_gate (owner and descendants) already support it; task/atelier-fngz runs four follow-ups this way. The new test adds what the existing one did not show: several children each with its own code commit and close, and the base land check after merge.
  Rejected: A dedicated batch command or follow-up grouping record (the parent increment already is the batch)
