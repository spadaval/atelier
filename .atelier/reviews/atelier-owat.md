---
schema: atelier.review
schema_version: 2
id: atelier-owat
title: 'crates/atelier-cli/src/commands/session.rs:51-57 — records_uncommitted() falls back to ''land_records_uncommitted: {error:#}'' for any error from read_record_files or verified_working_paths that is not RecordsUncommitted, such as a git ls-tree or ls-files failure or an invalid index entry. A Git failure is then reported as an uncommitted-records check with a ''commit records'' step, contradicting ''Neither does a failure of Git or the filesystem'' (human-cli-output.md:242). This predates the change (the old map_err did the same); the new rule makes it visible. Pass non-RecordsUncommitted errors through unchanged.'
target: atelier-91w1
identity: Independent reviewer (Claude subagent)
commit: b3a32046fd1d563706e20e9f7e672b687b8a11db
created_at: 2026-10-17T08:20:43.165985586Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 32365e408530d872d6c2b830c60032e79771c4ce
    recorded_at: 2026-10-17T08:29:29.132438384Z
    follow_up: atelier-467y
    reason: Pre-existing land code mislabel; filed atelier-467y.
    migrated_from: atelier-t0tl
---

crates/atelier-cli/src/commands/session.rs:51-57 — records_uncommitted() falls back to 'land_records_uncommitted: {error:#}' for any error from read_record_files or verified_working_paths that is not RecordsUncommitted, such as a git ls-tree or ls-files failure or an invalid index entry. A Git failure is then reported as an uncommitted-records check with a 'commit records' step, contradicting 'Neither does a failure of Git or the filesystem' (human-cli-output.md:242). This predates the change (the old map_err did the same); the new rule makes it visible. Pass non-RecordsUncommitted errors through unchanged.
