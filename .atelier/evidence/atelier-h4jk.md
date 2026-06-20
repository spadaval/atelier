---
created_at: "2026-06-20T13:24:34.968665339+00:00"
id: "atelier-h4jk"
evidence_type: "validation"
captured_at: "2026-06-20T13:24:34.968651106+00:00"
target:
  kind: "issue"
  id: "atelier-wee2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-wee2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Final validation after provider PR fixture cleanup passed: cargo nextest run -p atelier-cli --lib (185/185), cargo nextest run -p atelier-app --lib (79/79), cargo fmt -- --check, git diff --check, cargo check -p atelier-cli, cargo nextest run -p atelier-cli --test cli_integration -E 'test(setup_guidance::test_man_lists_roles)', target/debug/atelier workflow check, and target/debug/atelier lint atelier-wee2."
updated_at: "2026-06-20T13:24:39.615365698+00:00"
---

Final validation after provider PR fixture cleanup passed: cargo nextest run -p atelier-cli --lib (185/185), cargo nextest run -p atelier-app --lib (79/79), cargo fmt -- --check, git diff --check, cargo check -p atelier-cli, cargo nextest run -p atelier-cli --test cli_integration -E 'test(setup_guidance::test_man_lists_roles)', target/debug/atelier workflow check, and target/debug/atelier lint atelier-wee2.
