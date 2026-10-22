---
created_at: "2026-10-22T19:44:07.933934859+00:00"
id: "atelier-ss25"
issue_type: "bug"
labels:
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Adding an issue to a mission re-checks every done issue in that mission"
updated_at: "2026-10-22T22:20:04.350549232+00:00"
---

## Goal

`land` on the base re-checks only claims a change can affect. Adding a new issue that advances a mission today pulls the mission into the touched set (`touched_claims` follows `advances`), and through it every done issue of the mission is re-verified, so a one-record planning commit costs a mission-wide check and fails on any old problem in it.

## Done when

- A commit that only adds a new issue advancing a mission re-checks no done claim but the new issue's own (none, when it is not done); a test covers it with an older done issue in the mission whose proof no longer verifies, which `land` must skip and `doctor` must still report.
- Changing a done issue's own record, or one it is blocked by, still re-checks it (existing tests keep passing).

## Context

Bisected in a scratch clone of brewer-corp/astromech at 2ba8bfd2: an unpushed commit adding an unlinked issue, a note, or an edit to an unlinked issue lands; one new issue with `advances astromech-7q0z` re-checks the mission's done issues and hits the merge-commit review bug (atelier-as7i). `touched_claims` and `claim_nodes` are in `crates/atelier-cli/src/commands/session.rs`; atelier-37uq introduced the changed-claims design.

## Unknowns

Whether following `advances` into a mission was meant for the mission's own claim (a mission's completion can depend on its issues) rather than for its siblings. If so, the fix is to stop the walk at the mission rather than to drop the link.
