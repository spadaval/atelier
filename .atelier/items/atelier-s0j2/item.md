---
schema: atelier.item
schema_version: 3
id: atelier-s0j2
title: Make one gardener run in atelier under the rewritten procedure, focused on structure
status: building
mission: atelier-g961
labels:
- agent-factory
priority: medium
blocked_by:
- atelier-wb05
implementers:
- thread:delegated-task:command%3Amcp%3Abe2146c6-f4c6-4217-8f3a-ee1dc9eec577%3Adelegate-task%3Abc9f4804-261c-43c2-a446-3d0cd4a70d1f
ready:
  by: sumeet, in conversation on 2026-11-13
  at: 2026-11-13T08:27:11.885174987Z
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
- [ ] Its window starts at the end atelier-0019's page gives, found by `garden.md`'s own rule and not by a hint in the task.
- [ ] At least one proposal rests on a trip the gardener checked in a thread, given as the thread's ID and what happened; at least one is about the code's structure and names files; none only restates a line of `readiness.md`.
- [ ] The page and the run's `artifacts/` hold no text copied from a transcript, no thread title, no home path and no personal detail.
- [ ] This item's Report lists every place the gardener said `garden.md` or `readiness.md` made it guess, and each is fixed on this item's branch in a sentence or filed as an item.
- [ ] The Report says how many threads the run read, how they were chosen, and what the run cost in wall-clock time.

## Scope

In: one run in atelier; fixes of a sentence or less to `garden.md` and `readiness.md` that the run shows are needed.

Out: runs in the other four repositories; the session itself and the run's merge, which are Sumeet's; any change to the limit of three proposals or to what a run may fix.

## Context

The run's window starts where atelier-0019's page ends. atelier-0019 may still be waiting for its own session; the two runs do not depend on each other. atelier-613a is the same kind of wrapper item for the first five runs.
