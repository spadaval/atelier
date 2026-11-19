---
schema: atelier.item
schema_version: 3
id: atelier-8e8e
title: Run a matrix in a named setup, a model and an effort per role, and report time and tokens per cell
status: in_review
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A15ed7340-46a6-4e3a-b0c2-fdc0661b938f%3Adelegate-task%3A4aecee6c-0370-419a-9f62-26c766c86207
ready:
  by: 'the human, in conversation on 2026-11-18: put the mission together and then run it'
  at: 2026-11-18T23:49:47.284668623Z
trivial: null
caused_by: null
created_at: 2026-11-18T20:05:51.873675436Z
---

## Goal

A matrix runs one model in every role at the harness's default effort. The
work host does run `gpt-6.1-sol` in every role, at `high`. This host runs
Fable as lead, Opus as implementer and Sol at `xhigh` as reviewer, and no
run has had a reviewer from another lab than its implementer. Time and
tokens are in each `result.json` and in no table.

## Done when

- [x] `run.py --setup <name>` reads a checked-in setup: an agent (Claude
  Code or Codex), a model and an effort for each of lead, implementer and
  reviewer. Two are checked in. `sol` is Codex with `gpt-6.1-sol` in every
  role, at `high`. `house` is `claude-fable-5-2` as
  lead, `claude-opus-5-5` as implementer and Codex `gpt-6.1-sol` at `xhigh`
  as reviewer.
- [x] A setup also names its judges: a model for each lab its threads come
  from, from another lab than that one. `sol` has one and `house` has two.
- [x] Each case names the role of its first agent, and a thread it starts
  for a role runs as that role's agent, model and effort.
- [x] A setup of one model in every role can be given on the command line,
  for a new model's first run.
- [x] At most two eval agents are alive at once in a setup that mixes Claude
  Code and Codex, and the canary and the init checks hold for each agent of
  the setup. `harness/test_run.py` tests both with no model.
- [x] Every `result.json` names its setup and gives the run's seconds, its
  tokens in and out for each thread, the threads it started and its failed
  `atelier` calls. One command prints a table from `results.jsonl`: for each
  cell its passes, median seconds and tokens.

## Scope

`evals/agent-factory/harness/`. The scenarios change only to name their
first agent's role.

## Report

A matrix now runs in a named setup, and every result gives its time and
tokens. `run.py --setup sol` or `--setup house` reads
`evals/agent-factory/harness/setups/<name>.json`, and `harness/table.py
<out>` prints passes, nulls, median seconds and median tokens in and out for
each setup, case and arm. A matrix also prints its own table when it ends.

It also fixes a bug that made every Codex result so far wrong about its
effort. `codex exec` was never given an effort, so it ran at the model's
default: the session records of the 2026-11-17 smoke runs show `gpt-6.2-sol`
at `low`, while their `result.json` says `high`. Codex now gets
`-c model_reasoning_effort=<effort>`, and a Codex thread whose own record
shows another model or effort than its role's makes the run invalid.

### What changed

- Setups. `sol` is Codex `gpt-6.1-sol` at `high` in every role, with one
  judge, Claude `claude-opus-5-5`. `house` is Claude Code
  `claude-fable-5-2` as lead and `claude-opus-5-5` as implementer, both at
  `high`, and Codex `gpt-6.1-sol` at `xhigh` as reviewer. Its two judges are
  Codex `gpt-6.1-sol` at `xhigh` for Anthropic's threads and Claude
  `claude-opus-5-5` for OpenAI's. `setup_problem` refuses a setup whose
  judges do not match the labs of its threads one for one, or whose judge
  comes from the lab it judges. This item only names the judges;
  atelier-b9o6 puts questions to them.
- One model on the command line. `--agent`, `--model` and `--effort` give
  a setup of one model in every role, named `<model>-<effort>`, with the
  judge from the other lab (`DEFAULT_JUDGES`). With no flags it is still
  Claude `opus` at `high`. `--setup` together with any of them is refused.
- Roles. Each `scenario.json` names `role`: `lead` for
  `next-settle-within-authority`, `reviewer` for the two review cases and
  `implementer` for the other six. The first agent runs as that role's
  agent, model and effort.
