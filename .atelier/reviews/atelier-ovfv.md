---
schema: atelier.review
schema_version: 2
id: atelier-ovfv
title: approval on atelier-zrss @93cb2a62
target: atelier-zrss
identity: gpt-6.1-sol
commit: 93cb2a626731c8e885c5081e35d201d4a4bce14d
created_at: 2026-11-10T22:57:34.560783442Z
event:
  event: approval
---

Planning delta approved. The pre-commit implementation chains items and missions, then matches TABLE rows by from/to alone (next/crates/atelier-cli/src/check.rs:111-124). Act::Close accepts draft to done for items, while Act::CloseMission accepts ready to done only. The added negative commit test would expose that mismatch; its positive ready-to-done case guards the allowed mission transition. Matching the record kind implements the existing Status table and Hooks and the bypass contract, adds no policy or interface, and belongs in zrss because it already changes this hook rule and its bypass. Verdict: merge. Complexity low, value high (existing specified rule), risk low. No findings. Static review only; no builds or runtime tests run.
