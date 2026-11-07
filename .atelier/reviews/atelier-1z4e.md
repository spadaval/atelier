---
schema: atelier.review
schema_version: 2
id: atelier-1z4e
title: 'crates/atelier-cli/src/commands/bundle.rs:89 and docs/spec/bundle/schema.md:19-23,55: renaming the bundle field depends_on to blocked_by breaks schema_version 2 bundles without a version bump. Thanks to deny_unknown_fields, an old bundle fails before any mutation with ''unknown field depends_on, expected one of ... blocked_by ...'' (checked with bundle preview on the fixture with depends_on restored), so the adopter constraint (fail clearly, never ignore data silently) holds. Still, the schema doc''s own precedent rejects v1 by version, and adopters (astromech) get a generic serde error rather than a rename hint. Suggested follow-up: a targeted error (''depends_on was renamed blocked_by'') or a schema_version bump with a note in schema.md.'
target: atelier-kaor
identity: Independent reviewer (Claude subagent)
commit: 3f41d55973b5f9469c0ed8066ded2673c0be2259
created_at: 2026-10-15T23:46:44.018367757Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 0f489f2f14870835465001d44cd3f6e024830c81
    recorded_at: 2026-10-15T23:50:09.893153507Z
    resolution_commit: 0f489f2f14870835465001d44cd3f6e024830c81
    reason: Fixed in 0f489f2f14870835465001d44cd3f6e024830c81
    migrated_from: atelier-n7nt
---

crates/atelier-cli/src/commands/bundle.rs:89 and docs/spec/bundle/schema.md:19-23,55: renaming the bundle field depends_on to blocked_by breaks schema_version 2 bundles without a version bump. Thanks to deny_unknown_fields, an old bundle fails before any mutation with 'unknown field depends_on, expected one of ... blocked_by ...' (checked with bundle preview on the fixture with depends_on restored), so the adopter constraint (fail clearly, never ignore data silently) holds. Still, the schema doc's own precedent rejects v1 by version, and adopters (astromech) get a generic serde error rather than a rename hint. Suggested follow-up: a targeted error ('depends_on was renamed blocked_by') or a schema_version bump with a note in schema.md.
