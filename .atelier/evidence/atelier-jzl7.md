---
created_at: "2026-06-17T23:25:42.360667655+00:00"
id: "atelier-jzl7"
evidence_type: "validation"
captured_at: "2026-06-17T23:25:42.360660302+00:00"
target:
  kind: "issue"
  id: "atelier-adub"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-adub"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Split atelier-records ownership entrypoints by supported durable record area. Added issue, mission, evidence, document, validation, and store modules over the existing canonical implementations; activity, relationships, record_id, and record_kinds remain separate modules. Public APIs and Markdown formats are preserved. File review via rg --files shows the new ownership modules; plan/milestone ownership search returned no first-class plan or milestone modules/types/schemas. Validation passed: cargo test -p atelier-records; target/debug/atelier export --check; rg plan/milestone ownership search; cargo fmt -- --check; target/debug/atelier lint atelier-adub; git diff --check; cargo build -p atelier-cli."
updated_at: "2026-06-17T23:25:46.334319179+00:00"
---

Split atelier-records ownership entrypoints by supported durable record area. Added issue, mission, evidence, document, validation, and store modules over the existing canonical implementations; activity, relationships, record_id, and record_kinds remain separate modules. Public APIs and Markdown formats are preserved. File review via rg --files shows the new ownership modules; plan/milestone ownership search returned no first-class plan or milestone modules/types/schemas. Validation passed: cargo test -p atelier-records; target/debug/atelier export --check; rg plan/milestone ownership search; cargo fmt -- --check; target/debug/atelier lint atelier-adub; git diff --check; cargo build -p atelier-cli.
