---
created_at: "2026-06-19T04:44:40.966778421+00:00"
id: "atelier-k1xc"
evidence_type: "validation"
captured_at: "2026-06-19T04:44:40.966776848+00:00"
target:
  kind: "issue"
  id: "atelier-69g3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-69g3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Review command dispatch and help implemented. Root help lists review instead of pr; atelier review help exposes open/link/status/show/merge/comments/comment/approve/request-changes/resolve; telemetry labels use review; role guides use review commands. Proof: target/debug/atelier --help shows review; target/debug/atelier review --help passed; cargo test -p atelier-cli --test cli_integration setup_guidance::test_top_level_help_only_shows_core_commands passed; setup_guidance::test_spec_representative_commands_match_signpost_surfaces passed; atelier lint atelier-69g3 passed; git diff --check passed."
updated_at: "2026-06-19T04:44:43.862161624+00:00"
---

Review command dispatch and help implemented. Root help lists review instead of pr; atelier review help exposes open/link/status/show/merge/comments/comment/approve/request-changes/resolve; telemetry labels use review; role guides use review commands. Proof: target/debug/atelier --help shows review; target/debug/atelier review --help passed; cargo test -p atelier-cli --test cli_integration setup_guidance::test_top_level_help_only_shows_core_commands passed; setup_guidance::test_spec_representative_commands_match_signpost_surfaces passed; atelier lint atelier-69g3 passed; git diff --check passed.
