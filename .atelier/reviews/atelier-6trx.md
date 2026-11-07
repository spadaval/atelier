---
schema: atelier.review
schema_version: 2
id: atelier-6trx
title: crates/atelier-cli/src/commands/work.rs:235 — the work missions omitted line points to 'atelier issue list --issue-type mission', which lists done missions too and stops at --limit 50, while the omitted rows are open missions from the overview; no flag of work missions shows them all. Only reachable past 20 open missions, so low impact; could name '--category' filters or add a flag. Fits atelier-ytxg's scope.
target: atelier-ksx6
identity: Independent reviewer (Claude subagent)
commit: b14cd5e157c4e2392e7ded21a46961d63da7e3a8
created_at: 2026-10-17T08:27:59.775731765Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 28a064cf3306e0cfa9d71990c3c86a84730f6e49
    recorded_at: 2026-10-17T08:39:27.602590478Z
    follow_up: atelier-ytxg
    reason: Only past 20 open missions; atelier-ytxg covers lists without a command for the rest.
    migrated_from: atelier-e3wl
---

crates/atelier-cli/src/commands/work.rs:235 — the work missions omitted line points to 'atelier issue list --issue-type mission', which lists done missions too and stops at --limit 50, while the omitted rows are open missions from the overview; no flag of work missions shows them all. Only reachable past 20 open missions, so low impact; could name '--category' filters or add a flag. Fits atelier-ytxg's scope.
