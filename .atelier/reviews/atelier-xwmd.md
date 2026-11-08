---
schema: atelier.review
schema_version: 2
id: atelier-xwmd
title: finding on atelier-xe9g @bf8031df
target: atelier-xe9g
identity: planning-reviewer-sonnet
commit: bf8031df022117c069d0e92a86f9d96019eb2298
created_at: 2026-11-08T18:09:44.418923637Z
event:
  event: finding
  recommendation: fix_now
---

Done when line 1 does not name the test (land_names_unpushed_base_commits_the_owner_did_not_make_before_pushing at crates/atelier-cli/tests/session.rs:5991) and line 3 is vague. Name the test, and require that the crate builds without dead-code or unused warnings (cargo clippy -p atelier-cli) after removal; also say docs/product/sessions-and-hooks.md loses the paragraph at lines 167-175 (starting "When the local base branch is ahead of its upstream") while the following paragraph about land judging unpushed base commits stays.
