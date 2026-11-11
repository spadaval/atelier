---
created_at: "2026-11-10T21:30:48.869299317+00:00"
id: "atelier-zrss"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-zrss
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-axfa"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-pz1y"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Add the `ATELIER_SKIP` bypass to every rule of a command or a hook"
updated_at: "2026-11-10T23:42:39.469755839+00:00"
---

## Goal

Give every rule one intentional way past it. Atelier's rules are nudges for trusted agents, not hard blocks: when a rule is wrong for the case in hand, an agent sets `ATELIER_SKIP="<reason>"` for one command, the tool says which rule it let pass and why, and where Atelier makes the commit the reason stays in history.

## Done when

- [x] With `ATELIER_SKIP` set to a non-empty reason, a command goes ahead where a rule would have refused it. A rule is any refusal that depends on the state of the work or on who is asking: a role, a status or a condition in the act table; a merge check that fails or a CI run that failed or has not finished; a planning command's guard, such as running off the base or changing a started item; a record that parses and fails validation; and a rule of a Git hook. The tool prints each rule it let pass and the reason. Tests: `merge` on an item with no approval; `merge` with a merge check that fails; `merge` after the forge reported that CI failed; a commit of a record that parses and fails validation; `submit` with an unticked Done when line; `start` on a draft item; `set` on a started item; a commit of a status change the act table does not list; and a push of the base holding a plain commit that touches code.
- [x] Where Atelier makes the commit (`start`, `submit`, `verdict`, `merge`), its message gains one trailer for each rule passed, `Atelier-Skip: <rule>: <reason>`. A skip in a hook, or on a command that makes no commit, writes nothing.
- [x] `ATELIER_SKIP` set to an empty or blank value is refused by every command and by the pre-commit and pre-push hooks, with a line saying a reason is required. The post-checkout hook has no rule to pass: it prints that line and still exits 0, so the checkout is never failed.
- [x] What is not a rule cannot be skipped, and the refusal says so: the tool has nothing to act on, or an operation it ran failed. That covers a record that does not parse, an ID or a finding that does not exist, a missing or malformed argument, a merge that conflicts, inputs that moved while `merge` ran, and a Git or forge command that failed. A test sets the variable against each.
- [x] Every refusal of a rule ends with the way out first and the skip second, from one place in the code, so the two never disagree. Refusals that cannot be skipped do not mention it.
- [x] The pre-commit rule asks the act table for a row of the record's own kind. atelier-pz1y's rule asks only whether any row goes from the old status to the new, so a mission changed by hand from `draft` to `done` passes on the strength of an item's `close` row. A test commits that change and sees it refused, and commits a mission closed from `ready` and sees it allowed.
- [x] Fresh agents do not reach for the skip when the way out is open: the `next-loop` grader fails a run in which `ATELIER_SKIP` was set, and one run on Claude Opus and one on Codex pass it with the new refusal wording, recorded in the scenario's results file.
- [x] `docs/strategies/redesign-architecture.md` matches what was built, including the list of what is and is not a rule.

## Scope

`ATELIER_SKIP` in every command and hook of `next/`. No flag and no config option: the variable is the whole interface. Counting skips is the scorecard's, from transcripts. Nothing is copied from the old workspace. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, "Hooks and the bypass". Behaviour: `docs/strategies/redesign-product.md`, Principle 6 and "Hooks and the bypass". The act table is in `atelier-app`'s `acts.rs`, and the refusals for a role, a status or a condition come from it. Other rules are refused elsewhere: the merge checks and CI in `atelier-cli`'s `merge.rs`, the planning guards in `plan.rs`, validation in `atelier-records`, and the hook rules in `check`. The skip has to reach each of those places, and the one place the item asks for is where a refusal is turned into output, so that a rule cannot be refused without offering the skip. The hooks are atelier-pz1y's, and `escalate` and `decide` are atelier-axfa's; this item follows both so that it covers every command. The fresh-agent scenario is `evals/agent-factory/scenarios/next-loop/`.

This is the third of step D's three items in `docs/strategies/redesign-2-core.md`.

## Report

