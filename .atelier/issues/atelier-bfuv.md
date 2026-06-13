---
created_at: "2026-06-13T02:33:42.569206086+00:00"
id: "atelier-bfuv"
issue_type: "epic"
labels:
- "evidence"
- "schema"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-dv3d"
  - kind: "issue"
    id: "atelier-h2tq"
  - kind: "issue"
    id: "atelier-rzsg"
  - kind: "issue"
    id: "atelier-s8z0"
  - kind: "issue"
    id: "atelier-xmss"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Redesign evidence capture and claim mapping"
updated_at: "2026-06-13T02:36:00.415872253+00:00"
---

## Description

Redesign evidence so proof is structured around claims instead of long prose summaries. Evidence should be easy for agents to capture, easy for validators to inspect, and suitable for mission closeout without private chat context.

## Outcome

- Evidence records can identify targets, covered claims, proof kind, result classification, commands or artifacts, agent identity, independence level, residual risks, and follow-up IDs.
- Evidence capture commands reduce hand-written summaries for shell commands, tests, audits, and transcripts.
- Evidence rendering defaults to compact summaries with drill-down for full details.
- Parent closeout can verify whether required claims are covered by suitable evidence, not just whether any evidence is attached.

## Evidence

- Evidence schema or record contract is documented before implementation.
- Focused tests prove claim-mapped evidence creation, attachment, rendering, and closeout lookup.
- Command transcripts prove agents can capture a test run or CLI transcript without manually writing a huge summary.
