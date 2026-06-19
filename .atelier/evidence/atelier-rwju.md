---
created_at: "2026-06-19T04:37:59.934839937+00:00"
id: "atelier-rwju"
evidence_type: "validation"
captured_at: "2026-06-19T04:37:59.934838412+00:00"
target:
  kind: "issue"
  id: "atelier-5d7i"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5d7i"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Structured review issue field implemented. Issue records render top-level review objects, workflow validation accepts review.kind room and provider pull_request shapes, child issues inherit the nearest parent epic review field, and child-local review fields are rejected. Proof: cargo test -p atelier-workflow --lib passed; cargo test -p atelier-app workflow_policy --lib passed; cargo test -p atelier-app rebuild --lib passed; atelier lint atelier-5d7i passed; git diff --check passed."
updated_at: "2026-06-19T04:38:02.651811474+00:00"
---

Structured review issue field implemented. Issue records render top-level review objects, workflow validation accepts review.kind room and provider pull_request shapes, child issues inherit the nearest parent epic review field, and child-local review fields are rejected. Proof: cargo test -p atelier-workflow --lib passed; cargo test -p atelier-app workflow_policy --lib passed; cargo test -p atelier-app rebuild --lib passed; atelier lint atelier-5d7i passed; git diff --check passed.
