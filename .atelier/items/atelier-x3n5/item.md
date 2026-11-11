---
schema: atelier.item
schema_version: 3
id: atelier-x3n5
title: Take merged items and thread ties in the scorecard scripts from `atelier log`, and delete their own history walks and title matching
status: changes_requested
mission: atelier-295u
labels:
- redesign
priority: medium
blocked_by:
- atelier-8b0a
implementers:
- thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Ac0bf3903-20c0-4459-bc9e-8bb9a1af8d59
ready:
  by: sumeet, relayed by the task that started the lead
  at: 2026-11-11T17:10:10.404909184Z
trivial: null
caused_by: null
created_at: 2026-11-11T17:05:18.253820205Z
---

## Goal

Make the scorecard scripts run on the repositories as they are after the cutover, taking from `atelier log` what it now supplies. Today they read record files of the old layout, walk Git for commit messages and old record versions, and match item IDs in a thread's title, branch and first message. The first reads nothing since 2026-11-11, and `log` supplies the rest. The rerun two weeks after the cutover (atelier-pwhn) needs scripts that run, and the mission criterion `from-log` needs them to stop walking history and matching titles.

## Done when

- [x] The log is a frozen input like the others. `freeze.py copy` writes each repository's whole `atelier log` output into the snapshot as `logs/<repository>.txt`, and the manifest's hashes cover it. The scripts read only those files and never run `atelier`, so a base that moved on, a branch deleted or an item moved to another mission after the freeze changes no number. A test reruns one manifest and snapshot after the fixture repository's base has advanced and a branch is gone, and gets the same output.
- [x] `counted.py` takes its candidates for the window's merged items from the log's `merge` lines, and keeps the baseline's population: an item counts as merged only when its merge commit, the one its line names, changes a file outside `.atelier/` from its first parent. A fixture holds a merge that changes code and one that changes only records, and only the first is counted. It takes each thread's item and role from the lines that name it: `start` and an implementer's take-over make it an implementer of that item, `review` and a reviewer's take-over a reviewer. A Codex subagent's identity, `<T3 thread>/<child>`, ties its T3 thread. A T3 thread that Claude's Agent tool started is tied to its parent's item, since it acts under its parent's identity. A thread the log names on more than one item is tied to the first and counted in the coverage the output already gives. Any other thread is untied.
- [x] `draw_samples.py` takes each closed mission's start and close from the mission's `ready` and `close` lines in the frozen log, its items from the item records at the manifest commit that name it, and its threads from the thread IDs on the mission's and those items' lines and T3's parent links. It reads fix-now findings from the review files of the new layout at the manifest commit.
- [x] Deleted, with nothing left that does their job another way: `records.py`'s reader of `.atelier/issues/` and its walk of commit messages, `threads.tie()`, the role read from "Act as the ... sub-agent", and `draw_samples.py`'s `git log` and `git show` walk of a mission record's versions. A search of `scripts/scorecard/` finds no match of an item ID against a thread's title, branch or message.
- [x] The scripts read the log as text, by the line layout `docs/design.md` gives; lines after the manifest's cutoff are dropped. They compute none of the numbers the gardener's procedure takes from the log itself (time in each status, rounds, take-overs, escalations, skips).
- [x] `python3 -m unittest discover scripts/scorecard` passes, with the fixtures in the new layout and a test of each tie rule above.
- [x] `scripts/scorecard/README.md` describes the scripts as they now are, and says which commit's scripts reproduce the November baseline (the base commit this item started from), since these no longer read the baseline's old-layout records. The Rerunning section of `docs/architecture/quality/scorecard-baseline-2026-11.md` names that commit too. No baseline number, sample or label changes.

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

## Report

The scorecard scripts now run on the repositories after the cutover. They take merged items and each thread's item and role from `atelier log`, frozen in the snapshot as `logs/<repository>.txt`, and no longer walk commit messages, read old record versions or match item IDs in thread titles, branches or messages. The scripts outside the tests lost 3 lines net: they gained 168 and lost 171.

| File | Added | Removed |
| --- | --- | --- |
| `atelier_log.py` (new) | 59 | 0 |
| `records.py` (deleted) | 0 | 61 |
| `threads.py` | 12 | 35 |
| `counted.py` | 16 | 17 |
| `draw_samples.py` | 73 | 56 |
| `freeze.py` | 8 | 2 |
| `test_scorecard.py` | 170 | 38 |
| `README.md` | 45 | 22 |
| `scorecard-baseline-2026-11.md` | 4 | 0 |

What changed:

