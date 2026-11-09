---
created_at: "2026-11-09T20:24:27.371132427+00:00"
id: "atelier-fnlu"
issue_type: "task"
labels:
- "agent-factory"
- "review"
workflow_branch:
  branch_base: master
  work_branch: atelier-fnlu
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-09T22:17:38.299146806+00:00"
status: "done"
title: "Have reviewers flag code no requirement needs: 1 shrink verdict and no overbuilt-code findings in 1,252 reviews"
updated_at: "2026-11-09T22:17:38.299146806+00:00"
---

## Goal

Reviewers ask "which requirement needs this?" of every moving part a change adds, and flag the ones nothing needs, such as 200 lines that do nothing. Today they almost never do: of 1,252 review records, one carries a `Shrink:` verdict, none a `Reject:`, and essentially none flags overbuilt code. The skill asks for it, but the printed review brief never mentions complexity, and `review.md` lets a reviewer raise complexity only after designing the simpler alternative.

## Done when

- [x] `skills/agent-factory/constitution.md` Simplicity section says code should be as complex as its requirements call for and no more; every moving part (type, layer, flag, option, cache, branch) traces to a current requirement; and simplicity is judged on the system after the change, not the diff, so a larger change that removes parts is simpler.
- [x] `skills/agent-factory/procedures/review.md` replaces the "Complexity is a defect only when you can name ... a simpler construction" rule: for each moving part the change adds, the reviewer asks which requirement needs it; a part that traces to none is `FIX NOW` (`DEFER` if it predates the change); the author answers by naming the requirement or removing the part. The smallest-proof tie-break for disputes stays.
- [x] The brief printed by `atelier review request` (`crates/atelier-cli/src/commands/review_request.rs`) has a working rule to size the change's complexity, value and risk and ask which requirement needs each moving part it adds, and its Report line asks for the Verdict (merge, shrink or reject) and Sizes as well as Contract and Fitness. Brief tests cover the new lines.
- [x] `skills/agent-factory/maintainers/skill-evaluation.md` no longer points to a "Good Code" section that does not exist (row "Spike-shaped production"); it points to the Simplicity section.

## Context

The trade-off between simplicity and scope is decided at planning, where items are sized on complexity, value and risk. At review the Done when lines fix the scope, so the rule is single, not two opposing demands: no more complexity than those requirements need. The weighing of value against complexity at review stays in the merge, shrink or reject verdict on the whole change. Code that skips something the domain requires already fails Contract.

Watch for over-correction (atelier-jxce): the rule asks for a traceable requirement, not the smallest diff. A patch that adds a special case or a second copy of logic is less simple than a larger change that unifies it.

## Notes

- 2026-11-09 [note] Handoff: approved by GPT-6.1 Sol r1 (atelier-fstb @ e338da39) in one round with no findings. Not merged; merge --check passes all but ci.passed, which needs merge --push --wait. See ## Report; follow-up atelier-5lv9.

## Report

All four Done when lines hold. The constitution's Simplicity section and `review.md` carry the single rule (every moving part traces to a current requirement; judge the system after the change, not the diff), with `FIX NOW`/`DEFER` and the smallest-proof tie-break kept. The item brief gains a working rule to size the change and ask which requirement needs each moving part, and its Report line asks for Verdict and Sizes. `the_brief_renders_as_dense_markdown` failed with the new expected text before the render change and passes after it; `cargo nextest run -p atelier-cli` passes (883 tests), and `scripts/smoke-repos.sh` reports no regression across 5 repositories. `skill-evaluation.md`'s spike row now links the Simplicity section.

### Open items

- The light-path brief (`light_path.rs`, `atelier review request` with no id) still asks only for Correctness and Fitness; it was outside this item's named file. Filed as atelier-5lv9 (backlog, low).
