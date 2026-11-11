---
created_at: "2026-11-11T02:50:24.589875035+00:00"
id: "atelier-1oel"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-1oel
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
  - kind: "issue"
    id: "atelier-t6o6"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Give a Codex subagent its own identity, show the round and the three-round rule in status, and name the pull request in the review brief"
updated_at: "2026-11-11T03:08:00.211457711+00:00"
---

## Goal

Build three small changes to the new binary in `next/` and bring the two specifications into line with them. Each is Sumeet's answer of 2026-11-11 to a point of atelier-t6o6, which holds the answers in its Report.

1. **A Codex subagent gets an identity of its own (point 5).** Today a child a Codex thread starts with `spawn_agent` is seen as its parent, so `review` refuses it as the item's author. Sumeet's work host runs Codex only, and T3's guidance there is to start a same-provider child with the native tool, so today's rule would refuse every reviewer on that host.
2. **`status` shows the round and the three-round rule (point 1).** The Review section's three rules that the binary does not enforce (a rebuttal's one try, escalation after three rounds, a fresh reviewer for a rebuttal) stay with the skill, and the section is reworded to say so. `status` already shows the one-try rule; nothing shows how many rounds an item has had.
3. **The review brief names the pull request (point 3).** The Review section promises "the latest merge check results" in the brief, and nothing records a check's result. The promise is dropped, and the brief names the branch's pull request, where CI's result can be read.

## Done when

