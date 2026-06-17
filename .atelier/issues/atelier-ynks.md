---
created_at: "2026-06-17T20:03:45.912615253+00:00"
id: "atelier-ynks"
issue_type: "task"
labels:
- "architecture"
- "records"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Fix canonical record-kind APIs so issues are not special-cased"
updated_at: "2026-06-17T20:03:45.912615253+00:00"
---

## Description

Fix the record-kind API leak where issue records are canonical but still require
special-casing outside the first-class record registry. The audit found
`ISSUE_KIND` defined separately while `canonical_record_kind("issue")` rejects
issue, encouraging callers to branch on issue versus non-issue records.

## Outcome

- Canonical record-kind APIs give callers one coherent way to resolve supported
  canonical records, including issues.
- Callers no longer need ad hoc issue special cases merely to compute paths,
  validate schemas, or dispatch parse/render behavior.
- The API still distinguishes records with issue-specific behavior from generic
  first-class records where that distinction is semantically meaningful.

## Evidence

- Focused unit tests cover `issue`, `mission`, `evidence`, and unsupported kinds
  through the canonical kind API.
- Search transcript proves unnecessary issue special cases were removed or
  documented as semantically required.
