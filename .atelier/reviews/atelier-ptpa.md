---
schema: atelier.review
schema_version: 2
id: atelier-ptpa
title: docs/adr/0020-agent-factory-targets-atelier-only.md:5 (Status) gets no 'Amended by ADR 0022' note, while ADR 0022 amends its Decision 3 (0020:41 'session context from prime'). Repository convention annotates amended ADRs (0015, 0016, 0018), and docs/index.md tells readers to consult each decision's status. Because bw0b's Done when makes acceptance and merge one step, add the note in this branch when Sumeet flips 0022 to Accepted, before merge; not needed while it is Proposed.
target: atelier-bw0b
identity: Independent reviewer 3 (Claude subagent)
commit: 6c51447373552e58ab54acc8783d3a64c6a9991e
created_at: 2026-10-16T23:56:15.506586819Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (author)
    commit: 30dffbfe201ad35937d483d46fa38f5aa9d125ee
    recorded_at: 2026-10-16T23:57:14.360866947Z
    resolution_commit: 30dffbfe201ad35937d483d46fa38f5aa9d125ee
    reason: ADR 0020 Status notes the Decision 3 amendment; it merges only with 0022 accepted
    migrated_from: atelier-lgvg
---

docs/adr/0020-agent-factory-targets-atelier-only.md:5 (Status) gets no 'Amended by ADR 0022' note, while ADR 0022 amends its Decision 3 (0020:41 'session context from prime'). Repository convention annotates amended ADRs (0015, 0016, 0018), and docs/index.md tells readers to consult each decision's status. Because bw0b's Done when makes acceptance and merge one step, add the note in this branch when Sumeet flips 0022 to Accepted, before merge; not needed while it is Proposed.
