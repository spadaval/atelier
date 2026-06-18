---
created_at: "2026-06-18T23:33:22.501746337+00:00"
id: "atelier-lr1e"
evidence_type: "validation"
captured_at: "2026-06-18T23:33:22.501745202+00:00"
target:
  kind: "issue"
  id: "atelier-cp7i"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-cp7i"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Role-guide and PR workflow guidance drift cleaned. Proof: docs/product/cli-surface.md, docs/product/command-audit/index.md, docs/product/command-audit/man.md, and docs/product/command-audit/role-guides.md consistently list worker, reviewer, validator, manager, and admin; rg found no stale role list omitting validator; target/debug/atelier man validator prints the documented validator command loop; git diff --check and target/debug/atelier lint passed."
updated_at: "2026-06-18T23:33:25.152164856+00:00"
---

Role-guide and PR workflow guidance drift cleaned. Proof: docs/product/cli-surface.md, docs/product/command-audit/index.md, docs/product/command-audit/man.md, and docs/product/command-audit/role-guides.md consistently list worker, reviewer, validator, manager, and admin; rg found no stale role list omitting validator; target/debug/atelier man validator prints the documented validator command loop; git diff --check and target/debug/atelier lint passed.
