---
schema: atelier.item
schema_version: 3
id: atelier-u6oy
title: Let atelier new file an item on whatever branch the agent is on, and lose none when that branch is dropped
status: in_review
mission: atelier-dh7a
labels: []
priority: medium
blocked_by:
- atelier-v06d
implementers:
- thread:delegated-task:command%3Amcp%3A134a3ba0-c5f0-4828-a968-a4cfb219e451%3Adelegate-task%3A4b486e8a-dd69-4ad4-8a51-84eddea7bba4
ready: null
trivial: null
caused_by: null
created_at: 2026-11-19T15:25:03.190889589Z
---

## Goal

`atelier new` belongs on a backlog branch or a proposed mission's, and the
pre-commit hook refuses a new item's record anywhere else. An implementer
that notices a problem outside its item must make a second branch and merge
it to record one sentence, so it mostly does not. One exception already
works: `new --from <finding>` on the branch of an item the caller
implements. After this item `atelier new` works on any branch `start`
made, and the new item reaches the base with that branch's merge.

## Done when

- [x] On an item's branch, `atelier new "<title>"` writes an open item, the
  commit that adds it passes the pre-commit hook, and after `atelier merge`
  of that branch the item is open on the base. A test holds each step.
- [x] Before that merge, `atelier list` run elsewhere shows the new item as
  proposed with its branch, as it shows any record waiting to merge.
- [x] The new item joins no mission unless `--in` names one, and is not
  held by the thread that filed it.
- [x] A reviewer of an item, who works in the implementer's checkout, can
  file the same way. The brief `atelier review` prints says so in place of
  "change no other file", and a test follows that brief as printed: the
  review and the filed item are both committed, and the filed item reaches
  the base with the merge.
- [x] A commit that only adds a new item's record does not make the
  approval of the branch's own item stale, whoever makes it and whether
  before or after `verdict approve`; `atelier merge` goes through with no
  further round, and a test holds it. Any other change to another item's
  record still makes the approval stale, as now.
- [x] A plan reviewer, in a detached checkout of its own, is told by its
  brief to publish its verdict first and then file an unrelated problem on
  a backlog branch, so the plan under review and the verdict are untouched.
- [x] Dropping a started item whose branch holds newly filed items loses
  none silently: `drop` carries them to the base with the drop's own
  record change, or names each in what it prints. A test holds whichever
  it is, and `docs/design.md` says which.
- [x] `atelier new --help`, the action table in `docs/design.md`, and
  `docs/product.md` and `AGENTS.md` where they say which branch filing
  needs, say the same thing as the hook.

## Scope

The rule for `new` in the pre-commit hook and wherever `crates/` holds the
action table; the briefs `atelier review` prints; `drop` for a started
item; those documents. `new` still
commits nothing and merges nothing. Not `set`, `close`, `drop` of another
item or `lead` on an item's branch: only adding a new item's record. Not
the skill's instruction to use this (atelier-8phx).

## Report

An implementer or a reviewer can now record a problem outside its item
with `atelier new` and one commit, where it stands. The new item reaches
`master` with that branch's merge and costs no new round of review. Once
this merges, a host needs the new binary: the installed one still refuses
`new` on an item's branch and calls such a commit stale.

One premise of the Goal was wrong. The pre-commit hook never refused a
new item's record on an item's branch; only `new` did. I committed one by
hand through the hook of master's binary (3f97eaae8) and it passed. So
the hook is unchanged, and `docs/design.md` now says that the branch is
the command's to check.

What changed:

- `new` runs on a started item's branch for any thread. It is refused on
  the branch of an item that is done or dropped, which will not merge;
  that refusal is a rule a skip passes. `new --from` keeps its old rule,
  since it also changes the finding's item.
- An approval ignores one kind of path outside the item's directory: the
  `item.md` of an open item that neither the reviewed commit nor the
  base's tip holds. An edit, a deletion, a restore, a record in another
  status or any other file beside a new record still makes it stale.
  Editing an item that the review already saw filed is such an edit.
- `merge` now refuses while a filed item is uncommitted. Before, it
  ignored untracked records of other items, and with `new` committing
  nothing, the filed item would have been left behind with nothing said.
- `drop` names what the branch filed and carries none, since the dropping
  thread judges which are still wanted. It now also names one not yet
  committed in the branch's checkout, with the commit to make first.
- An item's brief allows the one extra write. A plan's brief, and what
  `verdict` prints after it, say to push the verdict and then file on a
  backlog branch. `verdict` needed it too, since it ended with "and stop".

### Validation

`crates/atelier-cli/tests/filing.rs` runs on clones of a bare remote with
the hooks `atelier setup` installs, and takes the reviewers' commands from
the printed briefs.

| Done when | Test |
| --- | --- |
| files, commits through the hook, open after merge; proposed elsewhere; no mission, no holder | `an_item_filed_on_an_items_branch_is_proposed_until_that_merges_and_then_open`, which also starts the item afterwards |
| a reviewer follows its brief | `a_reviewer_files_as_its_brief_says_before_or_after_its_verdict` |
| approval holds, whoever files | `an_item_filed_after_approval_by_anyone_leaves_the_approval_standing` |
| any other change is stale | `a_change_to_another_item_after_approval_makes_it_stale`, `what_else_a_filing_commit_holds_still_makes_the_approval_stale` in `tests/merge.rs` |
| a plan reviewer publishes, then files | `a_plan_reviewer_publishes_then_files_on_a_backlog_branch_as_its_brief_says` |
| drop loses none | `a_drop_names_what_was_filed_committed_or_not_and_its_commands_carry_them` |

- `a_change_to_another_item_after_approval_makes_it_stale` used to use an
  added open item as its example of a change. It now covers edits,
  deletions, a restore and added records that are not open.
- I removed each of six rules in turn (the exemption, its base check, its
  open check, the uncommitted check, the dropped-branch refusal and
  drop's look at the checkout) and a test failed each time.
- On the branch with `origin/master` (cc62f7819) merged: `cargo fmt --
  --check`, `cargo clippy --all-targets -- -D warnings` and `cargo
  nextest run --workspace` (470 passed) are clean, and `atelier check`
  passes with the built and the installed binary.
- The built binary's `list` exits 0 in astromech, life-on-rails,
  cosplay-studio, demiurge and worldcrafter. I wrote nothing there.
- I ran review, `new` after approval, `merge` and `drop` by hand in a
  scratch repository with the built binary and its hooks.
- Not run here: the Desk's and the extension's tests. Neither reads what
  changed, and CI runs whichever `scripts/ci-jobs.sh` selects.

### Filed on this branch, with the new `new`

They reach `master` with this merge.

- atelier-fuzq, for the lead before atelier-8phx: an item filed on an
  item's branch cannot be labelled, so a question filed that way is not
  `needs-human`.
- atelier-5q9j: `new` on a dropped plan's branch still loses the item.
- atelier-s2ho: filing after CI ran makes `merge` wait for CI again. The
  approval holds; only the wait is left, and `docs/product.md` says so.
- atelier-jk0s: `new` can reuse the ID of an item on another unmerged
  branch, though `docs/design.md` says it cannot.
