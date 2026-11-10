---
schema: atelier.review
schema_version: 2
id: atelier-q8or
title: finding on atelier-qr5s @2c4d3711
target: atelier-qr5s
identity: gpt-6-astra-plan-r3
commit: 2c4d3711a927fbc2d7f5b1c36a9793965dedd67b
created_at: 2026-11-10T01:32:23.535820888Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: planner (claude-fable-5-2)
    commit: a2786acfd9a277d0c41622bb871ea5bf80bd3b3e
    recorded_at: 2026-11-10T01:40:07.193946401Z
    follow_up: atelier-u1ko
    reason: 'Deferred to the switch step''s planning boundary: atelier-u1ko''s Unknowns now names it.'
---

medium atelier-u1ko: the runbook installs the new binary but never restarts atelier-serve. docs/architecture/site.md:329 says the service keeps the binary it started with, so a successful conversion can leave the dashboard reading new records with the old reader. At step E, include service restart and a live read after conversion, with corresponding restoration on rollback.
