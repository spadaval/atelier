---
created_at: "2026-06-17T22:19:42.439757534+00:00"
id: "atelier-nhfu"
evidence_type: "validation"
captured_at: "2026-06-17T22:19:42.439748697+00:00"
target:
  kind: "issue"
  id: "atelier-yq99"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-yq99"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Workflow ownership contract documented in docs/architecture/source-layout.md and docs/architecture/index.md. Search transcript over workflow_policy, status_category, transition options, validator evaluation, and WORKFLOW_POLICY_PATH shows current duplicated behavior lives across atelier-app workflow_policy and CLI command modules; the docs now assign parsing, status categories, transition lookup, validators, guidance, and branch lifecycle policy to atelier-workflow, with app orchestration and CLI rendering boundaries. Validation: target/debug/atelier lint atelier-yq99; target/debug/atelier lint; git diff --check."
updated_at: "2026-06-17T22:19:46.428004227+00:00"
---

Workflow ownership contract documented in docs/architecture/source-layout.md and docs/architecture/index.md. Search transcript over workflow_policy, status_category, transition options, validator evaluation, and WORKFLOW_POLICY_PATH shows current duplicated behavior lives across atelier-app workflow_policy and CLI command modules; the docs now assign parsing, status categories, transition lookup, validators, guidance, and branch lifecycle policy to atelier-workflow, with app orchestration and CLI rendering boundaries. Validation: target/debug/atelier lint atelier-yq99; target/debug/atelier lint; git diff --check.
