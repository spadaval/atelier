---
created_at: "2026-06-30T19:09:35.279398985+00:00"
id: "atelier-uiwn"
evidence_type: "test"
captured_at: "2026-06-30T19:09:35.279380049+00:00"
target:
  kind: "issue"
  id: "atelier-otxv"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-otxv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo fmt -- --check; cargo test -p atelier-cli workflow --lib; cargo test -p atelier-cli git_sync --lib -- --nocapture; cargo test -p atelier-cli review_open_action_persists_room_review_field --lib -- --nocapture; target/debug/atelier check atelier-otxv; git diff --check all passed"
updated_at: "2026-06-30T19:09:41.052923350+00:00"
---

cargo fmt -- --check; cargo test -p atelier-cli workflow --lib; cargo test -p atelier-cli git_sync --lib -- --nocapture; cargo test -p atelier-cli review_open_action_persists_room_review_field --lib -- --nocapture; target/debug/atelier check atelier-otxv; git diff --check all passed
