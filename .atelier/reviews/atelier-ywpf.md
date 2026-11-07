---
schema: atelier.review
schema_version: 2
id: atelier-ywpf
title: finding on atelier-md0v @acaaf9c7
target: atelier-md0v
identity: independent reviewer subagent (claude-opus-5-5), atelier-md0v
commit: acaaf9c70cc5653d1de633f8d449828742b4d5c0
created_at: 2026-11-07T10:01:32.147912976Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: orchestrator (claude-opus-5-5), mission atelier-mmpv
    commit: 124a6d9f6989f431f8e159f7ddda9914c44dc6cc
    recorded_at: 2026-11-07T10:07:18.349938326Z
    follow_up: atelier-ml3r
    reason: atelier-ml3r rewrites the start guidance and drops --base; its Done when now covers the detached-HEAD suggestion and the doc sentence.
---

low: crates/atelier-cli/src/commands/start.rs:92 the hidden start --base flag keeps the skill's 'atelier start <id> --base <base> from any checkout' from failing only on the base. From a detached HEAD, which is what --base used to serve, it prints the deprecation note and refuses with start_detached_head, and that refusal's Next ('git switch <base> && atelier start <id>') fails in a linked worktree while the base is checked out in the main checkout. Reproduced in a scratch repo: 'fatal: 'main' is already used by worktree'. docs/product/work-model.md's 'so guidance that still passes it does not fail' overstates this. Refusing is per the Done when, and agents on this host usually create the branch before start, so this is friction, not a safety gap. Fix when start.rs or the guidance is next touched (atelier-ml3r rewrites the start guidance): make the Next 'git switch -c <id> <base> && atelier start <id>', which works in any checkout, and narrow the doc sentence.
