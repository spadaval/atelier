---
schema: atelier.review
schema_version: 2
id: atelier-p7uh
title: finding on atelier-u1ko @d41d3bcd
target: atelier-u1ko
identity: GPT-6 Astra independent planning reviewer
commit: d41d3bcd1be9bacd36781ba370621136144f6a60
created_at: 2026-11-11T04:50:11.123725450Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-u1ko.md:83 The README must explain how the work host later runs the same script, but the specified script requires old local bases to equal the remote tips and then converts and pushes all five bases anew. By then this host has already published converted records; the old work host either fails preflight while retaining old records or fetches the converted tree without the new binary/skill installed. No delayed-host path is settled. Specify the reviewed adoption sequence for an already-converted remote, preserving the protocol's no-fetch-until-ready rule, or explicitly resolve this requirement before implementation rather than leaving the operator to invent it on that host.
