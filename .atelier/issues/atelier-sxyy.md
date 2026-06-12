---
created_at: "2026-06-12T19:29:06.182335463+00:00"
id: "atelier-sxyy"
issue_type: "task"
labels:
- "docs"
- "mission"
- "records"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-7r55"
  - kind: "issue"
    id: "atelier-8ec6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Define readable mission record contract"
updated_at: "2026-06-12T19:29:06.182335463+00:00"
---

## Description

Define the canonical Markdown shape for mission records before implementation.
The contract should make mission content reviewable in normal diffs without
forcing operators to understand `RecordStore` internals.

## Outcome

- The mission record contract names the allowed front matter, Markdown sections,
  and deterministic rendering order.
- The contract states where mission narrative, constraints, risks, validation
  expectations, closeout notes, linked work, blockers, evidence, checkpoints,
  and supporting records live.
- Escaped mission `data` JSON is rejected as the authoring contract.
- Product and storage documentation include a representative before/after
  mission example.

## Evidence

- Documentation diff for the mission record contract.
- Before/after fixture or example showing an existing escaped-JSON mission and
  the readable replacement.
- Review note confirming the contract can be understood without reading the
  implementation.
