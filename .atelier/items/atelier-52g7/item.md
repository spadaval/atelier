---
schema: atelier.item
schema_version: 3
id: atelier-52g7
title: Make the cutover find the docs, skills and scripts that still name removed commands and records, before the work host adopts
status: draft
mission: null
labels:
- redesign
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-11T18:39:09.320822774Z
---

## Goal

Stop the cutover leaving a repository's own signs pointing at things it removed. On this host the conversion of 2026-11-11 rewrote each repository's records and its `AGENTS.md` process section, and left the rest. Four of the five first gardener runs then found, and fixed by hand, docs, skills and scripts that still named removed commands or read removed paths. The work host has not adopted yet (atelier-7w9s), and would get the same.

What the runs found:

- **A script broke silently.** astromech's `scripts/check-release-notes.py` reads the old record paths and now finds 0 open needs-human items where it found 45 (filed there as astromech-xj3u, caused by the conversion commit 775b8544).
- **Docs and skills name removed commands.** astromech's `CONTRIBUTING.md` (`atelier doctor`, `atelier land`); cosplay-studio's `README.md`, `docs/quality/validation.md` and `docs/planning/mission-portfolio.md` (`atelier work`, `atelier issue`, `atelier evidence`); life-on-rails' verify skill (`atelier review request`, `atelier evidence record`); atelier's `desk/README.md` and `docs/architecture/desk.md` (the deleted `next/` directory).
- **IDs that no longer resolve.** The cutover removed finished records, so `atelier show` no longer finds 17 item IDs named in life-on-rails' `AGENTS.md`, verify skill and spec, or 35 of the 65 IDs cosplay-studio's docs name. astromech's `docs/ARCHITECTURE.md` linked two decisions in the removed `.atelier/issues/`.

## Done when

- [ ] `scripts/cutover/` reports, for the repository it converts, each tracked file outside `.atelier/` that names a removed command or reads a removed record path, so whoever runs the cutover sees the list before closing it. It changes none of them.
- [ ] `scripts/cutover/README.md` says what to do with that list and with item IDs that no longer resolve.
- [ ] The work host's runbook item, atelier-7w9s, points at it.

## Scope

In: `scripts/cutover/` and its README. Out: fixing any repository's files, which the gardener runs did on this host; any change under `crates/`.

## Context

Sources: the pages of atelier-0019, astromech-qjle, lor-j26w and cosplay-studio-ktgi, and their gardeners' reports in atelier-613a. The kit stays until the work host has adopted (`AGENTS.md`).
