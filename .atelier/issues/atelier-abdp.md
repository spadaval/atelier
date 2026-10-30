---
created_at: "2026-10-30T06:40:24.501359055+00:00"
id: "atelier-abdp"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Count review rounds in branch readiness and the review request brief"
updated_at: "2026-10-30T06:40:24.501359055+00:00"
---

## Goal

Make a review loop visible while it is happening. A round is the set of review events (finding, approval, review account) pinned to one commit of the owner's branch; an owner whose findings keep arriving at new commits has as many rounds as distinct pinned commits. Show the count, and the fix-now findings raised so far, so the third round of the same flaw is obvious to the orchestrator and the human.

## Done when

Tests in crates/atelier-cli/tests/session.rs and the review_request unit tests show, for an owner with findings pinned at three distinct commits, `3 review rounds` (with the fix-now count) in its `status` Branches row and `round 4` in the next `review request` brief; an owner with no review events shows neither. docs/product/human-cli-output.md (Branches) defines a round.

## Scope

Branch readiness facts (status Branches) and the review request brief. No warning threshold: whether a third round calls for a redesign is process guidance, kept in the skill.

## Context

- Brief renderer: crates/atelier-cli/src/commands/review_request.rs (`render`, Findings so far).
- Readiness row: atelier-efnf, a sibling, adds open fix-now findings to the same facts.
