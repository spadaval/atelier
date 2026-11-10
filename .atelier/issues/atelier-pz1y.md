---
created_at: "2026-11-10T00:56:08.329209632+00:00"
id: "atelier-pz1y"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-pz1y
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-jciw"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-jq24"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-yptz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Add `check` and the three Git hook rules"
updated_at: "2026-11-10T22:05:56.194959770+00:00"
---

## Goal

Move Atelier's rules about Git actions to where Git acts. `check` validates every record, and with `--hook` it is what each of three Git hooks runs: a commit of a broken record, a status change no command could have made, and a push that puts unreviewed code on the base are each stopped by the Git command that would have done it.

## Done when

- [x] `atelier check` validates every item, review, mission and `config.toml` in the checkout, prints each problem with its file and field, and exits non-zero when it found one. It also lists record changes not yet committed, which is information and does not change its exit status.
- [x] `check --hook pre-commit` validates every staged `item.md`, `mission.md`, review file and `config.toml`, as staged and not as in the working tree. Other files under `.atelier/` are not examined: a test stages a mission document with no front matter and an image in an item's `artifacts/`, and the commit goes through. Where a staged record's status differs from the same record's in `HEAD`, the act table must list a row from the old status to the new one; the conditions of that row are not checked here. A test commits each kind of refused change (an invalid record, and a status change with no row) and one allowed change made by hand.
- [x] `check --hook pre-push` reads what Git gives a pre-push hook. When the push updates the base branch, each new commit on the base's first-parent line must be a merge made by `atelier merge`, known by its `Atelier-Merge:` trailer, or touch only paths under `.atelier/missions/` and `.atelier/items/`. Commits a merge brought in on its second parent are not examined. A commit that is not such a merge may not set an item `done` while that item's branch exists. A push of any other branch is not examined.
- [x] `check --hook post-checkout` prints what `status` prints when a branch was switched, and always exits 0, whatever it finds: a record that does not parse is printed as a problem and the checkout still succeeds. A test switches to a branch that holds a broken record and checks that the switch happened and Git exited 0. This holds for everything later items add to this hook: nothing it runs or reports may fail a checkout.
- [x] Scenario tests in a scratch repository, with hook scripts the test writes: a commit of an invalid record refused; a commit of a status change the act table does not allow refused; a push of the base holding a plain commit that touches code refused, and one that changes `.atelier/config.toml` directly refused; a push holding only planning commits allowed; a push whose new commit is an Atelier merge of a branch with ordinary code commits allowed; a push of an item's branch allowed whatever it holds; a branch switch printing `status`.
- [x] Each refusal appears in the output of the Git command it stopped, says which rule and which file or commit, and names the way out. A record that does not parse is refused with a line saying it is not a rule and cannot be skipped.
- [x] `docs/strategies/redesign-architecture.md` matches what was built.

## Scope

`check` and its three hook modes, in `next/`. Installing the hooks is atelier-ym3z, so these tests write the one-line hook scripts themselves. `ATELIER_SKIP` is atelier-zrss: until it lands the hooks have no bypass but Git's own `--no-verify`. There is no `land` command. Nothing is copied from the old workspace. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, "Hooks and the bypass", "Where records are read and written" (the pre-push rule) and Testing. Behaviour: `docs/strategies/redesign-product.md`, "Hooks and the bypass" and Principles 6 and 9. The specification names the parser's inputs exactly so that a mission's documents and an item's artifacts are never parsed (planning finding atelier-1l4r). `atelier merge` already writes the `Atelier-Merge: <id>` trailer as the last line of its merge commit's message.

This is the first of step D's three items in `docs/strategies/redesign-2-core.md`.

## Report

All seven Done when lines hold. In `next/crates/atelier-cli/tests/hooks.rs` (10 scenarios, Git driving hook scripts the tests write): `check_reports_every_problem_with_its_file_and_field_and_lists_uncommitted_records` (a record that does not parse, an invalid one and a config with an unknown key each named with file and field, exit 1; uncommitted record changes listed with exit 0); `a_commit_of_an_invalid_record_or_of_a_status_no_act_sets_is_refused_and_a_hand_edit_goes_through` (invalid, unparseable, ready to approved refused; a retitle and ready to dropped by hand committed; a deletion committed); `the_pre_commit_hook_reads_records_as_staged_and_passes_over_other_files` (mission document with no front matter and a binary artifact committed; staged-broken refused, working-tree-broken committed; a rename judged at its new path; a commit in a second worktree refused); `the_first_commit_of_a_repository_is_checked`; `a_push_of_the_base_with_a_plain_commit_of_code_or_config_is_refused` (code, `config.toml`, another branch pushed to the base, a plain `git merge`, a non-merge carrying the trailer, a force push; code pushed under another name allowed); `planning_commits_reach_the_base_and_a_push_of_several_refs_judges_only_the_base` (planning allowed, several refs, a deletion); `an_atelier_merge_of_a_branch_with_code_commits_reaches_the_base_and_its_branch_any_time` (the real `start`, `submit`, `review`, `verdict`, `merge` under the hooks, with planning pushed beside the merge); `a_plain_commit_setting_an_item_done_while_its_branch_exists_is_refused` (and allowed once the branch is gone); `a_first_push_of_a_base_judges_its_whole_line`; `a_branch_switch_prints_status_and_succeeds_whatever_it_finds` (status printed; a switch to a branch with an unparseable record exits 0 and prints the problem; a file checkout prints nothing). `next/crates/atelier-records/tests/load.rs` covers the loader directly. Each test was seen failing under a mutation of the rule it covers (refusals disabled, working tree read instead of the index, the trailer or two-parent test dropped, the branch test dropped, `is_planning` ignored, mission documents read). Full suite: 181 passed after round 2's fixes; fmt and clippy `-D warnings` clean; the CI `next` job's steps pass in a scratch clone. The architecture specification's "Hooks and the bypass", the pre-push bullet, the planning bullet and Testing match the build.

