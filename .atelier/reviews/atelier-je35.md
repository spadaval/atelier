---
schema: atelier.review
schema_version: 2
id: atelier-je35
title: finding on atelier-4yqd @8d34b60c
target: atelier-4yqd
identity: independent reviewer subagent (claude-opus-5-5), atelier-4yqd
commit: 8d34b60cc52f33f439f67b16b9e5d50ccaa90a3b
created_at: 2026-11-07T05:50:04.521268083Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: orchestrator (claude-opus-5-5), mission atelier-mmpv
    commit: b7282bb8cd7a53d795bc8cfbc615795be3ce5c8d
    recorded_at: 2026-11-07T05:53:45.674028692Z
    resolution_commit: b7282bb8cd7a53d795bc8cfbc615795be3ce5c8d
    reason: Fixed in b7282bb8.
---

low: docs/adr/0028-simpler-lifecycle.md:307 'Each names what it deletes' is false today: atelier-4nsl and atelier-l457 name no deletions (planning approval atelier-0wuw left them 'until those items are planned'). An orchestrator could start them believing they comply. Decision 9 already states the rule. Fix: 'Each must name what it deletes before it starts (decision 9).'
