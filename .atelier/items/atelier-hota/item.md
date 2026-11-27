---
schema: atelier.item
schema_version: 3
id: atelier-hota
title: Print atelier log newest first with a default limit
status: approved
mission: atelier-oqsz
labels: []
priority: low
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A97bcfb9a-33e0-42f5-882a-004f5d9c6644%3Adelegate-task%3Aoqsz-impl-hota
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T07:19:32.618377817Z
---

## Goal

`atelier log` prints the whole history, oldest first. On the Hub's `v2` it
is 170 KB in 675 lines, so a reader with a cut view sees 2026-11-25 and
not the last hour. One mission's log is 30 KB, and a line reaches 2,456
characters where a skip's reason is quoted in full.

## Done when

- [x] With no option, `atelier log`, `atelier log <item>` and `atelier log
  <mission>` print the newest action first and stop at a default limit,
  and say how many older actions there are and how to see them.
- [x] The whole history is one flag away, in an order a script can rely
  on.
- [x] No line is over 300 characters by default; a cut reason says where
  to read it whole.
- [x] On the Hub's history the default output is under 10 KB; the Report
  gives the size.
- [x] Everything in this repository that reads `atelier log`, the garden
  procedure's counter and the scorecard scripts among them, gives the same
  answer as before, shown by its tests or by a run on this repository.
- [x] `docs/design.md` ("The log") and the command's help say the order
  and the limit.

## Scope

`atelier log`'s output, in `crates/` and their tests, its readers in
`skills/` and `scripts/`, and `docs/design.md`. Not what `log` records,
and not `atelier_core::log`, which the Desk reads. `atelier-3sym`,
`atelier-tuln` and `atelier-itqc` fix lines `log` gets wrong; leave them
to those items.

## Report

With no option, `atelier log`, `atelier log <item>` and `atelier log
<mission>` now print the 30 newest actions, newest first, and end with
`<n> older actions not shown. Add --all to print every action, oldest
first, each line whole.` A line over 300 characters is cut and ends with
`[cut; add --all to print it whole]`. Neither names the command run, so no
ID takes a line past 300 (round 2). `--all` prints exactly what `log`
printed before. The change is in `crates/atelier-cli/src/history.rs` and
the help in `main.rs`; nothing in `atelier_core::log` changed.

- **Size on the Hub.** In `~/.t3/worktrees/industrial-experience-hub/v2`
  on 2026-11-27, `atelier log` printed 6,160 bytes in 31 lines, against
  174,415 bytes in 695 lines whole. Its longest line was 271 characters;
  `atelier log hub-v2-524p` cut its 1,000-character merge line to 300.
  The largest mission log there, `hub-v2-j0ms`, printed 6,713 bytes.
  The actions are at most 9,000 characters, 9 KB when ASCII.
- **Same answers for readers.** `atelier log --all` from this build is
  byte for byte the installed 0.7.0's `atelier log`, on this repository
  and on the Hub (whole and for `hub-v2-524p`). The garden counter's awk,
  run on both, lists the same items. `freeze.py`, `garden.md` (window
  start, counter, copy per item), `readiness.md` line 10 and the
  scorecard README now name `--all`; the scorecard tests (22) pass. The
  scorecard's other scripts read the frozen file, not `atelier`.
- **Tests.** `tests/log.rs`'s helper and the four tests elsewhere that
  read order or a whole skip line pass `--all`. New: the plain path and
  the mission test check the default is the `--all` lines reversed; one
  test makes 32 actions and checks the 30 newest and the footer, with
  `--since`; one checks a 500-character skip line is cut to at most 300,
  with and without `--since`; a unit test in `history.rs` checks that a
  cut of a line with a 250-character ID, and the closing line for any
  count, stay within 300. `cargo nextest run -p atelier-cli`: 283 passed. `cargo fmt`, `cargo clippy
  --all-targets -- -D warnings` and `git diff --check` are clean.
- **Docs.** `docs/design.md`, The log, says the order, the limit, the cut
  and `--all`, with the Hub's numbers; `--since` applies before the limit.

A cut can in principle reach into the thread field when an ID and a
thread together pass about 250 characters; none in either repository do. Scripts should
read `--all`, where nothing is cut.

The limit is a fixed 30, with no `--limit`; `--all | tail -n <n>` covers
the rest. Installed copies keep printing the old way until they are
rebuilt from this tree.

### Responses

1-ja9k.1: fixed in 4972586d7
1-ja9k.2: fixed in 4972586d7
