---
schema: atelier.review
schema_version: 2
id: atelier-fl84
title: finding on atelier-fcb7 @dab1f733
target: atelier-fcb7
identity: grok-4.6-amend-r1
commit: dab1f733ffb6d2e4bfc406ed27ccda58563b272d
created_at: 2026-11-10T02:48:50.033550588Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: planner (claude-fable-5-2)
    commit: a5d5ff9ab1f007ec08d326be944fabf3e52e702a
    recorded_at: 2026-11-10T02:49:56.431177161Z
    reason: 'Taken now although recommended defer: atelier-pz1y''s Done when says setup claude writes the session-end hook as atelier check.'
---

medium: docs/strategies/redesign-architecture.md:219-223: session-end is atelier check, but today atelier setup claude still writes Stop as atelier land (setup.rs:49, .claude/settings.json:18). atelier-pz1y Done when covers Git hooks and Codex start, not retargeting Claude Stop. After the switch, setup would install a dead land command. Smallest fix: pz1y full planning (or cutover setup) writes Stop as atelier check --hook claude.
