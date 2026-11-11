---
schema: atelier.item
schema_version: 3
id: atelier-ivbq
title: 'Fix what the new skill and the eval fixtures showed status gets wrong: six small gaps in what a thread is told'
status: in_review
mission: atelier-qr5s
labels:
- redesign
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3Ae81e5c93-fcde-468f-a99a-334005140e39
ready:
  by: sumeet
  at: 2026-11-11T15:43:38.001833021Z
trivial: null
caused_by: null
created_at: 2026-11-11T06:17:49.741506717Z
---

## Goal

Close the gaps in what the new binary tells a thread that writing the skill, the eval fixtures and the eval runs each ran into. None is a new rule: each is `status` or a handoff saying less, or something other, than the specifications' own commands allow, and each sent a fresh agent the wrong way at least once.

## Done when

- [x] In a Git checkout with no `.atelier/`, `status` says this is not an Atelier repository and exits as it does today for that case, where it now says only that it cannot read the repository's Atelier config. A checkout whose config exists and does not parse still gets the parse error.
- [x] A thread that holds nothing on an item with an unanswered question is told of the question and of `atelier decide`, before any offer to take the item over. A thread that leads the item's mission is not offered the take-over as its next step on that item's branch.
- [x] After three rounds without an approval, the line that names `atelier escalate` includes `--finding <name>` when a kept finding is open, and is not shown once a question about it has been asked.
- [x] `status` shows the whole of a Done when line that wraps over several lines of the record, not its first line.
- [x] The handoff line a Codex thread reads tells it to start the other thread itself, without waiting to be asked. The line still has no backticks, and is still true under Claude Code.
- [x] Each line above has a test that fails before the change, and `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo nextest run` pass, each with `-j 4` where it builds. The product page's `status` example and Handoff lines passage, and the design record's Threads and roles, match what the binary now prints.

## Scope

`crates/atelier-app`, `crates/atelier-cli`, their tests, the passages of the two specification pages named above (wherever atelier-6w8d has put them by then), and this item's record. No new command, flag, record field or rule. Not the eval harness (atelier-dytk).

Left out on purpose, each for whoever plans it: thread identity inside Codex's session-start hook, which needs `status` to read the hook's input and is more than wording; `atelier log` printing a `<parent>/<child>` identity whole, which is for the item that builds `log` (atelier-8b0a); and two things that would be new rules and are Sumeet's to decide: a lead can `start` an item of its own mission, and `check` accepts a status set to `done` by hand because `close` makes the same change.

## Context

Mission atelier-qr5s, step H ("fix what first use breaks"). Where each was seen: the Reports of atelier-orb2, atelier-1oel, atelier-a4rs and atelier-t747, which Git keeps (`git log --all --oneline -- .atelier/issues/atelier-a4rs.md`), and the results file `evals/agent-factory/results/2026-11-11-next-skill.md`.

This host runs a service its owner depends on: one cargo command at a time, with `-j 4`; set `TMPDIR` to a directory under `$HOME` outside any repository, since `/tmp` is a small quota'd tmpfs.

Expected size: about 120 lines of Rust outside tests.

## Report

