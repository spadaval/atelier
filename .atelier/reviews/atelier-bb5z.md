---
schema: atelier.review
schema_version: 1
id: atelier-bb5z
title: 'Resolved: a mapping is parsed once; the visitor builds the serde_json::Value tree and the second parse runs only for non-mapping front matter, which is an error anyway. On the kept 1,000-issue bench fixture, 60 interleaved runs of ''atelier check'' (release): master 88e8964a median 49.7 ms (min 41.6), two-pass da89a657 71.9 ms (min 56.6), this branch 50.7 ms (min 40.6). Back to master speed.'
target: atelier-tqru
identity: Independent reviewer (Claude subagent)
commit: f1704ae119e304c37463ac237e2ef3dcef1fab91
created_at: 2026-10-16T22:18:43.312760199Z
event:
  event: disposition
  finding: atelier-n6zq
  decision: fix_now
  supersedes: null
  resolution_commit: 4ff56376f14ac333b3df565a079cc17605cec399
  follow_up: null
---

Resolved: a mapping is parsed once; the visitor builds the serde_json::Value tree and the second parse runs only for non-mapping front matter, which is an error anyway. On the kept 1,000-issue bench fixture, 60 interleaved runs of 'atelier check' (release): master 88e8964a median 49.7 ms (min 41.6), two-pass da89a657 71.9 ms (min 56.6), this branch 50.7 ms (min 40.6). Back to master speed.
