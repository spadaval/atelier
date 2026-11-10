---
created_at: "2026-11-10T00:56:07.800857010+00:00"
id: "atelier-yptz"
issue_type: "task"
labels:
- "redesign"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-67nn"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Add mission records and the commands that write a plan: `new`, `set`, `ready` and `start --new`"
updated_at: "2026-11-10T19:34:04.477915158+00:00"
---

## Goal

Let a planner write work down and say go on the new records: a mission record, the commands that create and change drafts, and `ready`. With them `start` can check everything the act table asks of it, and work Sumeet asks for directly can begin with `start --new`.

## Done when

- [ ] `missions/<id>/mission.md` reads, writes and validates as the architecture specification's Records section defines it, with its body kept byte for byte. Validation names the file and the field for an unknown status and for a mission with no Outcome section. Any other file in a mission's directory is not examined: a test puts a Markdown file with no front matter and a file that is not text beside `mission.md`, and the mission still validates. An item whose `mission` names no mission is rejected, naming the item and the field.
- [ ] `new "<title>"` on the base writes `items/<id>/item.md` as a draft with empty Goal, Done when and Scope sections and prints its path. `--in <mission>` sets its mission and is refused for a mission that does not exist. `--mission` writes `missions/<id>/mission.md` as a draft with its section headings instead. An ID is the config's slug and four characters, and no mission and item share one.
- [ ] `set <id>` changes a draft or ready record's title, priority, labels, mission, blocked-by and, on an item, caused-by, and nothing else in the file: a test compares the body byte for byte. It is refused, with nothing written, for a blocked-by that names no item, a mission that names no mission, a caused-by that is neither an item ID nor a commit hash in form, a started item (naming its branch as where its record now changes), and a done or dropped record.
- [ ] `ready <id>... --by <name>` sets each named draft item or mission to `ready` with who said so and when. If any named ID is unknown or not a draft, nothing is written and each such ID is named. `--by` is required. Readying a mission changes no item.
- [ ] `new`, `set` and `ready` make no commit. Each prints the paths it wrote and ends with a `Then:` line giving the Git command that commits them. `status` on the base lists record changes not yet committed.
- [ ] `start` checks every condition of its row in the act table, each with a test of its refusal and the way out it names: a blocker that is not done; a mission that is not ready, naming `ready`; and an item that is ready in the base checkout but not in the base's last commit, saying the change is not committed yet.
- [ ] `start --new "<title>"` creates an item already `building` on a new branch named for it, with the caller as its implementer, in one commit made the way `start` makes its own, so an interrupted run leaves no branch. `status` then tells the implementer to write the item's Goal and Done when lines, and `submit` is refused until the Goal has text and Done when has at least one line.
- [ ] The act table gains the rows for `ready` on an item, `ready` on a mission and `start --new`, and the unit test over the table covers them from every status.
- [ ] `docs/strategies/redesign-architecture.md` matches what was built: the mission record, the three rows, where planning records are read (the checkout by the planning commands, the base's last commit by `start`), and the flags of `set`, which leaves the Open questions list.

## Scope

Mission records and `new`, `set`, `ready` and `start --new` in `next/`. `drop`, `close` and plan reviews are atelier-jq24; `show`, `list` and `lead` are atelier-jciw; `new --from <finding>` is step C; the Git hooks are step D, so nothing validates a hand-made commit yet and each command validates what it writes. Nothing is copied from the old workspace: ID generation is already in `atelier-records`. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, Records, Status and "Where records are read and written". Behaviour: `docs/strategies/redesign-product.md`, Lifecycle, Commands (Planner, Implementer) and "Where records change". The specifications say planning is committed and pushed with Git and that no act makes a commit only to change a status, which is why these commands write and do not commit. Step A's fresh-agent runs (atelier-67nn, `evals/agent-factory/results/2026-11-10-next-loop.md`) set the conventions for what commands print: a `Then:` line naming the next command, and no backticks around a command an agent may paste into a shell.

This is the first of step B's three items in `docs/strategies/redesign-2-core.md`.
