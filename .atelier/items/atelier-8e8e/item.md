---
schema: atelier.item
schema_version: 3
id: atelier-8e8e
title: Run a matrix in a named setup, a model and an effort per role, and report time and tokens per cell
status: ready
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers: []
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

- [ ] `run.py --setup <name>` reads a checked-in setup: an agent (Claude
  Code or Codex), a model and an effort for each of lead, implementer and
  reviewer. Two are checked in. `sol` is Codex with `gpt-6.1-sol` in every
  role, at `high`. `house` is `claude-fable-5-2` as
  lead, `claude-opus-5-5` as implementer and Codex `gpt-6.1-sol` at `xhigh`
  as reviewer.
- [ ] A setup also names its judges: a model for each lab its threads come
  from, from another lab than that one. `sol` has one and `house` has two.
- [ ] Each case names the role of its first agent, and a thread it starts
  for a role runs as that role's agent, model and effort.
- [ ] A setup of one model in every role can be given on the command line,
  for a new model's first run.
- [ ] At most two eval agents are alive at once in a setup that mixes Claude
  Code and Codex, and the canary and the init checks hold for each agent of
  the setup. `harness/test_run.py` tests both with no model.
- [ ] Every `result.json` names its setup and gives the run's seconds, its
  tokens in and out for each thread, the threads it started and its failed
  `atelier` calls. One command prints a table from `results.jsonl`: for each
  cell its passes, median seconds and tokens.

## Scope

`evals/agent-factory/harness/`. The scenarios change only to name their
first agent's role.
