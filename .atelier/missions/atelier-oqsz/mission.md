---
schema: atelier.mission
schema_version: 3
id: atelier-oqsz
title: Show a lead what waits on it and what its reviews cost, in a status under 2 KB, and write plans as outcomes
status: open
labels: []
lead: null
planners:
- mcp:5e35d5af-d02e-4108-865d-84da8e4d0e82
created_at: 2026-11-27T07:19:17.565104737Z
---

## Why

Sumeet asked for this on 2026-11-27, in a side chat of the Hub v2 lead's
thread (`b408a0ba-35e3-4640-8a09-c40ffa375c86`), after that lead ran
`hub-v2-bg14` overnight on Atelier 0.7.0. Checked in that repository:

- The lead's `status` passed 10 KB 41 times and reached 17,870 bytes. The
  harness then shows the first 2 KB, which is the backlog; the missions
  led print last. "This is bad design. We need to fix this pronto."
- Round one found a blocking fault in eight of nine merged items. Round
  two approved all eight and added five findings, none blocking. "A
  message saying 'your last five reviews took over an hour each and caught
  nothing' would have changed behavior."
- Merges `122403ea` and `acf8a7f4` went past a failed test run on a reason
  written for another rule, and their messages list the check as passed.
- `atelier log` is 170 KB, oldest first.

`atelier-ww7w`'s plan of 2,400 words drew 45 findings in five rounds. The
side chat proposed loose items to avoid that, and he refused: "the right
answer is usually to fix the process." So this plan is one page of
outcomes, reviewed under today's rules, and its own review is the first
measurement (`notes.md`).

## Outcome

- A thread reads what waits on it at the top of `status`, and a lead sees
  what its reviews cost.
- A failed merge check is never passed by a reason given for something
  else, and never recorded as passed.
- The skill says what level a plan is written at, what a plan review is
  for and when it stops, and what a later code-review round is for,
  following what this plan's own review showed.
- `atelier log` shows the latest actions first and stays short.

## Success criteria

- status-fits: for the lead of a twelve-item mission with items building,
  in review and blocked, `status` is under 2 KB, and everything that waits
  on that thread comes before anything that does not. For the state behind
  the Hub's 17,870 bytes it is under 4 KB, in the same order.
- pace-shown: a lead's `status` holds one line for each mission it leads,
  with review rounds per item, the time a round took, and the new findings
  after round one. On `hub-v2-bg14`'s records it agrees with a hand count.
- skip-honest: the two Hub merges, replayed, are refused and name the
  failed check. A lead that then merges past it on purpose sees a last line
  saying so, and the merge commit lists the check as failed.
- log-short: on the Hub's history `atelier log` is under 10 KB, newest
  first, with no line over 300 characters and the whole history one flag
  away. Everything in this repository that reads `log` passes its tests.
- skill-measured: `notes.md` holds each round of this plan's review: the
  verdicts, and each new finding marked outcome or mechanism. The skill
  change cites it, and one plan reviewed under the old text is reviewed
  again under the new by fresh reviewers and counted the same way.
- hosts: no record changes form, the new binary reports a new version, and
  the Result names each host and what it must do.

## Scope

Items, in priority order: `atelier-f0sn` (`status`), `atelier-7ci6`
(`merge`), `atelier-qut2` (the skill), `atelier-hota` (`log`).

- `atelier-ww7w` is done, on `master` since 2026-11-22. Its later-round
  plan diff, two-split stop and three-round cap stay: they limit how a
  spiral ends, and `atelier-qut2` is about why one starts.
- `atelier-oiwy` is testing `roles/reviewer.md` and four parts of
  `SKILL.md`. While it is open `atelier-qut2` changes neither, and files
  what can only go there as an item blocked by it.
- `atelier-5asm`, `atelier-3sym`, `atelier-tuln` and `atelier-itqc` fix
  wrong lines in `status` and `log` and stay separate. `atelier-b0vb`
  stays the human's.
- Hosts: this host has one installed `atelier` for every repository. The
  lead installs from `master` after `hub-v2-bg14` closes. A vendored skill
  is refreshed by an item in its repository. The work host and astromech's
  pin are the human's (`atelier-7w9s`, `atelier-pv58`).

Out: the pace table in `atelier show <mission>`, a counted checkpoint for
a lead, a cap on work in flight and an error budget. They were discussed
and not decided, and wait on whether a lead shown the pace line acts on
it. Also out, each filed as an item once this plan merges: `submit`
telling its runner to start a reviewer when a lead runs the queue, and
three different refusals for one branch with two merge bases.

## Authority

The human withholds any new role, command or record, and installing a
binary under a mission in progress. Where an item finds a criterion here
loose or wrong, the lead rules and writes it in `notes.md`.

## Result
