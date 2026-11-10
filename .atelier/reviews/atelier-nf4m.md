---
schema: atelier.review
schema_version: 2
id: atelier-nf4m
title: finding on atelier-fcb7 @6f837e9f
target: atelier-fcb7
identity: grok-4.6-plan-r1
commit: 6f837e9f52a011db821640751ec20b664759a850
created_at: 2026-11-10T01:06:06.299733470Z
event:
  event: finding
  recommendation: fix_now
---

high redesign-architecture.md:252-256: after the swap the tree binary cannot read this repo's old records, CI pins 0.4.0 for check (ci.yml:72 currently runs target/debug/atelier check), and mission 3 still tracks work with 0.4.0 until cutover. Nothing says how an agent invokes the tracker versus the tree, so cargo-built atelier will refuse the repo's records and look like a failed cutover. Smallest fix: the swap item installs 0.4.0 at a pinned path for tracking, CI and hooks, and documents that cargo run and target/debug/atelier are fixtures-only until convert.
