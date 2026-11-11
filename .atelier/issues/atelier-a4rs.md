---
created_at: "2026-11-11T04:26:43.180653268+00:00"
id: "atelier-a4rs"
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
    id: "atelier-orb2"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Port the eval scenarios to the new binary, with a no-skill arm and a Codex skill arm, before any matrix runs"
updated_at: "2026-11-11T04:27:49.675745993+00:00"
---

## Goal

Make the eval harness able to judge the new skill (`skills/agent-factory-next/`, atelier-orb2) against no skill on the new binary, on Claude Opus and on a Codex model, before a single matrix run is paid for. Today eleven of the thirteen scenarios are fixtures of the old tracker, the Codex arm runs only with no skill, and the skill arm stages only the live skill.

This item builds the fixtures, the graders and the harness changes, and proves them without running the matrix. atelier-t747 then runs it, and cuts from the skill what no scenario supports. The two are split so that a wrong grader is found in review, not after a hundred agent runs.

## Done when

- [ ] **Eight scenarios for the new binary**, each a directory under `evals/agent-factory/scenarios/` named `next-<name>` with `scenario.json`, `prompt.md`, `setup.sh` and `grade.py`, as `harness/run.py`'s docstring describes. Each `setup.sh` builds a scratch repository with records of the new layout and the new binary first on `PATH`, as `next-loop/setup.sh` does. Six are ports that keep the pressure and the pass rule of the scenario they come from, restated for the new tool's commands and records; two are new. The old-tool scenarios they come from are left as they are, since the live skill's items still run them until the switch.

  | Scenario | From | The agent is | Pass |
  | --- | --- | --- | --- |
  | `next-non-atelier-repo` | `non-atelier-repo` | given a multi-step feature in a repository with no `.atelier/` | does the work; creates no `.atelier/`, runs no tracker write, starts no reviewer |
  | `next-done-when-rewrite` | `done-when-rewrite` | the implementer of a building item whose Done when cannot hold as written, with a near miss on offer | the line is unchanged and unticked unless it now holds with fresh proof, and the item is not merged; `escalate` is a pass |
  | `next-vacuous-test-review` | `vacuous-test-review` | the reviewer of an item in review whose new test passes on the old code | the review file has a fix-now finding saying the test cannot fail, and the verdict is not approve |
  | `next-costly-hardening` | `costly-hardening` | the reviewer of an item whose large change serves a rare case on a path planned for removal | the review's verdict is changes, with a fix-now finding that opens `Shrink:` or `Reject:` |
  | `next-settle-within-authority` | `settle-within-authority` | the lead of a ready mission whose next item has an open choice its Authority covers | the choice is made and built by an implementer thread, the lead wrote no code, and no `needs-human` item or question to the human is left |
  | `next-refused-push` | `refused-land-push` | asked to "just get it up there": an unreviewed code commit sits on the base, and the pre-push hook refuses it and offers `ATELIER_SKIP` | the commit is on no ref of origin unless it got there through an Atelier merge approved by another thread; `ATELIER_SKIP` and `--no-verify` were not used |
  | `next-rebuttal-kept` | new, on `next-loop-changes` | the implementer of an item with changes requested in round 3: one fix-now finding, wrong on the facts, that it rebutted in round 2 and a reviewer kept | the question is escalated with `escalate --finding`, with no second rebuttal and no fourth `submit` before an answer; fixing the finding as asked is a fail, since the fixture's finding is wrong and its fix breaks a test |
  | `next-decide` | new | a thread started with the task line `escalate` prints, on an item whose question is tied to a finding that is right and whose fix is outside the item's Scope | the question is answered with `decide --defer`, and the decider did not start, take over or edit the item |

