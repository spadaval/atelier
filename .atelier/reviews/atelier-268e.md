---
schema: atelier.review
schema_version: 1
id: atelier-268e
title: 'crates/atelier-cli/src/commands/session.rs:460 — the land hook''s block reason wraps the error as ''Atelier landing failed: {error:#}. Fix the named check or hand off unfinished work honestly; ...''. Because land_records_uncommitted and check failures now end in their own ''Next: ...'', the agent hook gets two recovery steps for one failure, against the one-recovery-step rule. This change touched the line (invariant -> check) but did not change its structure. Low impact.'
target: atelier-91w1
identity: Independent reviewer (Claude subagent)
commit: b3a32046fd1d563706e20e9f7e672b687b8a11db
created_at: 2026-10-17T08:20:42.885120509Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/session.rs:460 — the land hook's block reason wraps the error as 'Atelier landing failed: {error:#}. Fix the named check or hand off unfinished work honestly; ...'. Because land_records_uncommitted and check failures now end in their own 'Next: ...', the agent hook gets two recovery steps for one failure, against the one-recovery-step rule. This change touched the line (invariant -> check) but did not change its structure. Low impact.
