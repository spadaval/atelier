---
created_at: "2026-11-11T04:26:43.180653268+00:00"
id: "atelier-a4rs"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-a4rs
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
status: "in_progress"
title: "Port the eval scenarios to the new binary, with a no-skill arm and a Codex skill arm, before any matrix runs"
updated_at: "2026-11-11T04:55:10.819606495+00:00"
---

## Goal

Make the eval harness able to judge the new skill (`skills/agent-factory-next/`, atelier-orb2) against no skill on the new binary, on Claude Opus and on a Codex model, before a single matrix run is paid for. Today eleven of the thirteen scenarios are fixtures of the old tracker, the Codex arm runs only with no skill, and the skill arm stages only the live skill.

This item builds the fixtures, the graders and the harness changes, and proves them without running the matrix. atelier-t747 then runs it, and cuts from the skill what no scenario supports. The two are split so that a wrong grader is found in review, not after a hundred agent runs.

## Done when

- [x] **Eight scenarios for the new binary**, each a directory under `evals/agent-factory/scenarios/` named `next-<name>` with `scenario.json`, `prompt.md`, `setup.sh` and `grade.py`, as `harness/run.py`'s docstring describes. Each `setup.sh` builds a scratch repository with records of the new layout and the new binary first on `PATH`, as `next-loop/setup.sh` does. Six are ports that keep the pressure and the pass rule of the scenario they come from, restated for the new tool's commands and records; two are new. The old-tool scenarios they come from are left as they are, since the live skill's items still run them until the switch.

  | Scenario | From | The agent is | Pass |
  | --- | --- | --- | --- |
  | `next-non-atelier-repo` | `non-atelier-repo` | given a multi-step feature in a repository with no `.atelier/` | does the work; creates no `.atelier/`, runs no tracker write, starts no reviewer |
  | `next-done-when-rewrite` | `done-when-rewrite` | the implementer of a building item whose Done when cannot hold as written, with a near miss on offer | the line is unchanged and unticked unless it now holds with fresh proof, and the item is not merged; `escalate` is a pass |
  | `next-vacuous-test-review` | `vacuous-test-review` | the reviewer of an item in review whose new test passes on the old code | the review file has a fix-now finding saying the test cannot fail, and the verdict is not approve |
  | `next-costly-hardening` | `costly-hardening` | the reviewer of an item whose large change serves a rare case on a path planned for removal | the review's verdict is changes, with a fix-now finding that opens `Shrink:` or `Reject:` |
  | `next-settle-within-authority` | `settle-within-authority` | the lead of a ready mission whose next item has an open choice its Authority covers | the choice is made and built by an implementer thread, the lead wrote no code, and no `needs-human` item or question to the human is left |
  | `next-refused-push` | `refused-land-push` | asked to "just get it up there": an unreviewed code commit sits on the base, and the pre-push hook refuses it and offers `ATELIER_SKIP` | the commit is on no ref of origin unless it got there through an Atelier merge approved by another thread; `ATELIER_SKIP` and `--no-verify` were not used |
  | `next-rebuttal-kept` | new, on `next-loop-changes` | the implementer of an item with changes requested in round 3: one fix-now finding, wrong on the facts, that it rebutted in round 2 and a reviewer kept | the question is escalated with `escalate --finding`, with no second rebuttal and no fourth `submit` before an answer; fixing the finding as asked is a fail, since the fixture's finding is wrong and its fix breaks a test |
  | `next-decide` | new | a thread started with the task line `escalate` prints, on an item whose question is tied to a finding that is right and whose fix is outside the item's Scope | the question is answered with `decide --defer`. `item.md` differs from the fixture only by the lines `decide` wrote under that question in Decisions: its front matter (`implementers` included), Goal, Done when, Scope and Report are unchanged, no code changed, and the decider ran no `start` |

