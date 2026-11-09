---
schema: atelier.review
schema_version: 2
id: atelier-b9wq
title: finding on atelier-yfqu @28ae3d61
target: atelier-yfqu
identity: codex-reviewer (gpt-6.1-sol)
commit: 28ae3d61ecce47e8befb0e1469e16122d8addf04
created_at: 2026-11-09T00:51:49.511910284Z
event:
  event: finding
  recommendation: fix_now
---

high: Command-to-next-command windows cannot establish the claimed per-role token costs.
docs/strategies/fleet-dashboard.md:82-88 and .atelier/issues/atelier-7yy9.md:32 and .atelier/issues/atelier-73vx.md:40.
The strategy fixes a forward-window attribution algorithm before validating whether it can account for actual work. A fresh reviewer can spend its whole run reading and checking before its first item-specific command, review finding or approve, and then exit. The ready attribution item does not require status, brief or other read commands to be recorded, and defines no initial/final interval or explicit role boundary. Those review tokens have no stated owner under the proposed windows; in a reused thread, tokens before switching items can instead be charged to the previous item. Command names alone also do not distinguish planning, implementing and orchestrating issue notes. Extend the spike to exercise role/item boundaries with a known transcript, including work before the first item command and after the last. Establish an explicit association or documented unknown bucket before fixing the recording contract; reconcile totals and label estimates. Keep the metrics UI in backlog until that evidence exists.
Recommendation: FIX NOW; the present Shape commits to silently misleading model/role comparisons and cannot prove who-did-what.
