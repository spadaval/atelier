---
schema: atelier.review
schema_version: 2
id: atelier-ek0k
title: finding on atelier-qr5s @4110367e
target: atelier-qr5s
identity: gpt-6-astra-plan-r2
commit: 4110367ed43420776a2dc79903f251980c2ec2a0
created_at: 2026-11-10T01:17:33.385564911Z
event:
  event: finding
  recommendation: fix_now
---

high atelier-wd1i and docs/strategies/redesign-architecture.md:318-321 (atelier-vw77 still open): conversion on the other four bases deletes old .atelier/issues/reviews and strategy paths and changes config.toml, but new land permits only missions/items paths or an Atelier merge. land --push therefore refuses the conversion, and pinned old CI is updated only afterward. Define an independently reviewed one-time conversion publication path with the CI-version switch in that conversion; include it in rehearsal.
