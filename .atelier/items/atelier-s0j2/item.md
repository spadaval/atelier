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

- [x] The run item exists, labelled `garden`, is approved by a reviewer from another lab, and waits for its session with Session empty and nothing merged.
- [x] Its page records the focus, and is at most 60 lines with at most three proposals.
- [x] Its window starts at the end atelier-0019's page gives, found by `garden.md`'s own rule and not by a hint in the task.
- [x] At least one proposal rests on a trip the gardener checked in a thread, given as the thread's ID and what happened; at least one is about the code's structure and names files; none only restates a line of `readiness.md`.
- [x] The page and the run's `artifacts/` hold no text copied from a transcript, no thread title, no home path and no personal detail.
- [x] This item's Report lists every place the gardener said `garden.md` or `readiness.md` made it guess, and each is fixed on this item's branch in a sentence or filed as an item.
- [x] The Report says how many threads the run read, how they were chosen, and what the run cost in wall-clock time.

## Scope

In: one run in atelier; fixes of a sentence or less to `garden.md` and `readiness.md` that the run shows are needed.

Out: runs in the other four repositories; the session itself and the run's merge, which are Sumeet's; any change to the limit of three proposals or to what a run may fix.

## Context

The run's window starts where atelier-0019's page ends. atelier-0019 may still be waiting for its own session; the two runs do not depend on each other. atelier-613a is the same kind of wrapper item for the first five runs.

## Report

The run is atelier-qb4f, labelled `garden`, focused on structure. It was
started as a fresh Opus 5.5 thread whose whole task was "Garden this
repository; focus on structure". GPT-6.1 Sol approved it in round 2 (round 1
asked for two fixes, both made). It waits for its session: Session is empty
and nothing is merged (PR 152 is open).

Checked against this item's Done when:

- The page is 41 lines without `### Responses`, with three proposals. Its
  first line records the focus as named.
- Its window starts at 2026-11-11T17:56:43Z, the end of atelier-0019's
  `Window:` line. The gardener found it by step 2's rule, reading atelier-0019
  from its branch with `atelier show`. The task carried no hint.
- P2 rests on trips in two reviewer threads. `artifacts/threads.md` gives each
  as the thread's ID suffix and position and says what the agent did. I checked
  the f6dv round-2 reviewer's myself: at positions 9 and 10 it searched
  `settle.rs`, `submit.rs`, `status.rs` and `next.rs` under
  `crates/atelier-cli/src/`, and none exists. P1 and P2 are about structure and
  name files in `crates/atelier-cli` and `crates/atelier-app`. P3 rests on two
  implementer threads. No proposal restates a line of `readiness.md`.
- I searched the page and `artifacts/` for home paths, emails and thread
  titles and found none. Trips are given in the gardener's words.

Where the gardener said `garden.md` made it guess, each fixed on this branch
in a sentence (`readiness.md` made it guess nowhere):

1. It did not know whether to settle again the deferred findings that
   atelier-0019's unmerged branch already settles. Step 5 now says a fix or
   settled finding already on an earlier run's unmerged branch is not made
   again, and the page names that run.
2. It did not know whether to repeat a fix that an earlier unmerged run already
   made (the Desk README's `next/` line). The same sentence covers it.
3. It did not know whether to round each item's seconds to minutes before or
   after taking the median. Step 2 now sums seconds per item and rounds the
   median.
4. For carried Scorecard measures, it did not know which column takes the
   value. atelier-0019 had written "carried" under This run. Step 2 now puts
   the value under This run and "carried" under Last run, which is what
   qb4f did.
5. "On the base" did not say where the base is checked out. Here the main
   checkout is detached and master is in a T3 worktree. Step 1 now says the
   base's checkout is the one `git worktree list` shows.

Threads and cost: the run read 6 threads, the implementer's and latest
reviewer's for each of the 3 items merged in the window (atelier-s47q,
atelier-wb05, atelier-f6dv). Step 3's order picked all three, and it would
have picked no more. The gardener read wb05's two threads itself. A GPT-6 Luna
helper pulled trips from the other four, and the gardener checked each trip it
used against its thread. Wall-clock time was 15 minutes: the thread started at
08:41:04Z and finished at 08:55:55Z, with both review rounds inside that.

One thing to know: the gardener's final message had a section headed "For
atelier-s0j2's Report", so at some point it read this item, likely while
checking open missions' items for overlap. Its window and focus came from the
task and `garden.md`'s rules, not from this item, but it knew it was being
measured.
