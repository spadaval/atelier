---
created_at: "2026-06-19T20:14:19.582245658+00:00"
id: "atelier-68sm"
issue_type: "feature"
labels:
- "schema"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-yrql"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add workflow schema for declared transition effects"
updated_at: "2026-06-19T20:14:19.582245658+00:00"
---

## Description

Add workflow schema support for declared transition effects after the product
contract lands. This issue owns parsing, validation, and documentation of the
configuration shape.

## Outcome

- `.atelier/workflow.yaml` can declare supported effects on transitions using
  the contract-approved syntax.
- Strict validation rejects unknown effects, invalid params, invalid ordering,
  unsupported review modes, duplicate incompatible effects, and effect
  declarations on transitions where the contract forbids them.
- Schema examples and error text make the v1 review effect discoverable without
  implying review commands transition workflow.

## Evidence

- Focused parser/config tests cover accepted and rejected effect declarations.
- File changes in `.atelier/workflow.yaml`, `docs/product/workflow-configuration.md`,
  or workflow schema docs show the effect declaration shape.
- `atelier lint atelier-68sm` passes.
