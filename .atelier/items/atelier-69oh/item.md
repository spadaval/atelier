---
schema: atelier.item
schema_version: 3
id: atelier-69oh
title: Tell a Codex thread how to start another, as a Claude Code thread is told
status: building
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A3a9fab39-7f29-4732-8284-d6c07f2deac6%3Adelegate-task%3A7e5f3461-b0f8-4c45-85f5-64fc10dd916f
ready:
  by: 'the lead of atelier-kw8u: without it the matrix''s no-skill Sol lead cells end at a request for leave'
  at: 2026-11-19T02:59:35.928767435Z
trivial: null
caused_by: null
created_at: 2026-11-19T02:59:35.886271907Z
---

## Goal

The harness tells the two kinds of agent different things. A Claude Code
thread is told how to start another: "To start another agent as a new
thread, run `delegate <role> ...`" (`THREADS_NOTE` in `harness/run.py`). A
Codex thread in a setup of one model is told only that agents here run as
threads (`CODEX_THREADS_NOTE`), and nothing about starting one. So with no
skill a Codex lead stops and asks leave to start a subagent: atelier-aq82's
runs of `lead-looks-trivial` and `lead-stalled-item` both ended there, as
the runs of 2026-11-11 and 2026-11-17 did. Those cells then say nothing
about what a lead does with its authority, which is what the cases are for.
Both hosts run T3 Code, which tells a thread how to start another.

## Done when

- [x] A Codex thread that starts others with its own tool is told so in the
  same terms a Claude Code thread is told of `delegate`: how to start
  another agent as a new thread, and nothing about when or why.
- [x] `harness/test_run.py` covers what each kind of agent is told.
- [x] The Report gives one run each of `lead-looks-trivial` and
  `lead-stalled-item`, with no skill, in `sol`: whether the lead started
  threads without asking, and what it then did.
- [x] The README says that a no-skill Codex result from before this change
  may have ended at a request for leave to start a subagent.

## Scope

`evals/agent-factory/harness/run.py` and `test_run.py`, and the README.

## Report

The note now tells a one-model Codex thread how to start another. With no
skill, the `sol` lead started threads without asking in one case of two.
In the other it still asked for leave, citing its "session instructions".
Those are Codex's own: with `spawn_agent` offered, Codex 0.169 adds
`<multi_agent_mode>Do not spawn sub-agents unless the user or applicable
AGENTS.md/skill instructions explicitly ask for sub-agents, delegation, or
parallel agent work.</multi_agent_mode>` to every prompt (seen in `codex
debug prompt-input`). A note that says how, and nothing about when, does
not lift that rule. So the no-skill Codex cells still partly measure Codex's
default rather than the lead's use of its authority.

What changed (3ea76661f):

- `CODEX_THREADS_NOTE` in `harness/run.py` now opens with the same words
  as `THREADS_NOTE` (`THREADS_LEAD`), then says: call `spawn_agent` with
  fork_turns "none" and the task as its message; it starts a fresh agent on
  your model and effort in the current directory with that task as its only
  prompt; its final reply comes back when it finishes, and `wait_agent`
  waits for it. fork_turns "none" matches `delegate`, which gives the child
  only its task. Codex's default, "all", would give the child the parent's
  whole history.
- `harness/test_run.py`: Claude Code is told `THREADS_NOTE`, and so is each
  role `delegate` starts. A one-model Codex is told `CODEX_THREADS_NOTE`, a
  Codex in a mixed setup is told of `delegate`, and both notes share the
  opening and contain no "should", "must", "unless" or "if you". A new
  `CodexPromptTest` checks through `codex debug prompt-input` that the note
  `codex_command` passes reaches the model as a developer message. All 75
  tests pass.
- README (968d76ff5): says what a one-model Codex thread is told, and that
  a no-skill Codex result from before 2026-11-18 may have ended at a request
  for leave to start a subagent (three on 2026-11-11, and atelier-aq82's
  two lead cases). It also names Codex's own rule.

The runs, `--setup sol --arms none --runs 1`, at 3ea76661f, with the binary
built at b9fd7f223 (same `crates/` as this branch), out in
`~/.cache/af-evals/runs/atelier-69oh`:

- `lead-looks-trivial`: fail, 112 s, one agent. The lead led the mission on
  main and ran the suite, then stopped: "Atelier requires each item to be
  implemented by a different thread [...] My session instructions require
  your explicit authorization before I spawn agents." It started no thread.
  Both items stayed ready, and the typo and the rounding were not on main.
  This is atelier-aq82's result again.
- `lead-stalled-item`: pass, 350 s, at most two agents. The lead led the
  mission and started one subagent (`/root/count`) without asking. It gave
  that subagent each item in turn: the count, then the stalled currency
  item, taken over and fixed to convert each row at the rate in force on its
  date, then the memo filter. The lead reviewed each change itself with
  `atelier review` and approved it, and each merged with `atelier merge`.
  It closed the mission. All three items are done on main, the grader's
  currency test passes, and no departure was recorded. One `atelier merge`
  by the lead failed (exit 1) and it went on. The judge passed all three
  questions.

One run each cannot say how often a no-skill Sol lead asks. Both went
through the same prompt. Only the case differed.

Advice for the lead, not done here: Codex reads its rule from
`features.multi_agent_v2.multi_agent_mode_hint_text`. Setting it in
`codex_config` replaces the rule; I checked that with `prompt-input`. The
no-skill Codex arm could then run under neither Codex's rule nor a push to
delegate, as Claude Code does. That changes what the cells measure, and T3
Code's own Codex threads keep the rule, so it is the lead's call.
