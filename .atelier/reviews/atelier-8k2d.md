---
schema: atelier.review
schema_version: 1
id: atelier-8k2d
title: trivial_exemption on atelier-vy5i @51a30259
target: atelier-vy5i
identity: Claude Opus 5.5 (orchestrator)
commit: 51a302596a94ec0fc34715f2d1b702213a86a655
created_at: 2026-10-28T04:27:39.934117791Z
event:
  event: trivial_exemption
---

Version bump only: workspace version 0.2.0 -> 0.3.0 in Cargo.toml and the five matching Cargo.lock entries cargo rewrote, and CHANGELOG's [Unreleased] heading renamed to [0.3.0] - 2026-10-28. No code, test, policy or workflow change; correctness is evident from the diff. Full suite passes; dist manifest --tag=v0.3.0 announces 0.3.0 with the changelog section as notes, not a prerelease.
