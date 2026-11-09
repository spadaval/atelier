---
schema: atelier.review
schema_version: 2
id: atelier-726p
title: finding on atelier-yfqu @28ae3d61
target: atelier-yfqu
identity: codex-reviewer (gpt-6.1-sol)
commit: 28ae3d61ecce47e8befb0e1469e16122d8addf04
created_at: 2026-11-09T00:51:50.538541071Z
event:
  event: finding
  recommendation: fix_now
---

low: The unowned-branch reminder tells valid light-path work to create unnecessary issue ownership.
.atelier/issues/atelier-5kwk.md:28-33 versus PRODUCT_INTENT.md:42-46 and docs/product/reviews-and-merge.md's light path.
A release, docs change or mockup may deliberately own no issue and still be independently reviewed and merged. The proposed hook tests only branch ownership, so it prompts those valid commits to run atelier start as if that were required for attribution. Being nonblocking does not make the guidance correct. Preserve the light path explicitly in the reminder and its acceptance fixture, or remove the nudge until attribution supports unowned branch work. Do not introduce an issue requirement as an accidental side effect of telemetry.
Recommendation: FIX NOW; this is a direct conflict with a supported workflow, with a small wording/acceptance fix.
