---
created_at: "2026-06-23T16:21:20.080499829+00:00"
id: "atelier-nxq9"
issue_type: "feature"
labels: []
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-idwz"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-06T18:20:43.376126974+00:00"
status: "done"
title: "Implement domain-shaped cache tables"
updated_at: "2026-07-06T18:20:43.376126974+00:00"
---

## Description

Implement the new cache schema with domain tables for issues, evidence, reviews, links, labels, and cache source metadata.

## Outcome

- New cache databases initialize with the domain-shaped schema.

## Evidence

- `cargo nextest run -p atelier-sqlite` proves the domain schema, forbidden-layout assertions, cache-version classification, and atomic add/replace/delete behavior.
- `cargo fmt --all -- --check` and `git diff --check` prove formatting and whitespace integrity.
