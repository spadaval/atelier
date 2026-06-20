---
created_at: "2026-06-19T04:17:56.352991945+00:00"
id: "atelier-qxsg"
evidence_type: "validation"
captured_at: "2026-06-19T04:17:56.352990369+00:00"
target:
  kind: "issue"
  id: "atelier-cix4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-cix4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Docs contract updated: CONTEXT.md defines review mode, room, provider-backed artifact, room event, finding, approval, and merge authority; product workflow/validation/CLI docs describe the structured review field, inheritance, legacy pull_request rejection, review merge boundary, room/provider modes, and atelier review commands. Proof run: rg -n 'atelier pr|pull_request|review.mode|review merge' docs/adr CONTEXT.md docs/product/cli-surface.md docs/product/workflow-configuration.md docs/product/validation.md docs/product/command-audit/role-guides.md docs/index.md showed only ADR historical/rejected-context and intentional migration/provider-shape mentions; git diff --check -- '*.md' passed; atelier lint atelier-cix4 passed."
updated_at: "2026-06-19T04:17:59.483098995+00:00"
---

Docs contract updated: CONTEXT.md defines review mode, room, provider-backed artifact, room event, finding, approval, and merge authority; product workflow/validation/CLI docs describe the structured review field, inheritance, legacy pull_request rejection, review merge boundary, room/provider modes, and atelier review commands. Proof run: rg -n 'atelier pr|pull_request|review.mode|review merge' docs/adr CONTEXT.md docs/product/cli-surface.md docs/product/workflow-configuration.md docs/product/validation.md docs/product/command-audit/role-guides.md docs/index.md showed only ADR historical/rejected-context and intentional migration/provider-shape mentions; git diff --check -- '*.md' passed; atelier lint atelier-cix4 passed.
