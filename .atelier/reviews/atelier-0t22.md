---
schema: atelier.review
schema_version: 2
id: atelier-0t22
title: 'docs/product/work-model.md:35-38: says land and merge check a required part ''only on a done claim committed beside a policy that required it'', which describes the pre-7f471530 behaviour the atelier-ompn fix removed; the policy now consulted is the base''s at the merge-base of the claim''s commit and the base, never the policy committed beside the claim. Reword, e.g. ''only on a done claim made after the base''s policy required it, judged where the claim''s commit last met the base''.'
target: atelier-sfsq
identity: Independent reviewer 3 (Claude subagent)
commit: 81a42d6dc49f9b1504c564482ab8ee507e8a1263
created_at: 2026-10-17T18:20:22.406287790Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Claude Opus 5.5 (orchestrator)
    commit: 81a42d6dc49f9b1504c564482ab8ee507e8a1263
    recorded_at: 2026-10-17T18:27:32.920534091Z
    follow_up: atelier-4vda
    reason: 'Accept the reviewer''s DEFER: follow-up atelier-4vda; for u0i3 keep the exemption and document the limit, since the missing report shows in the reviewed diff'
    migrated_from: atelier-ne9l
---

docs/product/work-model.md:35-38: says land and merge check a required part 'only on a done claim committed beside a policy that required it', which describes the pre-7f471530 behaviour the atelier-ompn fix removed; the policy now consulted is the base's at the merge-base of the claim's commit and the base, never the policy committed beside the claim. Reword, e.g. 'only on a done claim made after the base's policy required it, judged where the claim's commit last met the base'.
