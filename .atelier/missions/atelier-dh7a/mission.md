---
schema: atelier.mission
schema_version: 3
id: atelier-dh7a
title: 'Make a filed item simply open: no draft or ready, filed on the branch you are on, and agents told to file what they find'
status: open
labels: []
lead: null
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
and no `ready` command. `start` takes any open item whose blockers are met.
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

Records written before the change still read: `draft` and `ready` both
read as `open`, in this repository and in every other the binary serves,
with no conversion run. Two things are given up, and the human chose both.
The Desk and the extension no longer split loose items into "next" and
"later", since that split was the ready mark. And the gardener's numbers
lose the time an item spent ready before it started.

## Success criteria

- no-gate: in a fixture repository, an item made by `atelier new` is
  started by `atelier start <id>` with no command between them, and
  `atelier ready` on any record is refused with a line that says items
  need no go-ahead.
- one-word: outside the code that reads old records and the tests of it,
  nothing in `crates/`, `desk/src`, `t3-extension/src`, `skills/`,
  `docs/product.md`, `AGENTS.md` or the current-state parts of
  `docs/design.md` names `draft` or `ready` as an item's status, and
  `atelier check` passes on this repository's records unchanged.
- old-records-read: `atelier list` and `atelier check` succeed in
  astromech, life-on-rails, cosplay-studio, demiurge and worldcrafter with
  the new binary and no change to their records, and an item a human
  readied still says who, wherever `atelier show` said it before.
- file-where-you-are: in a fixture, an implementer on its item's branch
  runs `atelier new`, commits, and after `atelier merge` the new item is
  open on the base; before the merge `atelier list` shows it as proposed.
- none-lost: in a fixture, dropping a started item whose branch holds a
  newly filed item either carries that item to the base or names it in
  what `drop` prints, and a test holds whichever it is.
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
