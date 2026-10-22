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
status: "todo"
title: "Find review records that were created inside a merge commit"
updated_at: "2026-10-22T19:44:07.832418357+00:00"
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
