---
schema: atelier.review
schema_version: 2
id: atelier-1cgn
title: 'crates/atelier-cli/src/commands/status.rs:487 and :500 — pre-existing (on master): status Next labels still carry counts in parentheses, ''Choose ready work (9 ready issues available)'' and ''Only work for the human is ready (1 item)'', and the first repeats the ''Ready work: 9'' summary line. Not a heading or group line, so outside this item''s Done when; worth a follow-up under atelier-o3lm against the Counts rule.'
target: atelier-u3vj
identity: Independent reviewer (Claude subagent)
commit: b8288f8555508d70806ebecd8ed5197ffe4f426c
created_at: 2026-10-17T09:45:14.638181633Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 06739335351f0973408dda1e032298644e735650
    recorded_at: 2026-10-17T09:48:19.605354837Z
    resolution_commit: 371ecf05393267ecfacc354cf622589980100aa7
    reason: 'Fixed now: status footer labels carry no counts.'
    migrated_from: atelier-g8vx
---

crates/atelier-cli/src/commands/status.rs:487 and :500 — pre-existing (on master): status Next labels still carry counts in parentheses, 'Choose ready work (9 ready issues available)' and 'Only work for the human is ready (1 item)', and the first repeats the 'Ready work: 9' summary line. Not a heading or group line, so outside this item's Done when; worth a follow-up under atelier-o3lm against the Counts rule.
