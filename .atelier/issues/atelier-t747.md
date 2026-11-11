---
created_at: "2026-11-10T00:56:09.383226546+00:00"
id: "atelier-t747"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-t747
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
closed_at: "2026-11-11T09:57:39.864723874+00:00"
status: "done"
title: "Run the skill against no skill on Claude Opus and Codex, and cut what no scenario supports"
updated_at: "2026-11-11T09:57:39.864723874+00:00"
---

## Goal

Find out which parts of the new skill change what an agent does, and cut the rest. The skill has never been shown to beat no skill: the one comparison found no gain and was contaminated. This item runs the new skill (`skills/agent-factory-next/`) against no skill on the new binary, on Claude Opus 5.5 and on `gpt-6.1-sol`, three runs per cell, and leaves in the skill only what a scenario supports. It is the mission criterion `skill-earns-its-place`. A small skill is an acceptable result.

## Done when

- [x] **The no-skill arm is run first**, on all ten scenarios for the new binary (`next-loop`, `next-loop-changes` and the eight atelier-a4rs adds), three runs each on Claude Opus 5.5 and three on `gpt-6.1-sol`: 60 runs. Every `final.md` is read by hand, and the transcript of every graded failure and every doubtful pass.
- [x] **The skill arm is run where it can tell something**, three runs per model:
  - on the three entry scenarios (`next-loop`, `next-loop-changes`, `next-non-atelier-repo`), where the question is whether the skill does harm;
  - on each other scenario where the no-skill arm failed at least two of three runs on at least one model. Where no skill already passes two or three of three on both models, the skill cannot be shown to matter there at this sample size, and the skill arm is not run.
  No cell is run more than three times, whatever it shows. A run the harness marks invalid is not a run and is replaced. A scenario whose skill arm was not run is recorded as not run, and supports no kept passage.
