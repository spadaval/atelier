---
created_at: "2026-11-09T03:59:33.124261341+00:00"
id: "atelier-v3nm"
issue_type: "validation"
labels:
- "agent-factory"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-cfq9"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fl12"
    type: "advances"
  - kind: "issue"
    id: "atelier-n7c4"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-ss4y"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-xb18"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-09T07:01:35.506812139+00:00"
status: "done"
title: "Validate planner-end-to-end: a fresh planner replans the site mission under the new skill"
updated_at: "2026-11-09T07:01:35.506812139+00:00"
---

## Goal

Show mission atelier-fl12's criterion planner-end-to-end holds: the planner role, given only the new skill, produces a plan that meets its directive on a real mission, the site mission atelier-yfqu, whose replan is the next planning job anyway.

## Done when

- [x] A fresh planner agent that did not write or review atelier-xb18, atelier-n7c4, atelier-cfq9, atelier-ss4y or atelier-ixdi, given only the installed skill, the repository and atelier-yfqu's 2026-11-08 note of Sumeet's direction, replans atelier-yfqu on the base, and the agent running this item reads the result by hand.
- [x] The replan's strategy has: a Design section that settles where the site's code lives, how it ships and which libraries it uses, with reasons (the note leaves these open, so any well-argued answer passes); a Sizing section with a reason per scale and a worth-it sentence; a Derisking section naming the riskiest assumption, with the item that tests it ordered first; and no `needs-human` item inside the mission's scope.
- [x] Two planning review passes are recorded on the replan, from two model families other than the planner's and different from each other, run without seeing each other's findings, and every fix they asked for is re-reviewed.
- [x] `atelier brief atelier-yfqu` shows the Sizing and Derisking sections under the Outcome.
- [x] A receipt for planner-end-to-end records the commits, the reviewers' model families and anything the replan lacked; Sumeet's approval of the site plan is the site mission's own gate and is not required here.

## Context

Run on the base once atelier-xb18, atelier-n7c4, atelier-cfq9 and atelier-ss4y have merged, so the replan uses the skill as it ships, and the installed binary and skill include them (`cargo install --path crates/atelier-cli --locked`; move `~/projects/atelier-skill` forward).

## Report

**Outcome.** planner-end-to-end passes (receipt atelier-ze4f). A fresh Opus 5.5 planner had only the installed skill, the repository and atelier-yfqu's 2026-11-08 note. It replanned the site mission to the planner's directive, and I read the result by hand. The plan has gaps, listed under Open items. The worst is that the two reviewers were blind to each other only because of timing.

**Changes.** The planner wrote these commits on the base, after `6cd7cb35`:
- `2580f29e`: the replan, with the strategy rewritten, new decision atelier-c5vk, new items atelier-uma8 and atelier-59fz, and its other items revised.
- `70f60544`: the records of the two planning reviews.
- `552a7a09`: fixes for the reviews' findings.
- `680492c8` and `1622beee`: the re-review approvals.

It changed only atelier-yfqu's records, its items, decision c5vk, review records and `docs/strategies/fleet-dashboard.md`. This item adds receipt atelier-ze4f and this Report.

**Proof.** Each Done when line, checked by hand:
1. **Fresh planner.** I started it in its own T3 thread with `delegate_task` (claudeAgent, claude-opus-5-5, high effort). Its brief named only the `plan` subskill, the mission, the note, the checkout, the commit and land rules, and the reviewer models. It said nothing about what a plan should contain.
2. **Strategy.**
   - Design settles crate `crates/atelier-site`, shipped in the one `atelier` binary as `atelier serve`. It uses axum, tokio and tower for HTTP, the existing HTML helpers rather than maud, and pulldown-cmark for Markdown, with reasons in the strategy and in atelier-c5vk's Options.
   - Sizing rates complexity high, value high and risk medium, each with a reason, and has a worth-it sentence.
   - Derisking names the riskiest assumption: the views can be built from a base commit without a checkout, cheaply enough for this host. The planner measured it: `debrief atelier-k8h8 -q` took 7.7 s and 64 MB.
   - atelier-zpmb tests that assumption and comes first: it is P1, and `atelier work mission` lists it first among ready work.
   - The only `needs-human` item is loose end atelier-pywh, which is outside the mission's scope.
   - The strategy has 140 lines outside the Design section.
3. **Reviews.** Both reviews ran on `2580f29e`:
   - GPT-6.1 Sol (codex) recorded atelier-7p31, s0mf, ugpo, 6sxq and wjok, all `fix_now`.
   - Grok 4.6 (grok) recorded x1xu (`fix_now`), and ei8w and 0p0q (both defer).
   - Neither family is Anthropic, and they differ from each other.
   - Record times and the Grok thread show they were blind to each other. Grok found no other records at 06:50:19, drafted its findings by 06:50:38 and recorded them at 06:50:42. Sol's records appeared at 06:50:36, and Grok first saw them at 06:50:49.
   - A fresh Sol re-reviewed every fix (atelier-em7e @552a7a09), and a fresh Grok re-reviewed the c5vk fix (atelier-yq0r). `atelier review list atelier-yfqu` and `atelier-c5vk` show every finding settled.
4. **Brief.** `atelier brief atelier-yfqu` prints Sizing and Derisking directly under the Outcome, with `Generated from: 1622beee`.
5. **Receipt.** Receipt atelier-ze4f records `planner-end-to-end=pass` at `1622beee`, with the commits, the reviewers' families and the gaps below.

**Review.** This is a validation item, so it gets no branch and no review. The planner's own reviews are listed above.

**Open items.** These went to the accountable agent and were not fixed here:
- **Blind only by timing.** Both reviewers wrote uncommitted records into the one shared checkout. The planner's reviewer brief (`~/.cache/yfqu-review-brief.md`) did not tell them not to read each other's findings. Grok ran `ls -lt .atelier/reviews` looking for "parallel reviewer records" 17 s before Sol's appeared. A worth-filing gap in plan.md's Planning Review: a reviewer should be told not to read the other's records, or should record somewhere the other cannot see.
- **Completion waits outside the mission.** atelier-59fz and atelier-mtb3 are in scope but wait on loose end atelier-6rts, which installs and publishes the site and sits outside the mission. That loose end is not `needs-human`, so the line holds. Still, the mission cannot finish until 6rts is done.
- **One choice left open.** atelier-59fz leaves open how the CLI learns the site's address. The choice is cheap to reverse.
- **An extra review round.** `atelier land` refused once because the re-review approval was recorded on the mission rather than on decision atelier-c5vk.
