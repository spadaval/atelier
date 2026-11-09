---
schema: atelier.review
schema_version: 2
id: atelier-79wo
title: finding on atelier-yfqu @28ae3d61
target: atelier-yfqu
identity: codex-reviewer (gpt-6.1-sol)
commit: 28ae3d61ecce47e8befb0e1469e16122d8addf04
created_at: 2026-11-09T00:51:48.740439263Z
event:
  event: finding
  recommendation: fix_now
---

medium: The attribution item's merge requirements cannot both hold.
.atelier/issues/atelier-7yy9.md:32,35 and docs/strategies/fleet-dashboard.md:49-51.
The first Done when line explicitly requires merge to write a SQLite row, while the fourth forbids merge from ever opening that database. A worker cannot satisfy both. Also, running commands with an unreadable database does not prove they never open it: the third line requires database errors to be swallowed, so an attempted open can leave the same result. Define the boundary as the human decided it: guard decisions consult Git only, with any permitted attribution write outside the guard, or explicitly exclude these commands from recording. Reconcile the every-command outcome with that exception. Use an observable open/read spy or equivalent for the guard-isolation proof, alongside the unavailable-database behavior test.
Recommendation: FIX NOW; this is an internal acceptance contradiction, not an unsettled implementation choice.
