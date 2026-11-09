---
created_at: "2026-11-09T03:59:21.097704169+00:00"
id: "atelier-n7c4"
issue_type: "task"
labels:
- "agent-factory"
workflow_branch:
  branch_base: master
  work_branch: atelier-n7c4
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fl12"
    type: "advances"
  - kind: "issue"
    id: "atelier-xb18"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Make the orchestrator complete the mission or abort it, and say what a mission is for"
updated_at: "2026-11-09T05:45:06.227786684+00:00"
---

## Goal

`orchestrate.md` tells the orchestrator to keep going "without asking between steps" and lists four exceptions for stopping to ask, so agents bring the human mid-mission questions that planning should have settled. Replace that with one instruction, and say in `CONTEXT.md` that a mission is a batch the human reviews twice, so nothing inside it waits on the human (strategy `docs/strategies/mission-review-boundary.md`, Shape).

## Done when

- [x] `procedures/orchestrate.md`'s Before Starting says: "Complete the mission. Abort only when its outcome can't be reached within the strategy, or carrying on would do harm." Aborting means stopping the mission's work, posting what was learned as a dated mission note, and reporting to the human, who replans with a planner. The four stop conditions are gone.
- [x] Keeping the Plan Current: a strategy problem aborts instead of pausing for the human. The orchestrator still plans a later backlog increment at its evidence boundary within the approved strategy and Authority, with one independent planning review and no brief (as atelier-xb18's plan.md says); progress goes in dated mission notes, not messages to the human. Apart from aborting, orchestrate.md has no instruction to ask, consult or wait for the human before the mission closes; the Report quotes every remaining sentence that mentions the human or the user, and each is about aborting or closing.
- [x] `CONTEXT.md`: Mission says it exists so the human reviews a batch of related work twice, its plan before it starts (plan review, `brief`) and its result after it merges (result review, `debrief`), with no human work inside it and human work after it as a loose end; Review names those two human reviews.
- [x] The Report gives the orchestrator's read path before and after, without a target (Sumeet dropped the limits for this mission).

## Scope

`skills/agent-factory/procedures/orchestrate.md` and `CONTEXT.md`. Not the implementer's escalation, which atelier-ss4y adds to orchestrate.md after this merges.

## Context

The planner item (atelier-xb18) rewrites plan.md first; this item points at it.

## Notes

- 2026-11-09 [decision] Mission close cleanup keeps untraceable branches, dirty worktrees and branches with open PRs and lists them in the handoff, instead of asking before deleting them.
  Reason: Asking the human mid-close is a wait inside the mission; keeping and listing loses nothing, and the human sees the list at the debrief.
  Rejected: Keep 'ask before deleting'
- 2026-11-09 [decision] Conflicting requirements or a strategic question from candidates are settled with decide within the strategy, or abort the mission; the 'ask the human' clause for a non-standard branch arrangement is dropped.
  Reason: Both were escalations to the human inside the mission; the complete-or-abort rule covers them without a separate rule.
- 2026-11-09 [decision] Link plan.md's Later Increments and Strategy Changes section from Keeping the Plan Current rather than restating it.
  Reason: The brief asks orchestrate.md to point at it; the cost is plan.md (158 lines) joining the orchestrator's read path, which has no target this mission.

## Report

### Outcome

Every Done when line holds. `procedures/orchestrate.md`'s Before Starting now says, word for word, "Complete the mission. Abort only when its outcome can't be reached within the strategy, or carrying on would do harm." It goes on: settle everything else within the strategy and its Authority, using `decide` where several paths are credible; to abort, stop the mission's work, post what was learned as a dated mission note, and report to the human, who replans with a planner (orchestrate.md:14-19). "Without asking between steps" and the four stop conditions are gone. Keeping the Plan Current says a problem only a strategy change would fix aborts the mission, as does a current hazard, instead of pausing and routing to `plan` (orchestrate.md:115-116). It plans the next backlog increment within the approved strategy and Authority, as plan.md's [Later Increments and Strategy Changes](../../skills/agent-factory/procedures/plan.md#later-increments-and-strategy-changes) says, with one independent planning review and no brief, and posts progress as dated mission notes, not messages to the human (orchestrate.md:118-125). `CONTEXT.md`'s Mission row says it exists so the human reviews a batch of related work twice, plan review (`brief`) and result review (`debrief`), with nothing inside it waiting on the human and human work after it a loose end; the Review row names planning review (the agents') and the human's two reviews.

### Changes

- `skills/agent-factory/procedures/orchestrate.md` (145 to 150 lines, +28 −23): Before Starting rewritten; Assignments drops "ask the human if the work truly needs another arrangement"; Integrating Results settles conflicting requirements or a strategic question with `decide` or aborts, instead of escalating; Keeping the Plan Current aborts on a strategy problem, links plan.md for later increments, and drops "do not plan an item that depends on an open human decision" (a mission has none); Closing a Mission keeps and lists untraceable branches instead of asking before deleting them; the handoff list drops "paused questions".
- `CONTEXT.md`: the Mission and Review rows (+2 −2).
- About 55 changed lines, within the 40-100 estimate.

### Proof

- Every remaining sentence in orchestrate.md that mentions the human or the user (`grep -n -i 'human\|user'` finds three lines):
  - Aborting, orchestrate.md:17-18: "To abort, stop the mission's work, post what was learned as a dated mission note, and report to the human, who replans with a planner."
  - Closing, orchestrate.md:124-125: "Post progress as dated mission notes, not messages to the human, who reads them in the debrief." This one is an instruction not to message the human; the Done when line asks for it, and its reason is the debrief at close.
  - Closing, orchestrate.md:144-146: "Close it with `atelier issue transition <mission> close --reason "<report>"`, then walk the human through `atelier debrief <mission> --html` and record their direction as review records, decisions, notes and issues."
  - The other "ask" left, "ask one focused question of the reviewer, the author or a `diagnose` worker" (Review and Validation), is about agents, not the human.
- Orchestrator read path (SKILL.md, orchestrate.md, references/atelier.md and the files orchestrate.md links), master 6d179d4f before and this branch after, no target: 458 to 621. SKILL.md 99 and 99, orchestrate.md 145 to 150, references/atelier.md 107 and 107, review.md 107 and 107, plan.md newly linked at 158. No other role's path changes.
- Link check over orchestrate.md, CONTEXT.md, references/atelier.md and SKILL.md (each relative link's file exists and its anchor matches a heading): 0 broken. The only inbound anchor into orchestrate.md, `#closing-a-mission` from references/atelier.md, is kept.
- `git diff --check`: clean.
- `atelier check atelier-n7c4`: passes.
- No eval was run; atelier-xc5y tests the rule with fresh agents after atelier-ss4y.

### Decisions

See the three decision notes: cleanup keeps and lists instead of asking; conflicts and strategic questions go to `decide` or abort; plan.md is linked rather than restated, at the read-path cost above.

### Review

Independent review outstanding.

### Open items

- `SKILL.md`'s Respect authority rule still says "Ask a human about product direction, risk tolerance and destructive or irreversible actions policy does not authorize", and `references/atelier.md` says agents leave open decisions to the human. Read by an orchestrator mid-mission, they can pull against complete-or-abort. Both are out of this item's scope; atelier-ss4y edits both files and could reconcile them, or a reviewer may judge them consistent (a new product-direction question is a strategy change, so it aborts).
- "Smaller work follows the same rule, reported to whoever assigned it" extends the rule to non-mission workstreams in one sentence; a reviewer may prefer it cut.
