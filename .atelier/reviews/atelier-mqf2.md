---
schema: atelier.review
schema_version: 2
id: atelier-mqf2
title: 'crates/atelier-cli/tests/cli_integration/issues.rs:2374-2410, 2416-2465, 2754-2771, 3202-3225: changed expectations checked. These tests used a backlog parent as stand-in epic context. The new rule intentionally reverses the old behavior for a backlog parent, so the fixtures now use a todo parent and assert the parent appears only as context (ready section excludes it, context section contains it) and that the ready child is still listed. They are not weaker, and the new test covers the old backlog case. Nit: test_work_ready_selects_unblocked_child_without_draft_epic_context now asserts the parent is shown as context, so the name no longer matches; there is also no unit test for parked_parent in work_classification.rs.'
target: atelier-28mz
identity: Independent reviewer (Claude subagent)
commit: 82f02bd8c6622240e624d952257e5f89a32c42e9
created_at: 2026-10-16T01:27:48.830538208Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: Claude Opus 5.5 (orchestrator)
    commit: 36db25576546f10c2b29dc596736255405a7f1d5
    recorded_at: 2026-10-16T01:29:44.792939678Z
    reason: 'Accepted: the misnamed test is renamed in 36db25576546f10c2b29dc596736255405a7f1d5; the integration test covers parked_parent''s cases (backlog parent, decision parent, in-flight child)'
    migrated_from: atelier-3po8
---

crates/atelier-cli/tests/cli_integration/issues.rs:2374-2410, 2416-2465, 2754-2771, 3202-3225: changed expectations checked. These tests used a backlog parent as stand-in epic context. The new rule intentionally reverses the old behavior for a backlog parent, so the fixtures now use a todo parent and assert the parent appears only as context (ready section excludes it, context section contains it) and that the ready child is still listed. They are not weaker, and the new test covers the old backlog case. Nit: test_work_ready_selects_unblocked_child_without_draft_epic_context now asserts the parent is shown as context, so the name no longer matches; there is also no unit test for parked_parent in work_classification.rs.
