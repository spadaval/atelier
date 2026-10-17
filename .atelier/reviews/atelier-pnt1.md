---
schema: atelier.review
schema_version: 1
id: atelier-pnt1
title: 'crates/atelier-cli/tests/session.rs:2079 — the (1, true) case of terminal_review_cluster_prune_remains_a_valid_handoff now depends on the runner''s local DST: it dates the cluster Utc::now() - 24h and expects prune --retention-days 0 to retire it, but with the cutoff on the local day, on a 25-hour fall-back day (e.g. America/New_York 2026-11-01 between 23:00 and 24:00 EST) now-24h falls on the same local date, local_date(latest) < cutoff is false, the issue is kept and assert!(!issue_path.exists()) fails. Under the old UTC days this could not happen. Test-only, one hour a year in DST zones; using days_ago 2 removes it.'
target: atelier-7iyw
identity: Independent reviewer (Claude subagent)
commit: e65c1fb0c327d2e93144387b0e81d0e298acdd47
created_at: 2026-10-17T07:03:36.975637792Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/tests/session.rs:2079 — the (1, true) case of terminal_review_cluster_prune_remains_a_valid_handoff now depends on the runner's local DST: it dates the cluster Utc::now() - 24h and expects prune --retention-days 0 to retire it, but with the cutoff on the local day, on a 25-hour fall-back day (e.g. America/New_York 2026-11-01 between 23:00 and 24:00 EST) now-24h falls on the same local date, local_date(latest) < cutoff is false, the issue is kept and assert!(!issue_path.exists()) fails. Under the old UTC days this could not happen. Test-only, one hour a year in DST zones; using days_ago 2 removes it.
