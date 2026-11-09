---
created_at: "2026-11-08T21:46:18.022579037+00:00"
id: "atelier-ss4y"
issue_type: "feature"
labels:
- "agent-factory"
- "review"
workflow_branch:
  branch_base: master
  work_branch: atelier-ss4y
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-ixdi"
  - kind: "issue"
    id: "atelier-ksme"
  - kind: "issue"
    id: "atelier-pxme"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-dkul"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fl12"
    type: "advances"
  - kind: "issue"
    id: "atelier-n7c4"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Orchestrators plan and implementers own the code: escalation instead of relay"
updated_at: "2026-11-09T06:20:38.446182273+00:00"
---

## Goal

Orchestrators spend about a third of their tokens implementing, and they relay review findings they lack the context to judge. Split the roles so the orchestrator holds the plan, the implementer owns its issue's code and its review loop through approval, and the orchestrator hears about an issue only when it crosses the plan, by escalation.

What the transcripts show for the 30 days to 2026-11-08 (Claude Code, priced at API rates):

- 90 main threads that delegated cost $1,587 across 18,605 calls. 48% of that went to coordination (delegating, Atelier, git, replies), 33% to implementing (edits, scripted fixes, builds, tests, driving the emulator) and 15% to reading code and logs.
- 86 of the 90 edited files, including 138 direct code edits and scripts such as "Fix the two tests, run all checks".
- Orchestrators ran `atelier review finding` 136 times and `review approve` 163 times, partly relaying Grok reviewers.
- A call costs about $0.08 whatever it does. 73% of orchestrator spend came from calls carrying more than 200K tokens of context.

The rules cause it:

- Agent Factory's `procedures/orchestrate.md`: "do small work yourself when you already hold the context."
- T3's orchestration prompt: "Fix small review findings yourself; start no delegate for them."
- `maintainers/skill-evaluation.md` scores "Needless delegation" (do a small task directly) and "Review deference" (the accountable agent picks the disposition) the old way.

The rule for findings after this change: correctness is settled between implementer and reviewer, and scope belongs to the orchestrator. The orchestrator's brief carries its standing scope decision, "fix every FIX NOW within these limits", so constitution §5 ("the agent accountable for the outcome decides what those facts mean for current scope") holds unchanged: the orchestrator stays accountable for scope, and an implementer never narrows scope on its own.

## Done when

- [x] `procedures/orchestrate.md` says the orchestrator writes no code. Only changes in SKILL.md's trivial category and planning records stay with it. The "Delegate or Do It Yourself" section says so, and Agent Factory states that for its work this overrides T3's "Fix small review findings yourself; start no delegate for them".
- [x] The implementer runs its own review loop. When its Done when lines are ticked and its Report written, it runs `atelier review request <id> --identity <reviewer identity>` and starts a fresh reviewer for each round (with `delegate_task` under T3, per atelier-ixdi), passing the printed brief unchanged (it carries no author notes or Report). The reviewer's model comes from the orchestrator's brief. The skill recommends a reviewer from a different lab than the implementer's when the team has one, and allows the same model in a fresh context when it does not: a team limited to one provider can still follow every step. The reviewer's completion wakes the implementer, which then reads the findings from Atelier: no polling and no `t3_thread_send`. Every round is a new reviewer, never a message to an earlier one.
- [x] The orchestrator's brief to an implementer authorizes those reviewer starts, overriding T3's default that a child starts no subagents. `orchestrate.md`'s Assignments list gains the reviewer model and an expected size of the change (as the plan sizes it).
- [x] Findings are decided by one rule, written in SKILL.md's Independent Review, `orchestrate.md`, `implement.md` and `review.md`:
  - FIX NOW within the brief: the implementer fixes it, and the next round re-reviews the fix. An approval after the fix resolves it.
  - DEFER recommended by the reviewer: the implementer files the follow-ups with `atelier review defer`. They're listed in the implementer's final report, and the orchestrator schedules them or pulls them in.
  - The implementer believes a finding is wrong: it puts its rebuttal (a fact or a test) in the next round's brief, marked as the author's claim, and that reviewer decides and records `no_action` or keeps the finding.
  - The implementer believes a finding is real but not worth fixing, or that fixing it leaves the brief: that's scope, so it escalates. Apart from running `atelier review defer` on findings the reviewer recommended DEFER, the implementer records no `defer`, `wont_fix` or `no_action` decision on its own work.
