---
schema: atelier.review
schema_version: 1
id: atelier-blgs
title: 'crates/atelier-cli/src/commands/session.rs:347-358: on a detached HEAD, checkout_branch returns None and branch becomes the empty string, so land now exits 0 with ''No work owner on ; it changes no Atelier records since main''. Before this change a detached HEAD always failed. Passing is reasonable, but the message prints an empty branch name. Suggested fix: print ''detached HEAD (<short sha>)'' when there is no branch name, and add a test for that case.'
target: atelier-fxbg
identity: Independent reviewer (Claude subagent)
commit: e4e1b6e78c9291ecbd3923b465f2cfd7f2e4a7a9
created_at: 2026-10-15T22:59:09.197023786Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/session.rs:347-358: on a detached HEAD, checkout_branch returns None and branch becomes the empty string, so land now exits 0 with 'No work owner on ; it changes no Atelier records since main'. Before this change a detached HEAD always failed. Passing is reasonable, but the message prints an empty branch name. Suggested fix: print 'detached HEAD (<short sha>)' when there is no branch name, and add a test for that case.