- Threads. Outside a setup of one Codex model and effort, every agent, Codex
  included, starts a thread with `delegate <role> "<task>"`. That starts
  the role's agent, model and effort, and a Codex role gets its own
  `CODEX_HOME` at `work/codex/<role>`. A setup of one Codex model and
  effort, such as `sol`, keeps `spawn_agent` as before: a subagent inherits
  its parent's model and effort, as Codex's tool description says.
- The limit of two. `delegate`'s lock is shared by every role and agent,
  Codex's `spawn_agent` is removed wherever `delegate` is installed, and a
  child still cannot delegate. A canary runs for each agent of the setup
  before the matrix: both in `house`, Codex only in `sol`. Every thread's
  own init event or session record now gets the checks the first agent
  got: MCP servers, agent tools, foreign skills, `spawn_agent`, and the
  skill in the right arm.
- `result.json` gains `setup`, `role`, `roles`, `judges`, `threads` (each
  thread's role, agent, model, effort, seconds, exit and tokens in, cached
  in and out), `tokens` (their sum), `atelier_calls` and `atelier_failed`.
  Claude Code's tokens come from its result event and Codex's from its turn
  events or, for a subagent, its session record. Both count input with
  cache reads, so the two agents' numbers compare.
- Failed `atelier` calls are read from a log, not parsed from transcripts.
  The harness moves the fixture's `bin/atelier` to `libexec/` and puts
  `harness/atelier_log.py` in its place. Each call is logged with its
  arguments, directory, thread and exit status, including the fixtures'
  Git hooks, which run `exec atelier check --hook ...` from `PATH`, so a
  refused push shows up as a failed call. atelier-b9o6's graders can read
  the same log.

### Choices the item left open

- `delegate` takes the role as its first argument rather than guessing it
  from the task's text. This changes what an agent is told (`THREADS_NOTE`),
  so Claude Code results from before this change and after it differ in
  that one line. It is how this host works: the orchestrator picks each
  child's model by its role.
- Fable and Opus in `house` run at `high`, the harness's old default. The
  item gave efforts only for Sol.
- In a mixed setup with the skill arm, Claude Code's staged skill lives in
  the workspace's `.claude/skills`, where a Codex thread could read it as a
  file. Codex does not load it from there, and the init checks show what
  each agent loaded.

### Verification

- `evals/agent-factory/test.sh ~/.cargo/bin/atelier` (atelier 0.7.0):
  every case's tests pass. The harness's 49 tests pass, 18 of them new,
  and none calls a model. The new ones cover the two setups and their
  judges, refused setups, the one-model setup, every case naming a role,
  Codex's effort flag, `house`'s per-role commands and homes, staging for
  each agent, a Codex first agent in a mixed setup, a thread at the wrong
  effort, tokens, the `atelier` log, `delegate` refusing an unknown role,
  and canary selection in `main`. `RunOneTest` runs `run_one` in `house`
  end to end, with stand-ins for both agents: a Claude Code implementer
  calls `atelier` and `delegate reviewer`, and the result shows the Codex
  reviewer at `gpt-6.1-sol`/`xhigh` with its tokens, the failed call with
  its thread, a peak of 2 and the auth link removed. When the stand-in
  Codex reports `low`, the run is invalid.
- Against real agents, with a temporary setup (Haiku as lead and
  implementer at `low`, Codex `gpt-6-luna` at `medium` as reviewer), both
  canaries passed. `next-loop` ran and was graded, with tokens per thread
  and the `atelier` log matching the grader's count of 2 calls. Haiku at
  `low` stopped before it delegated, so I ran `delegate reviewer` and
  `delegate lead` directly in that setup's thread config. The Codex child
  ran `gpt-6-luna` at `medium` by its own session record, loaded the staged
  skill and no `spawn_agent`, and its sign-in link was gone afterwards. The
  Claude child reported `claude-haiku-5-5` and its tokens. A Codex first
  agent given `medium` ran at `medium`. Results are in
  `~/.cache/af-evals/runs/8e8e-smoke`.
- `git diff --check` is clean.

### For the items that follow

- atelier-b9o6 also changes `harness/run.py`. The two will conflict around
  `run_one` and `invalid_reason`; whichever merges second needs a merge of
  the base.
- The graders still read `delegate` commands from transcripts.
  `next-non-atelier-repo` matches the word `delegate` and so still sees a
  `delegate reviewer ...`. atelier-xfsn's move to facts can read
  `result.json`'s `threads` and roles instead.
