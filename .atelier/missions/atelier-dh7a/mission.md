---
schema: atelier.mission
schema_version: 3
id: atelier-dh7a
title: 'Make a filed item simply open: no draft or ready, filed on the branch you are on, and agents told to file what they find'
status: done
labels: []
lead: mcp:launch:d9ce201b02acc943fe31a743127b2bc6eadad19a1d9e5ad882baa055012ff415
planners:
- 62596020-99cc-45fd-b632-3d3d6dc63385
created_at: 2026-11-19T15:23:31.634513877Z
---

## Why

The human asked for this on 2026-11-19, in conversation. They called the
way draft and readiness work infuriating, and over the conversation settled
what replaces it: an item needs no approval to exist, as in any issue
tracker, where the list holds junk and the judgment is made when someone
picks work up. They asked for a mission that removes the gate, lets an agent
file on the branch it already has, and tells agents to file what they find.

Three things stand between an agent that notices a problem and an item on
the backlog.

- **Every item is born `draft` and needs `atelier ready` before `start`
  takes it.** The decision of 2026-11-13 kept this so that a lead could hold
  later items back as drafts. Leads do not: all nine items of atelier-osel
  were readied in one commit on `backlog/osel-ready`, merged the moment the
  plan had its two approving reviews. A gardener runs `atelier ready` on its
  own run before it may start it. Nothing starts a ready item unasked: work
  begins because the human, a lead or a plan said so, and that is where the
  item is judged. `ready` checks nothing about the item either
  (atelier-k55o: it accepts one whose Goal and Done when are empty). The
  status records "nobody has started this", which the lack of a started
  status already says.
- **`atelier new` is refused on an item's branch.** The action table in
  `docs/design.md` puts it on a backlog branch or a proposed mission's, and
  the pre-commit hook enforces the table. An implementer with a loose end
  must make a second branch, commit, submit and merge to record one
  sentence. One exception exists and has caused no trouble: `new --from
  <finding>` on the branch of an item the caller implements.
- **No role is told to file what it finds.** `roles/implementer.md` says
  nothing about a problem outside its item. The only such instruction is
  the gardener's, for something broken today, and that one makes a draft
  that waits for a session.

So the likely outcome today is that a problem an agent notices is mentioned
in a Report or lost. This repository holds 35 draft and 9 ready items, and
the five others the installed binary serves hold 216 draft and 147 ready.

## Outcome

An item is open from the moment it is filed. Its statuses are `open`, then
the started ones, then `done` or `dropped`; there is no `draft`, no `ready`
and no `ready` command. `start` takes any open item that is on the base and
whose blockers are met.
An item in a mission needs nothing after the plan merges, and a gardener
starts its run without readying it.

An agent files an item with `atelier new` on whatever branch `start` gave
it, an item's included, and the item reaches the base with that branch's
merge. Until then `atelier list` shows it as proposed, as it shows any
record waiting to merge. A backlog branch is needed only by a thread that
holds no branch.

Each role's instructions say to file a problem found outside the work in
hand and carry on, and an eval case shows that saying so changes what an
agent does.

Records written before the change still read: an item's `draft` and
`ready` both read as `open`, in this repository and in every other the
binary serves, with no conversion run. This is about items only. A mission
in the old form reads as it does now: its `ready` is `open`, and its
`draft` stays proposed and cannot be led or have its items started. The
other five repositories hold nine such draft missions, and the binary's
action table writes a proposed mission as an item's `draft` today, so the
two must be separated before the item's status goes. Two things are given up, and the human chose both.
The Desk and the extension no longer split loose items into "next" and
"later", since that split was the ready mark. Both keep showing and
counting every unstarted item from the merge that changes the binary on. And the gardener's page
loses its "waiting to start" number: time spent open is the backlog's age,
not a wait, so the cell goes and nothing replaces it.

## Success criteria

- no-gate: in a fixture repository, an item made by `atelier new` and
  merged to the base on the branch it was filed on is started by `atelier
  start <id>` with no readying and no approval between. An item still on
  an unmerged branch cannot be started, as now. `atelier ready` on any
  record is refused with a line that says items need no go-ahead.
- one-word: outside the code that reads old records and the tests of it,
  nothing in `crates/`, `desk/src`, `t3-extension/src`, `skills/`,
  `docs/product.md`, `AGENTS.md` or the current-state parts of
  `docs/design.md` names `draft` or `ready` as an item's status, and
  `atelier check` passes on this repository's records unchanged.
