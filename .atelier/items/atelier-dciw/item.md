---
schema: atelier.item
schema_version: 3
id: atelier-dciw
title: Read missions and items from named branches, local and fetched, so list and show cover work in motion
status: draft
mission: atelier-to83
labels:
- redesign
priority: medium
blocked_by:
- atelier-ob9u
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-13T06:14:32.646572592Z
---

## Goal

Make work in motion visible from any clone. The tool reads a started item from
the local branch named for it and everything else from the base, so a plan on
a branch, a backlog change waiting to merge, or an item started on another
host appears nowhere.

## Done when

- [ ] The reader in `atelier-app` finds records on every branch the convention names, local and remote-tracking for the configured remote, and gives each record the ref it was read from. The rule for which copy wins is the one in `docs/design.md`.
- [ ] A mission whose record is only on its branch is read from there, with its items.
- [ ] For a proposed mission the reader also finds the `review/<mission-id>/...` branches, and `show <mission>` says how many verdicts each round has. It gives the verdicts and their findings only for a round already brought onto the mission's branch, so a reviewer who runs `show` learns nothing of another's verdict in the round it is reviewing. A test publishes one verdict and `show` in a second clone prints the count and no verdict.
- [ ] Proposed means status `open` read from a branch that has not merged, in the reader and in every view. A mission dropped on its branch is not listed as proposed: `list` and `show` give it as dropped, with the reason, and `--proposed` leaves it out.
- [ ] `atelier list` shows, apart from the base's backlog, what is proposed: missions whose status is `open`, and record changes, on branches not yet merged, each with its branch. `--proposed` narrows to them.
- [ ] `atelier show <id>` finds a record wherever it is and says where it read it from. For a record read from a remote-tracking ref it says how long ago that remote was fetched.
- [ ] `atelier status` lists a started item held on another host by the thread its branch names.
- [ ] No command fetches. A test builds two clones of one bare remote, plans in the first, pushes, fetches in the second and reads the plan there.
- [ ] With 200 named remote branches, `atelier list` takes under two seconds on this host, and the Report gives the time.

## Scope

In: `crates/atelier-app` (`repo.rs`, `git.rs`, `view.rs`, `show.rs`), `crates/atelier-cli`'s output for `list`, `show` and `status`, and their tests.

Out: any write; fetching on a timer, which the Desk does (atelier-v958); a cache on disk, unless the last line cannot be met without one and the Report says so.

## Context

`docs/architecture/desk.md` describes the reader as it is: `Located`, `ReadFrom::Base` and `ReadFrom::Branch`, and a cache keyed on the base's tip and the tip of every local branch named for an item.
