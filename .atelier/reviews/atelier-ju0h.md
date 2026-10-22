---
schema: atelier.review
schema_version: 1
id: atelier-ju0h
title: 'low: docs/product/reviews-and-merge.md:115 still says ''Under fast_forward_only, the default, merge fast-forwards only'', and .agents/skills/agent-factory/references/atelier.md:118 says ''Under the default branch_policy.merge_strategy: fast_forward_only''. After atelier-2yl7 the default is merge_commit when merge_checks include a shell.check and fast_forward_only otherwise (workflow-configuration.md now says so), so the two sources contradict it and agents reading the skill reference get the old rule. Fix: replace ''the default'' with the conditional default or a link to workflow-configuration.md in both places.'
target: atelier-c3kh
identity: Independent reviewer (Claude subagent)
commit: f5733347e44e5cbc88ce25b5680ea8e91d92ddfc
created_at: 2026-10-22T20:52:27.903726865Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/product/reviews-and-merge.md:115 still says 'Under fast_forward_only, the default, merge fast-forwards only', and .agents/skills/agent-factory/references/atelier.md:118 says 'Under the default branch_policy.merge_strategy: fast_forward_only'. After atelier-2yl7 the default is merge_commit when merge_checks include a shell.check and fast_forward_only otherwise (workflow-configuration.md now says so), so the two sources contradict it and agents reading the skill reference get the old rule. Fix: replace 'the default' with the conditional default or a link to workflow-configuration.md in both places.
