---
schema: atelier.review
schema_version: 2
id: atelier-blgs
title: 'crates/atelier-cli/src/commands/session.rs:347-358: on a detached HEAD, checkout_branch returns None and branch becomes the empty string, so land now exits 0 with ''No work owner on ; it changes no Atelier records since main''. Before this change a detached HEAD always failed. Passing is reasonable, but the message prints an empty branch name. Suggested fix: print ''detached HEAD (<short sha>)'' when there is no branch name, and add a test for that case.'
target: atelier-fxbg
identity: Independent reviewer (Claude subagent)
commit: e4e1b6e78c9291ecbd3923b465f2cfd7f2e4a7a9
created_at: 2026-10-15T22:59:09.197023786Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 02fb56812ce87e436330ef557d2192798fc72fc8
    recorded_at: 2026-10-15T23:00:44.944696570Z
    resolution_commit: 02fb56812ce87e436330ef557d2192798fc72fc8
    reason: Fixed in 02fb56812ce87e436330ef557d2192798fc72fc8 with test coverage
    migrated_from: atelier-u4rc
---

crates/atelier-cli/src/commands/session.rs:347-358: on a detached HEAD, checkout_branch returns None and branch becomes the empty string, so land now exits 0 with 'No work owner on ; it changes no Atelier records since main'. Before this change a detached HEAD always failed. Passing is reasonable, but the message prints an empty branch name. Suggested fix: print 'detached HEAD (<short sha>)' when there is no branch name, and add a test for that case.
