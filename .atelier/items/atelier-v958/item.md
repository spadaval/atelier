---
schema: atelier.item
schema_version: 3
id: atelier-v958
title: Show on the Desk what is proposed and in motion and who is doing it, from the live repositories
status: ready
mission: atelier-to83
labels:
- redesign
priority: medium
blocked_by:
- atelier-dciw
- atelier-2vdq
- atelier-xbog
implementers: []
ready:
  by: 'sumeet, on 2026-11-13: execute atelier-to83, relayed by the planner thread'
  at: 2026-11-14T01:30:23.044847409Z
trivial: null
caused_by: null
created_at: 2026-11-13T06:14:32.720158409Z
---

## Goal

Make the Desk the place Sumeet sees everything, since nothing asks for his
go-ahead any more. It reads scratch clones frozen on 2026-11-11, shows only
what is on the base, and its "say go" has no meaning under this mission. This
item takes in atelier-qxmr.

## Done when

- [ ] The installed service's roots are the five live repositories, and for one open mission in each the page's title, status and item counts match `atelier show <mission>` there.
- [ ] The Desk fetches each repository's remote on a timer the config sets, and each page says when it last did.
- [ ] A page lists what is proposed in each repository: plans and record changes on branches, with the branch, its pull request where there is one, and its current round's verdicts. A plan dropped on its branch is not listed as proposed.
- [ ] A page lists work in motion: each started item with its status, read from its branch, and the thread on it, with whether T3 says that thread is alive; and each mission in progress with its lead.
- [ ] Sumeet is told when a plan is proposed and when it merges, by the means `docs/product/desk.md` settles, and the page links the plan.
- [ ] The decide page and its act are gone, with the code that finds a checkout to write an answer into. A page lists what `atelier list --disputed` gives, with each side's model where T3 knows the thread.
- [ ] "Say go" is gone. A mission's page has "stop", which drops the mission with his reason as atelier-2vdq defines a stop, for a proposed plan and for an open mission, and the act is the same record `atelier drop` writes. The Desk carries the stop through itself: it writes the drop, commits it, and publishes it by the path atelier-2vdq gives. A proposed plan is shown as stopped once the drop is pushed and its pull request closed. An open mission's stop is shown as pending until its record-only branch has merged, and then as stopped. The interface test covers both.
- [ ] The Desk's writes go through a checkout of the branch they change and never touch a served repository's own checkout.
- [ ] `docs/product/desk.md` and `docs/architecture/desk.md` describe all of this, and `pnpm test` and `pnpm e2e` pass.

## Scope

In: `desk/`, `crates/atelier-desk`, the Desk's two documents, the installed service's configuration on this host.

Out: a finder (atelier-ic22); a mission's history page (atelier-hj6b); any read the reader of atelier-dciw does not give, which is filed against that item.

## Context

`crates/atelier-desk/src/config.rs` holds `roots`. `desk/README.md` describes the checks `desk-matches-cli` and `act-is-a-record`, which this item keeps passing against the live roots.

How Sumeet is told of a plan is open, and this item settles it in `docs/product/desk.md`. Read `.atelier/missions/atelier-to83/notes.md` first: it names a candidate he agreed to as a separate item (atelier-plr9) and one means he turned down (T3's Browser panel).
