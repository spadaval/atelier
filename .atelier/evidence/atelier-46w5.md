---
created_at: "2026-06-19T23:39:48.212164921+00:00"
id: "atelier-46w5"
evidence_type: "validation"
captured_at: "2026-06-19T23:39:48.212163202+00:00"
target:
  kind: "issue"
  id: "atelier-h7n4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-h7n4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Docs define .atelier/config.toml/.atelier/workflow.yaml/local runtime ownership boundary; workflow review artifact configuration is documented as target transition actions; local runtime/cache ownership is documented. Manual check found .atelier/config.toml selects provider=forgejo, names runtime_dir=.atelier/runtime, runtime_database=.atelier/runtime/state.db, cache_dir=.atelier/cache, and token env FORGEJO_ADMIN_TOKEN; .atelier/workflow.yaml declares branch_policy and review_artifact_open on review transitions but still uses legacy effects key, which remains out of scope for atelier-h7n4 and is left for dependent migration work. Checks passed: atelier lint atelier-h7n4; git diff --check."
updated_at: "2026-06-19T23:39:50.968348399+00:00"
---

Docs define .atelier/config.toml/.atelier/workflow.yaml/local runtime ownership boundary; workflow review artifact configuration is documented as target transition actions; local runtime/cache ownership is documented. Manual check found .atelier/config.toml selects provider=forgejo, names runtime_dir=.atelier/runtime, runtime_database=.atelier/runtime/state.db, cache_dir=.atelier/cache, and token env FORGEJO_ADMIN_TOKEN; .atelier/workflow.yaml declares branch_policy and review_artifact_open on review transitions but still uses legacy effects key, which remains out of scope for atelier-h7n4 and is left for dependent migration work. Checks passed: atelier lint atelier-h7n4; git diff --check.
