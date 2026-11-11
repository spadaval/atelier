---
schema: atelier.item
schema_version: 3
id: atelier-abdp
title: Count review rounds in branch readiness and the review request brief
status: draft
mission: atelier-4pzj
labels:
- feature
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-30T06:40:24.501359055Z
---

## Goal

Make a review loop visible while it is happening. A round is the set of review events (finding, approval, review account) pinned to one commit of the owner's branch; an owner whose findings keep arriving at new commits has as many rounds as distinct pinned commits. Show the count, so the third round of the same flaw is obvious to the orchestrator and the human. In the 2026-10-15 to 10-29 transcripts atelier-vkzj took 7 review rounds, efkh about 6 and 37uq 5, each round finding a wider form of the same flaw. This shows state; it gates nothing.

## Done when

- [ ] Tests in crates/atelier-cli/tests/session.rs and the review_request unit tests show, for an owner with findings pinned at three distinct commits, `3 review rounds` in its `status` Branches row and `round 4` in the next `review request` brief; an owner with no review events shows neither.
- [ ] docs/product/human-cli-output.md (Branches) defines a round.

## Scope

Branch readiness facts (status Branches) and the review request brief. No warning threshold: whether a third round calls for a redesign is process guidance, kept in the skill.

## Context

- Brief renderer: crates/atelier-cli/src/commands/review_request.rs (`render`, Findings so far).
- The brief's round number is one more than the rounds so far, unless the head already has review events, in which case it is that head's round.
- atelier-efnf adds open findings to the same facts; whichever lands second joins them in one fact (`3 review rounds, 1 open finding`). Neither waits on the other.
- Kept in decision atelier-mzpn from the superseded atelier-ll3p.

## Links

Relationships of the old record that the new format has no field for.

- related: atelier-efnf (Show an open finding in branch readiness, so the approval mark agrees with merge)
