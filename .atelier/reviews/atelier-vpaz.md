---
schema: atelier.review
schema_version: 2
id: atelier-vpaz
title: 'low: crates/atelier-cli/src/commands/git.rs:146-148. When main has an upstream configured that does not resolve, land on main still prints ''Base commits unchecked: main has no upstream, so there is no pushed point to check from.'' and ''Landing checks passed'' before the new land_push_base_upstream_unresolved refusal, and plain ''land'' (no --push) prints only that note. ''has no upstream'' is wrong in that case and contradicts the refusal printed two lines later. Fix: distinguish ''no upstream'' from ''upstream <remote>/<merge> does not resolve (gone, or a URL/path remote)'' in the note.'
target: atelier-fngz
identity: Independent reviewer (Claude subagent)
commit: 97e785a471c9a95cea99441e15682e56a1c90826
created_at: 2026-10-22T23:43:27.142742146Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: ef59b1109fdeb322bba48bdded0f8a16981360c1
    recorded_at: 2026-10-22T23:45:57.042221689Z
    resolution_commit: 5959474657df56a8fd49bab0b7215c7fb6a05268
    reason: Fixed in 59594746
    migrated_from: atelier-pl9q
---

low: crates/atelier-cli/src/commands/git.rs:146-148. When main has an upstream configured that does not resolve, land on main still prints 'Base commits unchecked: main has no upstream, so there is no pushed point to check from.' and 'Landing checks passed' before the new land_push_base_upstream_unresolved refusal, and plain 'land' (no --push) prints only that note. 'has no upstream' is wrong in that case and contradicts the refusal printed two lines later. Fix: distinguish 'no upstream' from 'upstream <remote>/<merge> does not resolve (gone, or a URL/path remote)' in the note.
