---
schema: atelier.item
schema_version: 3
id: atelier-xbog
title: 'Remove escalate and decide: a question is asked, and a disputed finding goes to a fresh reviewer'
status: done
mission: atelier-to83
labels:
- redesign
priority: medium
blocked_by:
- atelier-ogpg
implementers:
- thread:delegated-task:command%3Amcp%3A9679df7a-3d14-4844-bae1-e4e8bb337119%3Adelegate-task%3A7c726c12-b16c-463e-a7e1-334572e4aebd
ready:
  by: sumeet, in conversation on 2026-11-13, after reading docs/design.md
  at: 2026-11-13T16:37:33.471518539Z
trivial: null
caused_by: null
created_at: 2026-11-13T08:16:08.586117508Z
---

## Goal

Take out a process built for a case that does not happen. `escalate` and
`decide` give a question a section in the item, a block on `submit` and
`merge`, a role, a guard and a page on the Desk. No item in this repository
has used them since the cutover, and each of their two jobs has a plainer
answer already. A question is advice: the implementer asks the thread that
started it, or an advisor, and writes into the Report anything that changed
the work. A disputed finding is a rebuttal: the implementer writes it in
Responses and a fresh reviewer keeps or withdraws the finding in the next
round, as the skill already says.

Where two models do disagree, the records keep it without any of this, and
this item makes it easy to find.

## Done when

- [x] `escalate` and `decide` are gone from the binary, with the Decisions section, the `Decided:` line of a review file, the check that refuses `submit` and `merge` on an open question, and the decider's guard and routing in `status`.
- [x] A review's reviewer can close its own carried finding as `Now: withdrawn`, or lower it to defer or no action with its reason, and nothing else can settle a fix-now finding without a fix. A test walks a rebuttal kept and a rebuttal withdrawn.
- [x] The binary still reads an item that has a Decisions section and a review that has a `Decided:` line, and treats a finding decided as defer or no action as settled, so no merged item in the five repositories reads differently. A test reads each such record that exists today.
- [x] `atelier list --disputed` lists, across open and done items, each finding whose Response is `rebutted`, with what the next round did (kept, withdrawn, or not yet reviewed), and each round of a plan review whose reviewers gave different verdicts. Nothing refuses or warns on any of them. `--mission <id>` narrows it, and `show <mission>` and the lead's `status` list the mission's disputed findings where they listed open questions.
- [x] `docs/product.md` and `docs/design.md` no longer describe either command, and say where a question and a disputed finding go.

## Scope

In: `crates/atelier-core`, `crates/atelier-app` (`decide.rs`, `acts.rs`, `review.rs`), `crates/atelier-cli` (`questions.rs`, `commands.rs`, the views), their tests, and the two documents.

Out: the skill's wording (atelier-j5lr); the Desk's decide page (atelier-v958); any rule about when to rebut or how many times, which stays in the skill.

## Context

`docs/design.md` gives the review file's format: a carried finding has `Was:`, the author's `Response:` and one of `Now: fixed`, `Now: withdrawn` or `Now: kept`, and Responses in the item's Report hold lines such as `2-m2bn.1: rebutted: <fact>`. Those two are the whole record of a disagreement. The parallel Sumeet named is Claude's advisor tool: the working model asks a stronger one at a decision point and carries on, with no record and no role.

## Report

`atelier escalate` and `atelier decide` are gone. Nothing blocks `submit`
or `merge` on a question, and `status` no longer routes anyone to a decider
or says to escalate after three rounds. A disputed finding now ends with a
reviewer: the next round's reviewer withdraws it, keeps it, or keeps it at
`defer` or `no action` with a line that says why. `atelier list --disputed`
finds every disagreement the records hold.

What changed:

- `atelier-cli`: the two commands, `questions.rs` and their output are
  removed. `list --disputed [--mission <id>]` is new. `show <mission>` has a
  Disputed section where Questions was, and a lead's `status` lists its
  mission's disputes where it listed questions. The review template's
  comment says how to lower a carried finding.
- `atelier-app`: `Act::Escalate`, `ESCALATE_AFTER`, `unapproved_rounds`,
  `Entry::unapproved` and `Entry::questions`, the decider handoff, and the
  `Unanswered` and `DecisionDropped` conditions are removed. A verdict now
  refuses any `Decided:` line in its own file, and any finding that carries
  one a decision settled (`Condition::Decided`). A new round no longer
  carries a decided finding. The new `disputed.rs` finds rebuttals and
  split plan rounds. `log` no longer has escalate or decide lines.
- `atelier-records`: `ask` is removed. A carried finding kept at defer or
  no action needs a line of the reviewer's own, not a field or an HTML
  comment, single-line or multi-line, or the file does not read.
- `atelier-core`: `is_open` and `is_deferred` still read Decisions entries
  and `Decided:` lines, so old records read as before.
- `AGENTS.md` still sent questions to `escalate`. I fixed its two lines,
  though it is outside the listed Scope, because it is the map every agent
  reads first.

Kept on purpose for the Desk: `atelier_app::decide`, `Act::Decide`,
`Role::Decider`, `acts::question`, `records::answer` and
`MissionView::questions`. The Desk's decide page and its Rust and e2e tests
call them, and atelier-v958 (blocked by this item) removes that page and
its act. Since no command can ask a question any more, the Desk's `decide`
has nothing to answer, so it cannot settle a fix-now finding in practice.

### Validation

- `cargo nextest run --workspace`: 306 passed. `cargo clippy --all-targets
  -- -D warnings`, `cargo fmt -- --check` and `git diff --check` are clean.
- `crates/atelier-cli/tests/disputes.rs` replaces `decisions.rs`:
  - a rebuttal withdrawn, as `list --disputed` shows it before review,
    after review and once merged;
  - a rebuttal kept, whose `status` line says it had its one try and names
    no escalate or decide. A third reviewer keeps it at defer: refused
    without a reason, approved with one. The item merges, and the carrier
    shows in `list --deferred`;
  - `escalate` and `decide` fail as unknown subcommands and are not in
    `--help`;
  - an item with a Decisions section and a round-2 review with
    `Decided: defer`, written as the old tool wrote them. `check` passes,
    `show` shows both findings closed, `list --deferred` lists the carrier
    and `drop` settles it;
  - a mission's rebuttal and split plan round, in `list --disputed
    --mission`, `show <mission>` and the lead's `status`.
- `crates/atelier-app/tests/acts.rs`: old decisions still close their
  finding and are not carried, and a new review cannot reopen them or write
  `Decided:`. A withdrawal or a lowering with a reason lets approve pass,
  and a lowering without a reason does not read.
- Records that exist today: I searched every branch and remote ref of the
  twelve directories under `~/projects` that hold `.atelier/` (eleven with
  a config), with `git grep
  -nE '^## Decisions|^Decided: |^Outcome: |^Finding: '` over
  `.atelier/items` and `.atelier/missions`. Nothing matched outside
  `missions/atelier-to83/design.md`, which only describes this change. The
  only Decisions sections are in the old tool's `.atelier/issues/*.md`,
  which this binary does not read, and the cutover converter writes none.
  So there is no real record to test. The tests above use records written
  in the old format, and the existing `atelier-records` tests of the
  Decisions format still pass.

### Responses

- 1-aj9b.1: fixed in 42bc22e13