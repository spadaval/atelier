---
schema: atelier.review
schema_version: 2
id: atelier-on6c
title: finding on atelier-4yqd @8d34b60c
target: atelier-4yqd
identity: independent reviewer subagent (claude-opus-5-5), atelier-4yqd
commit: 8d34b60cc52f33f439f67b16b9e5d50ccaa90a3b
created_at: 2026-11-07T05:50:10.028664626Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: no_action
    identity: orchestrator (claude-opus-5-5), mission atelier-mmpv
    commit: b7282bb8cd7a53d795bc8cfbc615795be3ce5c8d
    recorded_at: 2026-11-07T05:53:46.158191364Z
    reason: 'Applied on master as planning edits in df2d7c0b: the fast_forward_only question is an Unknown on atelier-jnrh and the ADR 0025 decision 4 question an Unknown on atelier-md0v.'
---

low: docs/adr/0028-simpler-lifecycle.md:103 The ADR assigns two open questions to items whose records do not list them: how merge writes the done state under fast_forward_only, and so how 'Atelier never creates commits' reads (lines 103-105, atelier-jnrh, whose Done when says 'in the merge commit'); and whether ADR 0025 decision 4's comparison with the Git authors goes (lines 153-156, atelier-md0v, whose record does not point at ADR 0028). If they are not added, the md0v implementer may never see its question. The implementer report already flags this for the orchestrator. Fix: add both to those items' Unknowns before they start (a record edit outside this item's docs-only scope).
