---
schema: atelier.review
schema_version: 2
id: atelier-t728
title: 'crates/atelier-app/src/mission_review.rs:155,181,247: the dates of deviations, mission events and start/close are formatted in UTC (DateTime<Utc>.format), and the view model stores them as preformatted strings. docs/product/human-cli-output.md (Commits and Timestamps) requires a bare date to be the local day, and human_output::local_date exists for this. An evening commit in -04:00 shows the next day. Fix: keep DateTime<Utc> in the view model and format with local_date in both renderers.'
target: atelier-rx22
identity: Independent reviewer (Claude subagent)
commit: 7ad42cef3afe76518fe905a2633a939ceaa108f4
created_at: 2026-10-17T19:32:17.539390097Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-implementer
    commit: 7cc6d1f15d408b6f96d60cb84bc60a3e9e5bea8a
    recorded_at: 2026-10-17T19:39:11.624012136Z
    resolution_commit: 7cc6d1f15d408b6f96d60cb84bc60a3e9e5bea8a
    reason: Fixed in 7cc6d1f15d408b6f96d60cb84bc60a3e9e5bea8a with tests; see the commit message
    migrated_from: atelier-6012
---

crates/atelier-app/src/mission_review.rs:155,181,247: the dates of deviations, mission events and start/close are formatted in UTC (DateTime<Utc>.format), and the view model stores them as preformatted strings. docs/product/human-cli-output.md (Commits and Timestamps) requires a bare date to be the local day, and human_output::local_date exists for this. An evening commit in -04:00 shows the next day. Fix: keep DateTime<Utc> in the view model and format with local_date in both renderers.
