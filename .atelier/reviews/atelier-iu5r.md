---
schema: atelier.review
schema_version: 2
id: atelier-iu5r
title: finding on atelier-4yqd @8d34b60c
target: atelier-4yqd
identity: independent reviewer subagent (claude-opus-5-5), atelier-4yqd
commit: 8d34b60cc52f33f439f67b16b9e5d50ccaa90a3b
created_at: 2026-11-07T05:50:04.053422968Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: orchestrator (claude-opus-5-5), mission atelier-mmpv
    commit: b7282bb8cd7a53d795bc8cfbc615795be3ce5c8d
    recorded_at: 2026-11-07T05:53:44.205883168Z
    resolution_commit: b7282bb8cd7a53d795bc8cfbc615795be3ce5c8d
    reason: Fixed in b7282bb8.
---

low: docs/adr/0028-simpler-lifecycle.md:218 'evidence is no longer a guarded invariant' (also line 280 'evidence is no longer guarded', and ADR 0019's Status line 10) overstates the change. ADR 0019's invariant is 'evidence tied to a commit with an explicit result'; ADR 0028 itself keeps command capture refusing uncommitted code (line 266-267), and a receipt still must name a full commit and an explicit result (docs/product/validation.md:71,139). Only the gates that read receipts go. An atelier-md0v or atelier-ilba implementer could read it as licence to drop those guards. Fix: say no gate reads evidence, and a receipt stays tied to a commit with an explicit result.
