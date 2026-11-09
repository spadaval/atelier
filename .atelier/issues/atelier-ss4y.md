---
created_at: "2026-11-08T21:46:18.022579037+00:00"
id: "atelier-ss4y"
issue_type: "feature"
labels:
- "agent-factory"
- "review"
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-ixdi"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-dkul"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Orchestrators plan and implementers own the code: escalation instead of relay"
updated_at: "2026-11-08T22:01:42.235512691+00:00"
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

- [ ] `procedures/orchestrate.md` says the orchestrator writes no code. Only changes in SKILL.md's trivial category and planning records stay with it. The "Delegate or Do It Yourself" section says so, and Agent Factory states that for its work this overrides T3's "Fix small review findings yourself; start no delegate for them".
- [ ] The implementer runs its own review loop. When its Done when lines are ticked and its Report written, it runs `atelier review request <id> --identity <reviewer identity>` and starts a fresh reviewer for each round (with `delegate_task` under T3, per atelier-ixdi), passing the printed brief unchanged (it carries no author notes or Report). The reviewer's model comes from the orchestrator's brief. The skill recommends a reviewer from a different lab than the implementer's when the team has one, and allows the same model in a fresh context when it does not: a team limited to one provider can still follow every step. The reviewer's completion wakes the implementer, which then reads the findings from Atelier: no polling and no `t3_thread_send`. Every round is a new reviewer, never a message to an earlier one.
- [ ] The orchestrator's brief to an implementer authorizes those reviewer starts, overriding T3's default that a child starts no subagents. `orchestrate.md`'s Assignments list gains the reviewer model and an expected size of the change (as the plan sizes it).
- [ ] Findings are decided by one rule, written in SKILL.md's Independent Review, `orchestrate.md`, `implement.md` and `review.md`:
  - FIX NOW within the brief: the implementer fixes it, and the next round re-reviews the fix. An approval after the fix resolves it.
  - DEFER recommended by the reviewer: the implementer files the follow-ups with `atelier review defer`. They're listed in the implementer's final report, and the orchestrator schedules them or pulls them in.
  - The implementer believes a finding is wrong: it puts its rebuttal (a fact or a test) in the next round's brief, marked as the author's claim, and that reviewer decides and records `no_action` or keeps the finding.
  - The implementer believes a finding is real but not worth fixing, or that fixing it leaves the brief: that's scope, so it escalates. Apart from running `atelier review defer` on findings the reviewer recommended DEFER, the implementer records no `defer`, `wont_fix` or `no_action` decision on its own work.
- [ ] Briefs state escalation limits, and the implementer escalates when any of these hold:
  - a Done when line can't be met as written;
  - a fix would touch something the brief excludes, or an interface another issue depends on;
  - the change grows past about twice the brief's expected size, or a fix is bigger than the change it fixes;
  - FIX NOW findings remain after three review rounds;
  - a reviewer finds the approach is wrong, not merely buggy;
  - the work hits a risk the brief marks;
  - the implementer disputes a finding and the next reviewer keeps it;
  - the implementer wants to defer or not fix a FIX NOW finding.
- [ ] To escalate, the implementer commits its work in progress on its branch and ends its run with the escalation as its final report: what happened, which limit it hit, two or three options with cost and risk, its recommendation, and the Atelier IDs involved. Completion wakes the orchestrator, which answers with a plan-level choice (rescope, split, accept, re-plan, reimplement or abandon), either by sending the decision to the implementer's thread with `t3_thread_send` or by starting a new implementer.
- [ ] Without an escalation, the implementer's final report is one summary (approved, rounds, findings fixed, follow-ups filed) and the orchestrator merges.
- [ ] Under T3, every child agent is started with T3's `delegate_task` (child atelier-ixdi); elsewhere, with the harness's own subagent tool.
- [ ] `maintainers/skill-evaluation.md` is revised to match: "Needless delegation" and "Review deference" score the new rules, and a new case, an implementer given a Done when line that can't be met, expects an escalation. Before this change merges, the evaluation runs on at least one realistic workstream, and its report shows the orchestrator made no code edits, the implementer ran its own review rounds, and the escalation case escalated.

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
