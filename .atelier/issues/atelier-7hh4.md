---
created_at: "2026-10-16T18:33:32.869081622+00:00"
id: "atelier-7hh4"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-7hh4
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-1y1z"
    type: "discovered_from"
  - kind: "review"
    id: "atelier-6hpy"
    type: "discovered_from"
  - kind: "review"
    id: "atelier-9l63"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:29:44.671470592+00:00"
status: "done"
title: "Harden clean base merge detection"
updated_at: "2026-10-16T20:29:44.671470592+00:00"
---

## Goal

Close the gaps review found in atelier-n7g1's clean base merge rule.

## Done when

1. `clean_merge` cannot be satisfied by a configured merge driver or attributes producing content from neither side (atelier-6hpy): run `git merge-tree` isolated from repository merge drivers, or document the caveat.
2. With no work owner, the clean-merge side check uses the real base branch tip, not `HEAD^` (atelier-9l63).
3. A test covers an approval pinned before the branch left the base being refused after the base is merged in (atelier-1y1z).

## Context

Deferred findings from the atelier-n7g1 review (approval atelier-s7ea).

## Design

Resolution (2026-10-16):

1. Documented, not isolated (atelier-6hpy). A merge driver configured for the repository (for example `merge=ours` on a changelog, or `merge=union`) is the merge the repository chose, and the real merge used it too, so a result it reproduces is that repository's own merge. Isolating merge-tree from drivers (`--attr-source` with an empty tree, or dropping the driver config) would leave text and eol attributes intact but turn every driver-resolved file into a conflict, so each such base merge would need renewed review; review finding atelier-09wh corrected an earlier, wrong reason given here. reviews-and-merge.md says custom merge drivers count as Git's own merge and are reviewed as policy.
2. Accepted (atelier-9l63). With no owner, the clean-merge side check uses `HEAD^`. The alternative, the base's upstream tip, would refuse `land` on the base for a just-merged branch whose base merge brought in unpushed local base commits, before the push that follows. As the finding notes, it is not exploitable: `check_base_commits` still checks every unpushed side commit on its own.
3. Tested (atelier-1y1z): an_approval_from_before_the_branch_left_the_base_is_refused_after_a_base_merge.

## Close Reason

(1) merge drivers documented as part of Git's own merge and reviewed as policy, since a configured driver is the repository's chosen merge and isolating it would send every driver-resolved base merge to review; (2) the no-owner HEAD^ side check is accepted, with the reason recorded; (3) an_approval_from_before_the_branch_left_the_base_is_refused_after_a_base_merge tests the pin rule. See Design.