All eight Done when lines hold. `ATELIER_SKIP` set to a reason passes every rule one command or hook meets, prints `Skipped with ATELIER_SKIP: <rule>: <reason>` for each, and where Atelier commits (`start` and a take-over, `submit`, `verdict`, `merge`) adds `Atelier-Skip: <rule>: <reason>` trailers through `git interpret-trailers`. Tests, all in `next/` and passing (222 tests, `cargo nextest run`): `skip::a_skipped_rule_is_printed_and_trailed_in_the_commit_of_start_submit_verdict_and_merge` (start on a draft, submit unticked, `verdict changes` with no finding, merge with no approval, each trailer); `skip::set_with_a_skip_changes_a_started_items_base_record`; `skip::a_command_with_nothing_to_skip_runs_as_without_the_variable`; `skip::a_blank_skip_is_refused_by_every_command`; `skip::what_is_not_a_rule_is_refused_with_the_variable_set_and_says_so` (unknown ID, unknown finding, missing and malformed arguments, a command off its item's branch, a record that does not parse, a failed `git switch`); `merge::with_a_skip_a_failing_merge_check_or_uncommitted_change_does_not_stop_the_merge` (a check of two lines, both trailers read by Git); `merge::with_a_skip_ci_that_failed_or_has_not_finished_passes_and_a_failing_forge_does_not`; `merge::with_a_skip_a_conflict_or_an_input_that_moves_still_stops_the_merge` (the base moving, and a file or a symbolic link a skip let stay uncommitted changing during the checks); `merge::a_rule_the_pre_push_hook_skips_in_merges_push_of_the_base_is_printed`; `hooks::a_rule_a_hook_skips_in_an_atelier_commit_is_printed`; `hooks::a_commit_of_an_invalid_record_or_of_a_status_no_act_sets_is_refused_and_a_hand_edit_goes_through` (invalid record and unlisted status skipped; a record that does not parse, alone and beside a rule, not); `hooks::a_push_of_the_base_with_a_plain_commit_of_code_or_config_is_refused` (skipped from a second worktree); `hooks::a_branch_switch_prints_status_and_succeeds_whatever_it_finds` (blank on switch, commit and push); `hooks::a_status_change_is_judged_by_the_acts_of_its_records_kind` (mission draft to done refused, ready to done allowed); `hooks::check_reports_every_problem_with_its_file_and_field_and_lists_uncommitted_records` (`check` skipped). Each new test was seen failing: with `skip::pass` disabled and `acts::changes` ignoring the kind, 8 failed; with `gate` treating everything as a rule and the "cannot be skipped" line removed, 9 failed. The grader's two new tests failed against master's `grade.py`. Fresh-agent runs on binary 06783d8, recorded in `evals/agent-factory/results/2026-11-10-next-loop.md`: Claude Opus 5.5 run 0b85 and Codex `gpt-6.1-sol` run ded7 both pass, 11 calls each, 0 refused, 0 errored, no skip.

### How it is built

- **One place:** `atelier-cli/src/skip.rs`. `skip::refuse(rules, text)` is the only way a rule refuses: without a reason it returns the refusal with `output::skip_offer()` appended after the text's way out; with one it records each rule and returns nothing, so the command goes on. `main`'s `exit` prints the recorded lines first, and adds `output::not_skippable()` to any refusal or error while a reason is set; since every rule went through `refuse`, those are all non-rules. `git::commit_file` and `git::commit_paths` add the trailers, so every Atelier commit carries them; each rule and the reason are made one line first. Atelier's own `git commit` and `git push` pass on what Git printed (`git::relayed`), so a rule a hook skips inside `submit` or `merge` is printed.
- **All or nothing for an act:** `acts::refusals` returns every refusal of an act (`check` still returns the first, and still says a role line before reading findings that may not parse). `commands::gate` offers and passes the skip only when every one is a rule (`Refusal::is_rule`, `Condition::is_rule`).
- **Pre-commit by kind:** `acts::changes(mission, from, to)` asks only the rows of the record's kind; `drop` is both.
- **Merge:** CI that failed is refused through `refuse`; CI not finished passes at once under a skip, with no wait; a forge that does not answer stays "not yet". A failing merge check is passed and the merge commit is written again so its message names it. `moved` now compares what the item checkout holds uncommitted (paths, index entries, modes and contents, `merge::Dirt`) with what was read, so the uncommitted-changes rule can be skipped without `merge` then refusing as "changed while its checks ran".
- A verdict's review-file validation problem is now `Condition::Invalid`, a refusal (exit 1) rather than an error (exit 2), since a record that reads and fails validation is a rule.

### Every refusal, sorted

Rules (skippable): the act table's role, status, holder (`NotHolder` of an implementer's act) and conditions, through `commands::refusal`, `gated`, `gate` and `mission_gate`, for `start`/take-over, `submit`, `review` (the author), `verdict`, `escalate`, `decide` (former implementer, reviewer of the finding), `merge` (its role check in `read` too), `ready`, `drop`, `close` (item, including `close_started`, and mission), `lead`, both plan verdicts, and settling a finding (status, not deferred); a verdict's invalid review file; `merge`'s CI failed, CI not finished and failing merge check; `off_base` (`new`, `set`, `ready`, `drop`, `close`, `lead`) and `off_settling`; `set` on a record that is not draft or ready (`unplanned`); `ready` on a non-draft; `save`'s validation; `check`'s invalid records; pre-commit's invalid records and unlisted statuses; pre-push's code commits and done-outside-a-merge commits.

