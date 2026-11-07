---
schema: atelier.review
schema_version: 2
id: atelier-xb4j
title: finding on atelier-nn67 @ae99fb06
target: atelier-nn67
identity: independent reviewer subagent (claude-opus-5-5), atelier-nn67
commit: ae99fb066f77ea27886e584db6d77b84bad698e3
created_at: 2026-11-07T14:50:10.056681798Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/product/workflow-configuration.md:139 step 2 says to copy `branch_policy.merge_strategy: merge_commit` from the policy a fresh `atelier init` writes, but a fresh init writes no merge_strategy (starter_policy_yaml writes only base_branch; confirmed on a scratch init), and line 209 says the field is optional and every value reads as a merge commit. The four rollout agents act on this section next and will look for a key that is not there or add a no-op one. Fix: say an old merge_strategy can stay or be removed, since a fresh init writes none and fast_forward_only and squash already read as merge commits.
