---
schema: atelier.review
schema_version: 1
id: atelier-rczt
title: 'Resolved: structural now records (kind, id) for each dropped duplicate, using the literal kinds issue/evidence/review, which match ISSUE_KIND/EVIDENCE_KIND/REVIEW_KIND and the kind strings ensure_issue (''missing issue X'') and ensure_record (''missing {kind} X'') emit. The filter takes text after the last '' reference to missing '' and splits once on a space into kind and ID; IDs are validated by validate_record_id (lowercase ASCII, digits, hyphens) on load, so they cannot contain spaces. Reverting the filter to the kind-less match makes the extended test fail (the missing issue atelier-twin reference is hidden); with the fix it passes, and cargo nextest run --workspace passes 632/632.'
target: atelier-ip9e
identity: Independent reviewer (Claude subagent)
commit: cf5d6d5e9f0117d2463ecde74afe5d99ab6d5162
created_at: 2026-10-16T19:25:28.911514009Z
event:
  event: disposition
  finding: atelier-zvuy
  decision: fix_now
  supersedes: null
  resolution_commit: 713ee09cc38535d3ba0fc6701995252aea49f4ae
  follow_up: null
---

Resolved: structural now records (kind, id) for each dropped duplicate, using the literal kinds issue/evidence/review, which match ISSUE_KIND/EVIDENCE_KIND/REVIEW_KIND and the kind strings ensure_issue ('missing issue X') and ensure_record ('missing {kind} X') emit. The filter takes text after the last ' reference to missing ' and splits once on a space into kind and ID; IDs are validated by validate_record_id (lowercase ASCII, digits, hyphens) on load, so they cannot contain spaces. Reverting the filter to the kind-less match makes the extended test fail (the missing issue atelier-twin reference is hidden); with the fix it passes, and cargo nextest run --workspace passes 632/632.
