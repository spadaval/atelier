---
schema: atelier.review
schema_version: 2
id: atelier-lmx5
title: finding on atelier-xtvh @9c77b784
target: atelier-xtvh
identity: gpt-6.1-sol
commit: 9c77b7846b86dd445fac6b429dcbe8f0f9eff552
created_at: 2026-11-10T17:42:53.409186761Z
event:
  event: finding
  recommendation: fix_now
---

Medium: .atelier/issues/atelier-xtvh.md:42 requires merge to finish after the base moves during its unlocked CI wait, but does not require a fresh validation after that wait. In atelier-syyj at d30b1f26, merge.rs:42-108 locks, reads the base config, builds the merge commit and runs local checks before consulting CI; lines 154-175 then refuse a moved base. Merely releasing the lock and relaxing that refusal would use the old base/config/check results. The planned harmless-base-commit test would not catch skipped checks when the base adds a failing merge check, drops the item, or the work branch changes during the wait. State that after waiting merge acquires the lock, rereads current inputs and base config, rechecks eligibility and approval, builds and checks the current merged tree, and retains the final moved-input guard. Add a failing-new-base-check scenario and a branch-change or drop scenario. Amend the architecture Merge ordering and The forge lock/re-check paragraph too; the current document-update list omits the latter.
