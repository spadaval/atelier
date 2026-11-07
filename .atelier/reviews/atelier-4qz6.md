---
schema: atelier.review
schema_version: 2
id: atelier-4qz6
title: 'crates/atelier-cli/src/commands/bundle.rs:1111 and docs/spec/bundle/schema.md:97: the docs say a mission listing an epic in advances and the epic listing the mission ''are the same edge'' and that ''Duplicate normalized edges are errors'', but a bundle declaring both sides applies successfully and reports ''relationships: 2'' (both directions listed). depends_on+blocks duplicates are rejected (''Duplicate bundle relationship after normalization''). Stored records are correct (one advances entry per record), so there is no data harm. Suggested fix: normalise advances to (mission, work) in BundleGraphPlan so the existing duplicate check fires, or document that two-sided declaration is accepted and collapsed.'
target: atelier-1ioo
identity: independent-reviewer-2026-10-15
commit: d1cb7d908869c6ddadafcc69416ddf7aea42df75
created_at: 2026-10-15T19:13:37.187629641Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: rebuild-orchestrator
    commit: ba7512b7e96a2d027b7ff6fedde9889b73031ca1
    recorded_at: 2026-10-15T19:16:27.883441792Z
    resolution_commit: ba7512b7e96a2d027b7ff6fedde9889b73031ca1
    reason: Fixed in ba7512b7; needs independent re-review.
    migrated_from: atelier-xtyu
---

crates/atelier-cli/src/commands/bundle.rs:1111 and docs/spec/bundle/schema.md:97: the docs say a mission listing an epic in advances and the epic listing the mission 'are the same edge' and that 'Duplicate normalized edges are errors', but a bundle declaring both sides applies successfully and reports 'relationships: 2' (both directions listed). depends_on+blocks duplicates are rejected ('Duplicate bundle relationship after normalization'). Stored records are correct (one advances entry per record), so there is no data harm. Suggested fix: normalise advances to (mission, work) in BundleGraphPlan so the existing duplicate check fires, or document that two-sided declaration is accepted and collapsed.
