---
schema: atelier.review
schema_version: 2
id: atelier-kl16
title: finding on atelier-fad5 @0ae80c75
target: atelier-fad5
identity: codex-reviewer (gpt-6.1-sol)
commit: 0ae80c75f4325cfeab5d0802e3318af8b08f818a
created_at: 2026-11-09T04:34:49.509717986Z
event:
  event: finding
  recommendation: fix_now
---

high: R1 crates/atelier-cli/src/commands/merge_push.rs:171 The base fast-forward can overwrite an ignored, uncommitted tracker record before merge's canonical snapshot guard runs. fast_forward calls clean(), which checks only git status --porcelain --untracked-files=no; Git then overwrites ignored untracked files during its fast-forward. Reproduced in a scratch repository with main in a separate worktree, an ignored .atelier/issues/review-fld8.md containing an unsaved local plan, and the fetched main adding a different committed record at that path. Plain merge --check refused canonical_snapshot_uncommitted, but merge --push exited 0, overwrote the local draft with the remote record and published. This loses local record data and bypasses an existing merge safety constraint. Before fast-forwarding a checked-out base, use the same CommittedSnapshot cleanliness proof that merge uses, pinned to its captured base head, and preserve the existing ownership/state fences. Add a publishing test that verifies the ignored record bytes and base ref stay unchanged on refusal. Proof output: /home/supadava/.cache/atelier-tmp/fad5-review/fad5-probes-wx6rgw0c/ignored-base-record/output.txt. Recommendation: FIX NOW.