- `freeze.py copy` runs `atelier log` in each repository and writes the output to `logs/<repository>.txt`. `freeze.py hash` already hashes every file in the snapshot, so the manifest covers the logs. Nothing else runs `atelier`.
- `atelier_log.py` replaces `records.py`. `read()` splits each line by the layout in `docs/design.md` and drops lines committed after the cutoff. `merged()` keeps an item when its `merge` line falls in the window and `git diff <commit>^1 <commit>` shows a file outside `.atelier/`. `ties()` gives each T3 thread the item and role of the first `start`, `review` or take-over line that names it. A take-over whose text starts `from ` is an implementer's, and any other take-over is a reviewer's. A Codex identity `<T3 thread>/<child>` counts for the part before the `/`.
- `threads.py` lost `tie()`, the "Act as the ... sub-agent" role, and the title, branch and first-message reads. In their place it gives each thread `acts_as`: the thread itself, or, for a thread Claude's Agent tool started (a `subagent` item with driver `claudeAgent`), the parent it runs as. Claude Code gives a subagent no Agent tool, so there is one level to follow. `counted.py` looks the tie up under `acts_as`. `tied_by` in `counted.json` is now `log` or `parent`, where it used to be `branch`, `title` or `first_message`. Calls before first edit counts threads whose role is `implementer`.
- `done_in_window` counts the items, not missions, with a `merge` or `close` line in the window. Missions are the directories under `.atelier/missions/` at the manifest commit. Reconciling this count with `defects.md` is still atelier-c1z3's job.
- `draw_samples.py` takes a closed mission's start and close from its first `ready` line and its `close` line. It takes the mission's items from `git grep '^mission: '` over the item records at the manifest commit, and its threads from the thread IDs on its own lines and its items' lines, then walks T3's parent links upward as before. It draws fix-now findings from `reviews/*.md` files at the manifest commit and skips findings with a `Was:` line, since those were raised in an earlier round. It reads a review's body, and an item's `mission:` from its front matter only, outside fenced code by the rule of `unfenced_lines` in `crates/atelier-records/src/document.rs`, so an example in a fence is never a finding, a field or a membership. Findings are named `<item>/<review>.<n>`. The `git log`/`git show` walk of a mission record's versions is gone.
- The README describes the scripts as they now are. It says only the scripts at `df9c84a43` (this item's base) reproduce the November baseline, and gives the `git worktree add` command that checks them out. It also says how tie coverage differs from the baseline's and what the after-run should state beside its numbers. The Rerunning section of `scorecard-baseline-2026-11.md` names `df9c84a43` too. No baseline number, sample or label changed.

Tests: `TMPDIR=~/.cache/atelier-x3n5-tmp python3 -m unittest discover scripts/scorecard` runs 22 tests, all passing. The fixture repository uses the new layout, with real `--no-ff` merges: demo-aaaa's merge brings code and is counted, and demo-bbbb's brings only its record and is not. The fixture log holds a merge before the window, lines after the cutoff, a `lead` line, and a `merge` by a non-T3 identity. `test_each_tie_rule` checks every rule thread by thread: start, an implementer's take-over, review, a reviewer's take-over, a Codex subagent, an Agent-tool child, a thread named on two items, and threads left untied. `Samples` tests closed missions, interventions and findings, including a fenced example finding, a fenced `Was:` line inside a real finding, and an item whose body shows another mission in a fence. `test_a_rerun_after_the_freeze_gives_the_same_output` freezes the outputs, adds a commit to main that moves demo-aaaa to another mission and adds a fix-now review, deletes both item branches, and gets the same `counted.json`, `counted.md`, missions and findings. Breaking the Codex split or reading item records at `main` instead of the manifest commit fails these tests. As a smoke check, `read()`, `merged()`, `done()` and `ties()` parsed this repository's real `atelier log` (96 lines, 5 merged items, 25 tied threads). That output was not kept.

Surprises:

- The old test fixture tied 6 threads to merged items through titles and first messages. The log-based rules tie exactly the same threads with the same roles, so every measure's expected number in the tests stayed the same.
- The Agent-tool child rule depends on T3 writing `nativeItemRef.driver` on `subagent` items. The fixture has it, but I could not check the live database, which the brief rules out. If real items lack the field, those children would come out untied, and the after-run's `tied_by.parent` count would show it as 0.
- `freeze.py copy` is not under test, because its paths are fixed to the November snapshot, and making them configurable belongs to the after-run's item. Its new step is four lines.

### Responses

- 1-c6ma.1: fixed in fefff2355
- 1-c6ma.2: fixed in fefff2355
