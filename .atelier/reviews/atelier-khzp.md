---
schema: atelier.review
schema_version: 2
id: atelier-khzp
title: finding on atelier-fcb7 @733f0b52
target: atelier-fcb7
identity: gpt-6-astra-amend3-r1
commit: 733f0b52ebfdd61a652b4c309c8f6e9f8680094b
created_at: 2026-11-10T04:47:37.137818702Z
event:
  event: finding
  recommendation: fix_now
---

medium atelier-mal9: the required CI rule says no file under next/ may name a path under crates/, without defining resolution, while the switch requires next/crates/atelier-cli to become crates/atelier-cli. A textual crates/ ban rejects that workspace manifest and copied site instructions; checking every file also scans target and node_modules. Specify next/crates/<name> now and restrict the rule to tracked build/source dependencies resolving into the repository-root old crates/ (Cargo path dependencies, includes and symlinks), allowing new-workspace-relative crates/ and provenance text. Otherwise the first worker must choose between the isolation check and the promised move/install layout.
