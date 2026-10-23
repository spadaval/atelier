---
schema: atelier.review
schema_version: 1
id: atelier-s8kw
title: finding on atelier-h1nc @60f6746f
target: atelier-h1nc
identity: Independent reviewer 3 (Claude Sonnet subagent)
commit: 60f6746fda2847823f0253bb6cf33535cbaef11d
created_at: 2026-10-23T15:16:57.692922299Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/gate_report.rs:253 reopen_command is unreachable: the policy validator rejects any transition leaving a done_statuses status (workflow_config_invalid_transition: cannot leave terminal status), so a closed ancestor never has a reopen transition, and the 'or, if the work is still needed, reopen' branch plus the docs/product/human-cli-output.md:312 sentence can never print. Untested; and if it did print, the test helper's ', or ' split would not split ', or, if' and would run the reopen as a further step of the first alternative. Suggested fix: delete reopen_command and the doc clause (or test it if some policy can express it).
