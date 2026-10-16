---
schema: atelier.review
schema_version: 1
id: atelier-n6zq
title: 'crates/atelier-records/src/lib.rs:1130-1131: every issue and evidence read now parses its YAML twice. A microbenchmark of the copied visitor takes about 27 us per record per pass, the same as the real parse. On the kept 1,000-issue/200-evidence bench fixture, interleaved best-of-15 runs went from 49 to 73 ms for ''atelier check'' and from 55 to 93 ms for ''atelier issue list'' (master ab437340 vs branch). Medians were noisy but moved the same way. This adds roughly 30-40 ms to every whole-tracker read. Fix: one pass, using a DeserializeSeed or visitor that builds the serde_json::Value tree while checking String keys, which also fixes the key-type gap above. Alternatively, parse once into serde_yaml::Value (which already rejects duplicates in 0.9) and convert it.'
target: atelier-tqru
identity: Independent reviewer (Claude subagent)
commit: cabfe6b186ffc8b5d08c83295090f341843a5334
created_at: 2026-10-16T22:10:37.642636099Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-records/src/lib.rs:1130-1131: every issue and evidence read now parses its YAML twice. A microbenchmark of the copied visitor takes about 27 us per record per pass, the same as the real parse. On the kept 1,000-issue/200-evidence bench fixture, interleaved best-of-15 runs went from 49 to 73 ms for 'atelier check' and from 55 to 93 ms for 'atelier issue list' (master ab437340 vs branch). Medians were noisy but moved the same way. This adds roughly 30-40 ms to every whole-tracker read. Fix: one pass, using a DeserializeSeed or visitor that builds the serde_json::Value tree while checking String keys, which also fixes the key-type gap above. Alternatively, parse once into serde_yaml::Value (which already rejects duplicates in 0.9) and convert it.
