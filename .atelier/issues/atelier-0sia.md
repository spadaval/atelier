---
created_at: "2026-10-17T08:19:41.954488880+00:00"
id: "atelier-0sia"
issue_type: "bug"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-0sia
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T09:03:04.809004435+00:00"
status: "done"
title: "Leave the parent out of work parent proof gaps, and drop stale done-record hints"
updated_at: "2026-10-17T09:03:04.809004435+00:00"
---

## Goal

Found while implementing atelier-ksx6:
- `work parent` on a done parent counts the parent itself in `Proof gaps` (12 for 11 issues on atelier-1ioo) and prints `Transition readiness: closed`.
- The `issue link` footer (`relate.rs`) still offers transition options on done issues.
- The brief `issue show` of a done item prints `Closed …` and still lists "Close Reason" under `More with --full`.

## Done when

- Proof gaps count the parent's scope consistently with the scope size.
- Done records get no transition steps from `issue link`, and the brief view does not offer the Close Reason it already summarized.
- Tests cover each.

## Close Reason

work parent's Proof gaps now count only the parent's scope, matching its size; a done parent shows no Transition readiness line; issue link offers transition options only when the source has transitions out; the brief issue show of a done record leaves a one-line Close Reason out of More with --full. Tests cover each.
