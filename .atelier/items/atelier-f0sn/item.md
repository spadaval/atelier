---
schema: atelier.item
schema_version: 3
id: atelier-f0sn
title: Put what waits on the reader first in status, one line per item, with a line of review pace for a mission's lead
status: in_review
mission: atelier-oqsz
labels: []
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A7729b331-a9bb-417c-90ef-4521f33e62cd%3Adelegate-task%3Aoqsz-impl-f0sn
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T07:19:32.292565379Z
---

## Goal

A mission's lead on 0.7.0 could not read its own `status`: it printed the
backlog first and the missions led last, reached 17,870 bytes, and the
harness showed the first 2 KB. Of the largest output 51% was unticked Done
when lines of items other threads were building and 12% thread ids, and
much of the rest blocked-by chains. The 41 saved outputs are under
`~/.claude/projects/-home-supadava--t3-worktrees-industrial-experience-hub-v2/7041651b-a459-48d3-8e59-3d0651f2296a/tool-results/hook-*-stdout.txt`.

Nothing told that lead what its reviews cost either. Give `status` an
order and a size a reader can use, and give a lead one line of review pace
for each mission it leads.

## Done when

- [x] Whatever the reader is, what waits on that thread prints before
  anything that does not, and the backlog prints after both.
- [x] A test builds a twelve-item mission with items building, in review
  and blocked, and its lead's `status` is under 2 KB.
- [x] A test builds the state behind the Hub's largest saved output (three
  missions led, eight items held by other threads) and its lead's `status`
  is under 4 KB.
- [x] A lead's `status` holds one line of review pace for each mission it
  leads: rounds per item, the time a round took, and the new findings in
  rounds after the first. The Report gives the line for `hub-v2-bg14`
  beside a count made by hand from its review records.
- [x] What `status` no longer prints is still one command away, and
  `status` names that command.
- [x] `docs/product.md` and `docs/design.md` say what `status` shows and
  in what order. `evals/agent-factory/test.sh` passes.

## Scope

What `status` prints, in `crates/` and their tests, and the two documents.
Not `atelier show`, not the Desk, no new command and no change to a
record's form. `atelier-5asm` fixes three lines `status` prints that are
refused; leave them to it.

## Report

A lead's `status` now reads what waits on it first and fits a harness's
2 KB window. On the Hub's state it fell from 18,146 bytes to 3,826, and for
a twelve-item mission it is 1,981. Code in `e7ca022d2` and `1c8821395`.

Away from an item's branch `status` prints, in order: plan review
standing; what the caller holds and its `Then:` line; what the checkout
has not saved and unpublished drops; for a lead, what waits on it
(findings not built, round caps, responses that do not read, one handoff
for every startable item, or "Nothing in the missions you lead waits on
you now."); each mission led, as `You lead mission <id>:`, a review pace
line, one line per item not done and a count of the done ones; then the
backlog under a heading, without the led missions' items, with 3
startable items instead of 10 when the caller holds or leads something.
On an item's branch `status` already printed that item alone. The held
item's `Then:` line now comes before the backlog, so it is no longer the
last line when a backlog follows.

What an item line drops (holder thread ID, blocker statuses, the holder's
unmet Done when lines) is one command away, and `status` names it:
`atelier show <item>`. Disputes stay in full under their mission, since
the lead settles them and the agent-factory evals' kept-finding scenarios
read them there. The backlog's "Held by other threads" keeps
each holder, since a test relies on it for an item started in another
clone and the Hub's ID bloat was in the led missions, which the backlog
now skips. A stale approval in a led mission still shows, as "approval
gone stale, back to review".

Review pace, for `hub-v2-bg14` on its v2 checkout at `078774ac`:

    Review pace: 22 rounds for 11 items, 2.0 each; a round took 15m at the median, 31m at most; later rounds found 6 new findings, 0 to fix now.

The hand count, from the review files with `grep` and `git log -S"verdict:
<v>"` per file, a different method from the tool's one `git log -G`: 11
items with a verdict (12qi 2, 9xrx 3, 4m88 2, 7fyn 2, 7zwz 2, a6nb 2, gege
1, 6ce4 2, aj25 2, luxi 2, yemo 2 = 22 rounds). New findings in round 2 or
later (findings minus `Was:` lines): 12qi 1, 7fyn 1, 7zwz 2, a6nb 1, luxi 1,
the rest 0, so 6, none fix now. Round times sorted, in minutes: 2 3 4 6 11
12 12 13 14 14 15 15 15 15 19 20 23 24 25 25 30 31; the 11th is 15 and the
longest 31. Both agree.

A round's time runs from its review's `created_at` to the latest commit
that wrote its `verdict:` line, so it is the reviewer's time and leaves
out the implementer's fixing between rounds. On bg14 the reviews took
2 to 31 minutes, while round 1 to round 2 took 33 to 86 minutes. If the
lead wants the whole cycle instead, it is a one-line change in
`atelier_core::pace`. The mission's "over an hour each" was a
hypothetical, and on these records the reviews were shorter.

### Validation

- `cargo nextest run -p atelier-cli -p atelier-core`: 404 passed.
  `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings` and
  `git diff --check` pass.
- New `crates/atelier-cli/tests/status_size.rs`. The twelve-item mission (2
  done after timed rounds, 3 building, 2 in review, 5 open and blocked)
  comes to 1,981 bytes, under 2 KB. That includes the 120-byte "no hooks"
  line, which a clone with hooks does not print. The Hub fixture rebuilds
  the saved 17,870-byte output's state: three missions led, eight items
  held by other threads with their Done when lines, and the same blocker
  chains. The 0.7.0 binary prints 18,146 bytes on it and the new one 3,826,
  under 4 KB. The tests also check the order for a lead, for a thread
  holding an item, and for a lead with an item to hand off; the pace line
  on known commit times (40, 15 and 25 minutes give 25m and 40m); and that
  `show <item>` has the holder, blockers and Done when lines.
- Updated tests whose expected text moved: `planning.rs` (2) and
  `views.rs` (1).
- On the live Hub v2 checkout the lead's `status` is 1,564 bytes, and the pi
  extension's `ledMissionFromStatus` regex still finds `hub-v2-bg14` in it.
  `pi-extension/lib.ts` parses `You lead mission <id>:`, which is why each
  mission keeps that heading.
- `evals/agent-factory/test.sh` passes: "All eval fixtures and harness
  tests pass." Its first run failed `kept-finding-right` and
  `kept-finding-wrong`, which read a kept rebuttal from the lead's
  `status`; `1c8821395` puts disputes back in full under their mission.

### Responses
