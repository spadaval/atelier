---
created_at: "2026-06-23T16:21:20.074436041+00:00"
id: "atelier-ax1g"
issue_type: "feature"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-06T18:19:16.327942859+00:00"
status: "done"
title: "Introduce typed record-file storage abstractions"
updated_at: "2026-07-06T18:19:16.327942859+00:00"
---

## Description

Introduce clearer storage-layer types and naming for record files/documents, keeping typed codecs for issue, evidence, and review payloads.

## Outcome

- Storage types name record-file concerns without implying that all domain objects are generic records.

## Evidence

- `cargo nextest run -p atelier-records` focused typed-codec and concrete
  record-file service tests pass; transcript: `atelier-70vn`.
