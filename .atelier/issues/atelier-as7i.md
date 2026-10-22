---
created_at: "2026-10-22T19:44:07.832418357+00:00"
id: "atelier-as7i"
issue_type: "bug"
labels:
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
closed_at: "2026-10-22T20:52:15.983357090+00:00"
status: "done"
title: "Find review records that were created inside a merge commit"
updated_at: "2026-10-22T23:05:41.799978960+00:00"
---

## Goal

`land`, `merge` and `doctor` find the commit that introduced a review record even when the record first appears in a merge commit, and one record whose origin can't be found doesn't abort the whole run. Today astromech's `atelier doctor` fails outright with "review event history missing", and so does `land` on its base whenever a change reaches its older claims.

## Done when

- `introduced_query` (`crates/atelier-app/src/review_gate.rs`) lists a record added by a merge commit, for example with `--diff-merges=first-parent`; the scope queries built from it agree.
- The defer/no_action path in `evaluate_scope` treats a record whose history can't be found as not accepted, with a gate message naming the record, as the approval path does (`unwrap_or(false)`), instead of returning the error through `?`.
- Tests: a review record created in a merge commit where neither parent has the file (a conflict-resolving merge) is found and its disposition accepted; a record with no findable history gives a failing gate that names it, not an error.
- `atelier doctor` passes on brewer-corp/astromech at 649dc8ec, and `land` passes there on a commit that adds an issue advancing astromech-7q0z (see Context).

## Context

Found in astromech (its issue astromech-sx84, with the bisect). 20 defer/no_action review records there were created inside merges: `.atelier/reviews/astromech-3h36.md` first appears in merge 201c809b, and neither parent (78e598da, a76a7f7b) has it, so `git log --root --reverse --diff-filter=A HEAD -- <file>` lists no commit; `original_independent` then fails at `.context("review event history missing")` and the `?` at the defer/no_action branch aborts land. Backtrace: `original_independent` ← `evaluate_scope` ← `evaluate_claims`. Built from 4cf082ca.

## Notes

- 2026-10-22 [decision] A defer/no_action disposition whose original acceptance cannot be checked (history missing, predates its issue scope, rewritten) fails findings.disposed with 'disposition <id> could not be traced to the commit that introduced it: <error>'
  Reason: Matches the approval path's unwrap_or(false) while naming the record so the gate stays actionable, instead of aborting land and doctor for every claim
- 2026-10-22 [decision] introduced_query and the scope history query use --diff-merges=first-parent with --no-patch; the owner-updates query is unchanged
  Reason: A merge adding a path neither parent has is already kept by default history simplification but has no diff for --diff-filter=A to match; first-parent diffing gives it one, while a merge that takes the path from its second parent stays simplified away so the side commit remains the origin. --diff-merges prints the merge's patch, which --no-patch removes so the output is only commit IDs. The updates query has no diff filter, so it already lists such merges.
  Rejected: Diffing merges against every parent (-m), which also lists side-branch merges; --first-parent traversal, which would stop following the side branch that really introduced most records
- 2026-10-22 [note] Checked read-only against /home/supadava/projects/astromech at 649dc8ec with target/debug/atelier from faed7efe. All 2147 review records now have an introducing commit (25 had none with the old query, among them astromech-3h36 at merge 201c809b). In a --shared local clone, land on main with one new commit adding an issue that advances astromech-7q0z passes (the 4cf082ca binary fails there with 'review event history missing'). doctor no longer errors but still fails one gate: 'session findings.disposed: astromech-h9kb, astromech-zdy2: unresolved FIX NOW: resolution needs fresh independent approval'. astromech-y296 has author fix_now dispositions (astromech-clc7, astromech-f3ta) and only 'review' events (astromech-bt1w, astromech-a1ec), no approval, so no approval covers the resolutions. The old abort hid this. It is a gap in astromech's records, not a history lookup, so the doctor part of Done when is not met by this change alone.
- 2026-10-22 [note] Correction from review finding atelier-83w3, applied in atelier-7b6e: the decision note above says a merge that takes the path from its second parent stays simplified away under --diff-merges=first-parent. It does not. Any --diff-merges option, like -m, turns history simplification off, so git lists both the side commit that added the path and the clean merge. Callers take the first commit after --reverse as the introducer; git orders its walk by committer date, so the side commit, an ancestor of the merge, comes first while commit dates increase along history. The comment on introduced_query now says so.

## Close Reason

Review records and issues first added by a merge commit are now found. `introduced_query` and the scope history query in `crates/atelier-app/src/review_gate.rs` pass `--diff-merges=first-parent --no-patch`. In `evaluate_scope`, a defer/no_action disposition whose original acceptance cannot be traced is not accepted: `findings.disposed` fails naming it ("disposition <id> could not be traced to the commit that introduced it: <error>"), instead of aborting land and doctor. Tests in `crates/atelier-cli/tests/session.rs`: `review_records_first_added_by_a_conflict_resolving_merge_are_found` and `a_disposition_without_findable_history_fails_its_gate_by_name`. Both failed before the fix for the reported reasons. Evidence: atelier-l8hc (focused), atelier-q7v7 (full suite, 818/818).

### Deviations
`atelier doctor` on brewer-corp/astromech at 649dc8ec does not pass. It now runs to completion and fails only on `session findings.disposed` for astromech-h9kb and astromech-zdy2 ("unresolved FIX NOW: resolution needs fresh independent approval"). astromech-y296's fix_now dispositions (astromech-clc7, astromech-f3ta) were recorded by the author, and y296 has only `review` events (astromech-bt1w, astromech-a1ec), no approval, so nothing approves the resolutions. This is correct gate behaviour on a real gap in astromech's records; the old abort hid it. `land` on astromech at 649dc8ec, on a commit adding an issue that advances astromech-7q0z, passes (the 4cf082ca binary fails there with "review event history missing"). All 2147 astromech review records now have an introducing commit; 25 had none before.

### Decisions
See the decision notes on this item: the `--diff-merges=first-parent --no-patch` flag choice for the two queries, and the not-accepted-and-named treatment of untraceable defer/no_action dispositions.

### Surprises
`--diff-merges=first-parent` prints the merge's patch in `git log` output, so `--no-patch` is needed to keep the output to commit IDs. The no-history test could not use the merge case once it was fixed, so it commits review records before their issue (a "review predates its issue scope" failure).

### Open items
- astromech needs an independent approval of the astromech-y296 resolutions (an astromech-side action, no issue in this repo).
- Rename detection in `introduced_query` and the merge blind spot in `append_only_history`'s first-review query are noted on atelier-7b6e.
