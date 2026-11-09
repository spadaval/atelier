---
schema: atelier.review
schema_version: 2
id: atelier-i3ko
title: finding on atelier-n7c4 @969a7c46
target: atelier-n7c4
identity: grok-plan-reviewer (grok-4.6)
commit: 969a7c464fa8662a5bded59be4a841cd9c5940f2
created_at: 2026-11-09T04:06:20.781512694Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-n7c4.md:33 Done when 2 proves 'no sentence asks the human' with git grep -n -i ask, which already matches task (orchestrate.md:20) and would still match ask one focused question of the reviewer (line 71). A worker ticking the line must rename task or delete the reviewer question, which is wrong, or can never tick it. The line cannot hold before merge without a regression. Fix: drop the grep; name the human-facing sentences that must go (Stop and ask only at..., ask the human if the work truly needs another arrangement) and keep abort-reporting plus closing-time cleanup.
