---
created_at: "2026-10-01T21:19:17.628482220+00:00"
id: "atelier-kgcr"
issue_type: "mission"
labels: []
strategy:
  path: docs/strategies/atelier-rebuild.md
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1ioo"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Finish the verified repo-native Atelier rebuild"
updated_at: "2026-10-04T11:17:28.581348303+00:00"
---

## Outcome

Finish the verified repo-native tracker rebuild described in docs/strategies/atelier-rebuild.md. Track the remaining work here; the strategy owns intent and acceptance criteria.

## Notes

- 2026-10-04 [human] 2026-10-04 human direction: remove the performance target entirely and remove micro-optimizations added to chase it. Continue measuring command timings and address practical issues and regressions; no concrete time budget. This authorizes the S4 strategy change. The current 8ud1 slice now owns removing the budget and simplifying retained performance scaffolding; final rebuild acceptance still requires fresh correctness and independent review.
