---
schema: atelier.item
schema_version: 3
id: atelier-v06d
title: 'Make an unstarted item open: remove draft, ready and the ready command from the binary, the records and the documents'
status: draft
mission: atelier-dh7a
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-19T15:25:03.017123820Z
---

## Goal

An item is born `draft` and needs `atelier ready` before `start` takes it.
The step records nothing a started status does not, and leads run it as a
ceremony (the mission's Why has the evidence). After this item an unstarted
item is `open`, `start` takes any open item whose blockers are met, and the
`ready` command is gone. Records written before still read, here and in the
five other repositories the installed binary serves, because nothing
converts them.

## Done when

- [ ] `atelier new` writes `status: open`, and `atelier start <id>` starts
  that item with no command between; a test holds it.
- [ ] `atelier ready <id>` is refused for an item as it is for a mission,
  with a line that says items need no go-ahead, and it is in no help text,
  no `status` output and no row of the action table.
- [ ] A record that says `draft` or `ready` reads as `open` everywhere the
  binary reads items, and is never rewritten by a command that does not
  otherwise change it. A test reads one of each. An old record's `ready.by`
  still prints wherever `atelier show` printed it.
- [ ] `atelier list` keeps a way to ask for open items that can start now,
  under a flag that does not name a removed status.
- [ ] `atelier log` still prints an old record's ready line from history,
  and writes none for a new item.
- [ ] `atelier check` passes on this repository's records unchanged, and
  `atelier list` and `atelier check` succeed in a checkout of astromech,
  life-on-rails, cosplay-studio, demiurge and worldcrafter; the Report
  gives the commands and their exit statuses.
- [ ] Every fixture and case in `evals/agent-factory` sets up and grades
  under `test.sh` with no `atelier ready` and no draft step.
- [ ] The skill and the documents say what the binary does: no file under
  `skills/agent-factory` tells an agent to run `atelier ready` or calls an
  item a draft; `procedures/garden.md` starts its run without readying it,
  files something broken as an open item, and drops the time-in-ready
  number; `docs/product.md`'s Item table, `docs/design.md`'s status line,
  action table and decision row, `docs/architecture/desk.md`, `AGENTS.md`
  and `scripts/cutover/agents/` match.
- [ ] The Desk's app and the extension still pass CI's `desk` and
  `extension` jobs. What they show for an open item is atelier-rme3's.
- [ ] The Report gives the lines added and removed under `crates/`.

## Scope

`crates/`, the Desk's server included; `evals/agent-factory` fixtures and
cases; `skills/agent-factory`; the documents named above. In `desk/src` and
`t3-extension/src`, only what keeps their jobs passing. Not `new` on an
item's branch (atelier-u6oy), not the instruction to file (atelier-8phx),
not priority. No status, field or check is added in place of what goes.
atelier-kw8u's items change `evals/agent-factory` too: merge the base in
before submitting.
