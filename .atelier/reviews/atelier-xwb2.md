---
schema: atelier.review
schema_version: 2
id: atelier-xwb2
title: 'crates/atelier-app/src/lint.rs:205-206: any read_to_string error (the path is a directory, the file is not UTF-8, or permission is denied) is reported as ''Mission strategy <path> does not exist''. The finding is still correct to fail check, but the message misleads, as reproduced with --strategy docs/s (a directory). Optionally distinguish NotFound from other errors in the message.'
target: atelier-9m4k
identity: Independent reviewer (Claude subagent)
commit: 5ea0b021f0cd536f14bcbb311766fd499f2f023d
created_at: 2026-10-15T23:56:14.984490605Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 251dba146ba4ca2096b41a2c84ad1787d2e1a5b5
    recorded_at: 2026-10-15T23:59:22.477946660Z
    resolution_commit: 251dba146ba4ca2096b41a2c84ad1787d2e1a5b5
    reason: Fixed in 251dba146ba4ca2096b41a2c84ad1787d2e1a5b5
    migrated_from: atelier-al66
---

crates/atelier-app/src/lint.rs:205-206: any read_to_string error (the path is a directory, the file is not UTF-8, or permission is denied) is reported as 'Mission strategy <path> does not exist'. The finding is still correct to fail check, but the message misleads, as reproduced with --strategy docs/s (a directory). Optionally distinguish NotFound from other errors in the message.
