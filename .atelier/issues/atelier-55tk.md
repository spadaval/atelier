---
created_at: "2026-06-24T19:25:52.609349735+00:00"
id: "atelier-55tk"
issue_type: "epic"
labels: []
review:
  kind: pull_request
  number: 25
  provider: forgejo
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3uew"
  - kind: "issue"
    id: "atelier-82u0"
  - kind: "issue"
    id: "atelier-fasv"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-z0ll"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-25T01:01:17.155413322+00:00"
status: "done"
title: "Define command surface and work-view product contract"
updated_at: "2026-06-25T01:01:17.155413322+00:00"
---

## Description

Settle the target product contract for the reduced command surface: root status, work views, issue record detail and mutation, explicit transitions, evidence, review, high-level history, bundle graph preview/apply, smart man guidance, check, and prune. Decide the public vocabulary, non-goals, migration boundaries, and how the internal read/view/panel pipeline is exposed without creating a new DSL, dashboard product, or replacement search surface prematurely.

Constraints:
- This issue decides and documents the command/product contract; it does not implement command removal or the shared read pipeline.
- The contract must name replacement owners for every removed public capability before any deletion issue starts.
- The contract must keep `work` as an operational view surface, not as a general selector DSL or dashboard language.
- The contract must explicitly preserve diagnostic depth for blocked, failed, and warning output while simplifying default human output.

## Outcome

The reduced public command contract is documented and testable, including command ownership, naming, read-only versus mutating behavior, replacement paths for removed commands, and explicit non-goals that prevent the work surface from becoming a public DSL, dashboard language, or backdoor search replacement.

## Evidence

- Command-audit docs record the surviving public command model, removed surfaces, replacement owners, and explicit non-goals.
- A review transcript or focused checklist confirms no removed command is deleted without an assigned replacement capability.
- `atelier lint atelier-55tk` passes after the record and linked documentation updates.
