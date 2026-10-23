---
schema: atelier.review
schema_version: 1
id: atelier-kq8v
title: finding on atelier-h1nc @b29d7717
target: atelier-h1nc
identity: Independent reviewer 2 (Claude Sonnet subagent)
commit: b29d77173838f0e67a446fd90643a9d7ff6666a8
created_at: 2026-10-23T15:08:08.561621564Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-cli/src/gate_report.rs closing_path/settle_command: for a backlog, todo or blocked item open in a done owner's scope the fix chain ends 'issue transition <id> start, then ... close --reason'. work.terminal only fires on a done owner, and a closed ancestor refuses start: 'closed ancestor <owner> retires open descendant <id>; cannot enter active-category status in_progress' (exit 1, reproduced for todo; blocked and backlog->plan->start the same). Review-status items do close verbatim; plan works, start does not. supersede --reason (from backlog/todo/blocked) works and satisfies work.terminal, but closing_path excludes it. Fix: when the owner is closed, offer 'issue transition <id> supersede --reason "..."' (or say to reopen the owner) instead of a start step; test by executing the printed chain.
