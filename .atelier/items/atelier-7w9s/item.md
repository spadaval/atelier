---
schema: atelier.item
schema_version: 3
id: atelier-7w9s
title: Convert the work host with the cutover runbook
status: ready
mission: null
labels:
- needs-human
- redesign
- loose_end
priority: medium
blocked_by:
- atelier-52g7
implementers: []
ready:
  by: conversion
  at: 2026-11-10T00:56:12.036820453Z
trivial: null
caused_by: null
created_at: 2026-11-10T00:56:12.036820453Z
---

## Goal

Move Sumeet's work host, which agents on this host cannot reach, onto the new binary, skill and records.

## Done when

- [ ] Sumeet ran the runbook there and `atelier check` passes in each repository on that host.

## Context

The runbook is `scripts/cutover/README.md`, run with `scripts/cutover/cutover.sh adopt`; `docs/design.md`'s Transition section describes it. atelier-52g7 makes the kit list the docs and scripts that still name removed commands, and comes first. Until this is done the work host keeps the old binary, skill and records, which still work together.
