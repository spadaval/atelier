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
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Harden clean base merge detection"
updated_at: "2026-10-16T20:27:48.898645305+00:00"
---

## Goal

Close the gaps review found in atelier-n7g1's clean base merge rule.

## Done when

1. `clean_merge` cannot be satisfied by a configured merge driver or attributes producing content from neither side (atelier-6hpy): run `git merge-tree` isolated from repository merge drivers, or document the caveat.
2. With no work owner, the clean-merge side check uses the real base branch tip, not `HEAD^` (atelier-9l63).
3. A test covers an approval pinned before the branch left the base being refused after the base is merged in (atelier-1y1z).

## Design

Resolution (2026-10-16):

1. Documented, not isolated (atelier-6hpy). Running merge-tree without the repository's drivers and attributes would also drop the eol and text attributes the real merge used, so ordinary merges would stop matching. reviews-and-merge.md now says custom merge drivers count as Git's own merge and are reviewed as policy.
2. Accepted (atelier-9l63). With no owner, the clean-merge side check uses `HEAD^`. The alternative, the base's upstream tip, would refuse `land` on the base for a just-merged branch whose base merge brought in unpushed local base commits, before the push that follows. As the finding notes, it is not exploitable: `check_base_commits` still checks every unpushed side commit on its own.
3. Tested (atelier-1y1z): an_approval_from_before_the_branch_left_the_base_is_refused_after_a_base_merge.

## Context

Deferred findings from the atelier-n7g1 review (approval atelier-s7ea).
