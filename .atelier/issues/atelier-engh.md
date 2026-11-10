---
created_at: "2026-11-10T00:56:06.737900196+00:00"
id: "atelier-engh"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-engh
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
    id: "atelier-mal9"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Give the new binary thread identity, the act table, and `start`, `submit` and `status`"
updated_at: "2026-11-10T15:56:20.817250837+00:00"
---

## Goal

Build the part of the loop that orients an implementer: the binary knows which thread is calling, finds started work by its branch, and `status` prints what the item needs and the next command.

## Done when

- [x] The new workspace builds an `atelier` binary at `next/target/debug/atelier`, from a new `atelier-cli` crate. No file under `crates/` is changed.
- [x] Scenario test: in a scratch repository with one ready item and `T3CODE_THREAD_ID=t1`, `atelier start <id>`, run with the new binary, sets the item `building` and appends `t1` to its `implementers` in one commit, then creates branch `<id>` pointing at that commit. `status` then names the role and the item, lists each unmet condition (each unticked Done when line, an empty Report) and ends with `Then: atelier submit`.
- [x] `submit` with an unticked line or an empty Report changes nothing, exits non-zero, and prints the same list `status` prints. With both met it sets `in_review` in one commit that also holds the Report, and prints the reviewer's handoff line.
- [x] `start` on a draft item changes nothing and names `ready` as the way out.
- [x] Scenario test, take-over: with the item in each of `building`, `in_review`, `changes_requested` and `approved` (later statuses written by the fixture), `start <id>` by thread `t2` switches to the branch, appends `t2` to `implementers` so that it holds the item, leaves the status unchanged, and says `t1` held it.
- [x] Scenario test, two workers: from two worktrees of one repository, `t1` and `t2` both run `start` on the same ready item. Exactly one creates the branch; the other changes nothing and is told `t1` holds the item and that running `start` again takes it over.
- [x] Test, interrupted start: when `start` is stopped after its commit is written and before the branch is created, no branch `<id>` exists, the item is still ready on the base, and a later `start` succeeds.
- [x] Records can be read at a commit, without a checkout. The object reader is copied from `ReadContext` in `crates/atelier-app/src/git_freshness.rs`: one `git cat-file --batch` process pinned to a commit. The record loader on top of it is written new. Unit tests read an item from an older commit while the working tree holds a newer one, and cover a path the commit lacks, a path that is not a regular file, and a `git` subprocess that fails; each gives an error that says which.
- [x] Scenario test, reading from the base: with the item started on its branch, `status` run on the base by a third thread shows the item as building and held by its implementer, and does not list it as ready. After the branch is merged, the base's record is used.
- [x] Unit test: identity is `T3CODE_THREAD_ID`, else `CLAUDE_CODE_SESSION_ID`, else `CODEX_SESSION_ID`, else `GROK_SESSION_ID`, else `human`.
- [x] Unit test over the act table: every act in the table, from every status, gives the status the architecture specification lists, or a refusal. For each case, the conditions `status` prints equal the conditions the command reports.
- [x] The Status, Threads and "Where records are read and written" sections of `docs/strategies/redesign-architecture.md` match what was built.

## Scope

`ready`, `new` and `start --new` belong to step B; fixtures write ready items as files. Missions do not exist yet, so `start` does not check a mission's status here. On the base, `status` lists ready and started items and nothing more yet. `review`, `verdict` and `merge` are the next two items.

## Context

Design: `docs/strategies/redesign-architecture.md`, Status, Threads and roles, Where records are read and written. Behaviour: `docs/strategies/redesign-product.md`, Lifecycle (including "When a thread dies") and Commands.

## Unknowns

- The wording of `status`. The first wording is a guess; the fresh-agent scenario at the end of step A is what tests it.

## Report

Every Done when line holds, each proved by a test seen failing under a mutation of the code it covers. `next/target/debug/atelier` is built by the new `atelier-cli`, and `git diff --stat master...HEAD -- crates/` is empty. In `next/crates/atelier-cli/tests/scenarios.rs`: `start_commits_building_then_creates_the_branch_and_status_says_what_is_next` (one commit on the base's tip changing `item.md` alone, then the branch; `status` text compared whole), `submit_refuses_with_the_status_list_and_then_commits_the_report_in_review` (the refusal's lines equal `status`'s; HEAD and the file unchanged; then one commit holding `in_review` and the Report, and the handoff line), `start_on_a_draft_changes_nothing_and_names_ready`, `another_thread_takes_over_a_started_item_in_any_started_status` (all four statuses), `of_two_threads_starting_one_item_from_two_worktrees_one_creates_the_branch` and `a_start_stopped_before_its_branch_leaves_the_item_ready` (both hold `start` between its commit and its branch through a debug-only hook; the second kills it there), and `status_on_the_base_reads_a_started_item_from_its_branch_until_it_merges`. In `next/crates/atelier-records/tests/at_commit.rs`: an older commit read while the checkout holds a newer record, a missing path, a directory and a symbolic link, a failing Git (not a repository; an unknown commit, after which the reader recovers), and a record that does not parse. In `atelier-app`: `identity::tests::takes_the_first_variable_set_in_order_else_human`, and in `tests/acts.rs` `every_act_in_the_table_from_every_status_sets_the_specified_status_or_is_refused` (against the specification's rows written out in the test) and `status_lists_the_conditions_the_command_reports` (every status, three kinds of caller, four sets of facts). The three sections of `docs/strategies/redesign-architecture.md` now describe what was built. `cargo fmt -- --check` and the 61 tests of `next/` pass, and the CI job's `next` steps pass as written in a fresh clone with `crates/` deleted.