Not rules (never skipped; say so when the variable is set): a malformed ID (`validate_id`), an unknown item, mission or finding (`no_item`, `set_unknown`, `unknown_refused`, `no_mission`, `lead_unknown`, `show`'s unknown, `NoFinding`), a finding settled already (`Settled`), `not_a_finding`, `item_only`, a missing or one-line-breaking argument (`no_drop_reason`, `no_result`, `close_mission_text`, `not_one_line`, `no_reason`, clap's usage errors), `escalate --finding` on a finding not open (`not_open`), `decide --defer` with no open finding (`not_settled`), no question to decide (`NoQuestion`), a command off the item's branch it writes (`not_on_item`, `elsewhere`), `review` of a plan on an item's branch, a verdict with no review file of the caller's (`NotHolder` of a reviewer, `no_review_held`), `start` where branch `<id>` exists (`BranchExists` for `start` only; for `close` it is a rule), a take-over with no branch (`NoBranch`), `lost_race`, a merge conflict, a dirty base checkout (`base_dirty`), inputs that moved (`moved`), a forge or push that failed (`forge_failed`, `branch_push_failed`, `base_push_failed`, the forge not answering), records that do not read (`unread`, `push_no_config`, every parse error), the clone's own kept hook failing, `setup`'s `hooks_elsewhere`, `hooks_taken` and `harness_refused`, and every Git error (exit 2).

### Decisions

- `base_dirty` is not a rule: merging moves that checkout, which holds someone's uncommitted work.
- `setup`'s three refusals are not rules: two would overwrite hooks Atelier did not write, one is a settings file that does not read.
- `merge`'s "not yet" offers no skip: it is not a refusal, and running again is the way. A skip set for another reason still passes an unfinished CI run without waiting.
- `set` with a skip on a started item writes the base's record, not the branch's, since planning is the base's.
- The unread-records header no longer says "cannot be skipped" without the variable ("These do not read as records, so no rule can be checked on them:"); line 5 says non-rules do not mention the skip.

### Surprises

- Git shows the pre-commit hook's output on stderr and the pre-push hook's on stdout.
- A reviewer whose draft finding does not parse is refused `submit` by role with no skip offered, because `submit`'s conditions read that finding: the refusal set holds a non-rule.

### Open items

- The skip passes every rule a command meets, so a skip given for one rule (no approval) also passes another the agent was not shown (CI still running). Every rule passed is printed and trailed; the specifications ask for this coarse interface.
- With two skips an implementer can review its own item (`review` as the author, then `verdict` across roles). Both are rules by the item's definition; the trailers record it.
- Size: non-test Rust in `next/` is 11,060 against master's 10,536 (+524, about 1.5 times the 350 expected). The specification grew about 60 lines against about 30 expected, for the two lists.
- `scripts/smoke-repos.sh` builds the old workspace and was not run; nothing under `crates/` changed.

### Review

- Round 1 (gpt-6.1-sol): three `fix_now` findings, each fixed in its own commit with a test seen failing first: atelier-4njn (a merge check of several lines broke the trailers; 68eab9dc), atelier-2xgh (a dirty file changed during the checks was not seen; 39c4682b), atelier-4em4 (a hook's skip inside `submit` was not printed; 10bba3ee, which also relays `merge`'s and `submit`'s pushes, since the pre-push hook can skip there too).
- Round 2 (gpt-6.1-sol): one `fix_now`, atelier-crry: the snapshot of uncommitted files followed symbolic links and read errors as empty. Fixed with a test seen failing first: a link's target is read with `read_link`, a missing file is `None`, and other read errors stop `merge`.
