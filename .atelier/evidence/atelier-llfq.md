---
created_at: "2026-06-14T07:36:32.534102890+00:00"
id: "atelier-llfq"
evidence_type: "validation"
captured_at: "2026-06-14T07:36:32.533975341+00:00"
target:
  kind: "issue"
  id: "atelier-vlw6"
  role: "validates"
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
status: "recorded"
title: "docs/product/work-model.md now documents the preferred mission graph shape: missions link to epics/root work, epics own executable child tasks, and duplicate direct mission links should be deliberate. Proof: rg found Mission Graph Shape example with mission atelier-hy2i and child tasks; git diff --check; atelier lint atelier-vlw6; atelier lint; cargo fmt -- --check."
updated_at: "2026-06-14T07:36:35.183410234+00:00"
---

docs/product/work-model.md now documents the preferred mission graph shape: missions link to epics/root work, epics own executable child tasks, and duplicate direct mission links should be deliberate. Proof: rg found Mission Graph Shape example with mission atelier-hy2i and child tasks; git diff --check; atelier lint atelier-vlw6; atelier lint; cargo fmt -- --check.
