---
schema: atelier.review
schema_version: 2
id: atelier-94ku
title: 'crates/atelier-app/src/lint.rs:191 — check still reports an invalid issue status or issue_type twice: once as ''records: Tracker record files are invalid'' from the index load and once as the issue''s own finding from issue_findings. Both now name .atelier/issues/<id>.md, so this is pre-existing duplication that the change did not make worse. Fixture: status bogus gives ''Check found 2 findings'' with the same error block twice. The item only requires a broken review target to be reported once, so this belongs in a follow-up.'
target: atelier-f4ko
identity: Independent reviewer (Claude subagent)
commit: 53c2cb090d43aa92080e077995e3eecf689f2fc1
created_at: 2026-10-17T09:10:21.223169364Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 6bed161ba53b9b851e87ff94006f3644c1a4f553
    recorded_at: 2026-10-17T09:14:28.376150595Z
    follow_up: atelier-ry7d
    reason: Pre-existing double report; filed atelier-ry7d.
    migrated_from: atelier-qvua
---

crates/atelier-app/src/lint.rs:191 — check still reports an invalid issue status or issue_type twice: once as 'records: Tracker record files are invalid' from the index load and once as the issue's own finding from issue_findings. Both now name .atelier/issues/<id>.md, so this is pre-existing duplication that the change did not make worse. Fixture: status bogus gives 'Check found 2 findings' with the same error block twice. The item only requires a broken review target to be reported once, so this belongs in a follow-up.