Timing, debug build, a fixture of 500 items with two reviews each (1,500 record files): `atelier check` 0.13 s, `check --hook pre-commit` with one staged change 0.15 s.

Hand transcript, new binary on `PATH` as `atelier`, base `trunk`, the three one-line scripts in `.git/hooks`, pushes to a local bare repository (setup and file edits elided):

```text
-- A first commit, and a first push of the base --
$ git commit -qm Set up Atelier
$ git push -q origin trunk
Atelier's pre-push hook refused this push; nothing was pushed.
Rule: code and config.toml reach trunk only through atelier merge. Each new commit on trunk's first-parent line is an Atelier merge, or changes only .atelier/missions/ and .atelier/items/. These are neither:
  77696df Set up Atelier: changes .atelier/config.toml
Take these commits off trunk and onto an item's branch (atelier start --new "<title>" starts one), and atelier merge brings them in once the item is approved.
error: failed to push some refs to '../remote.git'
[exit 1]
$ git push -q --no-verify origin trunk
-- Planning, committed and pushed --
$ atelier check
Every record is valid: 2 items, 0 missions and config.toml.
Record changes not committed yet: .atelier/items/demo-06wr/item.md, .atelier/items/demo-f746/item.md
$ git commit -qm Plan greetings
$ atelier ready demo-06wr demo-f746 --by sumeet
Made ready, as sumeet said: demo-06wr, demo-f746.
Wrote, and did not commit:
  .atelier/items/demo-06wr/item.md
  .atelier/items/demo-f746/item.md
Planning counts once it is committed on the base.
Then: git add -- .atelier/items/demo-06wr/item.md .atelier/items/demo-f746/item.md && git commit -m "Ready demo-06wr, demo-f746" -- .atelier/items/demo-06wr/item.md .atelier/items/demo-f746/item.md
$ git commit -qam Ready greetings
$ git push -q origin trunk
-- check with two broken records --
$ atelier check
`atelier check` found 2 problems in 1 item, 0 missions and config.toml.
These do not read as records. That is not a rule, and it cannot be skipped:
  .atelier/items/demo-06wr/item.md: front matter: status: unknown variant `finished`, expected one of `draft`, `ready`, `building`, `in_review`, `changes_requested`, `approved`, `done`, `dropped` at line 5 column 9
Rule: every record is valid. These are not:
  .atelier/items/demo-f746/item.md: blocked_by: `demo-zz99` names no item
Fix each in the file named, then run atelier check again.
Record changes not committed yet: .atelier/items/demo-06wr/item.md, .atelier/items/demo-f746/item.md
[exit 1]
-- A refused commit of each kind --
$ git commit -qam Break both
Atelier's pre-commit hook refused this commit; nothing was committed.
These do not read as records. That is not a rule, and it cannot be skipped:
  .atelier/items/demo-06wr/item.md: front matter: status: unknown variant `finished`, expected one of `draft`, `ready`, `building`, `in_review`, `changes_requested`, `approved`, `done`, `dropped` at line 5 column 9
Rule: every record is valid. These are not:
  .atelier/items/demo-f746/item.md: blocked_by: `demo-zz99` names no item
Fix each file named, stage it with git add, and commit again.
[exit 1]
$ git commit -qam Approve by hand
Atelier's pre-commit hook refused this commit; nothing was committed.
Rule: a status changes only as an act of the lifecycle changes it. These change it as no act does:
  .atelier/items/demo-06wr/item.md: status: ready to approved. Put back the status HEAD has; atelier status names the command for what comes next.
Fix each file named, stage it with git add, and commit again.
[exit 1]
-- An allowed commit made by hand --
$ git commit -qam Retitle
-- A refused push of the base: code, and config.toml --
$ git commit -qm Say hello on trunk
$ git push -q origin trunk
Atelier's pre-push hook refused this push; nothing was pushed.
Rule: code and config.toml reach trunk only through atelier merge. Each new commit on trunk's first-parent line is an Atelier merge, or changes only .atelier/missions/ and .atelier/items/. These are neither:
  bcbd7da Say hello on trunk: changes hello.txt
Take these commits off trunk and onto an item's branch (atelier start --new "<title>" starts one), and atelier merge brings them in once the item is approved.
error: failed to push some refs to '../remote.git'
[exit 1]
$ git commit -qam Add a merge check
$ git push -q origin trunk
Atelier's pre-push hook refused this push; nothing was pushed.
Rule: code and config.toml reach trunk only through atelier merge. Each new commit on trunk's first-parent line is an Atelier merge, or changes only .atelier/missions/ and .atelier/items/. These are neither:
  deefe43 Add a merge check: changes .atelier/config.toml
Take these commits off trunk and onto an item's branch (atelier start --new "<title>" starts one), and atelier merge brings them in once the item is approved.
error: failed to push some refs to '../remote.git'
[exit 1]
-- An Atelier merge of a branch with code commits, pushed --
$ atelier start demo-06wr
Started demo-06wr on branch demo-06wr.

You are the implementer of demo-06wr (thread impl).
Item: .atelier/items/demo-06wr/item.md
Title: Say goodbye
Status: building
To submit it for review:
  [ ] Done when: It greets.
  [ ] Report: written under `## Report`