- old-records-read: `atelier list` and `atelier check` succeed in
  astromech, life-on-rails, cosplay-studio, demiurge and worldcrafter with
  the new binary and no change to their records, and an item a human
  readied still says who, wherever `atelier show` said it before. Each
  mission stored as `draft` in those repositories still shows as proposed,
  and `atelier lead` and `atelier start` of one of its items are refused
  as before.
- file-where-you-are: in a fixture, an implementer on its item's branch
  runs `atelier new`, commits, and after `atelier merge` the new item is
  open on the base; before the merge `atelier list` shows it as proposed.
- none-lost: in a fixture, dropping a started item whose branch holds a
  newly filed item either carries that item to the base or names it in
  what `drop` prints, and a test holds whichever it is.
- approval-holds: a commit on an item's branch that only adds a new
  item's record, made after that item was approved, leaves the approval
  standing, and `atelier merge` goes through with no further round.
- told-to-file: the eval case of atelier-8phx is run with and without the
  skill's new line, at least five runs per arm in the `house` setup, and
  the results give for each arm how many runs filed the planted problem
  and how many still merged their item. The line stays only if more runs
  file with it.
- decision-recorded: `docs/design.md` reverses the row "Items keep `draft`
  and `ready` (2026-11-13)" with this mission's reason, and its action
  table has no `ready` row and puts `new` on any branch `start` makes.

## Scope

- In: the item's statuses and the `ready` command wherever `crates/` holds
  them, the Desk's server included; the pre-commit hook's rule for `new`;
  what `drop` does with items filed on a started item's branch; the Desk's
  app and the T3 extension where they show, sort or filter by `draft` or
  `ready`; the fixtures and cases in `evals/agent-factory` that write or
  run them, and one new case; the skill's role and procedure files where
  they name `ready` or a draft, and one instruction to file;
  `docs/product.md`, `docs/design.md`, `docs/architecture/desk.md`,
  `AGENTS.md` and `scripts/cutover/` where they name either.
- Out: priority, which stays the sort hint it is; a mark for what the
  human wants next, which they declined on 2026-11-19; a proposal record
  type, which they raised and set aside the same day; deferred findings,
  which stay in their review files and are settled as now; filing with no
  branch at all, or `new` committing and merging by itself; rewriting old
  records in any repository; the work host, which adopts whatever binary
  the cutover gives it.
- Target environment: this host, where one installed binary serves six
  repositories. The binary is reinstalled from the base after
  atelier-v06d merges and before any record is written `open` elsewhere.

## Authority

The lead may choose the name of the flag that replaces `list --ready` for
"open and can start now", the wording the binary prints, how old records
are read, where in the skill the instruction to file sits, and the eval
case's fixture and rubric. It may split atelier-v06d if a reviewer finds
it too big for one pass, and may drop atelier-k55o as moot once
atelier-v06d merges.

It may not add a status, a record type, a field or a check in place of
what is removed; make `start` refuse an open item for what its text lacks;
or keep the skill's new line on an eval result that does not show a gain.
The mission removes more than it adds, and an item whose change grows the
binary's line count says why in its Report. atelier-kw8u's items change
`evals/agent-factory` at the same time: an item here that touches a
fixture merges the base in before it submits.

## Result

Done on 2026-11-23. All four items merged, and `master` is at `1260e5f46`.
Seven of the eight success criteria hold as written. The eighth,
told-to-file, was decided the other way round from what the Outcome hoped:
the eval showed no gain from telling agents to file, so the skill does not
say it.

**What `master` has now**

