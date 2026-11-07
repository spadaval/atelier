---
schema: atelier.review
schema_version: 2
id: atelier-jsho
title: 'low: crates/atelier-cli/src/commands/start.rs:347 and crates/atelier-cli/src/commands/lease.rs:171-175: the lease is not a lock, as documented, but two edges are unguarded. (1) start runs `lease::claim(...)?` after the record transaction has committed and the branch switched, so a lease write failure (the primary checkout''s .atelier/runtime not writable) reports start as failed although it succeeded; make the claim best-effort with a warning line. (2) status''s refresh reads then rewrites the lease without a compare, so a status in the old holder racing a --take-over in another worktree can write the old holder back, undoing the take-over silently. Both are rare; fold into the follow-up for the open items (two sessions in one worktree, land not renewing the heartbeat).'
target: atelier-yim7
identity: Independent reviewer (Claude subagent)
commit: 06c2b327ae27786c9677118f6e4cecf8118fc638
created_at: 2026-10-22T21:17:50.929562027Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Claude Opus 5.5 (orchestrator)
    commit: db293ca49f0ecc5105fe7b8de8c2da091a2996a9
    recorded_at: 2026-10-22T21:18:32.730248835Z
    follow_up: atelier-x5j3
    reason: The lease only warns; tracked in atelier-x5j3.
    migrated_from: atelier-8h46
---

low: crates/atelier-cli/src/commands/start.rs:347 and crates/atelier-cli/src/commands/lease.rs:171-175: the lease is not a lock, as documented, but two edges are unguarded. (1) start runs `lease::claim(...)?` after the record transaction has committed and the branch switched, so a lease write failure (the primary checkout's .atelier/runtime not writable) reports start as failed although it succeeded; make the claim best-effort with a warning line. (2) status's refresh reads then rewrites the lease without a compare, so a status in the old holder racing a --take-over in another worktree can write the old holder back, undoing the take-over silently. Both are rare; fold into the follow-up for the open items (two sessions in one worktree, land not renewing the heartbeat).
