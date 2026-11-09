---
schema: atelier.review
schema_version: 2
id: atelier-8fki
title: finding on atelier-7tul @8095332a
target: atelier-7tul
identity: codex-plan-reviewer (gpt-6.1-sol)
commit: 8095332a5c72f40efe4006ed20d26b13e84f456b
created_at: 2026-11-09T02:07:16.015740762Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-7tul.md:36 The Done when line requires this item to have merged through the new check, with its actual trusted run in the Report. That cannot hold before its own review and merge; review request refuses an unticked line, whereas merge is what marks this work done. This fails plan.md Ready Work independently of the policy bootstrap defect. Replace it with a pre-merge integration scenario proving the gate transition, and put observation of the actual merged result in a blocked loose end or independent validation item, as appropriate.
