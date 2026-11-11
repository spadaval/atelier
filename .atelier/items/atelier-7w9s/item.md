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
- atelier-2mlh
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

The runbook is the Transition section of `docs/strategies/redesign-architecture.md`. Until this is done the work host keeps the old binary, skill and records, which still work together.
