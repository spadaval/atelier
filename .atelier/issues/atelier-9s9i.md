---
created_at: "2026-11-08T05:52:15.329120217+00:00"
id: "atelier-9s9i"
issue_type: "task"
labels:
- "agent-factory"
workflow_branch:
  branch_base: master
  work_branch: atelier-9s9i
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-dkul"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-g6rp"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-08T23:05:50.408183254+00:00"
status: "done"
title: "Fold codex-reflect's harness-neutral retrospective method into audit.md before it is deleted"
updated_at: "2026-11-08T23:05:50.408183254+00:00"
---

## Goal

`codex-reflect` in spadaval/.agents builds retrospectives from Codex session logs with Artifact Hub, which is being deleted. Keep the parts that apply to any harness in Agent Factory's `audit.md`, so retiring it loses nothing.

## Done when

- [x] `skills/agent-factory/procedures/audit.md` gains at most about 15 lines with what applies beyond Codex: start from raw transcripts rather than memory, verify the transcript around each failure and handoff claim, follow subagent and thread lineage to the root, and how to report a session retrospective (what went well, what went wrong, what to change in code, tooling, guidance or skills). Codex paths, Artifact Hub and the report app are not carried over.
- [x] The Report lists what was left out and why.
- [x] The skill stays within `atelier-qj9d`'s line limits.

## Scope

`skills/agent-factory/procedures/`.

## Context

The gardener (atelier-3sk9) reads transcripts too and comes after this item; it moves or links this method, so this item does not wait for it.

## Notes

- 2026-11-08 [handoff] Implemented in 519398e3: audit.md gains a 12-line Session Retrospectives section. See ## Report for proof and what was left out. Review outstanding.

## Report

Agent Factory's audit subskill now carries codex-reflect's harness-neutral retrospective method, so deleting codex-reflect (atelier-6oxt) loses none of it. `procedures/audit.md` gains a 15-line `## Session Retrospectives` section, at the 15-line cap. It says:

- Start from raw transcripts, and treat memory and summaries as hypotheses to check.
- Follow subagent spawns back to the root session and judge the run against the root task. A created or forked thread is a related peer, not a parent, and the primary thread is chosen from the task.
- Read the transcript around each failure, recovery and handoff claim before stating it. A child's report or a detected signal is a claim.
- Take Git history only from what the transcript recorded, never from the repository's current state.
- Report what went well, what went wrong and what to change in code, tooling, guidance or skills, each citing a session and event.

No Codex path, Artifact Hub or report app is mentioned (checked by reading the committed text). Line counts, at ceil(length/80) per line: the audit read path (SKILL.md, audit.md, `references/atelier.md`) goes from 231 to 246, within the 300 limit, and the agent-facing skill from 881 to 896, within 1,000. No line in audit.md is over 80 columns. Review finding atelier-nqkq added the peer-thread and Git-history rules and the detected-signal clause.

### Left out, and why

- **Codex paths and log formats** (`~/.codex/sessions`, `session_index.jsonl`, `history.jsonl`, `shell_snapshots`, `logs_*.sqlite`, `<codex_delegation>`/`source_thread_id`, `spawn_agent`, `create_thread`/`fork_thread`, `turn_context`, `patch_apply_end`). They apply only to Codex. The general rules they served, following subagent spawns to the root and treating created or forked threads as peers, are kept.
- **Scripts** (`discover_sessions.py`, `extract_evidence.py`, `create_report_app.py`, `reflect_core.py`) and their tests. They parse Codex JSONL and are deleted with the skill.
- **Artifact Hub and the Svelte report app** (template, `report.ts` authoring, typed phases and actions, viewer layout, validation and tunnel steps). Artifact Hub is being deleted, and the item excludes the report app.
- **STAR-I's fixed section order and the Context/Change/Payoff card format.** Its substance (what went well, what went wrong, what to change, each with evidence) is kept; the format is presentation, and audit's own Findings fields already give each change a problem, evidence and next step.
- **Bounding the analysis window when the root session spans several tasks.** It mattered for the extractor's `--since`/`--until` windows. Judging the run against the root task covers the general point, and a line of its own would pass the 15-line cap.
- **Stating the mission outcome separately from operational rates** (clean-completion and recovery percentages). The rates are computed by codex-reflect's extractor, so without it nobody produces them. Judging the run against the root task already makes the outcome the measure.
- **Token and metric semantics** (`references/metrics.md`: token snapshot attribution by model, linked-session confidence, recovery rates). They describe Codex's counters. The one general caveat, that a gap between logged events does not prove idleness, is kept.
- **Delegation thresholds for large retrospectives** (one analyst up to 3 sessions, partitioning at 4, a cross-thread analyst at 8). Orchestrate already governs delegation, and the numbers were tuned to the report app's workflow. Reusing an existing artifact before re-extracting is left out with Artifact Hub.
- Treating detected links and automatic failure or recovery signals as candidates is folded into the section ("a detected signal is a claim"), not left out.
