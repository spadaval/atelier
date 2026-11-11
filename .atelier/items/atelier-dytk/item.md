---
schema: atelier.item
schema_version: 3
id: atelier-dytk
title: 'Count a Codex agent''s idle subagents in the eval harness''s two-agent limit: max_threads lets a second start while the first sits idle'
status: draft
mission: null
labels:
- redesign
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-11T10:03:47.259644141Z
---

## Goal

Make the eval harness's limit of two agents hold for Codex as its README says. In atelier-t747's matrix, Codex's `agents.max_threads = 1` let a lead start a second subagent while the first sat idle, so three agents were alive in three runs of `next-settle-within-authority`, though no two children worked at once. The harness cannot tell an idle child from a running one.

## Done when

- [ ] Provisional: a threads run under Codex never has more than two agents alive, or the README states the limit as agents working at once and the harness measures that, with a test that does not start an agent.

## Context

Preliminary. Found by atelier-t747; the harness is atelier-a4rs's (`evals/agent-factory/harness/`). Pick it up before the next matrix is run.

## Links

Relationships of the old record that the new format has no field for.

- related: atelier-qr5s (Move all five repositories and Agent Factory onto the new loop, one cutover per host)
