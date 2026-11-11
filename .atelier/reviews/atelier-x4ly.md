---
schema: atelier.review
schema_version: 2
id: atelier-x4ly
title: finding on atelier-u1ko @d41d3bcd
target: atelier-u1ko
identity: GPT-6.1 Sol independent planning reviewer
commit: d41d3bcd1be9bacd36781ba370621136144f6a60
created_at: 2026-11-11T04:52:00.828841523Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-u1ko.md:68,83 The promised later work-host use of the same runbook has no executable path. That host is explicitly left on old records and must not fetch converted repositories, while the first run pushes converted commits to every base. Its old local base is therefore not the remote tip, so this runbook's mandatory preflight refuses before it can convert anything. Fetching/updating to satisfy the check instead gives it already-converted records, which the old-binary preflight and converter do not read (convert_records.py:606-615 reads workflow.yaml/old issue records). Also F's close step cannot be the first act on a second host where G is already closed. Settle a separate, reviewed later-host sequence that reuses the published conversion commits and switches local binary/skill/hooks safely, or define and rehearse that case in this runbook; reconcile The work host protocol with it. Do not leave Sumeet to invent exceptions to preflight and conversion on the day.
