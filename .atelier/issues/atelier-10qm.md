---
created_at: "2026-06-13T21:51:16.438009990+00:00"
id: "atelier-10qm"
issue_type: "task"
labels:
- "cleanup"
- "dead-code"
- "stabilization"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-e723"
    type: "related"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Inventory and remove dead code paths"
updated_at: "2026-06-13T21:51:16.438009990+00:00"
---

## Description

Identify unused modules, commands, compatibility helpers, inherited Chainlink residue, and unreachable code paths, then delete them or explicitly attach them to a current product contract.

## Outcome

- Dead code candidates are inventoried by category: unused Rust items, unreachable command paths, inherited product residue, compatibility helpers, stale docs fixtures, and dormant tracker/runtime paths.
- Each candidate is resolved by deletion, reconnection to a documented current product contract, or a follow-up issue with an explicit owner and validation target.
- Removed code has corresponding test, help, docs, or residue-search proof showing the old path no longer leaks into the supported CLI or data model.

## Evidence

- Inventory artifact or issue note lists the scanned files, commands, and dead-code categories with disposition for each candidate.
- Command transcripts from selected dead-code scans, residue searches, and focused tests are attached or cited.
- `cargo fmt -- --check`, relevant `cargo nextest` slices, `atelier lint`, `atelier export --check`, and `atelier doctor` pass after removals.
