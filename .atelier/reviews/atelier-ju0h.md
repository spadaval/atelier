---
schema: atelier.review
schema_version: 2
id: atelier-ju0h
title: 'low: docs/product/reviews-and-merge.md:115 still says ''Under fast_forward_only, the default, merge fast-forwards only'', and .agents/skills/agent-factory/references/atelier.md:118 says ''Under the default branch_policy.merge_strategy: fast_forward_only''. After atelier-2yl7 the default is merge_commit when merge_checks include a shell.check and fast_forward_only otherwise (workflow-configuration.md now says so), so the two sources contradict it and agents reading the skill reference get the old rule. Fix: replace ''the default'' with the conditional default or a link to workflow-configuration.md in both places.'
target: atelier-c3kh
identity: Independent reviewer (Claude subagent)
commit: f5733347e44e5cbc88ce25b5680ea8e91d92ddfc
created_at: 2026-10-22T20:52:27.903726865Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 6f7bffe48d815a32a546430881469d7c0bbf5791
    recorded_at: 2026-10-22T20:56:05.786395366Z
    resolution_commit: 2c18ab441b211bbe673db2fa821112da5c50d42e
    reason: Fixed in 2c18ab44
    migrated_from: atelier-7wg2
---

low: docs/product/reviews-and-merge.md:115 still says 'Under fast_forward_only, the default, merge fast-forwards only', and .agents/skills/agent-factory/references/atelier.md:118 says 'Under the default branch_policy.merge_strategy: fast_forward_only'. After atelier-2yl7 the default is merge_commit when merge_checks include a shell.check and fast_forward_only otherwise (workflow-configuration.md now says so), so the two sources contradict it and agents reading the skill reference get the old rule. Fix: replace 'the default' with the conditional default or a link to workflow-configuration.md in both places.