`atelier submit` commits only the item's directory, so commit your change first.
Then: atelier submit
$ git commit -qm Say hello
$ git commit -qam Say it louder
$ git push -q origin demo-06wr
$ atelier submit
Submitted demo-06wr for review.
A reviewer must be another thread: not you, and not a subagent of yours, which runs as your thread. Start one with your harness's tool for a new agent thread, and give it this task:
  Run atelier review demo-06wr and follow what it prints.
Then wait for its verdict, and run `atelier status`.
$ atelier verdict approve --model none
Recorded your verdict on demo-06wr, round 1: approved. Committed .atelier/items/demo-06wr/reviews/1-keip.md with it.
demo-06wr is back with its implementer, impl, to merge. Your review is done: say so to whoever started you, and stop.
$ atelier merge
Merged demo-06wr into trunk with merge commit 578e81b, which marks it done.
-- A branch switch --
$ git switch trunk
Switched to branch 'trunk'
You are thread planner, on the base branch trunk.
Ready to start:
  demo-f746 (medium): Say hello, warmly
Then: atelier start demo-f746
$ git log --oneline --first-parent -4
578e81b Merge demo-06wr: Say goodbye
7c70dc7 Retitle
8c74b8c Ready greetings
3842b4f Plan greetings
$ git push -q origin trunk
$ git ls-remote origin trunk
578e81b7917d8da97476319d15b86e26cca92838	refs/heads/trunk
```

### Decisions

- Every record is read one file at a time by `atelier-records`' new `load`, from a list of (path, bytes): the working tree for `check`, the index (`CommitReader::staged`, reading `:0:<path>` through the same `cat-file --batch`) for pre-commit. A file that does not read leaves the others read and validated. Problems that do not parse are listed apart from rule breaks, so atelier-zrss can let rules pass and never a parse failure.
- A review whose item or mission is missing or does not parse cannot be validated, so it is a problem on the review's own path (atelier-d7lx): a commit of such a review is refused. `check` lists an uncommitted `config.toml` with the other record changes (atelier-tv18), and reads records Git ignores, as the commands do (atelier-z02d). A pre-push refusal of a record that does not read says to rewrite the commit that broke it (atelier-dnlm).
- Pre-commit validates the whole index and refuses only the problems of records the commit adds or changes. A deletion is not judged, so deleting an item another names in `blocked_by` passes the hook and shows in the next `check`.
- A reference to a record that does not parse is reported as naming nothing as well (a cascade line beside the parse problem); stripping it was more code than it was worth.
- An Atelier merge needs its trailer and two parents: a cherry-picked or hand-written commit carrying the trailer is judged as a plain commit.
- The new commits of a push are the first-parent line not in the history of the remote's old commit, never "not on any remote branch": excluding remote branches would let a fast-forward of the base to a pushed item branch through.
- The status rule asks the act table only whether some row goes from the old status to the new, for items and missions alike, as the brief says; so a mission edited from draft to done by hand passes, as `close` on an item would.

### Surprises

- The first push of a repository's history is refused: every commit is new, and its first commit writes `config.toml`. Until atelier-zrss it passes only with `--no-verify`. The cutover pushes converted records directly; if hooks are installed before that push, it will be refused the same way.

### Open items

- `ATELIER_SKIP` (atelier-zrss) attaches in `check.rs` where each hook builds its refusal: `invalid`, `acts`, `code` and `done` are rules, `unread` is not. `setup` running a hook the clone already had (atelier-ym3z) attaches in `check::hook`, the one dispatch for `--hook`.
