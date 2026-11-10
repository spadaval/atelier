---
schema: atelier.review
schema_version: 2
id: atelier-hgz7
title: finding on atelier-qr5s @8f1d6bfc
target: atelier-qr5s
identity: grok-4.6-plan-r3
commit: 8f1d6bfce97e37869d729e90aec74f77020e457c
created_at: 2026-11-10T01:37:14.451778398Z
event:
  event: finding
  recommendation: fix_now
---

high docs/strategies/redesign-architecture.md:316-318 and :342: the skill rewrite merges to master and is supposed to stay dark until cutover moves the live checkout. That checkout is ~/projects/atelier-skill, linked from ~/.agents/skills/agent-factory, and the installed 0.4.x doctor warns when it is behind master and prints git -C <path> checkout --detach <base> (docs/product/sessions-and-hooks.md:209-214). After atelier-orb2 merges, SessionStart nags every agent to activate the new skill against the old binary. Smallest fix: pin the skill checkout at the 0.4.1 commit, and have the switch AGENTS.md say not to advance it until cutover step 3.
