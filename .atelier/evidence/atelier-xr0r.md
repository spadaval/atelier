---
created_at: "2026-06-15T07:28:31.015497023+00:00"
id: "atelier-xr0r"
evidence_type: "validation"
captured_at: "2026-06-15T07:28:31.015392311+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-fjmw"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-fjmw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "blocked"
title: "atelier-workflow extraction partial proof: cargo test -p atelier-workflow passed (7 tests); cargo check -p atelier-workflow passed; target/debug/atelier lint atelier-fjmw passed; target/debug/atelier export --check passed; git diff --check passed; target/debug/atelier issue transition atelier-fjmw --options showed close blocked only by missing proof/close_reason. Full affected root crate check is blocked by unrelated missing crates/atelier-sqlite/src/lib.rs, and residue search still finds tests/cli_integration/support.rs using atelier::workflow_policy::STARTER_POLICY_YAML, which this assignment forbids editing."
updated_at: "2026-06-15T07:28:34.092353192+00:00"
---

atelier-workflow extraction partial proof: cargo test -p atelier-workflow passed (7 tests); cargo check -p atelier-workflow passed; target/debug/atelier lint atelier-fjmw passed; target/debug/atelier export --check passed; git diff --check passed; target/debug/atelier issue transition atelier-fjmw --options showed close blocked only by missing proof/close_reason. Full affected root crate check is blocked by unrelated missing crates/atelier-sqlite/src/lib.rs, and residue search still finds tests/cli_integration/support.rs using atelier::workflow_policy::STARTER_POLICY_YAML, which this assignment forbids editing.
