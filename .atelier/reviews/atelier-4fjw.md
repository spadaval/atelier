---
schema: atelier.review
schema_version: 1
id: atelier-4fjw
title: 'crates/atelier-cli/src/commands/status.rs:380-390 — the new ''Unassigned work:'' summary line is suppressed in an owner''s session (documented in docs/product/sessions-and-hooks.md), but no test proves it: the owner test at crates/atelier-cli/tests/session.rs:124 checks only ''!contains("Unassigned Work")'' (capital W, the heading), so dropping the ''if owned { 0 }'' gate would print the count in every owner''s SessionStart view and all 755 tests would still pass. Add one assertion that owner status has no ''Unassigned work:'' line.'
target: atelier-u3vj
identity: Independent reviewer (Claude subagent)
commit: b8288f8555508d70806ebecd8ed5197ffe4f426c
created_at: 2026-10-17T09:45:14.331824161Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/status.rs:380-390 — the new 'Unassigned work:' summary line is suppressed in an owner's session (documented in docs/product/sessions-and-hooks.md), but no test proves it: the owner test at crates/atelier-cli/tests/session.rs:124 checks only '!contains("Unassigned Work")' (capital W, the heading), so dropping the 'if owned { 0 }' gate would print the count in every owner's SessionStart view and all 755 tests would still pass. Add one assertion that owner status has no 'Unassigned work:' line.
