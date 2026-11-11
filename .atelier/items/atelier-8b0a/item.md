---
schema: atelier.item
schema_version: 3
id: atelier-8b0a
title: Add `atelier log`, the history of acts
status: in_review
mission: atelier-295u
labels:
- redesign
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Ad3b10278-c0d6-48d8-ab9f-2f66c69a7e79
ready:
  by: sumeet, relayed by the task that started the lead
  at: 2026-11-11T17:10:10.404909184Z
trivial: null
caused_by: null
created_at: 2026-11-10T00:56:10.707960610Z
---

## Goal

Give the gardener, a lead and a thread taking over an item one read of what happened: `atelier log`, the history of acts, derived from Git. It is the one command this mission adds, and the gardener's procedure (atelier-9dku) and the scorecard scripts (atelier-x3n5) are built on its output.

## Done when

- [x] `atelier log` prints one line per act, oldest first, for the acts in the table of `docs/design.md`'s section The log: start, take-over, submit, review, verdict, escalate, decide, merge, settle, lead, ready, drop and close. `atelier log <item>` prints that item's acts, `atelier log <mission>` the acts on the mission, its plan reviews and its items, and `--since <time>` keeps the acts committed at or after an RFC 3339 time or a `YYYY-MM-DD` date. An ID no record has is refused as `show` refuses it. It adds nothing up.
- [x] Each line holds, in this order: the commit's time in UTC, the commit's short hash, the record's ID, the act, the thread where the table gives one and `-` where it gives none, and what changed. The first five are separated by single spaces and hold no space; what changed is the rest of the line. A thread's identity is printed whole, so a Codex subagent reads `<parent>/<child>`. A skip ends the line of the act whose commit carries its `Atelier-Skip` trailer, with the rule and the reason. The layout is in `docs/design.md`, with what "what changed" holds for each act.
- [x] `atelier-app` returns the history as data, and the command only formats it, so the Desk can read the same history without parsing output.
- [x] The walk is one pass: `log` starts a number of Git processes that does not grow with the number of records or commits. It covers the base's history, the second-parent side of each Atelier merge and the tips of started items' branches, and prints each act once. The Report gives the time `atelier log` takes in astromech (`~/projects/astromech`, read only), which is under one second today.
- [x] Scenario tests drive the real binary in a scratch repository: the plain path prints its acts in order with the right threads, and prints the same lines after the item's branch is merged and deleted; the take-over scenario shows both implementers, and a review taken over shows both reviewers; an escalation and its answer; a deferred finding settled on a gardener's branch and merged; a mission's `ready`, `lead`, plan review and `close`; an item dropped; a commit made with `ATELIER_SKIP` shows the skip on its act's line; `--since` leaves out earlier acts; two acts on one record in one commit show as their net change.
- [x] `docs/product.md` and `docs/design.md` say `log` is built: the count of commands, the `log` row, and The log section, which says how the walk works in place of "settled when atelier-8b0a is planned". `atelier log --help` says what the lines are.

## Scope

In: `crates/atelier-app` (the history as data), `crates/atelier-cli` (the command and its output), their tests, `docs/product.md` and `docs/design.md`.

Out, and a review finding that asks for one of these is escalated, not fixed:

- Any other new command, flag or output mode. `log` takes an ID and `--since` and nothing else. It has no JSON mode and no filter by act or thread; a reader narrows the lines with `grep`.
- Any count, total, duration or threshold in the binary. The arithmetic is the gardener's procedure.
- A change to `status`, `show` or a handoff line to mention `log`.
- The Desk (`crates/atelier-desk`, `desk/`). atelier-hj6b reads the history there.
- `scripts/scorecard/`. atelier-x3n5 changes it once this has merged.
- A stored history, a cache, or a new field or trailer in any record or commit.

## Design

Settled here so the implementer does not have to choose again. Change one only with a reason in the Report.

