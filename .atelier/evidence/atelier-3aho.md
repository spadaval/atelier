---
created_at: "2026-06-11T23:34:20.764137435+00:00"
id: "atelier-3aho"
data: "{\"captured_at\":\"2026-06-11T23:34:20.764092136+00:00\",\"kind\":\"test\",\"path\":null,\"producer\":\"codex\",\"result\":\"pass\",\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-ru15"
    type: "validates"
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "atelier-ru15 validation: canonical writes now target .atelier only; .atelier-state remains explicit read/migrate compatibility. Passed: cargo fmt -- --check; cargo test --no-run; cargo test storage_layout::tests::canonical_dir_does_not_fall_back_to_legacy_state -- --nocapture; cargo test commands::migrate::tests::markdown_first_moves_legacy_records_into_atelier -- --nocapture; atelier export --check; atelier lint; atelier doctor; atelier workflow validate issue atelier-ru15."
updated_at: "2026-06-11T23:34:25.769268098+00:00"
---

atelier-ru15 validation: canonical writes now target .atelier only; .atelier-state remains explicit read/migrate compatibility. Passed: cargo fmt -- --check; cargo test --no-run; cargo test storage_layout::tests::canonical_dir_does_not_fall_back_to_legacy_state -- --nocapture; cargo test commands::migrate::tests::markdown_first_moves_legacy_records_into_atelier -- --nocapture; atelier export --check; atelier lint; atelier doctor; atelier workflow validate issue atelier-ru15.
