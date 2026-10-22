---
created_at: "2026-10-17T19:00:36.353984020+00:00"
id: "atelier-1mn2"
issue_type: "task"
labels:
- "retro-2026-10-17"
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
closed_at: "2026-10-22T20:27:52.928580551+00:00"
status: "superseded"
title: "Scale independent review to docs-only and low-risk changes"
updated_at: "2026-10-22T20:27:52.928580551+00:00"
---

## Goal

Docs and wording branches paid full review cost: bw0b 3 rounds and 21 findings, mostly wording; wgil, cr0r, p2f3 cost more ceremony than change. About half of 1w8c's 47 findings were closed with no action.

## Done when

The Agent Factory skill (vendored and upstream) gives a review budget by change kind: for docs-only branches, one round with only high/medium findings blocking and the rest recorded as no-action or deferred in bulk. If atelier needs support (for example a severity threshold in `findings.disposed`), it is filed separately.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).

## Close Reason

Superseded 2026-10-22 by the human's decision that review has one standard: a review budget by change kind (lighter review for docs-only changes) is a second standard. Prose gets independent review like code, and the trivial exemption covers changes whose correctness is evident (atelier-mbrf). If review noise is still a problem, file it as general review calibration, not docs-specific.