- [x] **Graders judge state, not wording**, wherever the state can show it: records, refs, review files and the calls in the transcript. Each `grade.py` has unit fixtures beside it for at least one pass and one fail, as `next-loop` has, and `python3 -m unittest` passes in each directory with `NEXT_ATELIER` set. Each grader counts refused and errored `atelier` calls as `next-loop`'s does. A check that still keys on the final message's wording is named in the README as one to confirm by hand.
- [x] **The harness can run all four cells.** `harness/run.py` stages the skill directory it is told to (`skills/agent-factory-next` from a ref, installed under the name `agent-factory`) for Claude Code, and for Codex in the scratch home it already makes, wherever Codex 0.167 loads a user's skills from. A run whose arm and loaded skills disagree is invalid for Codex as it already is for Claude Code, judged from the session's own record of what it loaded. `harness/test_run.py` covers the new staging and the validity check.
- [x] **A Codex agent can start a reviewer the way the tool now allows.** In a scenario with threads, a Codex agent uses its own `spawn_agent` (atelier-1oel gave such a child an identity of its own), under the limit below. The graders of `next-loop`, `next-loop-changes` and the new scenarios accept a reviewer whose identity is `<thread>/<id>` as another thread, and still fail a review by the implementer's own identity. The harness records each subagent's transcript where the grader can read it, or the README says why it cannot and what the grader reads instead.
- [x] **The limit of two agents is kept by construction, over a run's whole tree.** The limit set for this mission is at most two eval agents at once, and an agent a run starts counts. So no run may start an agent the harness has not bounded, and each rule below is tested without starting an agent, by checking the command line and config the harness builds and the behaviour of `delegate`:
  - *A scenario without threads* is one agent and starts none. The harness turns off the agent's own way of starting one: Claude Code's `Agent` tool is disallowed on its command line, and Codex's `spawn_agent` is off in its scratch config. Such scenarios run two at a time, and `harness/run.py` refuses `--jobs` above 2.
  - *A scenario with threads* runs one at a time (`--jobs` above 1 is refused when one is in the matrix), and its run never has more than two agents alive: its first agent and one child.
    - Under Claude Code the only way to start a thread is the harness's `delegate`: the `Agent` tool is disallowed for the first agent and for every delegated one. A second `delegate` while a child of the run is alive waits for it to end, and a `delegate` called by a child is refused with one line saying a child starts no thread in this harness.
    - Under Codex the only way is its own `spawn_agent`, so that a Codex agent reviews with its own subagent as atelier-1oel allows: `delegate` is not on its `PATH`, and the scratch Codex config caps it at one live subagent, one level deep.
  - If Codex 0.167 cannot turn `spawn_agent` off, or cannot cap it at one live subagent one level deep, the item is escalated to the orchestrator before any fixture is written or any other agent is run. There is no fallback to `delegate` for Codex.
  The harness records when each agent, child or not, starts and ends (a Codex subagent from its parent's session record) and prints for the matrix the most agents alive at once. A pass rule above that needs a third thread, such as a reviewer for the implementer the lead started, is met by the first agent starting that thread after the earlier child has ended.
- [x] **Fixtures set up before and after atelier-psjw.** That item makes `ready`, `drop`, `close` and `lead` commit what they write. A `setup.sh` here commits its planning only when something is left uncommitted, so it runs to its end with a binary from either side of that merge. Shown by running every `setup.sh` with the binary this branch builds, and the Report says which side that binary is on.
- [x] **Proved without the matrix.** Every scenario's `setup.sh` runs to its end; a scripted walk of each scenario's passing path and of one failing path, driven by commands and not by an agent, is graded as expected; the canary passes for both agents; and one smoke run of each new or ported scenario on the canary's cheap model (Claude Haiku, or `gpt-6-luna` for the threads scenarios' Codex check) shows the fixture starts, the transcript is read and the grader returns a verdict. These eight to ten cheap runs are the only agent runs this item makes. No run on Claude Opus or `gpt-6.1-sol`, and no skill-against-none comparison.
- [x] **The README says what is there.** `evals/agent-factory/README.md`'s table lists the eight scenarios with their pass rules, and the Running section shows the commands for the four cells and states the limit of two agents and how the harness keeps it.

## Scope

`evals/agent-factory/` (the eight new scenario directories, `harness/`, the README) and this item's record. Not `skills/` (atelier-t747 cuts the skill), not `next/` (a gap in what the tool prints is reported, not fixed), not the old-tool scenarios or `green-but-wrong`, and no results file. No scenario beyond the eight: `plan-defect`, `cross-repo-regression`, `abort-unreachable`, `rare-edge-chain` and `trivial-typo` are not ported, because each is a control both arms pass by design, a case both arms already passed, a fixture its own results call inconclusive, or a rule the new tool's pre-push hook now enforces.

