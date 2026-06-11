---
acceptance: []
created_at: "2026-06-11T18:55:24.517430677+00:00"
evidence_required: []
id: "atelier-dydv"
issue_type: "validation"
labels:
- "storage"
- "validation"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Validate one-directory Atelier state cleanup end to end"
updated_at: "2026-06-11T18:55:24.517430677+00:00"
---

Prove the one-directory .atelier cleanup works across docs, init, export, rebuild, and query flows.

Scope:
- Validate fresh init behavior after the cleanup.
- Validate migration from existing .atelier-state records into .atelier canonical records.
- Validate export --check, rebuild, issue list/show/ready, lint, and doctor against the new layout.
- Confirm tracked config and canonical records appear in git status while DB/cache/locks remain ignored.
- Capture any remaining compatibility gaps as follow-up issues.

Acceptance criteria:
- cargo fmt -- --check passes.
- cargo nextest run passes or failures are documented with concrete follow-ups.
- atelier export --check, atelier lint, and atelier doctor pass on the migrated layout.
- The epic has evidence that .atelier-state is no longer required for normal operation.
