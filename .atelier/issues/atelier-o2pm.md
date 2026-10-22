---
created_at: "2026-10-22T22:42:57.061302606+00:00"
id: "atelier-o2pm"
issue_type: "task"
labels:
- "merge"
priority: "P3"
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
closed_at: "2026-10-22T22:58:05.093171505+00:00"
status: "done"
title: "Make merge --sync's resume and stale-approval verdict read plainly"
updated_at: "2026-10-22T23:16:02.453994546+00:00"
---

## Goal

Found using `atelier merge --sync` on its own branch (atelier-s2ei). After a hand-resolved conflict, the second run says "already contains master; nothing to merge" although it is resuming after a hand merge and rerunning the receipts that were current before it. And the verdict is `merge: passed` while the report says the approval needs fresh review, which reads as a contradiction although `merge --check` will then refuse. The rerun of a whole workspace suite also shows no progress.

## Done when

A resumed sync says it is resuming after a merge made by hand and names the merge commit. A sync that leaves the approval stale ends in a verdict that says so, for example `merge: synced — approval needs fresh review`, documented in docs/product/human-cli-output.md. Long reruns print the command before they start. Tests pin the three outputs.

## Notes

- 2026-10-22 [decision] A sync that leaves the approval stale ends 'merge: synced — approval needs fresh review' and still exits zero
  Reason: The sync did its work and a stale approval is the reviewer's next step, not a failed check; only the verdict wording changes, so scripts keyed on the exit status behave as before. The Next: --check line is dropped in that case because check will refuse.
  Rejected: Exit non-zero (turns an expected outcome into a failure); keeping 'merge: passed' (the reported contradiction)
- 2026-10-22 [decision] Sync's kept/stale report asks review_gate::approval_state for the fork-point rule, in merge_sync.rs only
  Reason: review.current is current_on_branch (approval not older than the fork point) plus approval_is_current; approval_stale_edges implements only the second and skips base-held commits. Reusing approval_state gives agreement by construction and leaves review_gate.rs untouched, which atelier-r28p is editing.
  Rejected: Dropping the ancestor skip in approval_stale_edges (it is right for the commits it names after a real sync; changes review_gate.rs)

## Close Reason

`merge --sync` now says `Resuming after a merge of <base> made by hand (<merge commit>)` when it resumes, prints `running <item>: <command>` before each rerun, and ends `merge: synced — approval needs fresh review` (exit zero) when the approval is left stale. docs/product/human-cli-output.md and reviews-and-merge.md document the forms.

### Deviations
After closing, the orchestrator added finding atelier-ns03 to this item; it was fixed in a follow-up commit on the branch (see Surprises).

### Decisions
Two decision notes on atelier-o2pm: the stale-approval verdict exits zero, and the sync report asks the gate's own approval_state for the fork-point rule.

### Surprises
atelier-ns03: after approve, a code commit, and a --no-ff merge of the branch into the base, `--sync` said `Approval kept` and `merge: passed` while `merge --check` failed review.current. approval_stale_edges skips commits the base holds, and the approval predates the new fork point, which only current_on_branch checks. The report now also requires approval_state (current_on_branch) to be Current, and otherwise says the approval predates the point where the branch left the base and lists the non-record paths changed since the approval.

### Open items
None.
