---
created_at: "2026-06-23T16:21:20.081249084+00:00"
id: "atelier-idwz"
issue_type: "feature"
labels: []
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0p7e"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-06T18:31:54.767293671+00:00"
status: "done"
title: "Rewrite cache rebuild for domain schema"
updated_at: "2026-07-06T18:31:54.767293671+00:00"
---

## Description

Rewrite cache rebuild logic to load typed record files and populate the new domain-shaped cache rows.

## Outcome

- A full cache rebuild from record files produces correct issue, evidence, review, relationship, label, and source metadata rows.

## Evidence

- `cargo nextest run -p atelier-app rebuild::tests` proves full/incremental equivalence, bounded repair, broad fallback, rollback safety, nested-transaction regression, and schema mismatch rebuild.
- `cargo nextest run -p atelier-sqlite` proves the domain indexer transaction and schema invariants used by rebuild.
- `cargo fmt --all -- --check`, `git diff --check`, and `atelier check` prove repository integrity.
