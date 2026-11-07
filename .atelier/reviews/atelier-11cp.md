---
schema: atelier.review
schema_version: 2
id: atelier-11cp
title: 'crates/atelier-cli/tests/cli_integration/mission_worktree.rs:1929: test_bundle_apply_accepts_graph_edit_to_ready_mission now adds a todo blocker, but client_ref issue.completed-blocker and title ''Completed graph addition'' still describe a done one. The test''s intent survives and is arguably stronger: the mission''s ready transition lands in status todo, which mission_dependency_errors does not treat as executable, so adding an open blocker is correctly accepted (reproduced; start then reports the blocker path). Only the names are stale; rename them if the file is touched again.'
target: atelier-9co2
identity: Independent reviewer (Claude subagent)
commit: c0a9e01540877c105b3e520de12e29b8c88505d6
created_at: 2026-10-16T18:54:54.733222480Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: Claude Opus 5.5 (orchestrator)
    commit: 5110803b582b916097196d6dd066a3de716be56e
    recorded_at: 2026-10-16T18:56:14.473151551Z
    reason: 'Accepted: the test keeps its intent; its blocker names were updated in 5110803b582b916097196d6dd066a3de716be56e.'
    migrated_from: atelier-r1em
---

crates/atelier-cli/tests/cli_integration/mission_worktree.rs:1929: test_bundle_apply_accepts_graph_edit_to_ready_mission now adds a todo blocker, but client_ref issue.completed-blocker and title 'Completed graph addition' still describe a done one. The test's intent survives and is arguably stronger: the mission's ready transition lands in status todo, which mission_dependency_errors does not treat as executable, so adding an open blocker is correctly accepted (reproduced; start then reports the blocker path). Only the names are stale; rename them if the file is touched again.
