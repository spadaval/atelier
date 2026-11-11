---
created_at: "2026-11-11T03:21:50.136807966+00:00"
id: "atelier-psjw"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: atelier-psjw
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1oel"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Move the acts and repository access into atelier-app as functions that return data, make ready commit its own write, and name the human caller"
updated_at: "2026-11-11T05:04:13.882190290+00:00"
---

## Goal

Let a program that links the tracker's core, such as the Desk, perform an act by calling a function and get the result as data. Today the new tool's acts and its repository access live in the CLI crate (`next/crates/atelier-cli/src/plan.rs`, `questions.rs`, `repo.rs`, `git.rs`), so anything but the command line has to run the binary and read its text. `ready` also writes its record and prints a `git commit` line for its caller to run, and `start` carries a refusal for the case where that commit was never made.

Sumeet decided on 2026-11-11 that the Desk (mission atelier-z4cr, `docs/strategies/desk.md`, Design, Acts) calls the core's functions and runs no binary. This item is the tracker's half of that. It is done before the switch (atelier-u1ko) so the new skill is written once, for a `ready` that commits.

## Done when

- [x] **Repository access is in `atelier-app`.** The `Repo` type and the Git calls it and the acts below need move from `atelier-cli` to `atelier-app`, and every CLI command uses them from there. Each function of today's `git.rs` lives in one crate only, in `atelier-app` when `Repo`, `ready`, `drop`, `close`, `lead` or `decide` calls it. A repository is opened by path, with the caller's identity given by the caller: `atelier-app` reads no environment variable to find the directory, the caller or the skip reason. The CLI passes its working directory, the identity the identity rule gives, and `ATELIER_SKIP`. `atelier-app` gains only dependencies the `next/` workspace already has.
- [x] **`ready` and `decide` are functions of `atelier-app` with a typed result.** Each returns either what it did (the paths written and, where it committed, the commit) or a refusal: the rules it breaks, one line each, as `skip::refuse` receives them today, its text, and its next step as a field of its own where today's text ends with one. The CLI's `ready` and `decide` call these functions and print what they printed before, shown by their existing tests passing unchanged except where a line below changes behaviour. One test in `atelier-app` calls each function on a scratch repository opened by path, from a working directory outside it, and reads the result as data, for a success and for a refusal.
- [x] **One refusal and skip policy, in `atelier-app`, with no state shared between acts.** The code that decides whether a refusal is a rule and whether a reason passes it (today `skip.rs`, and `gate`, `gated`, `mission_gate` and `refusal` in `commands.rs`) moves to `atelier-app`, and the CLI commands that stay behind call it there, so there is still one place. The text of a refusal is produced in `atelier-app` for the acts that live there, by the functions that produce it today, moved and not copied; `output.rs` keeps the rest, and no refusal text exists in both crates. The process-wide list of passed rules (`skip::PASSED`) goes. In its place is one value per invocation, holding the reason and the rules it has passed. Its caller makes it: the CLI once per process, from `ATELIER_SKIP`, and a library caller once per act it calls. It needs no `Repo`, since `check` and the Git hooks pass rules before any repository is opened, with a config that may not read; and it outlives any one `Repo`, since `merge` opens several in one run and prints its skips after the last. Whatever opens a `Repo`, makes a commit or prints the final output is given that value, so a passed rule reaches the invocation's commit trailers and its output, and nothing reaches another invocation. Tests: in `atelier-app`, two acts called in one process, one with a reason that passes a rule and one without, on two repositories, where the second's result and commit carry nothing of the first; and the CLI's existing skip tests pass unchanged, those of `check`, of the hooks and of `merge` among them.
- [x] **One rule for who commits.** A planning command whose write other threads act on at the base's tip commits it: `ready`, `drop` and `close`, which change a stored status, and `lead`, which binds the role that `status`, a question's handoff and `log` read from the base's tip. `new`, `set`, and settling a finding (`new --from`, `drop` on a finding) still leave the commit to Git, because a draft is edited several times and a settlement on a gardener's branch is committed with its run. The four commands that commit share one piece of code for it.
- [x] **The commit holds only the paths the command wrote**, each as the file stands in the checkout, with the message the command used to print, and leaves every other staged, changed or untracked file as it was. The commands no longer print a commit line; they say the commit they made. Tests: with an unrelated file staged and another changed, `ready` on a mission makes one commit holding only `mission.md`, the first file is still staged and the second still changed; `ready` on a draft item no commit holds yet commits its whole `item.md`; `drop` of an item that exists only on its branch commits the base record it writes; `close` and `lead` each make one commit of their one path. When the commit fails, as when the pre-commit hook refuses it, the files and the index are as they were before the command, as `submit` already guarantees, and a test shows it for `ready`. A rule passed with `ATELIER_SKIP` is printed as today and is a trailer `Atelier-Skip: <rule>: <reason>` of the commit the command now makes, shown by a test of `ready` on a record that is not a draft.
- [x] **`start`'s refusal for planning not committed** (`not_committed` in `commands.rs`) is removed with its text, or kept with a test that shows a case it still catches now that `ready` commits. The Report says which and why.
- [x] **A caller can name the person acting.** The identity a caller passes when it opens a repository is the one records get: an answer `decide` writes for a caller named `sumeet` reads `Answer by sumeet:`, where a caller with no identity is `human` today, and `ready`'s `by` is a parameter of the function, which `ready --by` fills. No flag is added. A name the record format cannot read back unchanged is refused before anything is written, with a test.
- [x] **The specifications say what was built.** In `docs/strategies/redesign-product.md`: the rows for `ready`, `drop`, `close` and `lead`, and "Where records change", say which planning commands commit and which leave it to Git. Nothing in Words, Lifecycle or Review changes. In `docs/strategies/redesign-architecture.md`: the crate table and the sentences that say `atelier-app` runs no Git; "Where records are read and written" (planning commands, `lead`, and `start`'s uncommitted-planning refusal); in Status, the sentence that no act makes a commit only to change a status and the one that says which crate gathers `acts::Checkout`; The log, where it lists the acts whose commit Atelier makes and so can carry a skip's trailer; Hooks and the bypass, where it says which module every refusal goes through and which commands' commits get a trailer; and Threads and roles, for a caller that names a person. Any other passage of either document that the change makes untrue is corrected too and listed in the Report, outside the product specification's Words, Lifecycle and Review. The `last_changed` lines are left alone; the orchestrator writes them once.
- [x] **Nothing else changed behaviour.** `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo nextest run` pass in `next/`, each with `-j 4` where it builds. The Report lists every existing test whose expected output changed, each with the line above that changes it. It gives the non-test line count of each `next/` crate before and after (`scripts/count_non_test_rust.py`); the total should move little, since code moves and is not added.
- [x] **The cutover's footing holds.** `scripts/cutover/rehearse.sh`, changed only where it ran a commit after one of the four commands, passes on fresh clones of all five repositories with a binary built from this branch. The eval fixtures under `evals/agent-factory/scenarios/next-loop*` still set up: their `setup.sh` runs to its end with that binary, changed only in the same way.

## Scope

`next/crates/atelier-app`, `next/crates/atelier-cli`, their tests, the specification passages the Done when names or allows, `scripts/cutover/rehearse.sh` and the `next-loop` fixtures' `setup.sh` where a commit follows one of the four commands, and this item's record.

No new command, flag, record kind or config option, and no new crate. Not `merge`, `submit`, `review`, `verdict`, `escalate`, `start`'s own commit or the forge code: they stay in `atelier-cli` and behave as they do. `new` and `set` move only as far as sharing `Repo` requires. Not `crates/`, `skills/` or anything of the Desk (`docs/strategies/desk.md`, mission atelier-z4cr and its items), which another thread owns.

A reviewer's finding that asks for behaviour the specifications and this item do not name is escalated to the orchestrator, not built.

## Context

It advances the Desk's mission, atelier-z4cr, and is run by the orchestrator of the cutover mission, atelier-qr5s, because it touches code the cutover switches on. It starts after atelier-1oel merges, since both change how a caller's identity is found, and it merges before the switch branch (atelier-u1ko) is cut.

Specifications: `docs/strategies/redesign-product.md` (Commands, Where records change) and `docs/strategies/redesign-architecture.md` (Workspace and crates, Status, Threads and roles, Where records are read and written, Hooks and the bypass). Today `save` in `plan.rs` writes the records for `new`, `set`, `ready`, `drop` and `close` and ends with a `Then:` line giving the commit; `lead` does the same for `mission.md`. `submit`, the take-over and `verdict` already commit named paths and put the files and the index back when the commit is refused, which is the behaviour to reuse.

The reason for the commit rule: other threads read a status and a lead at a commit, never from the base checkout's working tree, so a `ready` that is written and not committed is invisible to `start`, and the refusal that explains it exists only because the command left its own work half done.

The new binary is run only from `next/target/` in scratch repositories and is installed nowhere.

Expected size: 1,000 to 1,500 lines move between the two crates, and about 200 are new or changed outside tests.

## Unknowns

- Whether a planning act could commit straight onto the base branch through Git objects, from any directory, as `start` builds its commit. That would remove the base-checkout rule for `ready`. It is a separate question and not this item's to answer.

## Report

Every Done when line holds. A program now opens a repository by path with `atelier_app::Repo::open(dir, caller, &skip)` and calls `atelier_app::ready` and `atelier_app::decide`, which return `Done { text, paths, commit }` or `Refused { rules, text, next }`; `atelier-app`'s `tests/library.rs` shows it from a working directory outside the repository (`ready_returns_what_it_wrote_and_committed_or_its_refusal`, `decide_writes_the_answer_by_the_caller_named_or_returns_its_refusal`: pass). The skip is a `Skip` per invocation, and `a_skip_reaches_its_own_acts_commit_and_result_and_no_other_acts` shows two acts in one process keep theirs apart (pass). `ready`, `drop`, `close` and `lead` commit through one function, `atelier_app::plan::save` with `commit`, which uses the `write_and_commit` that `submit`, the take-over and `verdict` use; `planning::ready_commits_only_what_it_wrote_and_leaves_every_other_change_as_it_was` and `planning::a_refused_commit_leaves_the_files_and_the_index_as_they_were_before_ready` show the commit's contents and the rollback, `skip::ready_on_a_record_that_is_not_a_draft_passes_with_a_skip_trailed_in_its_commit` the trailer, and `a_caller_the_records_cannot_name_and_read_back_is_refused_before_anything_is_written` the name check (all pass). `start`'s refusal for planning not committed is removed with its text (`not_committed_ready`) and its test: `ready` commits on the base checkout's branch, so what it readies is ready at the tip at once, and the refusal could only catch a status hand-edited into front matter, which agents never do. In `next/`, `cargo fmt -- --check`, `cargo clippy -j 4 --all-targets -- -D warnings` and `cargo nextest run -j 4` pass: 247 tests, 0 failed (238 before the new ones; one old test was deleted). `scripts/cutover/rehearse.sh` passed on fresh clones of all five repositories, and both `next-loop` fixtures' `setup.sh` ran to their end, each with the debug binary built from this branch.

### What moved, and what stayed

- To `atelier-app`: `repo.rs` whole (`Repo`, `Links`, `planning`, `is_planning`, the path helpers), with `facts` and `write_and_commit` from `commands.rs`; from `git.rs`, every call `Repo`, the acts here and the commit make (`root`, `current_branch`, `branch_tip`, `branches`, `has_commit`, `is_ancestor`, `head`, `uncommitted`, `untracked`, `changed`, `changed_since`, `checkout_of`, `commit_paths`, `with_skips`, `relayed`) and the helpers the rest are built on (`git`, `output`, `run`, `run_with`, `listing`, `names`), as `atelier_app::git`; `skip.rs`, with `gate`, `gated`, `mission_gate`, `refusal` and `refused` from `commands.rs`, as `Skip`'s methods and `atelier_app::refused`; from `plan.rs`, `ready`, `plan_reviewed`, `plan_changed`, `planned_facts`, `is_mission`, `off_base`, `save` and `save_with`; from `questions.rs`, `decide`, `decider` and `one_line`; from `output.rs`, the 42 functions that word a refusal of any act, what an act needs, and the results and refusals of `ready` and `decide`, as `atelier_app::say`, which `output.rs` re-exports.
- Stayed in `atelier-cli`: `remote_tip`, `commit_file`, `create_branch`, `switch`, `hooks_dir`, `common_dir`, `files_at`, `staged_files`, `staged_changes`, `first_parent_line`, `merge_tree` and `approved_at` in `git.rs`; `new`, `set`, `drop`, `close` and settling a finding in `plan.rs`; `escalate` in `questions.rs`; `status`, `start`, `submit` in `commands.rs`; `lead` in `views.rs`; all of `merge.rs`, `review.rs`, `check.rs`, `forge.rs`, `setup.rs`; and the rest of `output.rs`. Reading `ATELIER_SKIP` and the identity variables is in `main.rs`.

### Non-test lines (`scripts/count_non_test_rust.py`)

| Crate | 399fca6a | 98f9f9a3 |
| --- | --- | --- |
| atelier-app | 1,720 | 3,971 |
| atelier-cli | 7,581 | 5,678 |
| atelier-core | 494 | 494 |
| atelier-records | 1,667 | 1,667 |
| Total | 11,462 | 11,810 |

The total grew by 348, 32 of them round 1's fixes. About 2,000 lines moved; what is new is the `Refused`, `Done` and `Skip` types and their conversions, the refusal producers that now return their way out apart, `save`'s split into a checked and a committing path, `write_and_commit` handling a file no commit held, the identity check and the CLI's helpers that open a repository and read the variables. Removed: `start`'s uncommitted-planning refusal and its text.

### Existing tests whose expected output changed

All by "One rule for who commits" and "The commit holds only the paths the command wrote", unless said otherwise.

- `planning::ready_marks_each_named_draft_with_who_and_when_and_changes_no_other_item`: the checkout is clean after `ready`, and each `ready` makes its commit, where it ran the printed line.
- `planning::status_on_the_base_lists_planning_not_yet_committed`: `ready` commits the item, so only `new`'s mission is listed as not committed and the item is ready to start.
- `planning::start_says_when_an_item_or_its_mission_is_ready_in_the_base_checkout_and_not_committed`: deleted, with the refusal ("`start`'s refusal for planning not committed").
- `planning::the_printed_commit_works_from_a_subdirectory_and_with_paths_a_shell_would_split`: its second half took the commit line from that refusal; it now takes `status`'s from a directory under a base checkout at a path with a space, and checks the same `git -C` and quoting.
- `planning::drop_puts_its_reason_first_in_the_report_and_needs_a_reason_and_a_record_not_done`: the output says `Committed` where it said `Wrote`.
- `planning::start_is_refused_until_the_items_mission_is_ready_and_names_ready`, `planning::set_is_refused_with_nothing_written_for_a_field_or_record_it_cannot_change`, `planning::a_started_item_dropped_on_the_base_is_refused_everywhere_with_its_reason`, `planning::a_terminal_status_on_the_base_wins_over_a_branch_record_that_does_not_parse`, `hooks::planning_commits_reach_the_base_and_a_push_of_several_refs_judges_only_the_base`, `views::list_shows_open_work_by_mission_and_label_and_each_flag_narrows_it`: they no longer commit by hand after `ready`, `drop` or `lead`.
- `planning`'s helpers: `plan` runs the printed commit line only after `new` and `set`, and `wrote` reads the paths under `Committed` too, which `a_mission_goes_from_draft_through_its_items_to_closed`, `a_dropped_mission_changes_no_item_and_start_on_one_says_it_was_dropped`, `close_ends_an_item_with_no_branch_with_its_result_and_names_merge_for_a_started_one` and `an_item_only_on_its_branch_is_dropped_with_its_definition_and_not_its_reviews` use; the last two also check the commit now.
- `views::lead_takes_a_ready_mission_and_hands_each_item_that_can_start_to_another_thread`: `lead` ends with the commit it made and leaves the checkout clean; the step that showed an item made ready and not committed is not handed out is gone, since `ready` commits.

### Specification passages changed

`redesign-product.md`: the rows for `ready`, `drop`, `close` and `lead`, and "Where records change". `redesign-architecture.md`: the crate table; Status, on which acts commit only a status and on what gathers `acts::Checkout`; Threads and roles, a paragraph on a caller that names a person; Where records are read and written, the base's record (the refusal), planning, Who decides what (no longer "runs no Git"), Who reads this way (`lead`), and also A refused commit changes nothing, which now names the four commands; The log; Hooks and the bypass, One place and the `Skip` value; and also The forge's last sentence, which said `atelier-app` runs no Git.

### Deviations

- `atelier --help` changed for `ready`, `drop`, `close` and `lead`, which said "without committing it".
- `Refused::next` is filled for the refusals `ready` and `decide` give. A refusal of another act's unmet conditions, or of a role, keeps its way out in `text`, since no caller reads it as data yet.

### Open items

None. The rehearsal (`all steps passed in 36 s; 14 gh calls, all to the double`, 40 steps ok over atelier, astromech, demiurge, life-on-rails and cosplay-studio) and both fixtures' `setup.sh` were run again on the binary built from 98f9f9a3, after round 1's fixes.

### Review round 1

- atelier-wwxe (fix now): `ready` given no ID reached `git commit --` with no path, which commits whatever is staged. Fixed in 91bd5db3: `ready` refuses an empty list, and `git::commit_paths` refuses no path. Test: `library::ready_given_no_id_refuses_and_commits_nothing`.
- atelier-t53b (fix now): the hooks a commit ran read the process's `ATELIER_SKIP`, not the library caller's `Skip`. Fixed in 98f9f9a3: `git::relayed`, which runs the commit and `merge`'s and `submit`'s push, sets the variable from the `Skip`, or clears it. Test: `library::the_hooks_a_commit_runs_are_given_the_invocations_reason_and_not_the_processs`, seen failing without the fix.

## Notes

- 2026-11-11 [decision] drop, close and lead stay in atelier-cli; only their commit (plan::save with commit) and their refusal policy are in atelier-app
  Reason: The item asks only ready and decide to be functions; the Desk calls only those two, and moving three more commands would grow the move without a caller
  Rejected: Move drop, close and lead into atelier-app as functions too
- 2026-11-11 [decision] A refusal's way out is its own field (Refused::next) for the refusals ready and decide give; for other acts, and for a refusal of unmet conditions, the text carries it as before and next is None
  Reason: The Done when asks the field where today's text ends with one, for the acts that live in atelier-app; the producers of those refusals now return text and way out apart, and the CLI prints text, way out and skip offer as before
  Rejected: Split every refusal's last line by a rule, which misreads texts that end in a fact rather than a step
- 2026-11-11 [decision] The skip is a Skip value with interior mutability, which Repo borrows (Repo<'s> holds &'s Skip); every gate is a method of Skip
  Reason: Facts borrow the Repo while a gate records a passed rule; a shared borrow with a RefCell lets both happen without changing every call that builds Facts, and the Skip outlives each Repo as merge needs
  Rejected: Repo owning the Skip and handing it on, which needs moving it between merge's Repos
- 2026-11-11 [decision] start's refusal for planning not committed (not_committed, not_committed_ready) is removed with its text and its test
  Reason: ready now commits its write on the base checkout's branch, so an item or mission it readies is ready at the tip at once; the refusal could only catch a status hand-edited into front matter, which agents never do, or a commit that failed, after which ready puts the file back
- 2026-11-11 [decision] Repo::open refuses a caller identity with a line break or a ': ', the two things that stop Answer by <thread>: and Dropped by <thread>: reading back; Repo keeps the directory it was opened from, and git_at compares it with the root instead of reading the process's working directory
  Reason: One check at the one entry point is before anything is written; and atelier-app then reads no process state to decide where a printed Git command runs
