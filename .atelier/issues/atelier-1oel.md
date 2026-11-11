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

- [ ] `next/crates/atelier-app/src/identity.rs`: when `CODEX_THREAD_ID` and `CODEX_SESSION_ID` are both set to non-empty values that differ, the caller is a Codex subagent, and its identity is the identity today's rule gives (its parent's), then `/`, then its `CODEX_THREAD_ID`. In every other case the identity is what today's rule gives, so a root thread under T3 Code keeps its T3 thread ID and a Claude Code subagent is still its parent. Unit tests cover: the two values measured on this host on 2026-11-11 (`/home/supadava/.cache/atelier-identity-probe/root.txt` and `child.txt`, a root whose two variables are equal and a child whose `CODEX_THREAD_ID` differs, both with one inherited `T3CODE_THREAD_ID`); a Codex subagent outside T3 Code; `CODEX_THREAD_ID` set with no `CODEX_SESSION_ID`, which changes nothing; and an empty value, which counts as unset.
- [ ] A scenario test drives the real binary with those variables: a thread starts and submits an item; a caller with the same parent variables and its own `CODEX_THREAD_ID` runs `review` and `verdict approve` and is accepted, and the review file's `reviewers` holds `<parent>/<its thread>`; the parent itself is still refused as reviewer; and the subagent is refused `submit` and `merge` on the item, since it is not its implementer. `log` and `show` print the identity whole.
- [ ] Nothing treats an identity as a path or splits it on `/`. The Report lists each place an identity is parsed out of a line (`Answer by <thread>:`, `Taken: <item> by <thread>`, `Dropped by <thread>:`, the front matter's `implementers`, `reviewers` and `lead`) with the test that reads an identity containing `/` back unchanged through it, and says how the search for other uses was made.
- [ ] The handoff line `submit`, `escalate` and `lead` print, and the refusal of an author's `review` and `decide`, are true under both harnesses: a subagent of a Claude Code thread runs as that thread and is refused, and a subagent of a Codex thread is a thread of its own and may review or decide. The line still has no backticks. It tells the reader to start the reviewer with fresh context, not as a fork of its own.
- [ ] One real Codex session proves the rule end to end, in a scratch repository with the binary built from this branch: a `codex exec` root thread starts an item, makes a small change and submits; a child it starts with `spawn_agent` runs `atelier review` and `atelier verdict approve` and is accepted; the root merges. The Report gives the identities the records hold, and says what `spawn_agent` does by default, a fresh context or a fork of its parent's, and how a caller asks for a fresh one if that is not the default. This costs one or two Codex runs on a small model; no more are run.
- [ ] `status` shows the round: the implementer of an item in review or with changes requested, and the round's reviewer, see which round it is. Once three rounds have ended with no approve verdict, the implementer's `status` says in one line that the Review section requires escalation now and names `atelier escalate`. Nothing is refused: `submit` still opens a fourth round. Tests cover round 1, round 3 with changes requested, and an item approved in round 3, which shows no such line.
- [ ] The brief `review <item>` prints names the branch's pull request where the base's `config.toml` configures a forge and the forge has one for the branch, found with the forge code `submit` and `merge` already use. With no forge it prints nothing about one. When the forge does not answer, or has no pull request for the branch, the brief says so in one line and `review` still succeeds. The lookup is bounded the way `merge`'s question under the lock is. Tests use the `gh` double the merge tests use.
- [ ] `docs/strategies/redesign-product.md`, Review section, reworded only as far as points 1 and 3 need, which Sumeet authorised: it says the one-try rule, the three-round rule and the fresh reviewer for a rebuttal are the skill's to apply and the tool shows the round and the one-try rule; and "What the reviewer is given" names the pull request in place of the latest merge check results. The Handoff lines passage and the `status` example under Commands match what the binary prints. Nothing else in Words, Lifecycle or Review changes: the Report gives `git diff` of those three sections and it shows only those sentences.
- [ ] `docs/strategies/redesign-architecture.md` matches: Threads and roles states the identity rule and what was measured for Codex, and no longer says only a harness-started thread can review; the paragraph of Status that lists what the binary does and does not enforce says `status` shows the round and the three-round rule; and the sentence that the brief carries no merge check results says what it carries instead. The `last_changed` lines of both documents are left alone; the orchestrator writes them once.
- [ ] In `next/`: `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo nextest run` pass, each with `-j 4` where it builds. The Report gives the non-test line count of `next/` before and after (`scripts/count_non_test_rust.py`).

## Scope

`next/` (the identity rule, `status`, the review brief, the handoff and refusal wording, and their tests), the passages of the two specifications named above, and this item's record. Not `crates/`, `skills/`, `evals/` or `scripts/`. No new dependency. Not in this change: thread identity inside Codex's session-start hook, where none of the variables the rule reads is set, unless the fix is a few lines inside this rule; otherwise the Report says what it would take and the orchestrator files it. Not in this change: any enforcement of the three Review rules, a stored round counter, or a stored check result.

A reviewer's finding that asks for behaviour the specifications and this item do not name is escalated to the orchestrator, not built.

## Context

Mission atelier-qr5s. Specifications: `docs/strategies/redesign-product.md` (Review, Commands) and `docs/strategies/redesign-architecture.md` (Status, Threads and roles). The measurement behind point 5, on Codex 0.167 `codex exec` started inside a T3 thread: the root has `CODEX_THREAD_ID` equal to `CODEX_SESSION_ID`; its `spawn_agent` child has the same `CODEX_SESSION_ID`, the same inherited `T3CODE_THREAD_ID`, and its own `CODEX_THREAD_ID`. The handoff text is in `next/crates/atelier-cli/src/output.rs`; the round is computed in `next/crates/atelier-app/src/acts.rs`; the forge code is `next/crates/atelier-cli/src/forge.rs` and the files beside it that `submit` and `merge` share.

The new binary is run only from `next/target/` in scratch repositories, and is installed nowhere. Step A's skill (atelier-orb2) and step B's evals (atelier-t747) read this item's result: the skill says how a reviewer is started under each harness, and the Codex arm of the evals reviews with its own subagent once this merges.

Expected size: about 150 lines of Rust outside tests, and 40 changed lines across the two specifications.

## Unknowns

- What `spawn_agent` gives its child by default, a fresh context or a fork. The real session in Done when finds out.
