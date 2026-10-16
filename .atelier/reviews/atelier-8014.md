---
schema: atelier.review
schema_version: 1
id: atelier-8014
title: 'Resolved. test_bundle_apply_rejects_open_blockers_on_an_executable_mission_and_preserves_state (crates/atelier-cli/tests/cli_integration/mission_worktree.rs:1866) puts a mission in in_progress, confirms atelier check passes, then bundle-applies a new open task that blocks it. It asserts failure, the workflow_mission_dependency_bypass code in stderr, exactly one Next: line, and a byte-identical canonical tree. Mutation check: with failure_reason() suppressed in mission_dependency_errors (crates/atelier-app/src/record_index.rs:881), the test fails at its rejection assertion; unmutated it passes. A manual CLI run produced one Error line with the code and one Next: line.'
target: atelier-9co2
identity: Independent reviewer (Claude subagent)
commit: 912f0f6752e28f5f05f6e2455377ee618a0403da
created_at: 2026-10-16T18:58:41.846695117Z
event:
  event: disposition
  finding: atelier-clur
  decision: fix_now
  supersedes: null
  resolution_commit: 5110803b582b916097196d6dd066a3de716be56e
  follow_up: null
---

Resolved. test_bundle_apply_rejects_open_blockers_on_an_executable_mission_and_preserves_state (crates/atelier-cli/tests/cli_integration/mission_worktree.rs:1866) puts a mission in in_progress, confirms atelier check passes, then bundle-applies a new open task that blocks it. It asserts failure, the workflow_mission_dependency_bypass code in stderr, exactly one Next: line, and a byte-identical canonical tree. Mutation check: with failure_reason() suppressed in mission_dependency_errors (crates/atelier-app/src/record_index.rs:881), the test fails at its rejection assertion; unmutated it passes. A manual CLI run produced one Error line with the code and one Next: line.
