---
schema: atelier.item
schema_version: 3
id: atelier-ql64
title: Move the five repositories to the branch model and prove it on one real mission
status: draft
mission: atelier-to83
labels:
- redesign
priority: medium
blocked_by:
- atelier-v958
- atelier-j5lr
- atelier-yclc
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-13T06:14:32.768322272Z
---

## Goal

Put the five repositories on this host onto the branch model and show it
working end to end, since one installed binary serves all of them and a
half-moved repository would stop its agents.

## Done when

- [ ] atelier is moved first: its mission records are in the new format, its hooks are reinstalled with `atelier setup`, and a mission of at least two items is planned on its branch, reviewed twice, merged, led and closed there with no `ATELIER_SKIP`. The Result names it.
- [ ] Over that mission, `git log --first-parent` on the base shows only merges of named branches.
- [ ] Then astromech, demiurge, life-on-rails and cosplay-studio: records converted, hooks reinstalled, and each one's `AGENTS.md` changed where it says records are committed on the base.
- [ ] In each of the five, a branch that changes only records goes from `submit` to merged in under two minutes, timed from `atelier log`. Where a repository's CI has no fast path for records, this item adds one or files the item and says so.
- [ ] The Desk's five checks of atelier-v958 still pass after the conversion.
- [ ] atelier-7w9s's runbook, `scripts/cutover/README.md`, says what the work host adopts now, in one section, so that host is converted once.
- [ ] The mission's criterion `protected-base` is run as a script against the test remote in both modes, and the script is kept under `scripts/`.

## Scope

In: the records, hooks, `AGENTS.md` and CI rule of the five repositories on this host; `scripts/cutover/README.md`.

Out: branch protection on any GitHub repository; the work host itself; any open item's content in the four product repositories.

## Context

Memory of this repository: one binary serves all repositories, so a format change needs the old form read (atelier-2vdq does this), and a change the old binary cannot read is merged before `cargo install`. `scripts/ci-jobs.sh` holds atelier's own fast path for records.
