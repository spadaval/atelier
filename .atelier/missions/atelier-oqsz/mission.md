---
schema: atelier.mission
schema_version: 3
id: atelier-oqsz
title: Show a lead what waits on it and what its reviews cost, in a status under 2 KB, and write plans as outcomes
status: done
labels: []
lead: mcp:5e35d5af-d02e-4108-865d-84da8e4d0e82
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
  change cites it, and `atelier-ww7w`'s plan as its round 1 read it
  (`6a2c26e`, 20 findings) is reviewed again under the new text by the same
  two models, fresh, and counted the same way.
- hosts: no record changes form, the new binary reports a new version, and
  the Result names each host and what it must do.

## Scope

Items, in priority order: `atelier-f0sn` (`status`), `atelier-7ci6`
(`merge`), `atelier-qut2` (the skill), `atelier-hota` (`log`).

- `atelier-ww7w` is done, on `master` since 2026-11-22. Its later-round
  plan diff, two-split stop and three-round cap stay: they limit how a
  spiral ends, and `atelier-qut2` is about why one starts.
- `atelier-oiwy` is testing `roles/reviewer.md` and four parts of
  `SKILL.md`. While it is open `atelier-qut2` changes neither. What a
  plan's reviewer is told goes in the brief `atelier review <mission>`
  prints, which that test does not read, and what can only go in those
  two files is filed as an item blocked by it.
- `atelier-5asm`, `atelier-3sym`, `atelier-tuln` and `atelier-itqc` fix
  wrong lines in `status` and `log` and stay separate. `atelier-b0vb`
  stays the human's.
- Hosts: this host has one installed `atelier` for every repository. The
  lead bumps the version in a trivial merge once every item that changes
  `crates/` is merged, and installs after `hub-v2-bg14` closes. A vendored skill
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

Done on 2026-11-27, the day it was asked for. Six items merged, and
`master` at `cc97add1f` calls itself 0.8.0. Nothing is installed on any
host yet.

**What `master` has now**

- **`status`** (`atelier-f0sn`, `atelier-cpfd`): what waits on the reader
  prints first, then each mission led, then the backlog. Each mission led
  has one line of review pace. An item's holder, blockers and Done when
  lines moved to `atelier show <item>`.
- **`merge`** (`atelier-7ci6`): a failed merge check stops the merge unless
  the reason names that check. A merge past one says so on its last line,
  and its commit lists the check as failed.
- **`log`** (`atelier-hota`): the 30 newest actions, newest first, lines
  cut at 300 characters. `--all` prints the whole history as before.
- **The skill and the review briefs** (`atelier-qut2`): a plan is written
  as outcomes; a lead may reshape an item's Done when as earlier items
  land; a plan review is about what the mission makes true; an item's
  reviewer raises `Shrink:` against a Done when line the Outcome does not
  need; a later round checks the carried findings and the new commits.
- **The version** (`atelier-fbte`): 0.8.0, with a changelog entry that
  tells a host what to do.

**Against the criteria**

- status-fits: met. A twelve-item mission is 1,981 bytes and the Hub's
  largest state 3,826, down from 18,146 (`tests/status_size.rs`).
- pace-shown: met. On `hub-v2-bg14`: 22 rounds for 11 items, a reviewer
  took 15m a round at the median, one verdict to the next took 55m at the
  median and 1h 22m at most, and later rounds found 6 new findings, none
  to fix now. Two hand counts agree.
- skip-honest: met, by a test that replays the Hub's two merges.
- log-short: met. On the Hub's history 6,160 bytes and no line over 271
  characters, down from 174 KB. The garden counter and the scorecard's
  `freeze.py` pass `--all` and give the same answers.
- skill-measured: met in part. `notes.md` holds this plan's review: two
  rounds, 17 minutes, four findings, all about outcomes. The second review
  of `atelier-ww7w`'s plan was not done, by the human's direction of
  2026-11-27, so nothing has tested what a plan's reviewer is now told.
- hosts: met, with one claim corrected. No change here alters a record's
  form. But the plan's "a 0.7.0 binary and a new one work on one
  repository" is false for a binary built at the `v0.7.0` tag: since that
  tag the tree writes `status: open` and a `not built:` Response, which it
  refuses. The binary installed on this host was built on 2026-11-22,
  after both, under the same version number.

**Each host, and what it must do**

- **This host:** `cargo install --path crates/atelier-cli --locked` from
  `master`, and move `~/projects/atelier-skill` to `master` at the same
  time, once `hub-v2-bg14` has closed. At this close it has one item
  building, so neither is done. The lead of this mission does both when it
  closes.
- **A repository with its own copy of the skill**, the Hub's
  `.agents/skills/agent-factory` among them: refresh it with the binary,
  by an item in that repository's tracker. Its garden counter reads plain
  `atelier log` and must pass `--all`.
- **astromech's CI and the work host:** the human's, in `atelier-pv58` and
  `atelier-7w9s`. `CHANGELOG.md` says what each needs.
- No `v0.8.0` tag is pushed. A tag starts the release build, and nobody
  asked for a release.

**Not proven by use**

- No reviewer has yet worked from the new plan-review or item-review
  brief on a real mission, and no lead has reshaped an item under the new
  line in `roles/lead.md`.
- The pace line has been read by one lead, of this mission, where every
  round was short.

**This mission's own pace:** 10 rounds for 6 items. Round one found seven
findings on four items; later rounds found none new. A reviewer took 2
minutes a round at the median, and one verdict to the next took 4.

**Where the lead left the plan,** each with its reason in `notes.md`:
`atelier-qut2` was rewritten on the human's direction; the pace line's
time was ruled to be a later round's whole cost; and the version bump
became a reviewed item because it carries a changelog.

**Filed from what the work found,** all outside the mission:
`atelier-9f1j`, `atelier-nc7t`, `atelier-jahm` and `atelier-hlvq`. They
wait: the human froze Atelier's work on itself from this close, until a
mission in a product repository meets a concrete fault.
