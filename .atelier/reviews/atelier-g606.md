---
schema: atelier.review
schema_version: 2
id: atelier-g606
title: 'crates/atelier-core/src/lib.rs:362-366: a decision created without --body (or with whitespace) gets ''## Question\n\nGoal was not specified.''; the placeholder names the wrong section. Suggested fix: per-type placeholder (''Question was not specified.'').'
target: atelier-ox63
identity: Independent reviewer (Claude subagent)
commit: f4376ada15eaa6ee51fc9b97dd78e228648811b5
created_at: 2026-10-16T01:12:09.279747183Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: f68771dc51caf607340777f2c2889eb0e3718622
    recorded_at: 2026-10-16T01:20:04.399440431Z
    resolution_commit: f68771dc51caf607340777f2c2889eb0e3718622
    reason: Fixed in f68771dc51caf607340777f2c2889eb0e3718622
    migrated_from: atelier-vdpn
---

crates/atelier-core/src/lib.rs:362-366: a decision created without --body (or with whitespace) gets '## Question\n\nGoal was not specified.'; the placeholder names the wrong section. Suggested fix: per-type placeholder ('Question was not specified.').
