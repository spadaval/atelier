---
created_at: "2026-06-15T05:40:59.803157505+00:00"
id: "atelier-wagk"
evidence_type: "validation"
captured_at: "2026-06-15T05:40:59.803111016+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-14bv"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-14bv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Docs define current work as checked-out issue Markdown with status in_progress; command audit classifies root abandon/repair for removal and start as workflow-status transition. Proof: atelier lint atelier-14bv passed; atelier export --check reported 'Canonical export is current'; git diff --check passed; targeted rg found only negative/removal references for runtime association/work_associations/hidden claim guidance."
updated_at: "2026-06-15T05:41:01.685765059+00:00"
---

Docs define current work as checked-out issue Markdown with status in_progress; command audit classifies root abandon/repair for removal and start as workflow-status transition. Proof: atelier lint atelier-14bv passed; atelier export --check reported 'Canonical export is current'; git diff --check passed; targeted rg found only negative/removal references for runtime association/work_associations/hidden claim guidance.