- **Open items** (`atelier-v06d`, `3f97eaae8`): `atelier new` writes
  `status: open`, `start` takes any open item on the base whose blockers
  are done, and `atelier ready` only refuses, with "Items need no
  go-ahead". An old record's `draft` or `ready` reads as `open` and is not
  rewritten; an old mission's `draft` still reads as proposed. `list
  --startable` replaces `list --ready`. Non-test Rust under `crates/` fell
  from 24,409 to 24,333 lines.
- **Filing where you are** (`atelier-u6oy`, `405282844`): `atelier new`
  runs on any branch `start` made, a started item's included, for its
  implementer, its reviewer or anyone else. The item reaches the base with
  that branch's merge and makes no approval stale. `merge` refuses while a
  filed item is uncommitted, and `drop` names each item the dropped branch
  filed and prints the commands that carry it. Non-test Rust grew to
  24,507 lines; the item's Report splits the 174 by what needs them.
- **The Desk and the extension** (`atelier-rme3`, `1d815029a`): loose open
  items are one list by priority and then ID, a mission's page says of
  each unstarted item that it can start or what it waits on, and a
  proposed mission is drawn by its own state.
- **The skill** (`atelier-8phx`, `1260e5f46`): `roles/lead.md` and
  `procedures/garden.md` file on the branch they hold. The eval case
  `outside-problem` stays, as a guard that an implementer files on its
  item's branch and still merges.

**The success criteria**

- no-gate, file-where-you-are, none-lost, approval-holds: each has a test
  in `crates/atelier-cli/tests/filing.rs`, `planning.rs` or `branches.rs`, CI passed on
  `405282844`, and the ten eval runs of `atelier-8phx` each filed on an
  item's branch and merged with the installed binary. `drop` names what
  was filed and carries none; `docs/design.md` says so.
- one-word: on `405282844` the lead searched `crates/`, `desk/src`,
  `t3-extension/src`, `skills/`, `docs/product.md` and `AGENTS.md`. What is
  left names an old record's `ready` mark, an old mission's `draft`, a
  `readied` event in history, or T3's composer drafts. One thing is left
  on purpose: the Desk's `/api/launch` and `/api/motion` keep fields named
  `ready`, which hold open items that can start, so no reader of them
  breaks.
- old-records-read: with the installed binary, `atelier list` and `atelier
  check` exit 0 in astromech, life-on-rails, cosplay-studio and demiurge
  (`worldcrafter` is a symlink to demiurge, so the plan's five
  repositories are four and its nine draft missions are eight). `check`
  printed the same bytes as the binary from before. All eight draft
  missions show as proposed, `start` of an item of each is refused for
  that reason, and a readied item still prints "Made ready by".
- told-to-file: `outside-problem` in `house`, five runs an arm. 5 of 5
  filed the planted problem and merged without the line, and 5 of 5 with
  it, so `161da1c50` removed the line
  (`evals/agent-factory/results/2026-11-23-told-to-file.md`). Without it
  each Opus implementer chose to file and found how from `atelier new
  --help`. The case is at its ceiling for that model, and was not run in
  `sol`, the work host's setup.
- decision-recorded: `docs/design.md` reverses "Items keep `draft` and
  `ready`" and "`new` belongs on a backlog or mission branch", its action
  table has no `ready` row, and `new` is on any branch `start` made.

**What the plan missed, and what was done about it**

- A program from before cannot read `status: open`, and three programs
  read records, not one. The lead reinstalled `atelier` and the Desk
  (`atelier-desk.service`) from `3f97eaae8`, `1d815029a` and `405282844`,
  moved `~/projects/atelier-skill` forward each time, and released the
  extension as `t3-extension-v0.2.1` and `t3-extension-v0.3.0`. The human
  installs a release with `t3-custom update`.
- astromech's CI pins an Atelier that fails on the first open record
  there. Nothing was written in astromech; `atelier-pv58` waits on the
  human.
- With no ready mark, `status` would have listed every open item at each
  session start (166 in astromech). The lead ruled a cap: the ten most
  urgent, then a count and `atelier list --startable` (`a89f1fc9a`).

**Rulings and departures**

- `atelier-8phx`'s first Done when line required the skill to carry the
  instruction. Once the eval showed no gain it could not hold beside the
  last line, so the lead changed it on the item's branch (`ccd8f39b7`) to
  ask that the instruction was written and tried.
- `atelier-v06d`, `atelier-rme3` and `atelier-u6oy` were reviewed by
  Gemini 4 Neon alone, since Codex and Grok were out on 2026-11-22;
  `atelier-8phx` by GPT-6.1 Sol. No second opinion was taken on
  `atelier-v06d`.
- `atelier-k55o` was dropped as moot.

**Not proven by use yet**

- The extension's releases ran in CI and in headless Chromium, not inside
  T3 Code.
- A reviewer filing from an item's brief, and `drop` naming filed items,
  have run only in tests and a scratch repository.
- `atelier lead` of an old draft mission was checked by its test and by
  the reviewer, not by the lead.

**Filed from what the work found:** `atelier-pv58` (needs-human),
`atelier-1sqp`, `atelier-5c08`, `atelier-jr1b` (from the one deferred
finding), `atelier-ngzf`, `atelier-h7e9`, `atelier-fuzq`, `atelier-5q9j`,
`atelier-s2ho` and `atelier-jk0s`, all open. Not filed: the Desk's This
thread page draws a mentioned proposed mission as running, and
`atelier-yy4j` removes that page.
