---
created_at: "2026-11-09T03:59:22.001051041+00:00"
id: "atelier-cfq9"
issue_type: "feature"
labels:
- "agent-factory"
workflow_branch:
  branch_base: master
  work_branch: atelier-cfq9
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fl12"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Show a mission's sizing and derisking at the top of its brief"
updated_at: "2026-11-09T05:34:27.422660332+00:00"
---

## Goal

The human decides whether to approve a plan from its brief, but the brief shows nothing about the mission's cost, value or risk, and the planning reviewers' sizing sits at the bottom in review prose. Show the strategy's new Sizing and Derisking sections under the Outcome, and say plainly when they are missing, so an unsized plan is visible.

## Done when

- [x] `atelier brief <mission>` (Markdown and `--html`) shows the linked strategy's Sizing and Derisking sections directly under the Outcome, read the way the brief already reads Risks and Unknowns (`strategy_section` in `crates/atelier-app/src/mission_review.rs`). Integration tests cover a strategy with both sections.
- [x] A strategy missing either section, or a mission with no strategy, shows "Sizing: not written" or "Derisking: not written" in that place; a test covers it. Nothing refuses.
- [x] `docs/product/mission-reviews.md` describes the two sections in the brief.

## Scope

The brief only. Not the debrief, `atelier check`, or the site.

## Report

The brief now shows a mission's Sizing and Derisking under its Outcome, so the human sees cost, value and risk before scope. All three Done when lines hold. `brief_shows_sizing_and_derisking_under_the_outcome` proves both sections appear, in order, between the Outcome and the rest of the page in Markdown and `--html`, with HTML escaping. `brief_says_when_sizing_or_derisking_is_not_written` proves a strategy missing Derisking, and a mission with no strategy, show "not written" in both forms and still exit 0. Both tests failed against the unchanged code and pass now.

### Changes

- `crates/atelier-app/src/mission_review.rs`: `Brief` gains `sizing` and `derisking`, read with the existing `strategy_section` closure, and `plan_sections()`, the one place that names the two sections and their order for both renderers.
- `crates/atelier-cli/src/commands/mission_review.rs`: in Markdown, a written section follows the header as its own section, and a missing one is a header line under the Outcome (`Sizing        : not written`). In HTML, a card directly under the Outcome card holds each section, or a muted "Sizing: not written" line.
- `crates/atelier-cli/tests/cli_integration/brief.rs`: the two tests above.
- `docs/product/mission-reviews.md`: the Brief section describes both sections and the "not written" line.

### Proof

- `cargo nextest run -j 4 -p atelier-cli -p atelier-app -E 'test(/brief|mission/)'`: 150 passed. The two new tests failed with the source changes stashed.
- `cargo fmt -- --check` and `git diff --check`: clean.
- `scripts/smoke-repos.sh --new <this build>`: no regression across 5 repositories. It does not run `brief`, so `brief` was also run with the new binary on all 34 missions in the four local Atelier repositories, in Markdown and `--html`: none refused.
- `atelier brief atelier-fl12` top section:

```
Outcome       : Agent Factory treats a mission as a human review boundary. ...
Strategy      : docs/strategies/mission-review-boundary.md
Generated from: c901e445

Sizing
------
- Complexity: medium. It restructures three roles across the skill's
...
Derisking
---------
- Riskiest assumption: that one planner procedure can hold design, sizing,
...
Success Criteria
```

- `atelier brief atelier-1gb3` (strategy has neither section):

```
Outcome       : In a repository whose policy names a forge, ...
Sizing        : not written
Derisking     : not written
Strategy      : docs/strategies/forge-delivery.md
```

### Deviations

- In Markdown the missing line is a padded header line, `Sizing        : not written`, matching the brief's other header lines, not the literal `Sizing: not written`. The HTML shows the literal text. When a section is written, the Markdown puts it after the Strategy and Generated from lines, because a multi-line section cannot sit inside the header block.

### Review

Independent review outstanding.

### Open items

- Running `brief --html` across the local repositories rewrote their ignored `.atelier/runtime/reviews/*-brief.html` pages with this build's output; nothing tracked changed.
