---
created_at: "2026-11-08T05:52:15.097702619+00:00"
id: "atelier-nz6a"
issue_type: "task"
labels:
- "agent-factory"
workflow_branch:
  branch_base: master
  work_branch: atelier-nz6a
priority: "P1"
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
closed_at: "2026-11-08T23:40:01.985234256+00:00"
status: "done"
title: "Make the skill step aside outside Atelier repositories, and give its install lines first"
updated_at: "2026-11-08T23:40:01.985234256+00:00"
---

## Goal

Stop the skill loading where it does not apply, and tell a new user how to install both halves. Its description matches most multi-step software work, so it loads in repositories that do not use Atelier.

## Done when

- [x] The front matter description names Atelier, for example "Use in repositories tracked with Atelier, or to set a repository up with Atelier", and keeps the triggers that matter inside one (missions, delegation, review, diagnosis).
- [x] The first lines of SKILL.md, before any process guidance, say: run `atelier status`. If it succeeds, continue. If it fails with "Not an Atelier repository", stop using the skill, and offer `procedures/install.md` only if the user asked to set up Atelier. If `atelier` is not found, do the same and give the install lines: atelier's release installer (as in `README.md`) and `npx skills add spadaval/atelier --skill agent-factory -g`.
- [x] The plan-file fallback for repositories without Atelier is removed from SKILL.md's Scale section and anywhere else that offers it, and `references/atelier.md` and `procedures/install.md` agree with the new opening.
- [x] `README.md` gives the skill's install line next to the binary's.
- [x] A new eval scenario, `evals/agent-factory/scenarios/non-atelier-repo`, with setup, prompt and grader: a multi-step task in a git repository with no `.atelier/`. Pass when the agent runs `atelier status`, does not run `atelier init` or create `.atelier/`, and does not follow the skill's process (no issues, no review records). Its README row is added.
- [x] All scenarios, including the new one, run once each on the skill arm at this branch with Opus, read by hand, with results in `evals/agent-factory/results/`. Each existing scenario passes as it did in the last recorded results, or the Report explains the difference.
- [x] The skill stays within `atelier-qj9d`'s line limits, or the Report gives the counts.

## Scope

`skills/agent-factory`, `evals/agent-factory` and `README.md`.

## Context

Strategy: docs/strategies/agent-factory-in-atelier.md. The skill stays global; repository-specific guidance stays in the repository's `.atelier/`, `AGENTS.md` and verify skill.

## Notes

- 2026-11-08 [note] Handoff: implementation done at d6f61fe on branch atelier-nz6a (skill commit db05a14b; later commits are records only). Every scenario passed once on the skill arm with Opus, read by hand. See ## Report. Review outstanding; not merged or pushed. Follow-up atelier-wl6u.

## Report

Outside an Atelier repository the skill now steps aside, and its first lines give both install lines. The new `non-atelier-repo` scenario passed (1/1, read by hand): the agent ran `atelier status`, read "Not an Atelier repository", said it would not use the skill, ran no `atelier init`, created no `.atelier/`, wrote no plan or review files and did the feature directly. All eight existing scenarios passed once each on the skill arm with Opus at `db05a14b`, read by hand (`evals/agent-factory/results/2026-11-08-step-aside.md`). The skill stays within its limits: agent-facing total 881 to 883 of 1,000, SKILL.md 100 to 100, and the install path 263 to 265. No other path changed, so review (322), orchestrate (467) and plan (306) still overrun by the same amounts as before.

Each Done when line, checked against the committed text:

- **Description:** "Use in repositories tracked with Atelier, or to set a repository up with Atelier: planning or running a mission, delegating work to subagents, making a consequential product or architecture decision, diagnosing an unexplained failure, or independently reviewing or validating agent-written code."
- **Opening:** SKILL.md's first paragraph, before Core Rules, says to run `atelier status` and continue if it succeeds. On "Not an Atelier repository", or when `atelier` is not found, it says to stop using the skill and to offer `install` only if the user asked to set up Atelier. When `atelier` is not found, it also gives the README's `curl` installer line and `npx skills add spadaval/atelier --skill agent-factory -g`.
- **Fallback:** SKILL.md's Scale sentence offering a plan file or the PR is gone. A grep for plan file, fallback and "without it" in `skills/agent-factory` finds no other offer. `procedures/install.md` now opens "when the user asks to set a repository up with Atelier", and its Tracker section points at SKILL.md's install lines when `atelier` is missing. `references/atelier.md` never offered a fallback and is unchanged, because it is on every path and any line added there grows the three paths that already overrun.
- **README:** the skill's `npx skills add` line follows the binary's installer under Install.
- **Scenario:** `evals/agent-factory/scenarios/non-atelier-repo` has `setup.sh`, `prompt.md`, `grade.py` and `scenario.json`, and the README has its row. Before the matrix, I checked the grader against a fake transcript that ran `atelier status` (pass), and against one that ran `atelier init` and wrote `PLAN.md` (fail, all four reasons listed).
- **Matrix and limits:** see the first paragraph.

### Deviations

- **"offer `install`" instead of "offer `procedures/install.md`":** the literal path costs one line, and SKILL.md has none to spare. SKILL.md's subskill list says each subskill is `procedures/<name>.md`, so the two name the same file.
- **SKILL.md cuts that pay for the opening:** the Scale paragraph lost its heading and now sits under the opening. "Say which subskill you are using, and why" moved into the Subskills paragraph. The description dropped "or a multi-issue increment".

### Surprises

- **Two scenarios no longer test the skill.** The fixtures of `cross-repo-regression` (a stand-in `tasks.py` tracker) and `vacuous-test-review` (no tracker) are not Atelier repositories, so the skill now steps aside in both. Both still passed. That is only because each run read `procedures/review.md` in the same shell call as `atelier status`, before it saw the result. Filed atelier-wl6u to port both fixtures to Atelier.

### Open items

- atelier-wl6u, above.
- One run per scenario shows nothing broke. It cannot show a small change in pass rates.
- Whether `npx skills add` installs the published skill is not checked here; atelier-hrgx covers it.