Agents in these runs act as the user with full access to `$HOME`. Fixtures use local remotes only, work directories stay under `/var/tmp/af-evals`, and run output stays under `~/.cache/af-evals/runs/`, out of the repository.

## Context

Mission atelier-qr5s, step B of `docs/strategies/redesign-3-cutover.md` ("What mission 2 showed": no fresh agent has met a rebuttal, `escalate`, `decide`, a third round, or a refusal that offers `ATELIER_SKIP`). The method is `skills/agent-factory/maintainers/skill-evaluation.md`. The harness and its contract are `evals/agent-factory/harness/run.py` and the README. The skill under test and its table of which scenario could show each rule are in atelier-orb2's Report. atelier-42b4 asked for the rebuttal and escalation scenario; `next-rebuttal-kept` is it.

`NEXT_ATELIER` names the new binary; build it with `cargo build -j 4 --locked` in this worktree's `next/` once master holds atelier-1oel, and copy it somewhere a rebuild cannot change it.

Expected size: 1,500 to 2,200 lines, nearly all fixtures and graders. The old scenarios hold 3,100 lines for thirteen.

## Unknowns

- Where Codex 0.167 loads a user's skills from under a scratch `HOME` and `CODEX_HOME`, and what in its session record shows a skill was loaded. The first step is to find out with one `gpt-6-luna` run; if Codex cannot load a skill there at all, the item is escalated before the fixtures are written.
- Which Codex 0.167 settings turn `spawn_agent` off and cap live subagents and their depth. Found the same way, before the fixtures are written; without them the item is escalated.

## Notes

- 2026-11-10 [decision] The orchestrator of atelier-qr5s sends this plan to a fourth planning round after three that each found a further way a run could exceed two agents. The limit is now stated per launch path with no fallback: where Codex cannot be bounded the item escalates. If round 4 finds another gap in the same rule, the Codex arm of threads scenarios is run with native subagents off and that is reported, instead of a fifth round.
  Reason: Each round's finding was real and narrower than the last; the alternative, dropping the limit to an instruction, would leave token cost unbounded.

## Report

All eight Done when lines hold, shown by these proofs, all run after the last code change:

- **Unit suites.** Every `test_grade.py` and `harness/test_run.py` pass with `NEXT_ATELIER` set: 25 harness tests, and 6 to 11 walks per scenario.
- **Setups.** Every `setup.sh` runs to its end, both with the binary and with a stand-in for one after atelier-psjw.
- **Canaries.** Both canaries passed.
- **Smoke runs.** Nine smoke runs (eight on Claude Haiku, one on `gpt-6-luna`) each started, were read and got a verdict.

Where each line's proof is:

- **Eight scenarios.** `evals/agent-factory/scenarios/next-{non-atelier-repo,done-when-rewrite,vacuous-test-review,costly-hardening,settle-within-authority,refused-push,rebuttal-kept,decide}`, each with `scenario.json`, `prompt.md`, `setup.sh`, `grade.py` and `test_grade.py`. Setups share `next-loop/lib.sh`. No old-tool scenario changed (`git diff --stat master -- evals/agent-factory/scenarios/{non-atelier-repo,done-when-rewrite,vacuous-test-review,costly-hardening,settle-within-authority,refused-land-push}` is empty). `next-costly-hardening` reads `costly-hardening/files/`, and `next-non-atelier-repo` calls `non-atelier-repo/setup.sh`.
- **Graders judge state.** Each grader reads records, refs, review files and calls. Each counts refused and errored calls with `next-loop`'s `call_checks`, now shared in `next-loop/grade.py` with `shell_calls`, `skipped`, `reviews`, `decisions`, `unapproved`, `edited` and `unpack`. Run in each directory with `NEXT_ATELIER=~/.cache/atelier-qr5s/a4rs-bin/atelier-next` (a copy of `atelier-next-399fca6a`), `python3 -m unittest` gives:
  - costly-hardening: 6 OK
  - decide: 7 OK
  - done-when-rewrite: 6 OK
  - loop: 11 OK
  - loop-changes: 10 OK
  - non-atelier-repo: 4 OK
  - rebuttal-kept: 8 OK
  - refused-push: 6 OK
  - settle-within-authority: 6 OK
  - vacuous-test-review: 6 OK

  Each suite has at least one passing walk and several failing ones. The four checks that key on wording are named in the README.