- [x] Briefs state escalation limits, and the implementer escalates when any of these hold:
  - a Done when line can't be met as written;
  - a fix would touch something the brief excludes, or an interface another issue depends on;
  - the change grows past about twice the brief's expected size, or a fix is bigger than the change it fixes;
  - FIX NOW findings remain after three review rounds;
  - a reviewer finds the approach is wrong, not merely buggy;
  - the work hits a risk the brief marks;
  - the implementer disputes a finding and the next reviewer keeps it;
  - the implementer wants to defer or not fix a FIX NOW finding.
- [x] To escalate, the implementer commits its work in progress on its branch and ends its run with the escalation as its final report: what happened, which limit it hit, two or three options with cost and risk, its recommendation, and the Atelier IDs involved. Completion wakes the orchestrator, which answers with a plan-level choice (rescope, split, accept, re-plan, reimplement or abandon), either by sending the decision to the implementer's thread with `t3_thread_send` or by starting a new implementer.
- [x] Without an escalation, the implementer's final report is one summary (approved, rounds, findings fixed, follow-ups filed) and the orchestrator merges.
- [x] Under T3, every child agent is started with T3's `delegate_task` (child atelier-ixdi); elsewhere, with the harness's own subagent tool.
- [x] `maintainers/skill-evaluation.md` is revised to match: "Needless delegation" and "Review deference" score the new rules, and a new case, an implementer given a Done when line that can't be met, expects an escalation. Before this change merges, the evaluation runs on at least one realistic workstream, and its report shows the orchestrator made no code edits, the implementer ran its own review rounds, and the escalation case escalated.

## Scope

- In: Agent Factory's `SKILL.md`, `constitution.md` (only if the wording of §5 needs to point at the brief's standing decision), `procedures/orchestrate.md`, `implement.md`, `review.md`, `references/atelier.md` and `maintainers/skill-evaluation.md`, as far as they cover these roles.
- Out, filed separately if wanted: Atelier flagging a fourth review round or a size overrun mechanically; per-session cost logging for cost per issue; capping the auto-compact window (Sumeet is undecided on long context against compaction).

## Context

- Where to edit: ADR 0029 moves Agent Factory into this repository, and until atelier-dkul lands its one copy is `spadaval/.agents`. This item waits on atelier-dkul, so the change and its evaluation land here in one commit.
- `atelier review request` already prints a reviewer brief without the author's notes or Report, which is what lets the implementer brief its own reviewers.
- atelier-stnv (done 2026-11-08) gave review a recorded "no": a won't-fix disposition, a declined status, and `atelier review defer`, which files one follow-up per deferred finding. The DEFER path above uses those.
- The data and reasoning, including role spend and cost per task: https://claude.ai/artifact/RvFo55Eup7aTqshTycvpUx

## Unknowns

- Whether three rounds and twice the expected size are the right limits. Today issues average 2.4 to 2.8 reviewed commits.
- Whether implementers escalate when they should, or grind on. The evaluation's escalation case tests this once.
- Whether a nested chain (orchestrator, implementer, reviewer, all T3 threads) stays manageable in T3; the skill evaluation will show it.

## Notes

- 2026-11-08 [note] Re-review (atelier-h8uv) left one wording point for the implementer: Done when 2 passes the printed review brief unchanged, while Done when 4 adds a marked author rebuttal when disputing a finding. Write the rebuttal as the one allowed addition to that brief.
- 2026-11-09 [note] Implementer handoff: see ## Report. Independent review outstanding.

## Report

