---
schema: atelier.item
schema_version: 3
id: atelier-5xvl
title: 'Generate the planning review brief: planning reviewers look up record syntax twice as often as work reviewers'
status: dropped
mission: atelier-icv4
labels:
- review
- feature
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-09T05:11:56.245567219Z
trivial: null
caused_by: null
created_at: 2026-11-09T05:11:56.245567219Z
---

## Goal

Planning reviews have no generated brief, so each orchestrator writes one by hand and each planning reviewer works out the record commands: Sol planning reviews looked up record help 2.9 times per review against 1.4 for work reviews, and Claude planning reviews 0.9 against 0.5. A third of Sol's reviews (16 of the first 47) were planning reviews. Give `atelier review request` a planning brief.

references/atelier.md says "brief the reviewer with the planning commits (`review request` is for finished work)". A planning review often covers several records: five Sol planning reviews recorded on 4 to 8 items each, one approval or finding per item, which is why approvals outnumber reviews.

## Done when

- [ ] `atelier review request` can brief a planning review of a commit range: it lists each record the range creates or changes, the questions plan.md's Ready Work asks of a mission and of an issue, and for each record the `review finding` and `review approve` commands with the head filled in and `--stage-records` on all but the last.
- [ ] A test renders the planning brief for a range that creates a mission and two issues.
- [ ] references/atelier.md's Planning on the Base points to this brief instead of a hand-written one.

## Scope

`crates/atelier-cli/src/commands/review_request.rs` and its CLI arguments, `skills/agent-factory/references/atelier.md`.

## Context

Evidence: transcripts 2026-11-04 to 2026-11-09; Sol planning reviews 16, record-help lookups 46; Claude planning reviews 31, lookups 29. How a range is named (a commit range, or the planning commits since an item's last approval) is the implementer's choice; both must pin records to the range's head. It shares `review_request.rs` with this mission's other brief items, so their merge order matters only for conflicts. Its finding template takes the form atelier-4gsb settles for the severity prefix.

## Links

Relationships of the old record that the new format has no field for.

- related: atelier-4gsb (Drop severity from the review request brief to match Agent Factory's review output)

## Report

Dropped: Done by the 0.5 rewrite: `atelier review <mission>` prints a plan review's brief.
