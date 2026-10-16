---
schema: atelier.review
schema_version: 1
id: atelier-cnah
title: 'session.rs:2895 now asserts contains("Needs a human: {check} labelled needs-human is a child of {parent}") on the prime output. The only prime source of that text is the new block at crates/atelier-cli/src/commands/session.rs:110-117; the older line at :197 prints ''Needs a human: N ready item(s) ...'' and cannot match, and the start output comes from a separate invocation. Removing the prime block therefore fails the assertion, matching the author''s mutation check.'
target: atelier-1jll
identity: Independent reviewer (Claude subagent)
commit: 5250d3697e223ace8b7fea7ee14e96c693f94bf2
created_at: 2026-10-16T19:48:04.492983714Z
event:
  event: disposition
  finding: atelier-ajgl
  decision: fix_now
  supersedes: null
  resolution_commit: 45ccfad54324646a1525297033d0a55745c87734
  follow_up: null
---

session.rs:2895 now asserts contains("Needs a human: {check} labelled needs-human is a child of {parent}") on the prime output. The only prime source of that text is the new block at crates/atelier-cli/src/commands/session.rs:110-117; the older line at :197 prints 'Needs a human: N ready item(s) ...' and cannot match, and the start output comes from a separate invocation. Removing the prime block therefore fails the assertion, matching the author's mutation check.
