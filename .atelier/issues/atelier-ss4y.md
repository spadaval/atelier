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
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Orchestrators plan and implementers own the code: escalation instead of relay"
updated_at: "2026-11-08T21:46:30.333961183+00:00"
---

## Goal

Orchestrators spend about a third of their tokens implementing, and they relay review findings they lack the context to judge. Split the roles so the orchestrator holds the plan, the implementer owns its issue's code through approval, and the orchestrator hears about an issue only when it crosses the plan, by escalation.

What the transcripts show for the 30 days to 2026-11-08 (Claude Code, priced at API rates):

- 90 main threads that delegated cost $1,587 across 18,605 calls. 48% of that went to coordination (delegating, Atelier, git, replies), 33% to implementing (edits, scripted fixes, builds, tests, driving the emulator) and 15% to reading code and logs.
- 86 of the 90 edited files, including 138 direct code edits and scripts such as "Fix the two tests, run all checks".
- Orchestrators ran `atelier review finding` 136 times and `review approve` 163 times, partly relaying Grok reviewers.
- A call costs about $0.08 whatever it does. 73% of orchestrator spend came from calls carrying more than 200K tokens of context.

The rules cause it:

- Agent Factory's `procedures/orchestrate.md`: "do small work yourself when you already hold the context."
- T3's orchestration prompt: "Fix small review findings yourself; start no delegate for them."
- The orchestrator then decides a finding and writes its fix, which `SKILL.md`'s Independent Review rule otherwise forbids ("an author never decides findings on its own work").

## Done when

- [ ] `procedures/orchestrate.md` says the orchestrator writes no code; only Agent Factory's trivial category (typos, status changes) and planning records stay with it. The "Delegate or Do It Yourself" section and any conflict with T3's "fix small review findings yourself" line are resolved.
- [ ] The implementer owns its issue from `atelier start` through approval: it requests review, reads findings from Atelier, fixes FIX NOW findings within its brief, and requests re-review of each fix.
- [ ] Findings reach the implementer without the orchestrator. The reviewer records them in Atelier under its own identity, including reviewers from other labs, and may nudge the implementer's thread with `t3_thread_send`. The record carries the content; the message only says it exists.
- [ ] Deciding a finding stays independent of its author. To dispute or defer a finding, the implementer sends the reviewer a rebuttal (a fact or a test), and the reviewer decides and records it. An agreed deferral is filed as a follow-up with `--discovered-from`.
- [ ] Briefs state escalation limits. The implementer stops and escalates to the orchestrator when any of these hold:
  - a Done when line can't be met as written;
  - a fix would touch something the brief excludes, or an interface another issue depends on;
  - the diff grows past about twice its estimate, or a fix is bigger than the change it fixes;
  - FIX NOW findings remain after three review rounds;
  - the reviewer finds the approach is wrong, not merely buggy;
  - the work hits a risk the brief marks;
  - the reviewer and implementer can't settle a dispute with a fact or a test.
- [ ] An escalation is short: what happened, which limit it hit, two or three options with cost and risk, and the implementer's recommendation, pointing to the Atelier records. The orchestrator answers with plan-level choices: rescope, split, accept, re-plan, reimplement or abandon.
- [ ] Without an escalation, the orchestrator reads one summary per issue (approved, rounds, findings fixed and deferred, follow-ups filed) and merges.
- [ ] Every child agent is started with T3's `delegate_task` (child atelier-ixdi), so implementers are T3 threads a reviewer can message.
- [ ] Before the change is merged, Agent Factory's skill evaluation (`maintainers/skill-evaluation.md`) runs on at least one realistic workstream, and its report shows the orchestrator made no code edits and handled any escalations.

## Scope

- In: Agent Factory's `SKILL.md`, `procedures/orchestrate.md`, `implement.md`, `review.md` and the Atelier reference, as far as they cover these roles.
- Out, filed separately if wanted: Atelier flagging a fourth review round or a size overrun mechanically; per-session cost logging for cost per issue; capping the auto-compact window (Sumeet is undecided on long context against compaction).

## Context

- Agent Factory's source is `spadaval/.agents` (ADR 0023).
- The data and reasoning, including role spend and cost per task: https://claude.ai/artifact/RvFo55Eup7aTqshTycvpUx
- atelier-stnv (done 2026-11-08) gave review a recorded "no": a won't-fix disposition, a declined status, and `atelier review defer`, which files one follow-up per deferred finding. The deferral path above uses those.

## Unknowns

- Whether three rounds and twice the estimate are the right limits. Today issues average 2.4 to 2.8 reviewed commits.
- Whether implementers escalate when they should, or grind on. Skill evaluation should include a case that ought to escalate.
