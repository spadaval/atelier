---
schema: atelier.review
schema_version: 1
id: atelier-xnph
title: 'crates/atelier-cli/tests/cli_integration/output_terms.rs:16-48,95-140 — the Close Reason says this test enforces the rule, but it cannot see the failure in the land_policy_invalid finding. codes() counts codes per line, so a code stacked through anyhow''s ''Caused by:'' chain on the next line passes. The fixture renders no merge refusal, no policy error and only 3 of about 60 help pages. words() keeps ''-'' inside a word, so ''non-canonical'' or ''Canonical-foo'' escapes. The suite passes while two codes ship on land and merge. Fix: count codes across the whole stderr of each error, and render at least the merge refusals and the policy-invalid path. A help sweep is optional; a manual sweep of every help page found no ''lint'' or ''canonical'' today.'
target: atelier-91w1
identity: Independent reviewer (Claude subagent)
commit: b3a32046fd1d563706e20e9f7e672b687b8a11db
created_at: 2026-10-17T08:20:42.580368067Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/tests/cli_integration/output_terms.rs:16-48,95-140 — the Close Reason says this test enforces the rule, but it cannot see the failure in the land_policy_invalid finding. codes() counts codes per line, so a code stacked through anyhow's 'Caused by:' chain on the next line passes. The fixture renders no merge refusal, no policy error and only 3 of about 60 help pages. words() keeps '-' inside a word, so 'non-canonical' or 'Canonical-foo' escapes. The suite passes while two codes ship on land and merge. Fix: count codes across the whole stderr of each error, and render at least the merge refusals and the policy-invalid path. A help sweep is optional; a manual sweep of every help page found no 'lint' or 'canonical' today.