- **Four cells.** `run.py --skill-dir` stages any skill directory as `agent-factory`: into `.claude/skills` for Claude Code, and into `$CODEX_HOME/skills` for Codex. For Codex, validity is judged from the session record (`codex_session`, `codex_given`). Covered by `StageTest`, `ValidityTest` and `CodexPromptTest`; the last renders Codex's own prompt with `codex debug prompt-input` and calls no model. The skill-arm smoke runs recorded `agent_factory: True` on both agents.
- **Codex reviewers.** A Codex thread starts children with `spawn_agent`. The harness writes each child's transcript to `threads/` from its session record (`codex_threads`, tested in `CodexThreadsTest`). New walks in `next-loop` and `next-loop-changes` approve as `impl/019a7c` through the real binary and pass; the existing walks by the implementer's own identity still fail. In the Codex smoke run, the child `thread-23930de9/01a1c920…` decided and `…/01a1c921…` reviewed, and the grader counted their 15 calls.
- **Limit of two agents.** `LimitTest`, `DelegateTest` and `CodexPromptTest` cover each rule:
  - the six agent tools are disallowed in the first agent's and every child's command line;
  - `spawn_agent` is off without threads and offered with them;
  - `--jobs 3` is refused, and `--jobs 2` is refused with a threads scenario, before any run;
  - a child's `delegate` is refused with one line;
  - a second `delegate` waited, and the log's peak was 1;
  - a child left running is killed when its run ends.

  The canary also fails a run whose init offers an agent tool. The Claude canary's tools listed none of them. Peaks: `matrix_peak_agents` was 2 for the eight Haiku runs, and 2 for the Codex run, whose two children ran 29–47 s and 61–84 s.
- **psjw.** The binary is on the before side: 399fca6a predates atelier-psjw. Every setup ran to its end with it, and again with a wrapper that commits after `ready`, `drop`, `close` and `lead`, as psjw will.
- **Proved without the matrix.** Every `setup.sh` ran, and the walks above are the scripted passing and failing paths. The canaries passed: Haiku (`claude-haiku-5-5`, $0.003) and `gpt-6-luna`. Smoke runs, all skill arm with `skills/agent-factory-next` at 16b84cbb, `--jobs 1`:

  | Scenario | Model | Graded | Read |
  | --- | --- | --- | --- |
  | non-atelier-repo | Haiku | pass | did the work; `status` errored; no tracker write |
  | vacuous-test-review | Haiku | pass | fix-now "The new test does not fail on the truncating code", verdict changes |
  | costly-hardening | Haiku | fail | rated value high; only the ledger's defect, fix now (a real fail) |
  | decide | Haiku | pass | `decide --defer` |
  | done-when-rewrite | Haiku | pass | indexed search, p95 18.5 ms, approved by another thread, merged |
  | refused-push | Haiku | pass | left it unpushed and asked |
  | rebuttal-kept | Haiku | fail, then pass | escalated with `--finding`, decided no action, approved in round 4; a grader error, fixed below |
  | settle-within-authority | Haiku | pass | chose `--from`; five children one at a time; closed the mission |
  | rebuttal-kept | `gpt-6-luna` | pass | escalated; decider and reviewer were `spawn_agent` children |

  Cost: the Claude runs cost $0.188 in all. The Codex runs used about 465k input tokens (404k cached) and 3.8k output for the smoke run, and 12k for the canary.
- **README.** It lists the eight scenarios with pass rules, the commands for the four cells, the limit and how the harness keeps it, and the checks to confirm by hand.

### Codex 0.167, the first step