### Copied code

- `next/crates/atelier-records/src/at_commit.rs`: `ObjectReader` and the tree walk from `ReadContext` in `crates/atelier-app/src/git_freshness.rs`. Trimmed: every cache, the pinning of `HEAD`, the working-tree reads, the history, diff and ancestry queries, and the Unix socket with `MSG_NOSIGNAL`/`SO_NOSIGPIPE` that stood in for the child's stdin (the new binary keeps Rust's default of ignoring SIGPIPE, so a write to a dead child fails with an error, and `main` ignores a closed stdout). Changed: stderr is captured so a failing Git says why, a path's mode is checked so a symbolic link is not read as a file, and errors are a `ReadError` that names the kind. The record loader on top (`item`, `item_ids`) is new.
- `next/crates/atelier-cli/src/git.rs`: `run` and `current_branch` from `text` and `checkout_branch` in `crates/atelier-cli/src/commands/git.rs`, trimmed to plain errors. The other Git calls there (the temporary-index commit, `update-ref` with an empty old value, `merge-base --is-ancestor`, `for-each-ref`, `switch`, `add`/`commit -- <path>`) are new.

### Transcript

The final binary in a scratch repository with one ready item, as `T3CODE_THREAD_ID=t1`:

```text
$ atelier status
You are thread t1, on the base branch main.
Ready to start:
  demo-ab12 (high): Say hello
Then: atelier start demo-ab12
[exit 0]
$ atelier start demo-ab12
Started demo-ab12 on branch demo-ab12.

You are the implementer of demo-ab12 (thread t1).
Item: .atelier/items/demo-ab12/item.md
Title: Say hello
Status: building
To submit it for review:
  [ ] Done when: `hello` prints hello.
  [ ] Report: written under `## Report`
Then: atelier submit
[exit 0]
$ atelier status
(the same seven lines from "You are the implementer")
[exit 0]
$ atelier submit
`atelier submit` refused; nothing changed.
To submit it for review:
  [ ] Done when: `hello` prints hello.
  [ ] Report: written under `## Report`
Then: atelier submit
[exit 1]
$ (tick the line and write the Report)
$ atelier submit
Submitted demo-ab12 for review.
Start a reviewer with: Run `atelier review demo-ab12` and follow what it prints.
[exit 0]
$ git log --oneline --graph
* 386efc3 Submit demo-ab12 for review
* e8ff3f1 Start demo-ab12
* a679d0c init
```

### Decisions

- `atelier-app` runs no Git. It holds the act table, identity, the rule for which record of an item counts (`Located::record`) and the status view; `atelier-cli` gathers the base's tip and each branch and runs every Git command; `atelier-records` holds the commit reader, since it owns reading records from a commit. Every line `status` and the refusals print is in `atelier-cli/src/output.rs`.
- The base's record is the one at the base branch's tip, never the base checkout's working tree, so `status` and `start` agree on it. On an item's own branch the record is read from the checkout, so ticks count before they are committed.
- `status` lists only unmet conditions, as the Done when line asks, where the product specification's example also shows a met one ticked. Ready items that can start come before blocked ones.
- A refusal goes to stderr with exit 1; a failure (Git, an unreadable record) goes to stderr with exit 2.

### Surprises

- `start`'s condition "the base's record is not dropped" needs no check of its own: a dropped record on the base already is the item's status, and `start` does not run from `dropped`. The specification now says so.
- A mutation check found the merged-branch rule stated twice (the loader skipped merged branches and `Located::record` ignored them). The loader now reads every branch and the rule has one owner.

### Open items

- `merge`, `review` and `verdict` do not exist yet, though `status` names them as next steps (`atelier review <id>` in review, `atelier merge` when approved).
- A `start` killed inside its temporary-index step can leave a stray `.git/atelier-<pid>.index`; it is harmless and nothing reads it.

## Notes

- 2026-11-10 [decision] The orchestrator reworded the act-table Done when line to 'every act in the table, from every status'. The table holds only the work-loop rows of this step: start, start taking over, submit, review, verdict changes, verdict approve and merge (with --trivial). Rows for ready, escalate, decide, close, drop, lead, start --new and settling a finding arrive with their commands; this item builds the conditions and commands of start and submit only
  Reason: A row with no command is dead data, and each later step adds its rows in a few lines
  Rejected: Every row of the architecture specification's table now
- 2026-11-10 [decision] Expected size: about 1,800 non-test lines of Rust across atelier-app and atelier-cli (the reader and its record loader in atelier-records), about as much again in tests, and about 60 changed lines of the architecture specification; escalate past about twice that
  Reason: The orchestrator's brief gives this estimate
