---
schema: atelier.item
schema_version: 3
id: atelier-xbog
title: 'Remove escalate and decide: a question is asked, and a disputed finding goes to a fresh reviewer'
status: building
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

- [ ] `escalate` and `decide` are gone from the binary, with the Decisions section, the `Decided:` line of a review file, the check that refuses `submit` and `merge` on an open question, and the decider's guard and routing in `status`.
- [ ] A review's reviewer can close its own carried finding as `Now: withdrawn`, or lower it to defer or no action with its reason, and nothing else can settle a fix-now finding without a fix. A test walks a rebuttal kept and a rebuttal withdrawn.
- [ ] The binary still reads an item that has a Decisions section and a review that has a `Decided:` line, and treats a finding decided as defer or no action as settled, so no merged item in the five repositories reads differently. A test reads each such record that exists today.
- [ ] `atelier list --disputed` lists, across open and done items, each finding whose Response is `rebutted`, with what the next round did (kept, withdrawn, or not yet reviewed), and each round of a plan review whose reviewers gave different verdicts. Nothing refuses or warns on any of them. `--mission <id>` narrows it, and `show <mission>` and the lead's `status` list the mission's disputed findings where they listed open questions.
- [ ] `docs/product.md` and `docs/design.md` no longer describe either command, and say where a question and a disputed finding go.

## Scope

In: `crates/atelier-core`, `crates/atelier-app` (`decide.rs`, `acts.rs`, `review.rs`), `crates/atelier-cli` (`questions.rs`, `commands.rs`, the views), their tests, and the two documents.

Out: the skill's wording (atelier-j5lr); the Desk's decide page (atelier-v958); any rule about when to rebut or how many times, which stays in the skill.

## Context

`docs/design.md` gives the review file's format: a carried finding has `Was:`, the author's `Response:` and one of `Now: fixed`, `Now: withdrawn` or `Now: kept`, and Responses in the item's Report hold lines such as `2-m2bn.1: rebutted: <fact>`. Those two are the whole record of a disagreement. The parallel Sumeet named is Claude's advisor tool: the working model asks a stronger one at a decision point and carries on, with no record and no role.
