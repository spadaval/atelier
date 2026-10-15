---
schema: atelier.review
schema_version: 1
id: atelier-xwb2
title: 'crates/atelier-app/src/lint.rs:205-206: any read_to_string error (the path is a directory, the file is not UTF-8, or permission is denied) is reported as ''Mission strategy <path> does not exist''. The finding is still correct to fail check, but the message misleads, as reproduced with --strategy docs/s (a directory). Optionally distinguish NotFound from other errors in the message.'
target: atelier-9m4k
identity: Independent reviewer (Claude subagent)
commit: 5ea0b021f0cd536f14bcbb311766fd499f2f023d
created_at: 2026-10-15T23:56:14.984490605Z
event:
  event: finding
  recommendation: no_action
---

crates/atelier-app/src/lint.rs:205-206: any read_to_string error (the path is a directory, the file is not UTF-8, or permission is denied) is reported as 'Mission strategy <path> does not exist'. The finding is still correct to fail check, but the message misleads, as reproduced with --strategy docs/s (a directory). Optionally distinguish NotFound from other errors in the message.
