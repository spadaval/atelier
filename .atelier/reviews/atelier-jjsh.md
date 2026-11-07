---
schema: atelier.review
schema_version: 2
id: atelier-jjsh
title: finding on atelier-7v9q @575cf9d0
target: atelier-7v9q
identity: Independent reviewer (Claude subagent)
commit: 575cf9d0abbb3682dda04458261b4e1ced61d016
created_at: 2026-10-28T01:52:31.215638689Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Independent reviewer (Claude subagent)
    commit: 8faa5961a601a1f0efaef8cd3f1ce86cc29bbcad
    recorded_at: 2026-10-28T01:52:31.554190587Z
    follow_up: atelier-vy5i
    reason: 'Reviewer''s recommendation: a release-process rule, not a workflow edit; atelier-vy5i''s Done when now requires the tagged commit''s master CI run to have passed before the tag is pushed.'
    migrated_from: atelier-drh7
---

medium: .github/workflows/release.yml:41-45 — a v* tag push publishes regardless of whether CI passed on that commit, while the strategy Outcome says releases come from a commit that passed the checks. S2 and the issue do not require a gate. Defer: make 'tag only a master commit with green CI' a release-process rule when the first tag is cut, rather than editing the generated workflow.
