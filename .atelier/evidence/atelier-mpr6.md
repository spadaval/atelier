---
created_at: "2026-06-14T06:25:52.510330120+00:00"
id: "atelier-mpr6"
evidence_type: "validation"
captured_at: "2026-06-14T06:25:52.510295309+00:00"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-6187"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "ADR 0006 ownership boundary enforced in AGENTFACTORY.md, docs/index.md, docs/product/cli-surface.md, and docs/architecture/quality/validation.md. The binding now identifies itself as thin and routes repo-specific tactical guidance to Atelier-owned surfaces. Focused command-surface parity checks passed for atelier --help, atelier prime --help, atelier status --help, and atelier issue transition atelier-6187 --options. atelier lint passed. atelier export --check initially failed because the local projection was stale on canonical issue edits outside this slice; after atelier rebuild, export freshness passed. Residual repo drift remains elsewhere in docs/product/cli-surface.md and is captured in the issue note instead of widened into this docs-boundary task."
updated_at: "2026-06-14T06:25:54.538835255+00:00"
---

ADR 0006 ownership boundary enforced in AGENTFACTORY.md, docs/index.md, docs/product/cli-surface.md, and docs/architecture/quality/validation.md. The binding now identifies itself as thin and routes repo-specific tactical guidance to Atelier-owned surfaces. Focused command-surface parity checks passed for atelier --help, atelier prime --help, atelier status --help, and atelier issue transition atelier-6187 --options. atelier lint passed. atelier export --check initially failed because the local projection was stale on canonical issue edits outside this slice; after atelier rebuild, export freshness passed. Residual repo drift remains elsewhere in docs/product/cli-surface.md and is captured in the issue note instead of widened into this docs-boundary task.
