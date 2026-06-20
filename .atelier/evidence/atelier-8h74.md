---
created_at: "2026-06-19T23:48:06.624705558+00:00"
id: "atelier-8h74"
evidence_type: "validation"
captured_at: "2026-06-19T23:48:06.624704306+00:00"
target:
  kind: "issue"
  id: "atelier-jwvd"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-jwvd"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Validated workflow policy docs target schema: docs/product/workflow-configuration.md now includes issue_types registry, active-category review/validation statuses, namespaced validators, transition actions, static descriptions, and validation issue workflow examples; SPEC.md and CONTEXT.md use registry/category/action vocabulary. Checks passed: atelier lint atelier-jwvd; git diff --check. Manual rg evidence: no docs contain category: review or category: validation; review/validation only appear as not-required global categories in target docs. Old effects hits are limited to historical ADR/supersession context and rejected effects schema text. Flat validator hits remain outside updated target docs or as namespaced review.linked_pr_merged target references."
updated_at: "2026-06-19T23:48:09.655373923+00:00"
---

Validated workflow policy docs target schema: docs/product/workflow-configuration.md now includes issue_types registry, active-category review/validation statuses, namespaced validators, transition actions, static descriptions, and validation issue workflow examples; SPEC.md and CONTEXT.md use registry/category/action vocabulary. Checks passed: atelier lint atelier-jwvd; git diff --check. Manual rg evidence: no docs contain category: review or category: validation; review/validation only appear as not-required global categories in target docs. Old effects hits are limited to historical ADR/supersession context and rejected effects schema text. Flat validator hits remain outside updated target docs or as namespaced review.linked_pr_merged target references.
