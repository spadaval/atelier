---
schema: atelier.review
schema_version: 2
id: atelier-p936
title: finding on atelier-psjw @fc0fe395
target: atelier-psjw
identity: GPT-6.1 Sol independent planning reviewer
commit: fc0fe3950f4f91713f22e5ca294196e758bc21e7
created_at: 2026-11-11T04:09:16.922451949Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-psjw.md:35: medium severity. The proposed skip-state owner does not cover the existing callers of the one shared policy. All reason and passed-rule state is assigned to Repo, but main.rs:233-239 deliberately dispatches check and check --hook before opening Repo. check.rs:28-91,165-212 handles malformed config and the first commit of an unborn branch, while Repo::open (repo.rs:36-46) requires a readable config and an existing base tip. hooks.rs:102-163,506-531,661-678 explicitly covers those paths. Requiring Repo to get the skip policy would regress those diagnostics/hooks; retaining separate CLI state would contradict the one owner and no-global-state design. Also, merge.rs:57-64,353-355 opens separate Repo snapshots before and after its lock during one invocation, and main.rs:344-356 prints passed rules after the Repo is gone. Repo lifetime is therefore not act lifetime. A CI rule skipped before the lock must remain printed and trailed even if CI passes at the locked recheck. The refusal-text split itself is workable by moving shared formatting helpers once and calling them from both crates; atelier-mmqq is addressed by lines 37 and 40. atelier-irzs is only partly addressed: the new no-leak requirement and formatting ownership are clear, but the chosen state boundary is incomplete. Settle a single invocation-owned skip value that can exist without Repo, be used by every Repo snapshot and check/hook caller, and reach commit trailers and final output without a global or duplicate policy. Add proof for check with bad config, a first-commit hook, and skip preservation across merge's reopened Repo, alongside the two-library-call isolation test. This is required to preserve existing behavior, not new product scope. Planning review of fc0fe395; source/test inspection only.
