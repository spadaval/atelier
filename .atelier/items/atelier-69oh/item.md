---
schema: atelier.item
schema_version: 3
id: atelier-69oh
title: Tell a Codex thread how to start another, as a Claude Code thread is told
status: ready
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers: []
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

- [ ] A Codex thread that starts others with its own tool is told so in the
  same terms a Claude Code thread is told of `delegate`: how to start
  another agent as a new thread, and nothing about when or why.
- [ ] `harness/test_run.py` covers what each kind of agent is told.
- [ ] The Report gives one run each of `lead-looks-trivial` and
  `lead-stalled-item`, with no skill, in `sol`: whether the lead started
  threads without asking, and what it then did.
- [ ] The README says that a no-skill Codex result from before this change
  may have ended at a request for leave to start a subagent.

## Scope

`evals/agent-factory/harness/run.py` and `test_run.py`, and the README.
