---
schema: atelier.item
schema_version: 3
id: atelier-zno7
title: Re-count agents' Atelier command overhead a week after the CLI fumble fixes ship
status: dropped
mission: null
labels:
- review
- loose_end
priority: low
blocked_by:
- atelier-06jc
- atelier-5xvl
- atelier-jqzl
- atelier-tvfd
- atelier-zzwi
implementers: []
ready:
  by: conversion
  at: 2026-11-09T05:11:56.745208397Z
trivial: null
caused_by: null
created_at: 2026-11-09T05:11:56.745208397Z
---

## Goal

Check that the mission's fixes cut the turns agents spend on Atelier in real use, once the installed binary and skill on this host include all of them and agents have worked a week with them.

## Done when

- [ ] A note on this item gives, for a window of at least seven days after all five fixes are installed, the same per-role measures as the mission's Background (Sol reviews: turns per review and the share spent on `status`/`land`/`check`, history reads and record-help; non-reviewer `--help` lookups per session; failed `merge-ci-passed.sh` runs), next to the 2026-11-04 to 2026-11-09 figures, and files a bug for any pattern that did not fall.

## Context

The 2026-11-09 audit's extractor parsed Claude Code transcripts under `~/.claude/projects` and Codex rollouts under `~/.codex/sessions`; the mission's Background describes the method and cost model.

## Report

Dropped: Its five fixes were dropped as moot after the 0.5 rewrite; the scorecard rerun (atelier-pwhn) is the recount.
