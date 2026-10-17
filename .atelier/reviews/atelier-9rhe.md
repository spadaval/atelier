---
schema: atelier.review
schema_version: 1
id: atelier-9rhe
title: 'crates/atelier-app/src/base_policy.rs:55-75 — when the candidate workflow policy is invalid, land and merge --check print the same workflow_config_parse_error field block three times: the ''Invalid working policy:'' and ''Invalid policy at <commit>:'' diagnostics, then the error''s Caused by chain. The error now carries one code and one Next step, so this item''s rule holds. The two diagnostics come from base-policy fallback reporting that predates the change and that the change did not make worse. The repetition makes the actual error harder to find. Merge gate wording owns it (atelier-6rh7, or atelier-467y): print the parse block once and keep ''Using last valid policy from <commit>'' as one line.'
target: atelier-91w1
identity: Independent reviewer (Claude subagent)
commit: d61e7569d1f351ff0f6c379250395891bf06893b
created_at: 2026-10-17T08:32:10.943570476Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/base_policy.rs:55-75 — when the candidate workflow policy is invalid, land and merge --check print the same workflow_config_parse_error field block three times: the 'Invalid working policy:' and 'Invalid policy at <commit>:' diagnostics, then the error's Caused by chain. The error now carries one code and one Next step, so this item's rule holds. The two diagnostics come from base-policy fallback reporting that predates the change and that the change did not make worse. The repetition makes the actual error harder to find. Merge gate wording owns it (atelier-6rh7, or atelier-467y): print the parse block once and keep 'Using last valid policy from <commit>' as one line.
