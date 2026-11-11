---
schema: atelier.item
schema_version: 3
id: atelier-jy8k
title: Schedule the gardener to run daily on Sumeet's Atelier repositories
status: ready
mission: null
labels:
- agent-factory
- loose_end
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-08T21:02:48.544109589Z
trivial: null
caused_by: null
created_at: 2026-11-08T05:35:32.496582982Z
---

## Goal

Run the gardener procedure daily through a T3 Code scheduled task once the skill change merges.

## Done when

- [ ] A T3 scheduled task runs the gardener daily on atelier, astromech and life-on-rails (others on Sumeet's say), one gardener run per repository, with heavy builds sent to boat, and its first run's output is checked by hand: the routine fixes it merged after agent review, and the digest of recommendations it left for Sumeet.

## Context

Waits on atelier-3sk9, which adds procedures/garden.md.
