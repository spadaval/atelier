---
schema: atelier.review
schema_version: 2
id: atelier-763x
title: 'crates/atelier-cli/src/commands/workflow.rs:364 — issue transition <id> still titles its page ''Issue Transitions <id> - <title>'' with State ''Status:'' and ''Type:'' lines, the old '' - '' item shape, while the option lines moved to ''start: allowed''. The doc''s view list does not name transition options, so it is not a contract miss here, but it is a view that shows one item without the item line; belongs with the issue-commands mission (atelier-uf97) or a follow-up.'
target: atelier-i2qq
identity: Independent reviewer (Claude subagent)
commit: c4b06e91043b5f0b07fce4a2cb9a0ba9e6cd2945
created_at: 2026-10-17T06:18:11.576863709Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: a86670843ad47ce6e0931848c65b94532ecdeed2
    recorded_at: 2026-10-17T06:22:39.174866652Z
    follow_up: atelier-6gpl
    reason: issue transition options view is outside the doc's view list; atelier-6gpl rewrites it one line per option.
    migrated_from: atelier-589d
---

crates/atelier-cli/src/commands/workflow.rs:364 — issue transition <id> still titles its page 'Issue Transitions <id> - <title>' with State 'Status:' and 'Type:' lines, the old ' - ' item shape, while the option lines moved to 'start: allowed'. The doc's view list does not name transition options, so it is not a contract miss here, but it is a view that shows one item without the item line; belongs with the issue-commands mission (atelier-uf97) or a follow-up.
