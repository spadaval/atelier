---
schema: atelier.mission
schema_version: 3
id: atelier-qr5s
title: Move all five repositories and Agent Factory onto the new loop, one cutover per host
status: ready
labels:
- redesign
lead: null
ready:
  by: conversion
  at: 2026-11-11T15:17:33.448605463Z
created_at: 2026-11-10T00:56:04.626625678Z
---

## Why

Mission 2 leaves a second binary on master that nothing uses. The installed
binary and the installed Agent Factory skill are each one per host, so every
repository on a host changes process together. This mission makes that change
once, with a rehearsal behind it and a way back.

The skill has to change with the tool. It names commands that will no longer
exist, and its own evals have never shown it beating no skill: the one
comparison found no gain and was contaminated, and no clean no-skill arm has
run since.

## Outcome

On this host, atelier, astromech, demiurge, life-on-rails and cosplay-studio
run on the new records and the new binary. Agent Factory is rewritten against
the new tool, and each procedure it keeps has an eval scenario it changed.
The old workspace is deleted, and the site with it. The new binary has no `serve`. The old docs and the ADRs are replaced by the
design record. Sumeet has a runbook to convert his work host.

## Success criteria

- five-converted: with the new binary installed, `atelier check` passes in
  all five repositories, and one item has gone from ready to done under the
  new binary in atelier and in one product repository.
- no-old-surface: a search for each removed command and record path finds
  no instruction to use it in the skill, in the five repositories'
  `AGENTS.md` and `CLAUDE.md`, or under `docs/`. Naming one as history is
  allowed: in the design record's list of what was tried and removed, and in
  the baseline's reports.
- skill-earns-its-place: a results file records skill against no skill, on
  Claude Opus and on one Codex model, at least three runs per cell. Every
  procedure the skill keeps names a scenario where it changed the outcome.

## Scope

- In: the skill, its evals, the backlog sort, the fallback release, taking
  the site out, the switch, the cutover on this host, first use, and the
  docs.
- Out: the work host, which Sumeet converts with the runbook; the gardener,
  groom and audit procedures, which mission 4 rewrites and which are carried
  over unchanged except for command names; the Desk, which is mission
  atelier-z4cr; new scenarios
  beyond those needed to judge the kept procedures.
- Target environment: this host; Claude Opus and a Codex model for the evals.

## Authority

The orchestrator plans each step at its boundary. It may cut any skill
procedure no scenario supports, drop the backlog items the sort finds
obsolete, and fix what first use breaks inside the specifications.

Sumeet names the time of the cutover. The orchestrator takes the switch as
far as its approval and stops there, with everything the cutover will run
reviewed and the rollback tried. Once Sumeet has named the time, and no work
is in flight in any of the five repositories and no other thread is running
in them, it merges the switch and runs the runbook, which pushes the
conversion commits directly.

It may not push anything else directly, weaken the independent review rule
or the three exits of a fix-now finding, change the specifications' Words,
Lifecycle or Review sections, or touch the work host.

**Scope rule.** As in mission 2: a finding that asks for behaviour the
specifications do not name is escalated, not fixed in place.
