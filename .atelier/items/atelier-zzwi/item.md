---
schema: atelier.item
schema_version: 3
id: atelier-zzwi
title: 'Tell reviewers every command they run and that they run nothing else: Sol reviews spend 3 of 14 turns on status, land and syntax lookups'
status: ready
mission: atelier-icv4
labels:
- agent-factory
- review
priority: high
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-09T05:11:55.491797348Z
trivial: null
caused_by: null
created_at: 2026-11-09T05:11:55.491797348Z
---

## Goal

Sol reviews spend 3 of their 14 turns on Atelier commands a reviewer should not need: `atelier status`, `land` and `check` (1.8 turns, 14% of a review's tokens) and `--help` on the record commands (1.2 turns, 12%). Make the brief list every command a reviewer runs, say it runs no other, and stop the guidance a reviewer also reads from telling every session to start with `status` and end with `land`.

Two causes:

- Everything a Codex reviewer reads tells every session to run `status` and `land`. SKILL.md opens with "Run `atelier status` first". install.md has each repository's AGENTS.md say "`atelier status` to start, `atelier land` before handoff", and life-on-rails' says `atelier land --push`. `land`'s help says "Run at every handoff" and the top-level help says "Run land before every handoff". Codex has no SessionStart hook, so Sol ran `status` in nearly every review (53 runs in 50 reviews, about 2,400 tokens of output carried through the review) and `land` 34 times, including `land --push` in life-on-rails reviews. The brief says "do not ... merge or push" but not "do not run status, check or land", and AGENTS.md won.
- The brief's Record section gives `review finding` and `review approve` only. Reviewers then read `review --help`, `review record --help`, `review disposition --help` and `land --help` to learn whether anything else is expected: 126 lookups in 50 Sol reviews and 118 in 173 Claude reviews, including reviews whose prompt pasted the exact commands (2.6 lookups per Sol planning review). Help's usage line reads `atelier review <EVENT> ...` for every verb.

## Done when

- [ ] The brief's Working rules say the reviewer runs no `atelier status`, `check`, `land`, `merge` or `doctor` and pushes nothing, because its record commands commit what they write and the orchestrator merges and lands. A brief rendering test checks the line.
- [ ] The brief's Record section lists every command a reviewer may need, flags filled in, says when to use `review record` (or that it is not needed), and says no other Atelier command is needed. A brief rendering test checks it.
- [ ] SKILL.md's opening `atelier status` instruction, the AGENTS.md lines install.md prescribes, and this repository's AGENTS.md apply `status` at start and `land` at handoff to a session that owns work; an agent given a brief starts from the brief.
- [ ] `land`'s help and the top-level help say `land` is for whoever hands off work it owns, not for a reviewer.
- [ ] Each `atelier review <verb> --help` usage line names its verb, e.g. `atelier review finding [OPTIONS] --identity <IDENTITY> <ID> <SUMMARY>`.

## Scope

`crates/atelier-cli/src/commands/review_request.rs` (Working rules and Record), `crates/atelier-cli/src/main.rs` (help text), `skills/agent-factory/SKILL.md`, `skills/agent-factory/procedures/install.md`, `AGENTS.md`. Other repositories' AGENTS.md files change when their owners next take the install procedure's lines; this item does not edit them.

## Context

Evidence: transcripts 2026-11-04 to 2026-11-09 (see the mission's Background for the method). Sol: `status` 53, `land` 34, `check` 28 in 50 reviews. Claude reviewers: `status` 15, `land` 19, `check` 69 in 173. Record-help lookups by Sol reviews whose prompt pasted the commands: 2.6 per planning review, 0.7 per work review. Some `check` runs review record changes and are proof; `merge` runs `check` anyway. Claude reviewers also ran `doctor` 30 times. The Record section's finding template keeps whatever form atelier-4gsb settles for the severity prefix.

Claude reviewers run `status` far less (0.09 per review) because the SessionStart hook already printed it.

## Links

Relationships of the old record that the new format has no field for.

- related: atelier-4gsb (Drop severity from the review request brief to match Agent Factory's review output)
