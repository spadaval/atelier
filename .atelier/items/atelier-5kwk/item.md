---
schema: atelier.item
schema_version: 3
id: atelier-5kwk
title: Remind an agent committing in a T3 thread on an unowned branch to run atelier start
status: dropped
mission: atelier-bnek
labels:
- agent-attribution
priority: low
blocked_by:
- atelier-7yy9
implementers: []
ready:
  by: conversion
  at: 2026-11-09T01:04:27.532061591Z
trivial: null
caused_by: null
created_at: 2026-11-09T00:45:59.392192270Z
---

## Goal

Nudge agents to record their work: when an agent in a T3 thread commits code on a branch no item owns, the harness hook reminds it that work on an item should start with `atelier start`, while saying light-path work needs nothing, so attribution improves without implying every change needs an issue.

## Done when

- [ ] Under `atelier setup claude` hooks (and the Codex equivalent if one exists), a commit in a T3 thread on a branch no item owns, in a thread whose session has recorded no item, prints one reminder that names `atelier start <id>` for item work and says a light-path change needs no item. A commit on an owned branch, or in a thread that has recorded an item, prints nothing. Tests cover all three.
- [ ] The reminder never blocks the commit and appears at most once per thread and branch.

## Context

A nudge, not a gate: agents are trusted, and this only improves attribution. The light path (`PRODUCT_INTENT.md`, `docs/product/reviews-and-merge.md`) lets releases, docs and mockups merge with no issue (planning review finding atelier-726p).

## Report

Dropped: Sumeet, 2026-11-11: dropped with mission atelier-bnek; it names commands and the site removed in 0.5, and attribution would need replanning against the Desk
