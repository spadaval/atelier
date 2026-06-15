---
created_at: "2026-06-15T07:50:38.226877443+00:00"
id: "atelier-h8uk"
evidence_type: "validation"
captured_at: "2026-06-15T07:50:38.226762695+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-y3ur"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-y3ur"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Moved issue mutation and relationship construction/rendering behind atelier-records API. Proof: cargo test -p atelier-records record_store passed (8 tests); cargo check -p atelier-records passed; cargo check -p atelier-tracker passed; focused CLI tests passed for issue create durability, issue blocker durability, bulk plan links/export/rebuild, mission unlink, and records_evidence suite; target/debug/atelier lint atelier-y3ur passed; target/debug/atelier export --check reported canonical export current; git diff --check passed. Search transcript found relationship constructors and record-kind lists only in crates/atelier-records source/tests or command imports of records API constants; removed src/record_store duplicate record_kinds.rs and relationships.rs. Broad issue_records and mission_projection filters still include unrelated stale/removed-command fixture failures."
updated_at: "2026-06-15T07:50:41.350931191+00:00"
---

Moved issue mutation and relationship construction/rendering behind atelier-records API. Proof: cargo test -p atelier-records record_store passed (8 tests); cargo check -p atelier-records passed; cargo check -p atelier-tracker passed; focused CLI tests passed for issue create durability, issue blocker durability, bulk plan links/export/rebuild, mission unlink, and records_evidence suite; target/debug/atelier lint atelier-y3ur passed; target/debug/atelier export --check reported canonical export current; git diff --check passed. Search transcript found relationship constructors and record-kind lists only in crates/atelier-records source/tests or command imports of records API constants; removed src/record_store duplicate record_kinds.rs and relationships.rs. Broad issue_records and mission_projection filters still include unrelated stale/removed-command fixture failures.
