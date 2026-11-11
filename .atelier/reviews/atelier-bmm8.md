---
schema: atelier.review
schema_version: 2
id: atelier-bmm8
title: finding on atelier-94pv @6605ca7f
target: atelier-94pv
identity: gpt-6.1-sol-94pv-r1
commit: 6605ca7fce2df0611d022d32464bd26d569333da
created_at: 2026-11-11T06:40:02.896534205Z
event:
  event: finding
  recommendation: fix_now
---

Medium: desk/src/lib/group.ts:34 and desk/src/components/Reviews.tsx:15 treat a closed fix_now finding as fixed, but the core closes an original finding whenever a later round carries it, including as kept, withdrawn or decided (next/crates/atelier-core/src/review.rs:196). With round 1 finding 1-aaaa.1 (now null, open false) and round 2 finding 2-bbbb.1 (was 1-aaaa.1, now withdrawn), fresh headless Chromium still shows the original under Fixed in review and labels it Fixed on the item page. A real fix carried as fixed can also be listed twice. This misstates what was delivered and what review resolved. Require evidence of a fixed resolution, using explicit now=fixed and the carry/decision relationships where needed; do not infer fixed from !open. Cover withdrawn, kept, decided and genuinely fixed multi-round histories with tests.
