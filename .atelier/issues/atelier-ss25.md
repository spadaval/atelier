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
closed_at: "2026-10-22T22:24:54.831241145+00:00"
status: "done"
title: "Adding an issue to a mission re-checks every done issue in that mission"
updated_at: "2026-10-22T22:24:54.831241145+00:00"
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

## Notes

- 2026-10-22 [decision] Stop touched_claims' walk at the mission: an advances link, declared on either side, touches only the end that is a mission and is not followed further; the reverse walk keeps only blocked_by links.
  Reason: Answers the Unknown: the gates read advances only for a mission's own work.terminal (issue_descendant_ids), and blockers pass only from a parent, so a mission's claim depends on its advancing work but no claim depends on the mission. The mission itself is still re-checked, which a done mission gaining open work needs.
  Rejected: Drop advances from the walk entirely (would miss a done mission whose advancing work reopens, covered by land_checks_a_mission_whose_advancing_work_reopens)

## Close Reason

A commit that only adds an issue advancing a mission re-checks no other done claim in that mission: `land` on the base and on a work branch, and `merge --check`, pass while an older done issue in the mission has failing proof, and `doctor` still reports it. Changing a done issue's own record or a blocker still re-checks it, and a done mission is still re-checked when its advancing work changes.

### Deviations
None.

### Decisions
See the decision note on atelier-ss25: the walk stops at the mission (only the mission end of an `advances` link is touched, from either side), answering the item's Unknown.

### Surprises
Missions are still touched, so a commit adding open work to a mission that is already done re-checks (and fails) that mission's own claim, which is correct. The walk's forward `blocked_by` step remains broader than the claim dependencies need (a new issue blocked by an old done one re-checks the old one); left as is because the Done when keeps blocker re-checks.

### Open items
None.
