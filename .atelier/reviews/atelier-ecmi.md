---
schema: atelier.review
schema_version: 2
id: atelier-ecmi
title: finding on atelier-295u @e8873823
target: atelier-295u
identity: gpt-6-astra-amend2-r2
commit: e88738233cd37e72ce9f54d19caf2db66ee224b2
created_at: 2026-11-10T04:10:24.672537482Z
event:
  event: finding
  recommendation: fix_now
---

medium, docs/strategies/redesign-4-gardener.md:35-39 and redesign-architecture.md:316-330: from-log still promises time in each status using log alone, but log explicitly omits new and emits no creation timestamp. An item created on Monday and readied Friday has no log boundary for entering draft, so that duration cannot be computed. Retaining Git readers fixes the original source-line and classification problem, but this narrower mission criterion remains impossible. Smallest fix: limit timing to status intervals whose boundaries the log exposes, with coverage reported and missing intervals unknown, or expose creation facts without arithmetic. Also replace the strategy line 16 claim that every act changes status. This is the remaining contract gap in atelier-sm04.
