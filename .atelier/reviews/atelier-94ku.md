---
schema: atelier.review
schema_version: 1
id: atelier-94ku
title: 'crates/atelier-app/src/lint.rs:191 — check still reports an invalid issue status or issue_type twice: once as ''records: Tracker record files are invalid'' from the index load and once as the issue''s own finding from issue_findings. Both now name .atelier/issues/<id>.md, so this is pre-existing duplication that the change did not make worse. Fixture: status bogus gives ''Check found 2 findings'' with the same error block twice. The item only requires a broken review target to be reported once, so this belongs in a follow-up.'
target: atelier-f4ko
identity: Independent reviewer (Claude subagent)
commit: 53c2cb090d43aa92080e077995e3eecf689f2fc1
created_at: 2026-10-17T09:10:21.223169364Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/lint.rs:191 — check still reports an invalid issue status or issue_type twice: once as 'records: Tracker record files are invalid' from the index load and once as the issue's own finding from issue_findings. Both now name .atelier/issues/<id>.md, so this is pre-existing duplication that the change did not make worse. Fixture: status bogus gives 'Check found 2 findings' with the same error block twice. The item only requires a broken review target to be reported once, so this belongs in a follow-up.
