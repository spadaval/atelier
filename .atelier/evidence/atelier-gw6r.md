---
created_at: "2026-06-19T23:52:43.883922908+00:00"
id: "atelier-gw6r"
evidence_type: "validation"
captured_at: "2026-06-19T23:52:43.883921745+00:00"
target:
  kind: "issue"
  id: "atelier-33a4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-33a4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Independent review of master..epic/atelier-33a4 found two documentation contract defects: docs/product/work-model.md still grants transition actions issue-status/activity write authority, contradicting ADR 0013 and workflow-configuration's intrinsic status-write contract; docs/product/cli-surface.md still calls review artifact transition work an effect outside historical ADR context. Checks run: git diff --check master..HEAD; focused rg searches for effects/hooks/category/validator/action drift; atelier lint atelier-33a4."
updated_at: "2026-06-19T23:52:46.778516889+00:00"
---

Independent review of master..epic/atelier-33a4 found two documentation contract defects: docs/product/work-model.md still grants transition actions issue-status/activity write authority, contradicting ADR 0013 and workflow-configuration's intrinsic status-write contract; docs/product/cli-surface.md still calls review artifact transition work an effect outside historical ADR context. Checks run: git diff --check master..HEAD; focused rg searches for effects/hooks/category/validator/action drift; atelier lint atelier-33a4.
