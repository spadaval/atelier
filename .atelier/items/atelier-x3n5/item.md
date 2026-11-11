---
schema: atelier.item
schema_version: 3
id: atelier-x3n5
title: Take merged items and thread ties in the scorecard scripts from `atelier log`, and delete their own history walks and title matching
status: draft
mission: atelier-295u
labels:
- redesign
priority: medium
blocked_by:
- atelier-8b0a
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-11T17:05:18.253820205Z
---

## Goal

Make the scorecard scripts run on the repositories as they are after the cutover, taking from `atelier log` what it now supplies. Today they read record files of the old layout, walk Git for commit messages and old record versions, and match item IDs in a thread's title, branch and first message. The first reads nothing since 2026-11-11, and `log` supplies the rest. The rerun two weeks after the cutover (atelier-pwhn) needs scripts that run, and the mission criterion `from-log` needs them to stop walking history and matching titles.

## Done when

- [ ] The log is a frozen input like the others. `freeze.py copy` writes each repository's whole `atelier log` output into the snapshot as `logs/<repository>.txt`, and the manifest's hashes cover it. The scripts read only those files and never run `atelier`, so a base that moved on, a branch deleted or an item moved to another mission after the freeze changes no number. A test reruns one manifest and snapshot after the fixture repository's base has advanced and a branch is gone, and gets the same output.
- [ ] `counted.py` takes its candidates for the window's merged items from the log's `merge` lines, and keeps the baseline's population: an item counts as merged only when its merge commit, the one its line names, changes a file outside `.atelier/` from its first parent. A fixture holds a merge that changes code and one that changes only records, and only the first is counted. It takes each thread's item and role from the lines that name it: `start` and an implementer's take-over make it an implementer of that item, `review` and a reviewer's take-over a reviewer. A Codex subagent's identity, `<T3 thread>/<child>`, ties its T3 thread. A T3 thread that Claude's Agent tool started is tied to its parent's item, since it acts under its parent's identity. A thread the log names on more than one item is tied to the first and counted in the coverage the output already gives. Any other thread is untied.
- [ ] `draw_samples.py` takes each closed mission's start and close from the mission's `ready` and `close` lines in the frozen log, its items from the item records at the manifest commit that name it, and its threads from the thread IDs on the mission's and those items' lines and T3's parent links. It reads fix-now findings from the review files of the new layout at the manifest commit.
- [ ] Deleted, with nothing left that does their job another way: `records.py`'s reader of `.atelier/issues/` and its walk of commit messages, `threads.tie()`, the role read from "Act as the ... sub-agent", and `draw_samples.py`'s `git log` and `git show` walk of a mission record's versions. A search of `scripts/scorecard/` finds no match of an item ID against a thread's title, branch or message.
- [ ] The scripts read the log as text, by the line layout `docs/design.md` gives; lines after the manifest's cutoff are dropped. They compute none of the numbers the gardener's procedure takes from the log itself (time in each status, rounds, take-overs, escalations, skips).
- [ ] `python3 -m unittest discover scripts/scorecard` passes, with the fixtures in the new layout and a test of each tie rule above.
- [ ] `scripts/scorecard/README.md` describes the scripts as they now are, and says which commit's scripts reproduce the November baseline (the base commit this item started from), since these no longer read the baseline's old-layout records. The Rerunning section of `docs/architecture/quality/scorecard-baseline-2026-11.md` names that commit too. No baseline number, sample or label changes.

## Scope

In: `scripts/scorecard/` and the two documents named above. Reading a record file at the manifest commit, and the files one named commit changes from its first parent, is in: neither walks a record's history.

Out, and a review finding that asks for one of these is escalated, not fixed:

- The command forms, the refusal and error text the scripts recognise, the rubric, the done-item counts and the README's runtime. They are atelier-c1z3, which starts after this merges.
- Any new measure, and any change to how a measure that does not depend on the tie is taken (process share's command reading, pricing, rework, line counts).
- A new snapshot or a run on live data, and making `freeze.py` take a second snapshot's name or path, which the after-run's own item does. Tests use fixtures. The baseline's frozen inputs in `~/.local/share/atelier-baseline/2026-11/` hold private messages and are not needed here: do not read them, and commit nothing from them.
- Any change under `crates/`. If `log` lacks a fact the scripts need, escalate.
- Keeping a second code path that reads the old layout.

## Context

Blocked by atelier-8b0a, which adds `log` and fixes its line layout. The baseline's method is in `docs/architecture/quality/scorecard-baseline-2026-11.md`; a thread's identity in the records is its T3 thread ID (`docs/design.md`, Threads and roles), which is the `thread_id` of T3's database.

The coverage of the tie will differ from the baseline's, which tied by title: say so in the README, with what the after-run should state beside its numbers.
