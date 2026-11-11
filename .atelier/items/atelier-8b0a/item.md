---
schema: atelier.item
schema_version: 3
id: atelier-8b0a
title: Add `atelier log`, the history of acts, and remove from the scorecard scripts what it supplies
status: draft
mission: atelier-295u
labels:
- redesign
priority: medium
blocked_by:
- atelier-wd1i
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T00:56:10.707960610Z
---

## Goal

Give the gardener, a lead and a thread taking over an item one read of what happened: the history of acts, derived from Git. Then remove from the scorecard scripts what that read supplies.

## Done when

- [ ] Provisional: `atelier log` prints one line per act for an item, a mission, or everything since a time, as the architecture specification's section The log describes, and adds nothing up. The mission criterion `from-log` holds for the scripts that remain.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `.atelier/missions/atelier-295u/` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- Which measures still need T3's events and the transcripts; expected: tool calls, tokens and interventions.
- How `log` walks the base, the second-parent side of each merge and started items' branches in one pass. Today's debrief reader follows first parents only (planning findings atelier-mi8y and atelier-1ryt).
- How long that walk takes in astromech, which has the most records.
