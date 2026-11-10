---
schema: atelier.review
schema_version: 2
id: atelier-9jv4
title: finding on atelier-fcb7 @322c1efd
target: atelier-fcb7
identity: gpt-6-astra-plan-r2
commit: 322c1efd3d788b5ee359f75b3ae22d3c264f4adc
created_at: 2026-11-10T01:17:34.749124936Z
event:
  event: finding
  recommendation: fix_now
---

medium atelier-syyj and docs/strategies/redesign-architecture.md:256 (atelier-64fb still open): forge.rs is still required to be reused as-is, but it imports workflow_policy::ForgeConfig at line 14 and Github stores mandatory workflow:String at lines 99-103, populated from ci_workflow at line 131. The new optional-workflow config cannot feed this interface unchanged, and cleanup removes its policy owner. Explicitly adapt the forge interface to the new config and optional workflow while preserving behavior and contract tests.
