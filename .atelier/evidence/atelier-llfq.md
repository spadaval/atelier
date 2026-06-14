---
created_at: "2026-06-14T07:36:32.534102890+00:00"
id: "atelier-llfq"
evidence_type: "validation"
captured_at: "2026-06-14T07:36:32.533975341+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-vlw6"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vlw6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "docs/product/work-model.md now documents the preferred mission graph shape: missions link to epics/root work, epics own executable child tasks, and duplicate direct mission links should be deliberate. Proof: rg found Mission Graph Shape example with mission atelier-hy2i and child tasks; git diff --check; atelier lint atelier-vlw6; atelier lint; cargo fmt -- --check."
updated_at: "2026-06-14T07:36:35.183410234+00:00"
---

docs/product/work-model.md now documents the preferred mission graph shape: missions link to epics/root work, epics own executable child tasks, and duplicate direct mission links should be deliberate. Proof: rg found Mission Graph Shape example with mission atelier-hy2i and child tasks; git diff --check; atelier lint atelier-vlw6; atelier lint; cargo fmt -- --check.