- [x] `next/crates/atelier-app/src/identity.rs`: when `CODEX_THREAD_ID` and `CODEX_SESSION_ID` are both set to non-empty values that differ, the caller is a Codex subagent, and its identity is the identity today's rule gives (its parent's), then `/`, then its `CODEX_THREAD_ID`. In every other case the identity is what today's rule gives, so a root thread under T3 Code keeps its T3 thread ID and a Claude Code subagent is still its parent. Unit tests cover: the two values measured on this host on 2026-11-11 (`/home/supadava/.cache/atelier-identity-probe/root.txt` and `child.txt`, a root whose two variables are equal and a child whose `CODEX_THREAD_ID` differs, both with one inherited `T3CODE_THREAD_ID`); a Codex subagent outside T3 Code; `CODEX_THREAD_ID` set with no `CODEX_SESSION_ID`, which changes nothing; and an empty value, which counts as unset.
- [x] A scenario test drives the real binary with those variables: a thread starts and submits an item; a caller with the same parent variables and its own `CODEX_THREAD_ID` runs `review` and `verdict approve` and is accepted, and the review file's `reviewers` holds `<parent>/<its thread>`; the parent itself is still refused as reviewer; and the subagent is refused `submit` and `merge` on the item, since it is not its implementer. `log` and `show` print the identity whole.
- [x] Nothing treats an identity as a path or splits it on `/`. The Report lists each place an identity is parsed out of a line (`Answer by <thread>:`, `Taken: <item> by <thread>`, `Dropped by <thread>:`, the front matter's `implementers`, `reviewers` and `lead`) with the test that reads an identity containing `/` back unchanged through it, and says how the search for other uses was made.
- [x] The handoff line `submit`, `escalate` and `lead` print, and the refusal of an author's `review` and `decide`, are true under both harnesses: a subagent of a Claude Code thread runs as that thread and is refused, and a subagent of a Codex thread is a thread of its own and may review or decide. The line still has no backticks. It tells the reader to start the reviewer with fresh context, not as a fork of its own.
- [x] One real Codex session proves the rule end to end, in a scratch repository with the binary built from this branch: a `codex exec` root thread starts an item, makes a small change and submits; a child it starts with `spawn_agent` runs `atelier review` and `atelier verdict approve` and is accepted; the root merges. The Report gives the identities the records hold, and says what `spawn_agent` does by default, a fresh context or a fork of its parent's, and how a caller asks for a fresh one if that is not the default. This costs one or two Codex runs on a small model; no more are run.
- [x] `status` shows the round: the implementer of an item in review or with changes requested, and the round's reviewer, see which round it is. Once three rounds have ended with no approve verdict, the implementer's `status` says in one line that the Review section requires escalation now and names `atelier escalate`. Nothing is refused: `submit` still opens a fourth round. Tests cover round 1, round 3 with changes requested, and an item approved in round 3, which shows no such line.
- [x] The brief `review <item>` prints names the branch's pull request where the base's `config.toml` configures a forge and the forge has one for the branch, found with the forge code `submit` and `merge` already use. With no forge it prints nothing about one. When the forge does not answer, or has no pull request for the branch, the brief says so in one line and `review` still succeeds. The lookup is bounded the way `merge`'s question under the lock is. Tests use the `gh` double the merge tests use.
- [x] `docs/strategies/redesign-product.md`, Review section, reworded only as far as points 1 and 3 need, which Sumeet authorised: it says the one-try rule, the three-round rule and the fresh reviewer for a rebuttal are the skill's to apply and the tool shows the round and the one-try rule; and "What the reviewer is given" names the pull request in place of the latest merge check results. The Handoff lines passage and the `status` example under Commands match what the binary prints. Nothing else in Words, Lifecycle or Review changes: the Report gives `git diff` of those three sections and it shows only those sentences.
- [x] `docs/strategies/redesign-architecture.md` matches: Threads and roles states the identity rule and what was measured for Codex, and no longer says only a harness-started thread can review; the paragraph of Status that lists what the binary does and does not enforce says `status` shows the round and the three-round rule; and the sentence that the brief carries no merge check results says what it carries instead. The `last_changed` lines of both documents are left alone; the orchestrator writes them once.
- [x] In `next/`: `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo nextest run` pass, each with `-j 4` where it builds. The Report gives the non-test line count of `next/` before and after (`scripts/count_non_test_rust.py`).

## Scope

`next/` (the identity rule, `status`, the review brief, the handoff and refusal wording, and their tests), the passages of the two specifications named above, and this item's record. Not `crates/`, `skills/`, `evals/` or `scripts/`. No new dependency. Not in this change: thread identity inside Codex's session-start hook, where none of the variables the rule reads is set, unless the fix is a few lines inside this rule; otherwise the Report says what it would take and the orchestrator files it. Not in this change: any enforcement of the three Review rules, a stored round counter, or a stored check result.

A reviewer's finding that asks for behaviour the specifications and this item do not name is escalated to the orchestrator, not built.

## Context

Mission atelier-qr5s. Specifications: `docs/strategies/redesign-product.md` (Review, Commands) and `docs/strategies/redesign-architecture.md` (Status, Threads and roles). The measurement behind point 5, on Codex 0.167 `codex exec` started inside a T3 thread: the root has `CODEX_THREAD_ID` equal to `CODEX_SESSION_ID`; its `spawn_agent` child has the same `CODEX_SESSION_ID`, the same inherited `T3CODE_THREAD_ID`, and its own `CODEX_THREAD_ID`. The handoff text is in `next/crates/atelier-cli/src/output.rs`; the round is computed in `next/crates/atelier-app/src/acts.rs`; the forge code is `next/crates/atelier-cli/src/forge.rs` and the files beside it that `submit` and `merge` share.

The new binary is run only from `next/target/` in scratch repositories, and is installed nowhere. Step A's skill (atelier-orb2) and step B's evals (atelier-t747) read this item's result: the skill says how a reviewer is started under each harness, and the Codex arm of the evals reviews with its own subagent once this merges.

Expected size: about 150 lines of Rust outside tests, and 40 changed lines across the two specifications.

## Unknowns

- What `spawn_agent` gives its child by default, a fresh context or a fork. The real session in Done when finds out.

## Report

All ten Done when lines hold, with one deviation: `log` is not built in `next/` yet (atelier-8b0a builds it), so the scenario test checks that `show` prints the subagent's identity whole and nothing checks `log`. Proof, all run after the last code change at ba512223: in `next/`, `cargo fmt -- --check` passed, `cargo clippy -j 4 --all-targets -- -D warnings` passed, and `cargo nextest run -j 4` ran 239 tests, 239 passed. Non-test Rust in `next/` (`scripts/count_non_test_rust.py HEAD next`): 11336 before, 11447 after (+111).

- **Identity rule** (`next/crates/atelier-app/src/identity.rs`): unit tests `a_codex_root_thread_keeps_its_identity_and_its_subagent_has_its_own` (the measured root and child, with the inherited `T3CODE_THREAD_ID`), `a_codex_subagent_outside_t3_code_is_its_session_then_its_thread`, `a_codex_thread_without_both_variables_set_changes_nothing` (no `CODEX_SESSION_ID`; an empty or blank value on either side; a Claude Code session stays its parent), and the existing `takes_the_first_variable_set_in_order_else_human`.
- **Scenario** (`tests/scenarios.rs`, `a_codex_subagent_reviews_its_parents_item_as_a_thread_of_its_own`): the root (`T3CODE_THREAD_ID=t1`, `CODEX_THREAD_ID` equal to `CODEX_SESSION_ID`) starts and submits; it is refused as reviewer; a caller with the same parent variables and its own `CODEX_THREAD_ID` runs `review` and `verdict approve`, and the committed review's `reviewers` is `t1/<its thread>`; that caller is refused `submit` and `merge` ("do not hold demo-ab12; t1 does"); `show` prints `by t1/<its thread>`; the root merges.
- **Nothing splits an identity on `/`.** Each place an identity is read out of a line, and the test that reads `<parent>/<child>` back unchanged through it: `Answer by <thread>:`, `Taken: <item> by <thread>` and `Dropped by <thread>:` in `atelier-records/src/body.rs`, test `an_identity_holding_a_slash_reads_back_whole_from_each_line_that_names_one`; the front matter's `implementers`, `reviewers` and `lead`, test `an_identity_holding_a_slash_reads_back_whole_from_each_front_matter_list` in `atelier-records/tests/records.rs`. The search: every `Thread(` construction in `next/crates/*/src` (only `identity.rs` and those three line parsers build one from text), every access to a `Thread` or `Threads` inner value (`.0`: only `contains` equality, in `acts.rs`), and every `split('/')`, `split_once('/')` and `rsplit_once('/')` (a forge URL, a record path, and `<item>/<finding>` names, none an identity). No identity is joined to a path.
- **Handoff and refusals** (`output.rs`, `handoff`): one line now serves `submit`, `escalate`, `lead`, and the author's refused `review` and `decide`: "A reviewer must be a thread other than yours. Under Codex a subagent you start is one; under Claude Code a subagent runs as your thread and is refused, so use your harness's tool for a new agent thread. Start one with fresh context, not as a fork of yours, and give it this task:". No backticks. Pinned by the updated tests in `scenarios.rs`, `merge.rs`, `decisions.rs` and `views.rs`.
- **Real Codex session**, one run of `codex exec -m gpt-6-luna` (Codex 0.167) in `~/.cache/atelier-qr5s/1oel/codex-run/repo`, with this branch's debug binary on `PATH` and `T3CODE_THREAD_ID=t3-root`. (A first launch was stopped before it ran anything: `codex exec` was waiting on stdin.) The root started demo-qgzr, committed `hello.`, submitted; it started one `spawn_agent` child with `fork_turns: "none"`, which ran `atelier review` and `atelier verdict approve` and was accepted; the root merged (`main` holds `status: done`). The records hold `implementers: [t3-root]` and `reviewers: [t3-root/01a1c89d-0f6a-7051-a182-fef6ea11ea9a]`; the child's environment had `CODEX_SESSION_ID=01a1c89c-b1e0-75d1-b3e5-496840e988c3`, its own `CODEX_THREAD_ID=01a1c89d-0f6a-7051-a182-fef6ea11ea9a`, and the inherited `T3CODE_THREAD_ID=t3-root`. **`spawn_agent` forks by default**: its tool definition, as recorded in the session log, says of `fork_turns` "Parent history to inherit. Defaults to `all`; use `none` to start without parent history". A caller asks for a fresh context with `fork_turns: "none"`.
- **Round in `status`** (`acts.rs` `Facts::unapproved_rounds`, `view.rs`, `output.rs`): the implementer sees `Status: in review, round N` or `Status: changes requested in round N`; the round's reviewer already saw `round N`. Once three rounds have ended with no approve verdict, the implementer's `status` adds "3 rounds have ended without an approval, and the Review section requires escalation after three, so escalate now: atelier escalate \"<question>\"". Tests: `status_shows_the_round_and_after_three_without_an_approval_says_to_escalate` (round 1, round 2, round 3 with changes requested, only the implementer is told, `submit` still opens round 4 and its reviewer sees `round 4`) and `an_item_approved_in_round_3_is_not_told_to_escalate`.
- **Pull request in the brief** (`review.rs`, `output::brief_pull_request`): looked up with `Github::open_pull_request`, bounded by `merge::locked_until()`, which `merge`'s look at CI under its lock now uses too. Tests in `tests/merge.rs` with its `gh` double (two flags added, `pulls-down` and `pulls-stall`): `the_review_brief_names_the_branchs_pull_request`, `a_brief_whose_forge_has_no_pull_request_or_does_not_answer_says_so_in_one_line`, `a_brief_whose_forge_never_answers_is_printed_at_the_bound` (ends within the bound and kills the stalled `gh`), and `with_no_forge_the_brief_says_nothing_of_a_pull_request`.
- **Specifications**: the product's Review section changed only in these sentences, and Words and Lifecycle not at all. `diff` of each section, before and after:

```text
== ## Words
== ## Lifecycle
== ## Review
7,8c7,9
<   findings with the author's responses and any decisions, and the latest
<   merge check results. It carries no author reasoning beyond the Report.
---
>   findings with the author's responses and any decisions, and the branch's
>   pull request where a forge is configured, where CI's result can be read.
>   It carries no author reasoning beyond the Report.
20a22,25
> - **The skill applies three of these rules:** a rebuttal's one try, the fresh
>   reviewer for a rebuttal, and escalation after three rounds. The tool
>   refuses none of them; `status` shows the round, the one-try rule and the
>   three-round rule.
```

  The Handoff lines passage and the `status` example and the paragraph under it now match the binary. In the architecture, Threads and roles states the rule and both measurements, and says any harness-started thread and a Codex subagent have identities of their own; the Status paragraph says what `status` shows of the three skill rules and what the brief carries. Neither `last_changed` line changed.

### Responses

- atelier-yul7: fixed in cb648346. `status` away from the item's branch now gives the implementer the round and the escalation line in its list of what it holds; `status_shows_the_round_and_after_three_without_an_approval_says_to_escalate` checks both from a detached checkout of the base, and that the round's reviewer is not told. The architecture's Threads and roles says so. `cargo nextest run -j 4`: 239 passed; fmt and clippy clean.
- atelier-h4x4: escalated to the orchestrator. The `log` clause of the second Done when line cannot hold until atelier-8b0a builds `log`; this item does not build it.

### Deviations

- `log` does not exist in `next/` yet, so the Done when clause "`log` and `show` print the identity whole" holds only for `show`. atelier-8b0a builds `log`; its identity column needs no split, since the identity is one string.
- The specifications changed 77 lines (52 added, 28 removed by `git diff --stat`) against the 40 expected, mostly reflowed paragraphs in the two architecture passages.

### Open items

- Identity inside Codex's SessionStart hook is not in this change. The hook sets none of `CODEX_THREAD_ID`, `CODEX_SESSION_ID` or `T3CODE_THREAD_ID`; Codex passes the hook a JSON object on stdin whose fields, read from the 0.167 binary's strings and not run, include `session_id`, `cwd`, `hook_event_name`, `model` and `source`. Taking an identity there needs `status` to read that stdin when run as a hook, or `setup codex` to write a command that passes `session_id` on, more than a few lines in this rule.
- The handoff line tells a Codex caller to start with fresh context but does not name `fork_turns: "none"`; the skill (atelier-orb2) should, since the default is a fork.

