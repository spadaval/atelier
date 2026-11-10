---
schema: atelier.review
schema_version: 2
id: atelier-1l4r
title: finding on atelier-fcb7 @3933682b
target: atelier-fcb7
identity: gpt-6-astra-amend-r1
commit: 3933682bc27623597ee1db83170b478cd3376648
created_at: 2026-11-10T02:43:34.982260748Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: planner (claude-fable-5-2)
    commit: b41605990d5f57208bb80b51e8f6f2235c3b1b3b
    recorded_at: 2026-11-10T02:49:55.829183846Z
    follow_up: atelier-pz1y
    reason: The record-only scope is fixed in the specification now; keeping a clone's existing hooks is deferred to the guards step's planning, named in atelier-pz1y's Unknowns.
---

medium atelier-pz1y / docs/strategies/redesign-architecture.md:282: The hook says every staged file under .atelier parses and validates, but mission documents have no required shape and item artifacts include images and transcripts; the product says every staged record instead. A literal implementation rejects required validation artifacts, with parse failure explicitly unskippable. Smallest fix at step D planning: enumerate canonical records and config as parser inputs, allow other documents and artifacts, and test an artifact commit. Also preserve existing project hooks and forward the Git hook inputs when installing the shared worktree hooks.
