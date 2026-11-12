---
schema: atelier.item
schema_version: 3
id: atelier-s0j2
title: Make one gardener run in atelier under the rewritten procedure, focused on structure
status: draft
mission: atelier-g961
labels:
- agent-factory
priority: medium
blocked_by:
- atelier-wb05
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-12T01:03:20.880919366Z
---

## Goal

Show that the rewritten procedure produces what Sumeet asked for. Start one
gardener thread in atelier with the task "Garden this repository; focus on
structure" and nothing else, and let it follow `garden.md` to an approved run
that waits for its session.

## Done when

- [ ] The run item exists, labelled `garden`, is approved by a reviewer from another lab, and waits for its session with Session empty and nothing merged.
- [ ] Its page records the focus, and is at most 60 lines with at most three proposals.
- [ ] At least one proposal rests on evidence quoted from a thread, named by its ID; at least one is about the code's structure and names files; none only restates a line of `readiness.md`.
- [ ] This item's Report lists every place the gardener said `garden.md` or `readiness.md` made it guess, and each is fixed on this item's branch in a sentence or filed as an item.
- [ ] The Report says how many threads the run read, how they were chosen, and what the run cost in wall-clock time.

## Scope

In: one run in atelier; fixes of a sentence or less to `garden.md` and `readiness.md` that the run shows are needed.

Out: runs in the other four repositories; the session itself and the run's merge, which are Sumeet's; any change to the limit of three proposals or to what a run may fix.

## Context

The run's window starts where atelier-0019's page ends. atelier-0019 may still be waiting for its own session; the two runs do not depend on each other. atelier-613a is the same kind of wrapper item for the first five runs.
