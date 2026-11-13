---
schema: atelier.item
schema_version: 3
id: atelier-j5lr
title: Bring the skill, AGENTS.md and the readiness checklist to the branch model
status: draft
mission: atelier-to83
labels:
- redesign
priority: medium
blocked_by:
- atelier-2vdq
- atelier-t5am
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-13T06:14:32.744258640Z
---

## Goal

Bring what agents read into line with the tool. The skill and `AGENTS.md`
tell a planner to work on the base, cherry-pick reviews and wait for Sumeet's
go, and tell everyone that planning records may be committed to `master`
directly.

## Done when

- [ ] `roles/planner.md` describes planning a mission on its branch: `submit`, two blind reviewers each in a checkout of the branch, fixes, and `merge` once both approve. It says a mission starts from Sumeet's direction, that the Why names it, and that a planner never plans a mission nobody asked for. Nothing in it mentions `ready` on a mission or a cherry-pick.
- [ ] The backlog procedure (`backlog.md`, or `groom.md` if atelier-g961 has not merged) says a pass's drops and fixes are committed on a `backlog/` branch and merged through the record-only path.
- [ ] `roles/lead.md` says how a lead adds an item or a note mid-mission, through a record-only branch, and what `lead` now records.
- [ ] `readiness.md` gains one line: a branch that changes only records passes CI in under two minutes, with its test and the measure that shows its failure, the time from `submit` to `merge` on such branches in `atelier log`.
- [ ] `AGENTS.md`'s Work and Merge Policy sections describe the branch model, and nothing in the skill or `AGENTS.md` says a record may be committed on the base.
- [ ] The skill is no longer in total than before this item, and the Report gives the line counts.

## Scope

In: `skills/agent-factory/`, `AGENTS.md`, `CONTRIBUTING.md` where it repeats them.

Out: the gardener's procedure beyond the one checklist line; the other four repositories' own `AGENTS.md`, which atelier-ql64 changes.

## Context

Mission atelier-g961 rewrites `garden.md` and folds groom into the planner as `backlog.md`. This item is written to follow it; if it has not merged, this item edits the files as they are.