Outcome: every Done when line holds in the skill at `80ec4fc4`, and the evaluation passed its three observations once each. (1) `procedures/orchestrate.md`'s Delegate or Do It Yourself says the orchestrator writes no code, keeps only planning records and SKILL.md's trivial category, and that this overrides T3's "Fix small review findings yourself". (2) `implement.md`'s Review is the implementer's loop: `atelier review request <id> --identity`, a fresh reviewer per round on the brief's model, the printed brief unchanged, woken by completion, findings read with `atelier review list`, never a message to an earlier reviewer; SKILL.md's Who arranges it recommends another lab and allows the same model fresh. (3) orchestrate.md's Assignments gains Expected size and Review and limits (reviewer model, the standing decision, permission to start reviewers overriding T3's default, marked risks). (4) The findings rule is stated in full in SKILL.md's Findings bullet and applied by role in orchestrate.md (Results and Escalations), implement.md (Review step 3) and review.md (Recommendations). The rebuttal is "the one allowed addition" to the brief, as the planning note asked. (5) The eight limits are implement.md's Escalation list; a brief points at them and adds marked risks. (6) Escalation commits work in progress and ends the run with the report; the orchestrator answers with a plan-level choice by `t3_thread_send` or a new implementer. (7) Without escalation, the final report is one summary and the orchestrator merges. (8) The T3 rule is SKILL.md's "Start children fresh, one way" (atelier-ixdi). (9) `maintainers/skill-evaluation.md` rescored Needless delegation, Review deference, Orchestrator-authored code, Author dismissal, Biased reviewer brief, Context gap and Deferred finding fixed anyway, and added Unmeetable Done when. The evaluation is `evals/agent-factory/results/2026-11-09-implementer-owns-review.md`: the orchestrator made no code edit (its timeline has no Edit or Write, and its only commit is a record); the implementer ran `review request` and started its own Sol reviewer with `delegate_task`, one round, approved; the implementer given tally-7wok's contradictory Done when escalated with options and a recommendation and wrote no code.

### Deviations

- orchestrate.md's old Review and Validation, Stop signals and Integrating Results became one section, Results and Escalations. The stop signals are now the implementer's escalation limits. The rule "a new finding on the fix of a low-severity finding is deferred" moved to review.md as a reviewer recommendation, since the implementer may not record that defer itself.
- review.md's high-severity WON'T FIX rule ("the human's call... label the item `needs-human`") now says that inside an approved mission the orchestrator fixes or defers it instead, so no mid-mission wait remains (atelier-pxme's second line).
- Reviewer-recommended WON'T FIX and NO ACTION findings, which the rule leaves to nobody, are left undisposed in the implementer's summary and decided by the orchestrator before merge.
- constitution.md is unchanged: §5's "the agent accountable for the outcome decides" holds through the brief's standing decision, and the constitution is not on any role's read path.

### Surprises

- `atelier review request`'s brief prints Goal and Done when but not the item's Scope, so "unchanged" would hide Scope from the reviewer. review.md's Stance now points the reviewer at `atelier issue show <id>` for it.
- In the workstream run, the orchestrator caught tally-7wok's contradiction itself before starting anyone, so the escalation case needed its own run with a directly briefed implementer.
- T3's `delegate_task` refuses another project's `projectId`, so every eval child started in this worktree and was given the fixture path; the worktree stayed clean.

### Open items

- Each case ran once; the fix-and-re-review round, `review defer` and the rebuttal path were not exercised (no findings were raised). atelier-xc5y's evals can cover them.
- Whether three rounds and twice the expected size are the right limits is still open.
- Read-path counts, lines (words), master to this branch: orchestrator (SKILL + orchestrate + atelier.md) 357 (3,252) to 370 (3,383); implementer (SKILL + implement + atelier.md) 279 (2,545) to 328 (3,012); reviewer (SKILL + review + atelier.md + sizing) 327 (2,906) to 347 (3,114). SKILL.md is 109 lines, from 99.
- Fixture repositories stay in `~/.cache/ss4y-eval/` for this branch's review; delete after.

### Changes

- `skills/agent-factory/`: SKILL.md, procedures/orchestrate.md, implement.md, review.md, references/atelier.md, maintainers/skill-evaluation.md (commit `80ec4fc4`).
- `evals/agent-factory/results/2026-11-09-implementer-owns-review.md`: the evaluation record.
- Records: atelier-ss4y, atelier-ixdi, atelier-pxme. Branch `atelier-ss4y`, not pushed.
- Created outside the repo: `~/.cache/ss4y-eval/` (kept, see Open items) and a T3 project for the fixture, deleted unused.

### Proof

- Evaluation: see Outcome; every agent's timeline and the fixture's `git log --stat` were read by hand.
- Links and anchors: a checker over every relative link in `skills/agent-factory/` found 43 links, 0 broken, and caught a deliberately broken anchor in a scratch copy.
- `git diff --check`: clean. `atelier check` passes for atelier-ss4y, atelier-ixdi and atelier-pxme.

### Review

Independent review outstanding.
