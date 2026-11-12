---
schema: atelier.item
schema_version: 3
id: atelier-h201
title: Say in `garden.md` and `readiness.md` what four first runs each had to guess
status: dropped
mission: null
labels:
- redesign
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-11T18:39:09.256409128Z
---

## Goal

Close the gaps in the gardener's procedure that the first runs in astromech, demiurge, life-on-rails and cosplay-studio each filled by guessing. Where several runs guessed the same thing they agreed, so nothing came out wrong on the five pages, but the next gardener will guess again, and may not agree.

Do this after Sumeet's sessions on the five pages, which may change the page itself.

## Done when

- [ ] `garden.md` says where a repository other than atelier finds the Scorecard's measures. Step 2 names `docs/product.md`, which only atelier has; all three product runs that reported on it used atelier's list with their own column of the baseline.
- [ ] `garden.md` says whether the open counts include the run's own item (three runs counted it and marked it), and gives the wording for a window with no merged item and for a first run's Last run column.
- [ ] `garden.md` says what a first run with nothing merged reads for causes. cosplay-studio's run took its evidence from the records before the cutover, which the procedure never mentions.
- [ ] `garden.md` says what becomes of a bug the run files on the base (draft or ready, and on whose word), and where a page names items the run found moot or stale.
- [ ] `readiness.md` says which result a line gets when it covers several parts with different results, and whether a line is judged on the base or after the run's own fixes. The runs chose the worst part and the base.
- [ ] `readiness.md` settles the four places two runs read differently or had to choose: whether a command named only in a file `AGENTS.md` does not link counts as documented (lines 2 and 4); not met against could not test when there is no verify skill (line 8); line 7 where no forge is configured, so no run precedes a merge; and a bug whose cause is outside the repository (line 9).
- [ ] Line 3's script no longer reports a path under a home directory, a URL or an HTTP route as a missing file.
- [ ] Each change is a sentence or less where it can be, and the Report gives both files' line counts before and after (188 and 110 today).

## Scope

In: `skills/agent-factory/procedures/garden.md` and `readiness.md`.

Out: a command or script that takes the numbers or runs the checklist; any change under `crates/`; the definition of any measure in `docs/product.md`.

## Context

The gardeners' full lists are in the Report of atelier-613a. atelier-f6dv (pull request 149) fixed what atelier's own run, the trial, showed. Mission: atelier-295u.

## Report

Dropped: Folded into atelier-wb05, which rewrites garden.md and readiness.md and carries each of its gaps in a Done when line
