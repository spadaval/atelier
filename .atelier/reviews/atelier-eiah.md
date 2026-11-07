---
schema: atelier.review
schema_version: 2
id: atelier-eiah
title: 'crates/atelier-cli/src/commands/bundle.rs:664, :694, :1041: the recovery and cleanup messages still say ''Do not retry this create-only bundle'', which is now inaccurate. A bundle with update entries can be re-applied, and its creates are what duplicate. Low impact; the tests in tests/cli_integration/mission_worktree.rs:2072/2210 assert this wording. Fix: reword to ''Do not retry this bundle'' when convenient.'
target: atelier-4n7v
identity: Independent reviewer (Claude subagent)
commit: a9234343ba772ab7e86fa682752f752f73f4af7e
created_at: 2026-10-16T00:40:13.495780926Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 6aa8ac7c67c04110e133c12c55357ef908a9141d
    recorded_at: 2026-10-16T00:43:34.715879954Z
    resolution_commit: a89e76fd82b919e01d6ca894c52ac5c30a252c50
    reason: Fixed in a89e76fd82b919e01d6ca894c52ac5c30a252c50 with tests
    migrated_from: atelier-f1ul
---

crates/atelier-cli/src/commands/bundle.rs:664, :694, :1041: the recovery and cleanup messages still say 'Do not retry this create-only bundle', which is now inaccurate. A bundle with update entries can be re-applied, and its creates are what duplicate. Low impact; the tests in tests/cli_integration/mission_worktree.rs:2072/2210 assert this wording. Fix: reword to 'Do not retry this bundle' when convenient.
