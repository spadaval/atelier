---
schema: atelier.review
schema_version: 2
id: atelier-kt51
title: finding on atelier-fcb7 @d7010574
target: atelier-fcb7
identity: gpt-6-astra-amend2-r3
commit: d7010574622ab952b712553f33d22c3a1c56ba0f
created_at: 2026-11-10T04:12:10.302783827Z
event:
  event: finding
  recommendation: fix_now
---

medium, docs/strategies/redesign-architecture.md:316-318 and :255-261: the exhaustive list of acts whose commits carry skips omits start, although Starting is one atomic step explicitly says start writes a commit. Starting an item with an unfinished blocker under ATELIER_SKIP makes an observable start commit with a skip trailer under Hooks and the bypass. The log contract therefore still misstates which skips it exposes, leaving the skip part of atelier-tyrn unresolved. Smallest fix: include start in the list, or remove the exhaustive enumeration and require reading the trailer on every observable act commit Atelier creates. This is a source-contract correction, not a request to design the later history walk.
