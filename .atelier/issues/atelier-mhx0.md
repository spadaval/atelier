---
created_at: "2026-10-17T19:00:36.356125467+00:00"
id: "atelier-mhx0"
issue_type: "bug"
labels:
- "retro-2026-10-17"
- "start"
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
closed_at: "2026-10-22T21:02:01.710227861+00:00"
status: "done"
title: "Refuse a work branch whose type prefix does not match the issue"
updated_at: "2026-10-22T21:02:01.710227861+00:00"
---

## Goal

1jll's branch prefix did not match its issue type, so `start` recorded the wrong base and land later failed with land_base_mismatch ('recorded base task/atelier-1jll disagrees with trusted policy base master').

## Done when

`atelier start` on a branch whose `<type>/` prefix does not match the item's type refuses and names the right branch to create. A test covers it.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). Related: atelier-0vp3.

## Notes

- 2026-10-22 [decision] start refuses (start_branch_prefix_mismatch) only when it would create a new owner branch from a current branch named '<prefix>/<owner id>' whose name is not the owner's derived branch; the message names the derived branch and --base
  Reason: That is the exact 1jll failure: a hand-made wrong-prefix branch is recorded as branch_base. Matching on the id suffix avoids refusing legitimate bases such as master or release branches
  Rejected: refusing every non-base current branch (would break existing flows that start from other branches)

## Close Reason

`atelier start` on a branch named `<other prefix>/<owner id>` now refuses with `start_branch_prefix_mismatch`, naming the owner's correct branch and the `--base` fix; a test covers it.

### Deviations
The refusal triggers on a current branch whose last path segment is the owner id but whose name is not the derived branch, which is the reported failure; other non-base current branches are unchanged.

### Decisions
See the item's decision note.

### Surprises
None.

### Open items
None.
