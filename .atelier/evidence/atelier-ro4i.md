---
created_at: "2026-06-20T01:07:14.695278277+00:00"
id: "atelier-ro4i"
evidence_type: "validation"
captured_at: "2026-06-20T01:07:14.695252676+00:00"
target:
  kind: "issue"
  id: "atelier-v4ah"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-v4ah"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "review.complete now reads configured review artifacts: cargo test -p atelier-cli workflow --lib; cargo test -p atelier-cli issue_transition --lib; cargo fmt -- --check; cargo build -p atelier-cli; target/debug/atelier workflow check; target/debug/atelier lint atelier-v4ah; git diff --check"
updated_at: "2026-06-20T01:07:19.421120531+00:00"
---

review.complete now reads configured review artifacts: cargo test -p atelier-cli workflow --lib; cargo test -p atelier-cli issue_transition --lib; cargo fmt -- --check; cargo build -p atelier-cli; target/debug/atelier workflow check; target/debug/atelier lint atelier-v4ah; git diff --check
