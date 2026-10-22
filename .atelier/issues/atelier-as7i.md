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
status: "in_progress"
title: "Find review records that were created inside a merge commit"
updated_at: "2026-10-22T20:51:25.852759775+00:00"
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
