---
created_at: "2026-11-10T19:33:55.282106206+00:00"
id: "atelier-jq24"
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
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-yptz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Add `drop` and `close`, and plan reviews of a mission with `review` and `verdict`"
updated_at: "2026-11-10T19:33:55.282106206+00:00"
---

## Goal

Let work end without being built, and let a plan be reviewed before Sumeet says go: `drop` and `close` for items and missions, and plan reviews of a mission with the same `review` and `verdict` a reviewer already uses on an item.

## Done when

- [ ] `drop <id> --reason "<why>"`, run on the base, sets a draft or ready item or mission to `dropped` and stores the reason in the record. It is refused without a reason and on a done record. Dropping a mission changes no item; `start` on one of its items is then refused and says the mission was dropped.
- [ ] Scenario test, a started item dropped on the base: `drop` writes the base's record whatever the item's branch holds. Once that is committed, `status` from the base and from the item's branch says the item was dropped and why, and `submit`, `merge` and a take-over `start` on it are each refused with the same reason and nothing changed.
- [ ] `close <item> "<result>"` sets a draft or ready item with no branch to `done` and writes the result into its Report. It is refused without a result text, and for an item whose branch exists, naming `merge` as the way that item ends.
- [ ] `close <mission>` sets a ready mission to `done`. It is refused, listing what is in the way, while any of its items is neither done nor dropped or its Result section is empty.
- [ ] Scenario test, a mission from draft to closed: `new --mission`, two items made with `new --in`, `ready` on the mission and on one item, that item's work through `merge`, `close` refused while the other item is a draft, `drop` on it, the Result written, and `close`. A second scenario closes an item with no files on the base with a result.
- [ ] `review <mission>` opens a plan review: it creates a review file under `missions/<id>/reviews/`, binds the caller as its reviewer, and prints a brief holding the mission's sections, the paths of its other documents and of its items, the path of the review file and the two `verdict` commands. Run again by the same thread it only prints the brief; run by another thread on a file with no verdict it takes the review over and says who held it. A round with every file's verdict recorded is over, and the next `review` opens the next round, whose file carries the open fix-now findings of the rounds before, as an item's does.
- [ ] `verdict approve` or `verdict changes` on a plan review validates the file and commits it and nothing else; it changes no status. `approve` is refused while the file holds an open fix-now finding.
- [ ] Scenario test, two blind plan reviews: two threads each run `review <mission>` and `verdict` in their own detached worktree of the same commit, and a planner cherry-picks both commits onto the base with no conflict; both files are then round 1.
- [ ] `ready <mission>` prints whether the plan has a review, each latest review's verdict, and whether the mission's directory or any of its items changed since the commit reviewed. It never refuses for any of that.
- [ ] `drop` and `close` make no commit: each prints the paths it wrote and a `Then:` line with the Git command that commits them. `verdict` commits, as it does for an item.
- [ ] The act table gains the rows for `drop`, `close` on an item, `close` on a mission and `verdict` on a mission review, and the unit test over the table covers them from every status.
- [ ] `docs/strategies/redesign-architecture.md` matches what was built: where a drop's reason and a close's result are stored, the plan review's rounds, and the four rows.

## Scope

`drop`, `close`, and `review` and `verdict` on a mission, in `next/`. Dropping a deferred finding is step C. `show`, `list` and `lead` are atelier-jciw, which another implementer builds at the same time in new files of its own (`atelier-app`'s and `atelier-cli`'s view modules); keep out of those, and keep additions to shared files (`main.rs`, `output.rs`) to what this item needs. Nothing is copied from the old workspace. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, Records, Status and "Where records are read and written". Behaviour: `docs/strategies/redesign-product.md`, Lifecycle (Item, Mission, "When a thread dies"), Commands and Review (Plan reviews). What a plan reviewer judges is the skill's to say, not the brief's. The commands atelier-yptz built (`new`, `set`, `ready`) set the conventions for a planning command: it writes, does not commit, and ends with a `Then:` line.

This is the second of step B's three items in `docs/strategies/redesign-2-core.md`.
