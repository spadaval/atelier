---
schema: atelier.review
schema_version: 2
id: atelier-tys3
title: finding on atelier-5fsj @5ff588fb
target: atelier-5fsj
identity: gpt-6.1-sol-5fsj-r5
commit: 5ff588fb22b665eab363f80c3a085b948b8238ac
created_at: 2026-11-10T16:26:34.086036852Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/scorecard/commands.py:273 R13. Sorting Assign and Call nodes by source position applies a new binding before evaluating its right-hand side. The synthetic supported command python3 -c "p = 'src/a.rs'; p = open(p, 'w')" yields no shell_writes, because the second Assign clears p before the open Call reads it. In an implementation thread followed by cargo test and a file_change on src/b.rs, calls_before_edit is 2 instead of 0; without the later file_change the implementer is dropped. This is a regression from the AST replacement in round 4 and violates the listed open(p, mode) form with p bound earlier. Inspect RHS calls using the old bindings before updating assignment targets, and add an arithmetic regression.
