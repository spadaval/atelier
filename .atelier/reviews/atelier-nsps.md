---
schema: atelier.review
schema_version: 1
id: atelier-nsps
title: 'crates/atelier-cli/tests/cli_integration/quiet_tokens.rs:191-194 — work human and work active are run while empty in the fixture, so assert_ids passes vacuously there and this test alone would not notice either view printing nothing. No action: issues.rs:4885/5455/5665 pin non-empty quiet output for both views, so the token is covered elsewhere.'
target: atelier-pr4k
identity: Independent reviewer (Claude subagent)
commit: db348b9eb46038270d76680937219362478b5526
created_at: 2026-10-17T08:28:19.815161913Z
event:
  event: finding
  recommendation: no_action
---

crates/atelier-cli/tests/cli_integration/quiet_tokens.rs:191-194 — work human and work active are run while empty in the fixture, so assert_ids passes vacuously there and this test alone would not notice either view printing nothing. No action: issues.rs:4885/5455/5665 pin non-empty quiet output for both views, so the token is covered elsewhere.
