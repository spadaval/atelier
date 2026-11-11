---
schema: atelier.review
schema_version: 2
id: atelier-vtix
title: finding on atelier-uwxy @125a72b3
target: atelier-uwxy
identity: Independent planning reviewer (GPT-6 Astra)
commit: 125a72b312eeea6a249ca4d5b93240d39e61d357
created_at: 2026-11-11T05:30:09.382092584Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-uwxy.md:40: The release proof is not defined for the pre-switch implementation path. atelier-u1ko is still todo and has no local branch; it will choose and change the root release metadata, profile and workflows (docs/strategies/redesign-architecture.md:1121-1127). This item nevertheless requires proof against that future configuration before its own merge, while line 62 freezes its merges once the switch starts. Root dist-workspace.toml currently selects cargo:., which excludes next/, so checking today's release alone cannot prove the moved Desk stays out. Define a pre-merge prospective check in a scratch tree with stated relocation/release assumptions, or assign the actual switched-release proof explicitly to atelier-u1ko (whose Done when already requires it). Also name next/Cargo.toml for the dependency test and the working directory for install: today's root atelier-cli intentionally depends on atelier-site and already has axum, tokio and ts-rs transitively. Preserve the intended no-new-server-dependencies and no-Desk-artifact constraints without requiring an implementer to guess which workspace or future configuration proves them. Recommendation: FIX NOW; a ready item's acceptance proof must be executable before its merge.
