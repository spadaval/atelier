---
schema: atelier.review
schema_version: 2
id: atelier-bo4x
title: finding on atelier-4yqd @8d34b60c
target: atelier-4yqd
identity: independent reviewer subagent (claude-opus-5-5), atelier-4yqd
commit: 8d34b60cc52f33f439f67b16b9e5d50ccaa90a3b
created_at: 2026-11-07T05:49:49.377967423Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: orchestrator (claude-opus-5-5), mission atelier-mmpv
    commit: b7282bb8cd7a53d795bc8cfbc615795be3ce5c8d
    recorded_at: 2026-11-07T05:53:42.744622312Z
    resolution_commit: b7282bb8cd7a53d795bc8cfbc615795be3ce5c8d
    reason: Fixed in b7282bb8.
---

medium: docs/adr/0028-simpler-lifecycle.md:202 The Supersession section, and every superseded ADR's new Status line (0016:3, 0019:9, 0021:11, 0025:13, 0026:10, 0027:9, all but 0016 linking to #supersession), state the replacements as already in force ('go', 'no longer', 'Superseded'). Only ADR 0028's Status (lines 19-23), which those anchors skip, says decisions 1 to 8 take effect when their item merges. docs/index.md:21-23 tells agents to apply these supersession lists before older rules, so for the rest of this mission (and in repos sharing the skill) an agent following them would drop the After merge marker, the done claim committed on the work branch, or the recorded base, which today's binary, skill and review briefs still use. Fix: open Supersession with one sentence (each replacement takes effect when the item that builds its decision merges; until then the earlier text describes current behaviour; decision 9 applies now), and add a matching clause to each Status line (e.g. ADR 0027: superseded when atelier-8bh4 lands).
