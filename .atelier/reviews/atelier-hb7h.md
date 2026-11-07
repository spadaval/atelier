---
schema: atelier.review
schema_version: 2
id: atelier-hb7h
title: 'Cargo.toml:21 — clap still enables the "env" feature, which existed only for the removed ATELIER_LOG/ATELIER_LOG_FORMAT bindings on --log-level/--log-format. No #[arg(env = ...)] or Arg::env remains in crates/*/src, so the change leaves a dead dependency feature: residue of this removal, contrary to standards.md''s unused-dependency check, and it keeps signalling an env-var surface (the 2026-06-13 readiness audit already read clap env support that way) that development-setup.md now says does not exist. Fix: change features to ["derive"] and refresh Cargo.lock if it changes; the build already passes --locked without tracing.'
target: atelier-a76d
identity: Independent reviewer (Claude subagent)
commit: 65ddf3489ef83df65d9232468180e219f9c5e2ec
created_at: 2026-10-17T06:37:54.886890576Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 354f65160eb9b6402120c9fe88f9daf1986db2e6
    recorded_at: 2026-10-17T06:58:10.192073503Z
    resolution_commit: 354f65160eb9b6402120c9fe88f9daf1986db2e6
    reason: Workspace clap features are now ["derive"]; build and both suites pass.
    migrated_from: atelier-iwcc
---

Cargo.toml:21 — clap still enables the "env" feature, which existed only for the removed ATELIER_LOG/ATELIER_LOG_FORMAT bindings on --log-level/--log-format. No #[arg(env = ...)] or Arg::env remains in crates/*/src, so the change leaves a dead dependency feature: residue of this removal, contrary to standards.md's unused-dependency check, and it keeps signalling an env-var surface (the 2026-06-13 readiness audit already read clap env support that way) that development-setup.md now says does not exist. Fix: change features to ["derive"] and refresh Cargo.lock if it changes; the build already passes --locked without tracing.
