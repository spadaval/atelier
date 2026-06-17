---
created_at: "2026-06-17T23:22:53.507075220+00:00"
id: "atelier-dtnw"
evidence_type: "validation"
captured_at: "2026-06-17T23:22:53.507067159+00:00"
target:
  kind: "issue"
  id: "atelier-ynks"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ynks"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Fixed canonical record-kind API leakage. CANONICAL_RECORD_KINDS now includes issue, mission, and evidence; canonical_record_kind(\"issue\"), validate_canonical_record_kind(\"issue\"), and canonical_record_path for issue all work. Generic record parse/render/write dispatch handles issue through the canonical registry while preserving issue-specific payload semantics. SQLite projection source identity now resolves directories through CANONICAL_RECORD_KINDS instead of branching on ISSUE_KIND. Search transcript shows remaining issue special cases are semantic relationship/body handling or convenience wrappers. Validation passed: cargo check -p atelier-records; cargo check -p atelier-sqlite; cargo test -p atelier-records; cargo test -p atelier-sqlite projection_index; cargo test -p atelier-app export; cargo fmt -- --check; target/debug/atelier lint atelier-ynks; git diff --check; cargo build -p atelier-cli."
updated_at: "2026-06-17T23:22:57.698673942+00:00"
---

Fixed canonical record-kind API leakage. CANONICAL_RECORD_KINDS now includes issue, mission, and evidence; canonical_record_kind("issue"), validate_canonical_record_kind("issue"), and canonical_record_path for issue all work. Generic record parse/render/write dispatch handles issue through the canonical registry while preserving issue-specific payload semantics. SQLite projection source identity now resolves directories through CANONICAL_RECORD_KINDS instead of branching on ISSUE_KIND. Search transcript shows remaining issue special cases are semantic relationship/body handling or convenience wrappers. Validation passed: cargo check -p atelier-records; cargo check -p atelier-sqlite; cargo test -p atelier-records; cargo test -p atelier-sqlite projection_index; cargo test -p atelier-app export; cargo fmt -- --check; target/debug/atelier lint atelier-ynks; git diff --check; cargo build -p atelier-cli.
