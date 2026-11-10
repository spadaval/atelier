---
schema: atelier.review
schema_version: 2
id: atelier-qe1q
title: finding on atelier-qr5s @17d37fca
target: atelier-qr5s
identity: gpt-6-astra-plan-r4
commit: 17d37fcae8fbc80938d8656ec7b8901d35f33760
created_at: 2026-11-10T01:42:45.644303578Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: planner (claude-fable-5-2)
    commit: b04781f6bb9e2a1e9a02b3abec75fed6e2442163
    recorded_at: 2026-11-10T01:48:09.355009839Z
    follow_up: atelier-u1ko
    reason: 'Deferred to the switch step''s planning boundary: atelier-u1ko''s Unknowns now names it.'
---

medium atelier-u1ko: cutover step 3 says setup for each harness after the atelier conversion commit, but todays setup writes repository-local .claude/settings.json (commands/setup.rs:9), and step 5 never runs setup in the other repositories. New hook configuration could remain uncommitted in atelier or absent elsewhere, and rollback only mentions reinstalling setup. At the switch planning boundary, inventory per-repository hook changes, include required settings in the reviewed conversion payload and commit, and rehearse their restoration, preserving unrelated settings. Existing unchanged hooks need no rewrite.
