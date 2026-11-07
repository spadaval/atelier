---
schema: atelier.review
schema_version: 2
id: atelier-n6zq
title: 'crates/atelier-records/src/lib.rs:1130-1131: every issue and evidence read now parses its YAML twice. A microbenchmark of the copied visitor takes about 27 us per record per pass, the same as the real parse. On the kept 1,000-issue/200-evidence bench fixture, interleaved best-of-15 runs went from 49 to 73 ms for ''atelier check'' and from 55 to 93 ms for ''atelier issue list'' (master ab437340 vs branch). Medians were noisy but moved the same way. This adds roughly 30-40 ms to every whole-tracker read. Fix: one pass, using a DeserializeSeed or visitor that builds the serde_json::Value tree while checking String keys, which also fixes the key-type gap above. Alternatively, parse once into serde_yaml::Value (which already rejects duplicates in 0.9) and convert it.'
target: atelier-tqru
identity: Independent reviewer (Claude subagent)
commit: cabfe6b186ffc8b5d08c83295090f341843a5334
created_at: 2026-10-16T22:10:37.642636099Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: f1704ae119e304c37463ac237e2ef3dcef1fab91
    recorded_at: 2026-10-16T22:18:43.312760199Z
    resolution_commit: 4ff56376f14ac333b3df565a079cc17605cec399
    reason: 'Resolved: a mapping is parsed once; the visitor builds the serde_json::Value tree and the second parse runs only for non-mapping front matter, which is an error anyway. On the kept 1,000-issue bench fixture, 60 interleaved runs of ''atelier check'' (release): master 88e8964a median 49.7 ms (min 41.6), two-pass da89a657 71.9 ms (min 56.6), this branch 50.7 ms (min 40.6). Back to master speed.'
    migrated_from: atelier-bb5z
---

crates/atelier-records/src/lib.rs:1130-1131: every issue and evidence read now parses its YAML twice. A microbenchmark of the copied visitor takes about 27 us per record per pass, the same as the real parse. On the kept 1,000-issue/200-evidence bench fixture, interleaved best-of-15 runs went from 49 to 73 ms for 'atelier check' and from 55 to 93 ms for 'atelier issue list' (master ab437340 vs branch). Medians were noisy but moved the same way. This adds roughly 30-40 ms to every whole-tracker read. Fix: one pass, using a DeserializeSeed or visitor that builds the serde_json::Value tree while checking String keys, which also fixes the key-type gap above. Alternatively, parse once into serde_yaml::Value (which already rejects duplicates in 0.9) and convert it.