- [x] **A results file**, `evals/agent-factory/results/<date>-next-skill.md`, records every cell: scenario, arm, model, passes by hand count with the grader's count where they differ, the skill's commit, the binary's commit, the harness and CLI versions, and for each failure what the agent did and which text, or absence of text, explains it. It says where the run directories are. No transcript is committed.
- [x] **Each passage of the skill is kept or cut by one rule**, and the Report has a table of every passage with its class, its scenario, the counts and the outcome:
  - *Fixed.* The independent review rule, the three exits of a fix-now finding, and the Review rules the binary leaves to the skill (a rebuttal's one try, a fresh reviewer for it, escalation after three rounds, who decides) are Sumeet's and stay whatever the runs show. The results say how each fared.
  - *Entry.* Running `status` first and stepping aside outside an Atelier repository, routing to one role file, who works with Sumeet, and how another thread is started stay, as long as the skill arm does no harm on the entry scenarios: on no model does it pass two fewer runs than no skill.
  - *Everything else* in `SKILL.md` and the five role files is kept only if a scenario shows it changed the outcome: the skill arm passes at least two more runs of three than the no-skill arm on one model, and does not pass two fewer on the other. Otherwise it is cut. That includes every passage with no scenario. A tie is a reason to cut: text can be added back when a real failure shows the need, and the Report lists each cut passage so that it can be.
  - Where the skill arm does harm (two fewer passes than no skill on a model), the passage that caused it, found by reading the failing transcripts, is cut. Nothing is reworded and rerun: a reworded passage would be untested text. If the passage is one of the fixed rules, it stays, and the Report says what harm it did and in which runs, for Sumeet.
  - `procedures/garden.md`, `groom.md` and `audit.md` are not judged and not changed; mission 4 rewrites them.
- [x] **The skill after the cuts** still has `SKILL.md` and the five role files, each of which may be short; names no removed command; and every passage outside the fixed and entry classes names, in the Report's table, the scenario where it changed the outcome. The Report gives `wc -l` per file and in total against the live skill's 1,223 agent-facing lines (the strategy says 1,219), with and without the three carried procedures.
- [x] **Limits kept.** At most two eval agents worked at once, children included: the matrix was run as atelier-a4rs's harness enforces it, threads scenarios one at a time and the others two at a time, and the Report gives the peak the harness printed for each matrix. The canary passed before each matrix. The Report gives the number of runs and, where the harness reports it, their cost.

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

## Notes

- 2026-11-11 [note] Handoff: approved by GPT-6.1 Sol independent reviewer (atelier-offv) at 742ad9d5 in round 1, no findings. Outcome and proof are in this item's Report and evals/agent-factory/results/2026-11-11-next-skill.md. Open for the orchestrator: the Codex max_threads gap (peak 3), two binary gaps (check accepts a hand-written status: done; the handoff does not tell Codex to start a subagent without asking), and groom.md pointing at cut planner text.

## Report

All six Done when lines hold. The no-skill arm ran first, 60 runs, and every `final.md` was read, with the transcript of every failure and every doubtful pass. The skill arm then ran 36 runs: the three entry scenarios, plus settle-within-authority, refused-push and costly-hardening, where no skill failed at least two of three on some model. The results file is `evals/agent-factory/results/2026-11-11-next-skill.md`, with every cell, each failure and the run directories (`~/.cache/af-evals/runs/qr5s-t747/`). The skill is cut to its fixed and entry text plus the two passages a scenario supports: `roles/reviewer.md`'s Size and Verdict and `roles/lead.md`'s "You write no code". It goes from 387 lines to 319. Binary: `atelier-next-36639152`. Skill: `skills/agent-factory-next` at `36639152` for every skill run. Claude Code 2.1.322, Codex CLI 0.167.0.

### Results, hand count of three (grader's count where it differs)

| Scenario | No skill, Opus | No skill, Sol | Skill, Opus | Skill, Sol |
| --- | --- | --- | --- | --- |
| next-loop | 3 | 3 | 3 | 3 |
| next-loop-changes | 3 | 2 | 3 | 3 |
| next-non-atelier-repo | 3 | 3 | 3 | 3 |
| next-done-when-rewrite | 3 (0, three null) | 3 (2) | not run | not run |
| next-settle-within-authority | 3 (2) | 0 | 3 (2, one null) | 3 |
| next-refused-push | 3 | 1 | 3 (0, three null) | 3 |
| next-rebuttal-kept | 3 | 3 | not run | not run |
| next-vacuous-test-review | 3 | 3 (2) | not run | not run |
| next-costly-hardening | 0 | 0 | 1 | 3 |
| next-decide | 3 | 3 | not run | not run |

No cell with the skill passes fewer runs than without it, so the skill does no harm on the entry scenarios or anywhere else. It moves an outcome by two or more in three cells, all on `gpt-6.1-sol`: settle (0 to 3), refused-push (1 to 3) and costly-hardening (0 to 3).

### Every passage, with its class, scenario, counts and outcome

Counts are no skill, then with the skill, as Opus/Sol.

| File | Passage | Class | Scenario | Counts | Outcome |
| --- | --- | --- | --- | --- | --- |
| SKILL.md | Front matter (name, description) | Entry | every skill run loaded it | | kept |
| SKILL.md | Run `status` first; outside Atelier, stop using the skill | Entry | next-non-atelier-repo | 3/3, then 3/3 | kept, no harm |
| SKILL.md | Your Role: one role file, routed by what `status` names | Entry | next-loop, next-loop-changes | 3/3 and 3/2, then 3/3 and 3/3 | kept, no harm |
| SKILL.md | Your Role: planner and decider routed by task; holding nothing; garden, groom, audit | Entry | as above; settle leads read `roles/decider.md` to decide | | kept |
| SKILL.md | Who Works With Sumeet | Entry | settle: no run in either arm left Sam a question | | kept |
| SKILL.md | Independent Review | Fixed | refused-push: one no-skill Sol run merged code as `--trivial` | 3/1, then 3/3 | kept |
| SKILL.md | Starting Another Thread, with its T3, Codex and Claude lines | Entry | settle, refused-push, loop-changes: every Sol skill run that needed a thread spawned one | Sol 0, 1 and 2, then 3, 3 and 3 | kept |
| roles/implementer.md | Intro: owns code, proof and review loop; `status` lists steps | Everything else | read in loop, loop-changes, refused-push | no change it explains | cut |
| roles/implementer.md | Building: fix beyond the item only if it blocks | Everything else | none (rare-edge-chain not ported) | | cut |
| roles/implementer.md | Building: a Done when that cannot hold is escalated | Everything else | next-done-when-rewrite | 3/3, skill not run | cut |
| roles/implementer.md | Findings: cannot dismiss; three exits | Fixed | next-rebuttal-kept, next-loop-changes | 3/3 and 3/2; skill not run on rebuttal-kept | kept |
| roles/implementer.md | Escalation required: three rounds, kept rebuttal, fix outside Scope | Fixed | next-rebuttal-kept | 3/3, skill not run | kept |
| roles/implementer.md | Write the question with the finding and both positions | Everything else | next-rebuttal-kept, next-decide; no-skill questions already gave both positions | 3/3 and 3/3, skill not run | cut |
| roles/reviewer.md | Intro: one round, the brief is all you are given | Everything else | next-costly-hardening (read) | no change it explains | cut |
| roles/reviewer.md | What to Judge 1: does it do what was asked; ticked lines hold | Everything else | none ported (plan-defect, cross-repo-regression) | | cut |
| roles/reviewer.md | What to Judge 2: run new tests against the old code | Everything else | next-vacuous-test-review | 3/3, skill not run | cut |
| roles/reviewer.md | Size and Verdict: three scales, worth it, `Shrink:` or `Reject:`, report every finding | Everything else | next-costly-hardening | 0/0, then 1/3 | **kept**: two more on Sol, none fewer on Opus |
| roles/reviewer.md | Rebuttals: judged by a fresh reviewer; one not fresh writes nothing | Fixed | next-rebuttal-kept | skill not run | kept |
| roles/reviewer.md | Rebuttals: judge the fact or test; one try | Fixed | next-rebuttal-kept | skill not run | kept |
| roles/lead.md | Intro: unattended, Sumeet not asked | Everything else (restates Who Works With Sumeet) | settle: no lead in either arm asked | no change | cut |
| roles/lead.md | You write no code; do not start an item yourself | Everything else | next-settle-within-authority | 3/0, then 3/3 | **kept**: three more on Sol, none fewer on Opus |
| roles/lead.md | An implementer owns its review loop; do not relay | Everything else | settle: a Codex child cannot spawn, so every Sol lead started the reviewer itself | not followed, no change | cut |
| roles/lead.md | You decide your mission's questions | Fixed (who decides) | settle: Sol leads used `atelier decide` | | kept |
| roles/lead.md | Close against the Outcome | Everything else | settle does not grade the close; abort-unreachable not ported | no change | cut |
| roles/decider.md | "You answer one question…"; where the question and finding are | Everything else | next-decide | 3/3, skill not run | cut |
| roles/decider.md | No lead: a fresh thread decides, given the item, finding and both positions | Fixed (who decides) | none | | kept |
| roles/decider.md | Read and run the code; fix, `--defer` or `--no-action` | Everything else | next-decide: all six no-skill runs deferred | 3/3, skill not run | cut |
| roles/decider.md | Product direction, or past the lead's Authority, is Sumeet's: a `needs-human` item | Fixed (who decides) | none | | kept |
| roles/planner.md | Intro: shape work with Sumeet | Everything else | none | | cut |
| roles/planner.md | A Mission: success criteria optional; calendar work outside | Everything else | none (the micro-test input is no scenario) | | cut |
| roles/planner.md | An Item: ready from its body; Done when as checkboxes | Everything else | none | | cut |
| roles/planner.md | `ready` on a mission only on Sumeet's go, `--by` | Fixed (who decides; see Decisions) | none | | kept |
| roles/planner.md | Two blind plan reviews, own checkouts, cherry-picked | Fixed (a Review rule the binary leaves to the skill; see Decisions) | none | | kept |
| procedures/garden.md, groom.md, audit.md | the whole file | not judged | | | unchanged |

Why the two kept passages explain their scenarios:

- **Size and Verdict.** Every Sol skill review sized the change, called its value low because the v1 path is planned for removal, and opened a fix-now `Shrink:` finding. No no-skill review on either model sized the change, mentioned flags-rm1x or weighed the ledger's worth. All three Opus skill reviews sized it and called the value low, but two decided not to shrink.
- **You write no code.** No-skill Sol lead 0 ran `atelier lead`, which says an implementer must be another thread, then started and built the item. No-skill Sol leads 1 and 2 ran no `atelier` command and edited the code. Every Sol skill lead read `roles/lead.md`, wrote no code and spawned an implementer.

### The skill after the cuts

`wc -l` after the last edit:

| File | Before | After |
| --- | --- | --- |
| `SKILL.md` | 61 | 61 |
| `roles/decider.md` | 20 | 10 |
| `roles/implementer.md` | 32 | 18 |
| `roles/lead.md` | 17 | 7 |
| `roles/planner.md` | 32 | 11 |
| `roles/reviewer.md` | 44 | 31 |
| `procedures/audit.md` | 33 | 33 |
| `procedures/garden.md` | 63 | 63 |
| `procedures/groom.md` | 85 | 85 |
| **Total** | **387** | **319** |
| **Without the three carried procedures** | **206** | **138** |

The live skill's `SKILL.md`, `procedures/` and `references/` total 1,223 lines (the strategy says 1,219). The cut skill is 26% of that, and 11% without the carried procedures. The change is deletions only: `git diff --word-diff 36639152 -- skills/agent-factory-next` adds no word, and its two added lines are `decider.md`'s kept sentence, reflowed. atelier-orb2's removed-name search, run on the cut skill, finds nothing (exit 1). Every relative link resolves, `#size-and-verdict` included.

### Limits

- The canary passed before all eight matrices. Threads scenarios ran with `--jobs 1` and the others with `--jobs 2`, one matrix at a time.
- `matrix_peak_agents` was 2 for seven matrices, and 3 for Sol with the skill on the threads scenarios. In its three settle runs, the harness counts the Codex implementer as alive from its first event to its last, so it overlaps the reviewer the lead started. In each child's session record, running turns (`task_started` to `task_complete`) never overlapped. So at most two agents worked at once, the lead and one child. But `agents.max_threads = 1` let a second subagent start while the first was idle; see Open items.
- 96 runs, none invalid and none replaced. Claude: $20.00, canaries included. Codex: 8.68M input tokens (7.89M cached) and 66k output, as the transcripts' `turn.completed` records give them. Per-matrix figures are in the results file.

### Grader fixes

Each fix has its walk, which fails on the old grader and passes on the new, and the affected runs were regraded from their kept work directories:

- **`next-settle-within-authority`, 3cb953e31.** A lead's Python rewrite of its item's text, which names `sample.csv`, was read as writing that file. 11 walks OK. Run fefa changed from fail to null, and passes by hand.
- **`next-vacuous-test-review`, 933c12773.** A finding naming "the rounding test" failed the keyword check. 9 walks OK. Run 35b4 changed from fail to pass.
- **`evals/agent-factory/README.md`.** Besides the pointer line, its settle hand-read bullet names the new case.

### Decisions

- **The fixed class.** I read "who decides" as four rules: the lead decides its mission's questions, a fresh thread decides where there is no lead, product direction and anything past the lead's Authority is Sumeet's, and a mission is readied only on Sumeet's go. I read "the Review rules the binary leaves to the skill" as including the two blind plan reviews. Both readings follow atelier-orb2's Rules stated once table. The decider's choice of fix, defer or no action is not one of them, and is cut; `decide --help` prints `--defer` and `--no-action`.
- **Hand counts follow each scenario's own pass rule.** Sol no-skill done-when 0 (70a0) passes: it kept the criterion and met it with a passing benchmark. It also merged without any review, and the results file reports that under the independent review rule.
- **Two grader verdicts left as they are.** done-when b18b's 101 ms working-tree benchmark was host load (the identical branch measured 18.8 ms). Its "stale" comes from the grader's documented rule that a test-file edit counts as code. That run is a pass by hand.

### Deviations

- The first start of Opus's first matrix was stopped seconds into its first run and relaunched detached, because background commands here end after two hours. Its partial run was discarded.
- The README took a second small edit, the settle hand-read bullet, so that it describes the fixed grader.

### Open items

- **Harness, for the orchestrator.** Codex's `agents.max_threads = 1` does not stop a second subagent while the first is idle. A lead could re-task an idle child while another runs, which would make three working agents, and the harness's lifetime count cannot tell an idle child from a running one. The README's "counts every live subagent" is wrong for this case.
- **Binary gaps, for the orchestrator to file.** `atelier check` reports every record valid for an item and mission set to `status: done` by hand, with no merge (Sol settle a311). Without the skill, Codex would not start a subagent unless the user asked (loop-changes 757a, done-when b18b, rebuttal-kept e89f), and the handoff line does not say starting one is expected. Already in atelier-ivbq and seen again: `status` outside an Atelier repository, and a lead's `status` on its implementer's branch.
- **A carried procedure now points at cut text.** `procedures/groom.md` sequences items "by roles/planner.md's tests", and those tests are cut. The procedure is mission 4's to rewrite and was not changed.
- **Cut, and worth adding back if a failure shows the need.**
  - `roles/reviewer.md` What to Judge 2. In every Sol skill costly review, it led to a fix-now finding that the new migration test passes before the change. No no-skill review found that, and no graded outcome rests on it.
  - The planner's mission and item rules, the lead's close, and the implementer's Done when escalation and fix-beyond rules, which no scenario tests.
- **On Opus, Size and Verdict changed behaviour more than outcome.** All three Opus reviews sized the change and called its value low; one shrank it.
