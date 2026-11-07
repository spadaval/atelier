---
schema: atelier.review
schema_version: 2
id: atelier-268e
title: 'crates/atelier-cli/src/commands/session.rs:460 — the land hook''s block reason wraps the error as ''Atelier landing failed: {error:#}. Fix the named check or hand off unfinished work honestly; ...''. Because land_records_uncommitted and check failures now end in their own ''Next: ...'', the agent hook gets two recovery steps for one failure, against the one-recovery-step rule. This change touched the line (invariant -> check) but did not change its structure. Low impact.'
target: atelier-91w1
identity: Independent reviewer (Claude subagent)
commit: b3a32046fd1d563706e20e9f7e672b687b8a11db
created_at: 2026-10-17T08:20:42.885120509Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 32365e408530d872d6c2b830c60032e79771c4ce
    recorded_at: 2026-10-17T08:29:28.423015974Z
    follow_up: atelier-467y
    reason: Hook block reason; filed atelier-467y.
    migrated_from: atelier-zvm4
---

crates/atelier-cli/src/commands/session.rs:460 — the land hook's block reason wraps the error as 'Atelier landing failed: {error:#}. Fix the named check or hand off unfinished work honestly; ...'. Because land_records_uncommitted and check failures now end in their own 'Next: ...', the agent hook gets two recovery steps for one failure, against the one-recovery-step rule. This change touched the line (invariant -> check) but did not change its structure. Low impact.