All six Done when lines hold. Round 1: 1d08908bd (code and tests) and aa81e3641 (the two specification pages). Round 2: 6b5ec644d (both findings and the lead's handoff wording) and ec95db509 and 2c3e8c508 (the specifications). At 2c3e8c508, `cargo fmt -- --check` and `cargo clippy -j 4 --all-targets -- -D warnings` are clean, and `cargo nextest run -j 4` passes 292 of 292.

### What changed, by line

- **Not an Atelier repository** (`atelier-app/src/repo.rs`, `Repo::opened`). A checkout with no `.atelier/` now gets `atelier: this is not an Atelier repository: <root> has no .atelier/ directory`, exit 2 as before. A config that is there and does not parse still gets `cannot read this repository's Atelier config: ...` with the parse error. The check is skipped for `Repo::open_committed`, so the Desk's commit-only reads are unchanged. `check` is a separate path and still reports the missing `config.toml`. Test: `status_outside_an_atelier_repository_says_so_and_a_config_that_does_not_parse_says_why`.
- **A visitor's question, and the lead** (`say::visiting`, called from `output::item`; `commands.rs` works out whether the caller may decide and whether it leads the item's mission). On the item's branch, a thread holding nothing, including a reviewer whose review is done, gets `A question waits on a decider: <question>` for each one. If it may decide, it then gets `Then: atelier decide <id> "<answer>"`; if not, it is told who may. The take-over offer comes after both. A done reviewer that may decide gets no "stop" line; one that may not still gets it. The mission's lead gets `You lead mission <m>, which <id> is in: its implementer builds it, so you do not take it over.` and no offer, with a question waiting or not. Tests: `on_an_items_branch_a_thread_holding_nothing_is_told_of_its_question_before_the_take_over` (a thread that may decide, a former implementer that may not, and the lead before and after it answers), and `a_reviewer_whose_review_is_done_is_told_of_a_question_it_may_decide` (a question about no finding, then one about the reviewer's own finding).
- **Escalation line** (`view.rs`, `Entry::unapproved` now carries the finding; `output::escalate`). With changes requested after three rounds, the line reads `atelier escalate --finding <name> "<question>"` for the first open fix-now finding with no question about it, in its own round or any round it was carried from. Once each open finding has a question, answered or not, the line is gone. In review the line is unchanged, since which finding stands waits for the verdict. Tests: `status_shows_the_round_and_after_three_without_an_approval_says_to_escalate` (now expects `--finding`) and `the_escalation_line_goes_once_a_question_is_asked_about_the_kept_finding` (asked, then answered "fix it", on the branch and away from it).
- **Wrapped Done when lines** (`atelier-records/src/body.rs`, `done_when`). A line directly under a check is joined to its text with a space, unless it is blank, a list line, a heading (one to six `#` then a space or nothing, as in `document.rs`, so `#123` is text), or comes after a fence. Tests: `a_done_when_line_wrapped_over_several_lines_is_read_whole` (unit) and `status_shows_a_done_when_line_wrapped_over_several_lines_whole`, both with a wrapped line that begins with an issue reference. The existing unit test's `- [ ] One` plus `wrapped` now reads `One wrapped`.
- **Handoff line** (`say::handoff`). It now reads: "<Who> must be a thread other than yours, and starting it is your job: it is part of the task you were given, even as another thread's child or subagent, and nobody else will start it. So start one yourself now, without waiting to be asked. Under Codex a subagent you start is one; under Claude Code a subagent runs as your thread and is refused, so use your harness's tool for a new agent thread. Give it fresh context, not a fork of yours, and give it this task:". It has no backticks and is true under both harnesses. In round 2, at the lead's word, it says starting the thread is the reader's own job even as a child, because a Claude Code thread that T3 Code had started as a child stopped at the old handoff for its lead, in the first real item under the new tool. Test: unit `a_handoff_says_to_start_the_thread_unasked_and_holds_no_backtick`, which checks each of those three phrases. The four CLI tests that pin the whole line now share `START_ONE` in `tests/common/mod.rs`.
- **Specifications.** In `docs/strategies/redesign-product.md`, the paragraph under the `status` example now covers wrapped lines, `--finding`, the visitor and the lead, and the not-an-Atelier-repository case. The Handoff lines passage quotes the new line. The example itself, at round 2, prints no escalation line and still matches. In `docs/strategies/redesign-architecture.md`, Threads and roles says the same.

### Tests fail before the change

With master's `crates/*/src` checked out over the new tests (then restored), these eight failed: the outside-Atelier, wrapped-line, visitor and both escalation tests, plus three pinned handoff tests (`submit_refuses_with_the_status_list_and_then_commits_the_report_in_review`, `an_escalation_decided_as_defer_is_carried_as_decided_and_the_item_merges`, `lead_takes_a_ready_mission_and_hands_each_item_that_can_start_to_another_thread`). The two unit tests live in `src`, so they were checked by reading: the old `done_when` returns only a check's first line, and the old handoff has no "without waiting to be asked".

Round 2's two new regressions, `a_reviewer_whose_review_is_done_is_told_of_a_question_it_may_decide` and `status_shows_a_done_when_line_wrapped_over_several_lines_whole`, both fail with 122c0f5a7's `crates/*/src` checked out over them.

### Deviations

- `crates/atelier-records/src/body.rs` is outside the stated Scope. The Done when parser lives there, and fixing it there also fixes the Desk, which reads the same list.
- About 140 lines of Rust outside tests, against about 120 expected.

### Responses

- 1-o9m9.1: fixed in 6b5ec644d
- 1-o9m9.2: fixed in 6b5ec644d