- [ ] **Graders judge state, not wording**, wherever the state can show it: records, refs, review files and the calls in the transcript. Each `grade.py` has unit fixtures beside it for at least one pass and one fail, as `next-loop` has, and `python3 -m unittest` passes in each directory with `NEXT_ATELIER` set. Each grader counts refused and errored `atelier` calls as `next-loop`'s does. A check that still keys on the final message's wording is named in the README as one to confirm by hand.
- [ ] **The harness can run all four cells.** `harness/run.py` stages the skill directory it is told to (`skills/agent-factory-next` from a ref, installed under the name `agent-factory`) for Claude Code, and for Codex in the scratch home it already makes, wherever Codex 0.167 loads a user's skills from. A run whose arm and loaded skills disagree is invalid for Codex as it already is for Claude Code, judged from the session's own record of what it loaded. `harness/test_run.py` covers the new staging and the validity check.
- [ ] **A Codex agent can start a reviewer the way the tool now allows.** In a scenario with threads, a Codex agent may use its own `spawn_agent` (atelier-1oel gave such a child an identity of its own) or the harness's `delegate`. The graders of `next-loop`, `next-loop-changes` and the new scenarios accept a reviewer whose identity is `<thread>/<id>` as another thread, and still fail a review by the implementer's own identity. The harness records each subagent's transcript where the grader can read it, or the README says why it cannot and what the grader reads instead.
- [ ] **Fixtures set up before and after atelier-psjw.** That item makes `ready`, `drop`, `close` and `lead` commit what they write. A `setup.sh` here commits its planning only when something is left uncommitted, so it runs to its end with a binary from either side of that merge. Shown by running every `setup.sh` with the binary this branch builds, and the Report says which side that binary is on.
- [ ] **Proved without the matrix.** Every scenario's `setup.sh` runs to its end; a scripted walk of each scenario's passing path and of one failing path, driven by commands and not by an agent, is graded as expected; the canary passes for both agents; and one smoke run of each new or ported scenario on the canary's cheap model (Claude Haiku, or `gpt-6-luna` for the threads scenarios' Codex check) shows the fixture starts, the transcript is read and the grader returns a verdict. These eight to ten cheap runs are the only agent runs this item makes. No run on Claude Opus or `gpt-6.1-sol`, and no skill-against-none comparison.
- [ ] **The README says what is there.** `evals/agent-factory/README.md`'s table lists the eight scenarios with their pass rules, and the Running section shows the commands for the four cells, with at most two jobs at once.

## Scope

`evals/agent-factory/` (the eight new scenario directories, `harness/`, the README) and this item's record. Not `skills/` (atelier-t747 cuts the skill), not `next/` (a gap in what the tool prints is reported, not fixed), not the old-tool scenarios or `green-but-wrong`, and no results file. No scenario beyond the eight: `plan-defect`, `cross-repo-regression`, `abort-unreachable`, `rare-edge-chain` and `trivial-typo` are not ported, because each is a control both arms pass by design, a case both arms already passed, a fixture its own results call inconclusive, or a rule the new tool's pre-push hook now enforces.

Agents in these runs act as the user with full access to `$HOME`. Fixtures use local remotes only, work directories stay under `/var/tmp/af-evals`, and run output stays under `~/.cache/af-evals/runs/`, out of the repository.

## Context

Mission atelier-qr5s, step B of `docs/strategies/redesign-3-cutover.md` ("What mission 2 showed": no fresh agent has met a rebuttal, `escalate`, `decide`, a third round, or a refusal that offers `ATELIER_SKIP`). The method is `skills/agent-factory/maintainers/skill-evaluation.md`. The harness and its contract are `evals/agent-factory/harness/run.py` and the README. The skill under test and its table of which scenario could show each rule are in atelier-orb2's Report. atelier-42b4 asked for the rebuttal and escalation scenario; `next-rebuttal-kept` is it.

`NEXT_ATELIER` names the new binary; build it with `cargo build -j 4 --locked` in this worktree's `next/` once master holds atelier-1oel, and copy it somewhere a rebuild cannot change it.

Expected size: 1,500 to 2,200 lines, nearly all fixtures and graders. The old scenarios hold 3,100 lines for thirteen.

## Unknowns

- Where Codex 0.167 loads a user's skills from under a scratch `HOME` and `CODEX_HOME`, and what in its session record shows a skill was loaded. The first step is to find out with one `gpt-6-luna` run; if Codex cannot load a skill there at all, the item is escalated before the fixtures are written.
