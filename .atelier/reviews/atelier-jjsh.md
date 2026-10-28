---
schema: atelier.review
schema_version: 1
id: atelier-jjsh
title: finding on atelier-7v9q @575cf9d0
target: atelier-7v9q
identity: Independent reviewer (Claude subagent)
commit: 575cf9d0abbb3682dda04458261b4e1ced61d016
created_at: 2026-10-28T01:52:31.215638689Z
event:
  event: finding
  recommendation: defer
---

medium: .github/workflows/release.yml:41-45 — a v* tag push publishes regardless of whether CI passed on that commit, while the strategy Outcome says releases come from a commit that passed the checks. S2 and the issue do not require a gate. Defer: make 'tag only a master commit with green CI' a release-process rule when the first tag is cut, rather than editing the generated workflow.
