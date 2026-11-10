---
schema: atelier.review
schema_version: 2
id: atelier-sewr
title: finding on atelier-ym3z @1e0e6d51
target: atelier-ym3z
identity: gpt-6.1-sol
commit: 1e0e6d51877194c06d65736e7f7c7e760715d6e7
created_at: 2026-11-10T21:36:17.452568514Z
event:
  event: finding
  recommendation: fix_now
---

Medium: .atelier/issues/atelier-ym3z.md:33: The prescribed hook preservation move has no rule for an already occupied <name>.local. A clone may already have both pre-push and pre-push.local; a literal rename of the former over the latter silently loses an existing project hook, while the one existing-hook test still passes. Git hook files are usually untracked, so the loss is costly to undo. This is part of the promised preservation of a clone hook and planning finding atelier-1l4r, not a request for another installer feature. Smallest fix: state that setup never overwrites an unmanaged .local file and refuses before replacing the hook when its preservation destination is occupied, leaving both files unchanged. Require a fixture with both names present and assert unchanged contents after refusal, alongside the existing repeated-setup and stdin-forwarding scenarios. No backup registry, new option or numbered backup scheme is needed. Recommendation: FIX NOW. Planning verdict: merge after this preservation condition is made explicit.