- **One walk.** One `git log` over the base's tip and the tips of started items' branches, limited to `.atelier/items` and `.atelier/missions`, gives each commit with its parents, time, trailers and the old and new blob of each record it changed. One `git cat-file --batch` process, which `CommitReader` already runs, reads those blobs. An act is found by comparing a record's two versions across one commit, so no record's history is read on its own.
- **Merges.** A commit with one parent is compared with that parent. An Atelier merge, known by its `Atelier-Merge:` trailer, gives only its item's `merge` line: what its branch brought is found on the second-parent side. Any other merge commit gives no line.
- **History simplification is the trap.** Given paths, `git log` drops a merge's side when the merge matches one parent on those paths, and that can hide a branch's acts. The test of acts surviving a merged and deleted branch must fail without the fix (`--full-history` or its equal).
- **The cutover's conversion commit is read like any other.** Each record it added shows its acts at that commit's time. Nothing special-cases it; a reader who wants the time after it uses `--since`.
- **A version that does not parse.** History can hold a record that was invalid at one commit. `log` must not fail on it: it passes that version over, compares the versions on each side of it, and says on standard error which commit and path it passed over.
- **Time and order.** A line's time is its commit's committer time. Lines are ordered by it, and lines of one commit keep the order of the table in The log.

## Context

Specifications: `docs/product.md` (Commands, Scorecard) and `docs/design.md` (The log; Threads and roles for identity; Where records are read and written for which record counts; Testing). The old debrief reader followed first parents only and lost a merged branch's acts (planning findings atelier-mi8y and atelier-1ryt of the old tool).

Mission: `.atelier/missions/atelier-295u/`. Its Authority lets the lead choose the layout of a log line, which the second Done when line does.

This host: build in your own worktree with its own `CARGO_TARGET_DIR`, at `-j 4`, with `TMPDIR` under `$HOME` outside any Git repository. Run the tests that cover the change (`cargo nextest run -p atelier-cli -p atelier-app` or a filter); the full suite runs in CI on the pull request.

## Report

`atelier log` is built. A gardener, a lead or a thread taking over an item can now read what happened to it in one command, and the Desk can read the same history as data from `atelier_app::history`.

- The walk is `crates/atelier-app/src/log.rs`. One `git log --full-history --raw` over the base's tip and the tips of local branches named for items lists every commit that changed a record, parents first. The repository's one `git cat-file --batch` reads each record's old and new version as `<commit>:<path>`, and an act is found by comparing the two. `log` starts six Git processes whatever the history holds: `rev-parse` twice, `symbolic-ref`, `for-each-ref`, `log` and `cat-file` (counted with `strace` in astromech).
- The command and the line layout are `crates/atelier-cli/src/history.rs`, with its help in `atelier log --help`. The layout, and what "what changed" holds for each act, is in `docs/design.md`, The log, which now says how the walk works. `docs/product.md` counts eighteen commands and describes `log` as built.
- Tests: thirteen scenarios in `crates/atelier-cli/tests/log.rs` cover each case the Done when list names, plus a version that does not parse, one that is not UTF-8, one broken blob met twice, and an ID no record has. `crates/atelier-app/tests/library.rs` reads the history as data from a repository opened as the Desk opens it.
- Time in astromech (`~/projects/astromech`, read only): `atelier log` from a debug build printed 88 lines in 0.06 to 0.08 seconds, before and after round 1's fixes.
- One real item here, `atelier log atelier-6w8d`, prints twelve lines: `ready` by sumeet, `start`, three rounds of `submit`, `review` and `verdict` (changes, then approve twice), and the `merge`, whose line ends with the CI skip its merge commit carries.

Two changes from the Design, with their reasons:

- `crates/atelier-records`, outside the Scope's list, gains `CommitReader::blob`, 25 lines that read a blob by any name Git takes. The reader could only walk a commit's trees from the root, one object per directory, and had no way to read a version by name. With it every version is one read from the same `cat-file` process.
- The Design asked that the test of acts surviving a merged and deleted branch fail without `--full-history`. The plain path cannot: an Atelier merge marks its item done, so it differs from both parents on the record paths and Git follows both sides anyway. I checked by removing the flag, and all ten other scenarios still passed. So `acts_on_a_branch_whose_records_a_plain_merge_matches_survive_its_deletion` covers the case where simplification does lose acts: a plain merge of a branch whose start was reverted. Without the flag it prints only the `ready` line; with it, the `start` and the revert's `ready` too.

Choices the item left open, now written in `docs/design.md`:

- A record a commit adds moves from `new`, as in `new -> ready`. So the cutover's conversion commit shows each record's acts at its time, as the Design asks.
- A mission's items are those whose record on the base's tip, or else on its branch, names it.
- A skip goes on the lines of the acts Atelier commits; when a commit carrying one has no such act, every line of that commit shows it.

### Responses

- 1-kn9e.1: fixed in 01fbf8bb1
- 1-kn9e.2: fixed in 01fbf8bb1
