---
schema: atelier.review
schema_version: 3
round: 3
commit: b194fce2fa139e69a58975b856cd458dd6a40983
reviewers:
- 01a20240-b7ca-70e2-adee-538ca5f0d960
model: grok-4.6
verdict: approve
created_at: 2026-11-22T07:55:49.155947822Z
---

## Summary

Complexity: medium. Documents only, with one shared `backlog/garden-proposals`
branch that gardeners and planners both edit, merged or not. This round is
the merge of master after that approval.

Value: high. Approved garden upkeep has been sitting off `master` for over
a week; merging on approval is the outcome this item is for.

Risk of merging it: low. The lookup that would file a second `Garden
proposals` item is still a command that exists, the merge of js4a did not
touch it, and `plan-merge` is a different `$4` from `merge`.

Checked the commits since 45bd17abf: 4c5e6bd7f is a clean merge of
origin/master at a1ee47a81 (js4a). It changed none of `garden.md`,
`SKILL.md`, or `docs/product.md`. The 7637 Decisions row in `docs/design.md`
is the same text as at 45bd17abf; js4a's log-table rows landed in a
different section. `git merge-tree` of current master into this head reports
no conflict. The two commits master gained since the merge (follow-ups
atelier-itqc, jz8q, kjc4, tuln, vfnn and a notes.md line) are record-only.

The whole diff against master is still those four documents plus this item's
records. Walked each Done when line against `garden.md` (opening paragraph,
steps 1, 4, 5, 6, 7), `SKILL.md` ("Who Works With the Human"), the Gardener
row of `docs/product.md`, and the Decisions row. Finding 1-15h5.1 / 2-6qs6.1
is still the named lookup: `atelier list --label garden` (and with
`--done`), keep the item titled exactly `Garden proposals` that also has
`needs-human`, `--label` takes one label, `--needs-human` hides an item not
yet merged, and both steps read a branch copy when `list` still shows the
base's.

Read `list --label` (`narrows_item` tests `labels.contains` on one string,
`main.rs` takes `Option<String>`), `--needs-human` skipping the proposed
section (`views.rs` `narrowed`), `set --labels` splitting on commas,
`start --backlog` refusing when the branch exists locally or on a forge
remote ("Pick another slug"), `close` of a draft (`from: PLANNED`, refused
when a branch exists), and `merge::pull_request` (`state=open`). Log lines
are `<time> <commit> <id> <action> …` (`history.rs`); `Happened::PlanMerge`
prints `plan-merge`, and the js4a tests assert `$4` is `plan-merge` and never
`merge`. Installed 0.7.0's `atelier log` here prints 79 `merge` lines and 0
`plan-merge`, as that binary predates js4a.

On this clone, `atelier list --label needs-human,garden` printed "No open
work matches." `atelier list --label garden` listed atelier-j86z (not the
rolling item: title is not `Garden proposals`) and the two unmerged garden
runs this item is for. Ran `atelier check` (exit 0) and
`evals/agent-factory/test.sh` with installed 0.7.0 (exit 0, "All eval
fixtures and harness tests pass."). `git diff --check` was clean. GitHub CI
on PR 271 at b194fce2f: `evals` success, `test`/`desk` skipped as a
docs-only change.

Could not run the same cycle through GitHub; CI and `merge::pull_request`
cover the reused-branch PR. Did not repeat round 2's six-state scratch:
`garden.md` is byte-identical to 45bd17abf. Did not merge `atelier-0019` or
`atelier-qb4f` (out of Scope). `docs/product/desk.md` still describes
gardening as a session that does not leave a list; the Report already leaves
that for the lead.

## Finding 1: Step 4's list command will not find the rolling item
Recommendation: fix now
Was: 2-6qs6.1
Response: fixed in 84271944b
Now: fixed

Steps 4 and 7 still run `atelier list --label garden` (and with `--done`),
keep the item titled exactly `Garden proposals` that also has
`needs-human`, and say `--label` takes one label. Step 7 names the
`--needs-human` miss for an item not yet merged, and both steps read a
branch copy when `list` still shows the base's. That text is unchanged
since 45bd17abf. On this clone the comma-pair still matches nothing, and
`--label garden` still lists the garden items.
