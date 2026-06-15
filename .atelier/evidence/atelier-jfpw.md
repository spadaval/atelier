---
created_at: "2026-06-15T05:39:20.735436577+00:00"
id: "atelier-jfpw"
evidence_type: "validation"
captured_at: "2026-06-15T05:39:20.735396996+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-0rdo"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-0rdo"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "File review: docs/architecture/source-layout.md defines target crates, dependency direction, internal API policy, temporary-adapter limits, and current checkout orientation; docs/architecture/index.md and docs/index.md reference the target workspace contract. Targeted stale-guidance search: rg -n 'Do not look for a `crates/`|There is no multi-crate `crates/` tree|no multi-crate `crates/`|not look for `crates/`|not look for a crates|Do not look for.*crates|There is no.*crates' docs .atelier -g '*.md' exited 1 with no matches. Validation passed: git diff --check -- docs/architecture/source-layout.md docs/architecture/index.md docs/index.md; atelier lint atelier-0rdo -> Lint passed.; atelier export --check -> Canonical export is current."
updated_at: "2026-06-15T05:39:22.515645200+00:00"
---

File review: docs/architecture/source-layout.md defines target crates, dependency direction, internal API policy, temporary-adapter limits, and current checkout orientation; docs/architecture/index.md and docs/index.md reference the target workspace contract. Targeted stale-guidance search: rg -n 'Do not look for a `crates/`|There is no multi-crate `crates/` tree|no multi-crate `crates/`|not look for `crates/`|not look for a crates|Do not look for.*crates|There is no.*crates' docs .atelier -g '*.md' exited 1 with no matches. Validation passed: git diff --check -- docs/architecture/source-layout.md docs/architecture/index.md docs/index.md; atelier lint atelier-0rdo -> Lint passed.; atelier export --check -> Canonical export is current.
