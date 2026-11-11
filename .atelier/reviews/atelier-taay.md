---
schema: atelier.review
schema_version: 2
id: atelier-taay
title: finding on atelier-wd1i @d41d3bcd
target: atelier-wd1i
identity: GPT-6 Astra independent planning reviewer
commit: d41d3bcd1be9bacd36781ba370621136144f6a60
created_at: 2026-11-11T04:50:11.523181292Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-wd1i.md:45-46 Quiet first forbids every in-progress item BEFORE merging u1ko. Under the old lifecycle the approved switch remains in_progress until atelier merge marks it done; approval changes no status. Thus the required switch itself blocks this prerequisite. Name the switch as the sole permitted active work before merge, require it done before conversion, and perform the read-only quiet check before the irreversible merge (the script's later preflight alone is too late). Keep the active cutover thread exemption explicit.
