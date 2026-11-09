---
schema: atelier.review
schema_version: 2
id: atelier-gfwz
title: finding on atelier-fad5 @0ae80c75
target: atelier-fad5
identity: codex-reviewer (gpt-6.1-sol)
commit: 0ae80c75f4325cfeab5d0802e3318af8b08f818a
created_at: 2026-11-09T04:34:49.812816252Z
event:
  event: finding
  recommendation: fix_now
---

medium: R2 crates/atelier-cli/src/commands/merge_push.rs:50 The forge authority is cached before fetching and fast-forwarding the base, then reused to push even when the refreshed base policy no longer names a forge. Reproduced with an approved light-path branch and a remote main ahead by a commit removing forge and replacing ci.passed with blockers.none_open: merge --push fast-forwarded main to the no-forge policy, merged and pushed main to the old origin, exiting 0. The merged policy contained no forge. The gates use the refreshed base policy while publication uses the stale one, violating ADR 0031 decision 3 and the no-forge refusal contract. Revalidate the authoritative base forge configuration after refreshing the base, and refuse or restart when it changes before integration/publication; cover removal and remote changes, including a base retry that changes this authority. Proof output: /home/supadava/.cache/atelier-tmp/fad5-review/fad5-probes-wx6rgw0c/remote-disables-forge/output.txt. Recommendation: FIX NOW.
