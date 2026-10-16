---
schema: atelier.review
schema_version: 1
id: atelier-ygnm
title: 'Resolved: keys are now read with next_key::<String>(), the same coercion BTreeMap<String, Value> and serde_json''s KeyClassifier apply, and checked against the map being built. Probed against serde_yaml 0.9.34: 1 vs "1", true vs ''true'', ~ vs ''~'', 1.0 vs ''1.0'' (top-level and nested) are now rejected as duplicates, and ~ vs null or .nan vs .inf are accepted as the distinct keys they become. The typed test case fails without the fix.'
target: atelier-tqru
identity: Independent reviewer (Claude subagent)
commit: f1704ae119e304c37463ac237e2ef3dcef1fab91
created_at: 2026-10-16T22:18:43.254950011Z
event:
  event: disposition
  finding: atelier-s3fo
  decision: fix_now
  supersedes: null
  resolution_commit: 4ff56376f14ac333b3df565a079cc17605cec399
  follow_up: null
---

Resolved: keys are now read with next_key::<String>(), the same coercion BTreeMap<String, Value> and serde_json's KeyClassifier apply, and checked against the map being built. Probed against serde_yaml 0.9.34: 1 vs "1", true vs 'true', ~ vs '~', 1.0 vs '1.0' (top-level and nested) are now rejected as duplicates, and ~ vs null or .nan vs .inf are accepted as the distinct keys they become. The typed test case fails without the fix.
