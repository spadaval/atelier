---
schema: atelier.review
schema_version: 2
id: atelier-hytg
title: 'G11: Rename detection omits deleted source review paths from history guards. Independent reproduction renames an invalid self-disposition record and edits its header ID and identity (Git R090); landing accepts the rewritten event. This preexisting hole violates required append-only review history (R6), so both rename endpoints must be protected while legitimate whole terminal review-cluster retirement remains valid.'
target: atelier-1ioo
identity: session-reviewer
commit: 6f5188dd5bc990c88733a0027ee703eaaef9f737
created_at: 2026-10-02T00:46:26.809423139Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: orchestrator
    commit: 5277b54c6b3a787a36bb927928ec1771218da0cb
    recorded_at: 2026-10-02T01:03:03.286347810Z
    resolution_commit: b24fdc628c43c1beef67c9558cdfc7091b7894ac
    reason: 'G11 independently verified: both rename endpoints are checked; review laundering refuses and legitimate terminal-cluster retirement still passes.'
    migrated_from: atelier-kviu
---

G11: Rename detection omits deleted source review paths from history guards. Independent reproduction renames an invalid self-disposition record and edits its header ID and identity (Git R090); landing accepts the rewritten event. This preexisting hole violates required append-only review history (R6), so both rename endpoints must be protected while legitimate whole terminal review-cluster retirement remains valid.
