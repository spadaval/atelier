---
schema: atelier.review
schema_version: 1
id: atelier-ajgl
title: 'crates/atelier-cli/tests/session.rs:2895: the prime assertion output.contains("Needs a human: ") is vacuous. The pre-existing prime line at crates/atelier-cli/src/commands/session.rs:197 ("Needs a human: N ready item(s) labelled needs-human ...") also starts with that prefix, and in this test the check is a ready needs-human item, so the assertion passes with the new prime block (session.rs:110-117) deleted. Reproduced in a scratch repo: before detaching, prime prints both lines; after, neither. Fix: assert on the new text, e.g. contains(&format!("{check} labelled needs-human is a child of {parent}")) in the first prime output, as the start assertion does.'
target: atelier-1jll
identity: Independent reviewer (Claude subagent)
commit: 863a7d908942ea4589f3fd104f98ab9f0eabca9c
created_at: 2026-10-16T19:45:03.029403901Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/tests/session.rs:2895: the prime assertion output.contains("Needs a human: ") is vacuous. The pre-existing prime line at crates/atelier-cli/src/commands/session.rs:197 ("Needs a human: N ready item(s) labelled needs-human ...") also starts with that prefix, and in this test the check is a ready needs-human item, so the assertion passes with the new prime block (session.rs:110-117) deleted. Reproduced in a scratch repo: before detaching, prime prints both lines; after, neither. Fix: assert on the new text, e.g. contains(&format!("{check} labelled needs-human is a child of {parent}")) in the first prime output, as the start assertion does.
