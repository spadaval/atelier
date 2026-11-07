---
schema: atelier.review
schema_version: 2
id: atelier-ajgl
title: 'crates/atelier-cli/tests/session.rs:2895: the prime assertion output.contains("Needs a human: ") is vacuous. The pre-existing prime line at crates/atelier-cli/src/commands/session.rs:197 ("Needs a human: N ready item(s) labelled needs-human ...") also starts with that prefix, and in this test the check is a ready needs-human item, so the assertion passes with the new prime block (session.rs:110-117) deleted. Reproduced in a scratch repo: before detaching, prime prints both lines; after, neither. Fix: assert on the new text, e.g. contains(&format!("{check} labelled needs-human is a child of {parent}")) in the first prime output, as the start assertion does.'
target: atelier-1jll
identity: Independent reviewer (Claude subagent)
commit: 863a7d908942ea4589f3fd104f98ab9f0eabca9c
created_at: 2026-10-16T19:45:03.029403901Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: 5250d3697e223ace8b7fea7ee14e96c693f94bf2
    recorded_at: 2026-10-16T19:48:04.492983714Z
    resolution_commit: 45ccfad54324646a1525297033d0a55745c87734
    reason: 'session.rs:2895 now asserts contains("Needs a human: {check} labelled needs-human is a child of {parent}") on the prime output. The only prime source of that text is the new block at crates/atelier-cli/src/commands/session.rs:110-117; the older line at :197 prints ''Needs a human: N ready item(s) ...'' and cannot match, and the start output comes from a separate invocation. Removing the prime block therefore fails the assertion, matching the author''s mutation check.'
    migrated_from: atelier-cnah
---

crates/atelier-cli/tests/session.rs:2895: the prime assertion output.contains("Needs a human: ") is vacuous. The pre-existing prime line at crates/atelier-cli/src/commands/session.rs:197 ("Needs a human: N ready item(s) labelled needs-human ...") also starts with that prefix, and in this test the check is a ready needs-human item, so the assertion passes with the new prime block (session.rs:110-117) deleted. Reproduced in a scratch repo: before detaching, prime prints both lines; after, neither. Fix: assert on the new text, e.g. contains(&format!("{check} labelled needs-human is a child of {parent}")) in the first prime output, as the start assertion does.