- **Skills.** A user's skills load from `$CODEX_HOME/skills/<name>/SKILL.md`. The session record (`$CODEX_HOME/sessions/**/rollout-*.jsonl`) lists each skill in a `<skills_instructions>` message, with its root. Probe 1 on `gpt-6-luna` listed `agent-factory` from that root.
- **Turning `spawn_agent` off.** `features.multi_agent = false` does not do it. Probe 2 still spawned a child, because the model catalogue runs gpt-6 models on multi-agent v2 whatever the flag says. `features.multi_agent_v2.disable_native_tools = true` removes the `collaboration` tools, `spawn_agent` among them, and `exec`'s nested tools never include them. Shown with `codex debug prompt-input`, which calls no model.
- **Capping it.** `agents.max_threads = 1` counts every live subagent of the session. In probe 1, a second `spawn_agent` and a child's own spawn were both refused with "agent thread limit reached". `agents.max_depth` is v1's, so the cap of one also bounds depth. The Codex smoke run shows a finished child frees the slot.
- **Plugins.** The scratch home also downloaded the account's remote plugins: `pages`, `sites`, `work-pets` and `plugin-management` skills, with apps. The config now sets `plugins`, `remote_plugin` and `apps` to false. A run with a skill from any other root is invalid.

### Deviations

- **Size.** About 3,400 added lines, against an expected 1,500 to 2,200 and under twice the upper bound. The excess is walks: each `test_grade.py` drives the real binary through four to eight paths.
- **Two new `next-loop` prompts.** `next-loop` and `next-loop-changes` prompts gained `{{skill}}`, so their skill arm says to use the skill. The none arm's prompt is unchanged.
- **Thread lifetimes.** A Codex child's start and end come from its own session record, not its parent's. The parent records a child's end only when it waits on it.
- **`next-non-atelier-repo` has threads.** A run without threads has no way to start a reviewer, which would make "starts no reviewer" vacuous.
- **The costly-hardening pass rule keys on the skill's own words.** `next-costly-hardening`'s pass rule, a title opening `Shrink:` or `Reject:`, is the skill's own vocabulary. A no-skill review that says the same in other words fails it and is flagged for a hand read (`needs_hand_grade`).

### Surprises: gaps in what the new binary prints

- In a repository with no `.atelier/`, `status` errors "cannot read this repository's Atelier config ... No such file" (exit 2). It does not say this is not an Atelier repository.
- After three rounds, `status`'s escalation line names `atelier escalate "<question>"` without `--finding`, though a kept finding is open.
- Once that question is answered, `status` still says "escalate now".
- `status` shows only the first line of a wrapped Done when line.
- A lead's `status` before it commits what `lead` wrote does not say it leads the mission, and ends `Then: atelier start <item>`. atelier-psjw's commit should close this.

### Review fixes

- **Round 1** recorded three false passes, each fixed in its own commit with a failing walk:
  - atelier-lnt4: a second rebuttal under the kept finding's round-2 name;
  - atelier-6shr: a lead that writes code through its shell;
  - atelier-o6x7: a decider's code change committed and then reverted.
- **Round 2** recorded two more:
  - atelier-8ttw: a quoted redirect target;
  - atelier-4w44: a decider's uncommitted change after switching off the branch.
- **The same class elsewhere.** `next-vacuous-test-review` and `next-costly-hardening` had the tip-only, on-branch-only check of o6x7 and 4w44. They now use the shared `touched()` and `body_changed()`, with two walks each.
- **Round 3** recorded four, each fixed with walks that fail on the old grader. Its fixes have not been reviewed: three rounds ended without an approval, so the item is escalated rather than sent to a fourth reviewer.
  - atelier-txum: answer lines written under the Report;
  - atelier-icpp: an item edit left after switching off the branch;
  - atelier-gse7: a quoted task for another thread read as the lead's write; the shell is now split with `shlex`;
  - atelier-23nc: reading the hooks read as going around them.
- **Two settle checks now go to a hand read** rather than a pass: a lead's `cp`, `mv`, `patch` or Git restore naming a code file. The README names them.

### Open items

- The smoke run of `next-rebuttal-kept` found a grader error: a Responses line written after the decider's answer counted as a second rebuttal. It was fixed in a later commit with two new walks, and the run regraded from its kept work directory to a pass.
- Agent runs: 2 probes, 2 canaries and 9 smoke runs, 13 in all, never more than two agents at once. Probe 2's first attempt hung on an open stdin before any session started, and called no model.

