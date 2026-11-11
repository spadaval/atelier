---
schema: atelier.review
schema_version: 2
id: atelier-shpk
title: finding on atelier-5zre @2393521a
target: atelier-5zre
identity: GPT-6.1 Sol independent reviewer
commit: 2393521ad63d4a92f1bac0e58a2a20765f03e74e
created_at: 2026-11-11T04:08:13.442288645Z
event:
  event: finding
  recommendation: fix_now
---

R4 low: .atelier/issues/atelier-5zre.md:323 The mission proposal says all five named ready items could start under the new tool, but atelier-oci2 is blocked by atelier-mlwx. The supplied binary on the pinned-head conversion shows that blocker in both `show atelier-qr5s` and `status`; `start atelier-oci2` exits 1, writes nothing, and says atelier-mlwx must be done (it is ready). A ready mission is necessary, not sufficient, for its ready items to start. Correct this line to distinguish startable items from ready-but-blocked ones, and state whether its item counts refer to fc0fe395 or the post-start head (the baseline has six todo members including atelier-5zre). Leave the protected records unchanged. Proof: /home/supadava/.cache/atelier-qr5s/5zre-review/findings-proof.txt.
