---
schema: atelier.review
schema_version: 2
id: atelier-1hux
title: 'crates/atelier-cli/tests/session.rs:861 execute_bits_hidden_by_core_filemode_follow_git_status_in_land: the completion half only asserts that the output lacks land_completion_dirty and ignores the exit status. I reproduced the fixture: land <id> fails with land_invariants_unmet (review.current), so today it does prove the dirty gate passed, but the assertion would still pass if land failed earlier for an unrelated reason. Suggested fix: assert that the output contains land_invariants_unmet / review.current, or approve and assert success.'
target: atelier-1ioo
identity: independent-reviewer-2026-10-15
commit: d1cb7d908869c6ddadafcc69416ddf7aea42df75
created_at: 2026-10-15T19:13:37.221649176Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: rebuild-orchestrator
    commit: ba7512b7e96a2d027b7ff6fedde9889b73031ca1
    recorded_at: 2026-10-15T19:16:27.946921200Z
    resolution_commit: ba7512b7e96a2d027b7ff6fedde9889b73031ca1
    reason: Fixed in ba7512b7; needs independent re-review.
    migrated_from: atelier-fnfq
---

crates/atelier-cli/tests/session.rs:861 execute_bits_hidden_by_core_filemode_follow_git_status_in_land: the completion half only asserts that the output lacks land_completion_dirty and ignores the exit status. I reproduced the fixture: land <id> fails with land_invariants_unmet (review.current), so today it does prove the dirty gate passed, but the assertion would still pass if land failed earlier for an unrelated reason. Suggested fix: assert that the output contains land_invariants_unmet / review.current, or approve and assert success.
