---
created_at: "2026-11-10T00:56:09.383226546+00:00"
id: "atelier-t747"
issue_type: "task"
labels:
- "redesign"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1oel"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-a4rs"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-orb2"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-psjw"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Run the skill against no skill on Claude Opus and Codex, and cut what no scenario supports"
updated_at: "2026-11-11T04:27:50.146248922+00:00"
---

## Goal

Find out which parts of the new skill change what an agent does, and cut the rest. The skill has never been shown to beat no skill: the one comparison found no gain and was contaminated. This item runs the new skill (`skills/agent-factory-next/`) against no skill on the new binary, on Claude Opus 5.5 and on `gpt-6.1-sol`, three runs per cell, and leaves in the skill only what a scenario supports. It is the mission criterion `skill-earns-its-place`. A small skill is an acceptable result.

## Done when

- [ ] **The no-skill arm is run first**, on all ten scenarios for the new binary (`next-loop`, `next-loop-changes` and the eight atelier-a4rs adds), three runs each on Claude Opus 5.5 and three on `gpt-6.1-sol`: 60 runs. Every `final.md` is read by hand, and the transcript of every graded failure and every doubtful pass.
- [ ] **The skill arm is run where it can tell something**, three runs per model:
  - on the three entry scenarios (`next-loop`, `next-loop-changes`, `next-non-atelier-repo`), where the question is whether the skill does harm;
  - on each other scenario where the no-skill arm failed at least two of three runs on at least one model. Where no skill already passes two or three of three on both models, the skill cannot be shown to matter there at this sample size, and the skill arm is not run.
  No cell is run more than three times, whatever it shows. A run the harness marks invalid is not a run and is replaced. A scenario whose skill arm was not run is recorded as not run, and supports no kept passage.
- [ ] **A results file**, `evals/agent-factory/results/<date>-next-skill.md`, records every cell: scenario, arm, model, passes by hand count with the grader's count where they differ, the skill's commit, the binary's commit, the harness and CLI versions, and for each failure what the agent did and which text, or absence of text, explains it. It says where the run directories are. No transcript is committed.
- [ ] **Each passage of the skill is kept or cut by one rule**, and the Report has a table of every passage with its class, its scenario, the counts and the outcome:
  - *Fixed.* The independent review rule, the three exits of a fix-now finding, and the Review rules the binary leaves to the skill (a rebuttal's one try, a fresh reviewer for it, escalation after three rounds, who decides) are Sumeet's and stay whatever the runs show. The results say how each fared.
  - *Entry.* Running `status` first and stepping aside outside an Atelier repository, routing to one role file, who works with Sumeet, and how another thread is started stay, as long as the skill arm does no harm on the entry scenarios: on no model does it pass two fewer runs than no skill.
  - *Everything else* in `SKILL.md` and the five role files is kept only if a scenario shows it changed the outcome: the skill arm passes at least two more runs of three than the no-skill arm on one model, and does not pass two fewer on the other. Otherwise it is cut. That includes every passage with no scenario. A tie is a reason to cut: text can be added back when a real failure shows the need, and the Report lists each cut passage so that it can be.
  - Where the skill arm does harm (two fewer passes than no skill on a model), the passage that caused it, found by reading the failing transcripts, is cut. Nothing is reworded and rerun: a reworded passage would be untested text. If the passage is one of the fixed rules, it stays, and the Report says what harm it did and in which runs, for Sumeet.
  - `procedures/garden.md`, `groom.md` and `audit.md` are not judged and not changed; mission 4 rewrites them.
- [ ] **The skill after the cuts** still has `SKILL.md` and the five role files, each of which may be short; names no removed command; and every passage outside the fixed and entry classes names, in the Report's table, the scenario where it changed the outcome. The Report gives `wc -l` per file and in total against the live skill's 1,223 agent-facing lines (the strategy says 1,219), with and without the three carried procedures.
- [ ] **Limits kept.** At most two eval agents worked at once, children included: the matrix was run as atelier-a4rs's harness enforces it, threads scenarios one at a time and the others two at a time, and the Report gives the peak the harness printed for each matrix. The canary passed before each matrix. The Report gives the number of runs and, where the harness reports it, their cost.

## Scope

`skills/agent-factory-next/` (cuts only), `evals/agent-factory/results/`, a line in `evals/agent-factory/README.md` pointing at the results, and this item's record. A grader found wrong during the runs is fixed in its scenario directory with its unit fixture, named in the Report, and the affected runs are regraded from their kept transcripts, not rerun. Not `skills/agent-factory/`, not `next/`, and no new scenario. A gap in what the new binary prints that a run exposes is reported for the orchestrator to file, not fixed here and not papered over with skill text.

Eval agents run as the user with full access to `$HOME` and the network. Run only the ten scenarios, read each before running it, and keep work directories under `/var/tmp/af-evals` and run output under `~/.cache/af-evals/runs/`. If Claude or Codex fails with an authentication or usage-limit error, stop the matrix and report; do not retry in a loop.

## Context

Mission atelier-qr5s, step B of `docs/strategies/redesign-3-cutover.md`: Shape ("a procedure stays only if an eval shows it matters", and which rules are fixed), Derisking ("if a scenario cannot [tell the arms apart], its procedure is cut. That is the cheap direction to be wrong in"), and Authority. The method is `skills/agent-factory/maintainers/skill-evaluation.md`. The skill and its table of rules and scenarios are atelier-orb2's; the scenarios and the harness's four cells are atelier-a4rs's.

Settled at planning, from this item's old Unknown: planning finding atelier-395f argued that three tied runs are inconclusive and that judgment rules should be cut only on better evidence. The strategy answers it the other way, and this plan follows the strategy: at three runs a difference of one is noise, so only a difference of two counts, and a passage that cannot show one is cut and listed. Running the no-skill arm first is what keeps that affordable: a scenario no-skill agents already pass is not paid for twice.

The binary under test is built from master once atelier-1oel and atelier-psjw have merged, so that a Codex agent can review with its own subagent and `ready` commits; the Report names its commit.

Expected: 60 no-skill runs, 18 entry runs with the skill, and up to 42 more. The skill change is deletions.

## Unknowns

- Whether three runs per cell separate the arms anywhere. If they do nowhere, the skill is its fixed and entry text and the three carried procedures, and that is the result.
