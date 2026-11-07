---
schema: atelier.review
schema_version: 2
id: atelier-gx7t
title: 'crates/atelier-cli/tests (no test) — Done when requires rendering tests to enforce that status''s Current Work row agrees with its Owner row. No test compares them: tests pin Current Work as ''\n  {id} in_progress '' (setup_guidance.rs:1792, mission_worktree.rs:1214) and the active Owner row only as starts_with(id) (session.rs:172); the other Owner assertions are done owners, which never appear in Current Work. The rows agree today only because both use ItemLine; if Owner::new (status.rs:208-229) later adds a cue or the type, or the Current Work panel adds a lead, nothing fails. Fix: one status test with an in_progress owner asserting the Current Work row (trimmed) equals field(Owner).'
target: atelier-i2qq
identity: Independent reviewer (Claude subagent)
commit: c4b06e91043b5f0b07fce4a2cb9a0ba9e6cd2945
created_at: 2026-10-17T06:18:09.416725808Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: a86670843ad47ce6e0931848c65b94532ecdeed2
    recorded_at: 2026-10-17T06:22:37.545109482Z
    resolution_commit: a86670843ad47ce6e0931848c65b94532ecdeed2
    reason: session.rs status_current_work_row_reads_like_its_owner_row asserts the trimmed Current Work row equals the Owner field for an in_progress owner.
    migrated_from: atelier-dv2f
---

crates/atelier-cli/tests (no test) — Done when requires rendering tests to enforce that status's Current Work row agrees with its Owner row. No test compares them: tests pin Current Work as '\n  {id} in_progress ' (setup_guidance.rs:1792, mission_worktree.rs:1214) and the active Owner row only as starts_with(id) (session.rs:172); the other Owner assertions are done owners, which never appear in Current Work. The rows agree today only because both use ItemLine; if Owner::new (status.rs:208-229) later adds a cue or the type, or the Current Work panel adds a lead, nothing fails. Fix: one status test with an in_progress owner asserting the Current Work row (trimmed) equals field(Owner).
