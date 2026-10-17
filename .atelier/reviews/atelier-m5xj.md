---
schema: atelier.review
schema_version: 1
id: atelier-m5xj
title: 'The link closure now covers every link the claim checks read: blocks, parent, children, siblings for inherited blockers, advances, review targets including parent approvals, and evidence attachments. The one remaining input it cannot cover is lint.none_blocking. That validator is allowed in transition validators and lints every issue record (crate::lint::issue_findings over index.issue_records()), not the claim''s links, so any issue edit can fail every claim whose terminal transition uses it. Reproduced in a scratch repo by adding lint.none_blocking to the work close validators, on the base with an upstream: an old done task, then a commit giving an unrelated issue two size labels. land passes, and land itself reports no lint failure; doctor reports ''<old> lint.none_blocking: atelier check failed with 1 finding(s)''. Fix: when any issue record changed since the trusted point and a terminal transition''s validators include lint.none_blocking (or another validator that reads every record), check every claim of those types; or run lint once and fail land on blocking findings. Add a test. shell.check is limited to merge_checks, and the remaining validators read only the claim''s own record or its evidence, so nothing else is missing.'
target: atelier-37uq
identity: Independent reviewer (Claude subagent)
commit: f9985f084d3c729ca1ac568cf3ee603aea5ccf68
created_at: 2026-10-16T23:49:03.327953971Z
event:
  event: finding
  recommendation: fix_now
---

The link closure now covers every link the claim checks read: blocks, parent, children, siblings for inherited blockers, advances, review targets including parent approvals, and evidence attachments. The one remaining input it cannot cover is lint.none_blocking. That validator is allowed in transition validators and lints every issue record (crate::lint::issue_findings over index.issue_records()), not the claim's links, so any issue edit can fail every claim whose terminal transition uses it. Reproduced in a scratch repo by adding lint.none_blocking to the work close validators, on the base with an upstream: an old done task, then a commit giving an unrelated issue two size labels. land passes, and land itself reports no lint failure; doctor reports '<old> lint.none_blocking: atelier check failed with 1 finding(s)'. Fix: when any issue record changed since the trusted point and a terminal transition's validators include lint.none_blocking (or another validator that reads every record), check every claim of those types; or run lint once and fail land on blocking findings. Add a test. shell.check is limited to merge_checks, and the remaining validators read only the claim's own record or its evidence, so nothing else is missing.
