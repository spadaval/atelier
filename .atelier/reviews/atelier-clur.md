---
schema: atelier.review
schema_version: 1
id: atelier-clur
title: 'crates/atelier-cli/tests/cli_integration/mission_worktree.rs:1799: replacing test_bundle_apply_rejects_executable_missions_with_open_blockers_and_preserves_state removed the only test of workflow_mission_dependency_bypass (crates/atelier-app/src/record_index.rs:883); no other test in the workspace mentions it. The guard is still reachable through bundles: with a mission at in_progress and no blockers (atelier check passes), a bundle that creates a todo task with blocks: [{id: <mission>}] is refused with ''Bundle apply rejected the staged canonical state: workflow_mission_dependency_bypass ...'' (reproduced with the branch binary). If a later change drops or loosens mission_dependency_errors, bundle graph edits could put open blockers under active missions and no test would fail. Fix: keep a test that sets an existing mission to in_progress by hand-editing it, applies a bundle that adds an open blocker, and asserts workflow_mission_dependency_bypass, exactly one Next: line, and an unchanged canonical tree. That is the assertion set the old test had.'
target: atelier-9co2
identity: Independent reviewer (Claude subagent)
commit: c0a9e01540877c105b3e520de12e29b8c88505d6
created_at: 2026-10-16T18:54:52.121199953Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/tests/cli_integration/mission_worktree.rs:1799: replacing test_bundle_apply_rejects_executable_missions_with_open_blockers_and_preserves_state removed the only test of workflow_mission_dependency_bypass (crates/atelier-app/src/record_index.rs:883); no other test in the workspace mentions it. The guard is still reachable through bundles: with a mission at in_progress and no blockers (atelier check passes), a bundle that creates a todo task with blocks: [{id: <mission>}] is refused with 'Bundle apply rejected the staged canonical state: workflow_mission_dependency_bypass ...' (reproduced with the branch binary). If a later change drops or loosens mission_dependency_errors, bundle graph edits could put open blockers under active missions and no test would fail. Fix: keep a test that sets an existing mission to in_progress by hand-editing it, applies a bundle that adds an open blocker, and asserts workflow_mission_dependency_bypass, exactly one Next: line, and an unchanged canonical tree. That is the assertion set the old test had.
