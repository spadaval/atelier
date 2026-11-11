---
schema: atelier.item
schema_version: 3
id: atelier-06jc
title: 'Put open findings in full and the item''s Scope in the review brief: re-reviews spend 14% of Sol''s tokens fetching them one by one'
status: ready
mission: atelier-icv4
labels:
- review
priority: high
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-09T05:11:55.748621962Z
trivial: null
caused_by: null
created_at: 2026-11-09T05:11:55.748621962Z
---

## Goal

Re-reviewers fetch the findings they must judge one record at a time, because the brief cuts them short: Sol reviews ran `review show`, `review list` and `evidence show` 2.7 times per review, 11% to 14% of a review's tokens (the higher figure counts their output carried through later turns). Put what a re-review judges in the brief in full, and make every cut line name the command that shows the rest.

Causes, all in the brief and the review commands:

- `review_request.rs` cuts every finding, earlier review and proof line to 300 characters and ends it `... (atelier review show)` with no ID. Proof lines are receipts, which `evidence show` shows, not `review show`.
- `review list <id>` cuts each summary to about 90 characters.
- `atelier review show <item-id>` refuses: "atelier-3sk9 is an issue, not a review. Use `atelier issue show atelier-3sk9`." An orchestrator's prompt told Sol to run exactly that to see the findings.
- The brief leaves out the item's Scope and Context; references/atelier.md asks the orchestrator to add Scope by hand. Reviewers ran `issue show --full` 189 times, 38 to 63 of them on the item under review (depending on how the item is matched); the rest read linked items, which is legitimate.

## Done when

- [ ] A re-review brief prints in full every finding the reviewer must judge: each `fix_now` finding no approval has resolved, and each finding with no disposition. Settled findings and earlier reviews stay one line each. A brief rendering test covers both.
- [ ] Every line the brief cuts names the command and ID that shows the rest: `atelier review show <review-id>` for a review event, `atelier evidence show <receipt-id>` for a proof line.
- [ ] The brief prints the item's Scope and Context when the record has them, and references/atelier.md no longer asks the orchestrator to add Scope.
- [ ] `atelier review show <item-id>` prints that item's review events in full, oldest first, instead of refusing.

## Scope

`crates/atelier-cli/src/commands/review_request.rs`, `crates/atelier-cli/src/commands/review.rs`, `skills/agent-factory/references/atelier.md`.

## Context

Evidence: Sol reviews ran `review show` 74, `review list` 35 and `evidence show`/`list` 28 times in 50 reviews; Claude reviewers 53 such reads in 173. Re-reviews of atelier-3sk9 fetched each of four open findings by ID after reading the brief. atelier-pvo0 covers other bare truncations; atelier-abdp and atelier-4gsb also change the brief.

## Links

Relationships of the old record that the new format has no field for.

- related: atelier-pvo0 (Name the command for the remaining silent or bare truncations)
