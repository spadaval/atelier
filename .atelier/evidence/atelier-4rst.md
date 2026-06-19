---
created_at: "2026-06-19T04:44:56.965684942+00:00"
id: "atelier-4rst"
evidence_type: "validation"
captured_at: "2026-06-19T04:44:56.965682706+00:00"
target:
  kind: "issue"
  id: "atelier-rb5b"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-rb5b"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Forgejo provider behavior is preserved behind review commands. Existing provider app tests still pass with structured review storage; review approve/request-changes dispatch to provider review events; status/show/comment/comments/merge continue through the Forgejo client. Proof: cargo test -p atelier-app pr::tests --lib passed; cargo test -p atelier-cli pr --lib passed; target/debug/atelier review approve --help passed; atelier lint atelier-rb5b passed; git diff --check passed."
updated_at: "2026-06-19T04:44:59.915825691+00:00"
---

Forgejo provider behavior is preserved behind review commands. Existing provider app tests still pass with structured review storage; review approve/request-changes dispatch to provider review events; status/show/comment/comments/merge continue through the Forgejo client. Proof: cargo test -p atelier-app pr::tests --lib passed; cargo test -p atelier-cli pr --lib passed; target/debug/atelier review approve --help passed; atelier lint atelier-rb5b passed; git diff --check passed.
