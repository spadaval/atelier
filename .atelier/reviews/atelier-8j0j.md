---
schema: atelier.review
schema_version: 2
id: atelier-8j0j
title: finding on atelier-yfqu @28ae3d61
target: atelier-yfqu
identity: codex-reviewer (gpt-6.1-sol)
commit: 28ae3d61ecce47e8befb0e1469e16122d8addf04
created_at: 2026-11-09T00:51:50.792688370Z
event:
  event: finding
  recommendation: fix_now
---

medium: The deployment loose end contains repository work that needs implementation and review before merge.
.atelier/issues/atelier-6rts.md:29-35.
The loose end must add a unit file and repeatable setup commands to docs or README. Those are code/documentation changes that can and should be produced before the serve feature merges, while a loose end finishes on the base with no branch/review/merge and is meant only for work that cannot happen earlier. Its other proof also overclaims: is-enabled and is-active at one moment do not demonstrate restart after failure or survival of a reboot. Move the checked-in unit/setup documentation and premerge tests into the serve item or a reviewed implementation item; leave only installing, activating and publishing on this host in the loose end. State the precise service-start/restart evidence and avoid requiring an unapproved reboot of the host carrying T3.
Recommendation: FIX NOW; otherwise the worker must either bypass the review boundary for docs or leave this item's acceptance unfinished.
