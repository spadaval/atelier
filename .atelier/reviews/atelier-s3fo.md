---
schema: atelier.review
schema_version: 2
id: atelier-s3fo
title: 'crates/atelier-records/src/lib.rs:1155: UniqueKeysVisitor compares keys as serde_json::Value, but the real parse (lib.rs:1131, BTreeMap<String, Value> and nested serde_json maps) coerces keys to strings. Keys that differ only by YAML type therefore still collapse silently: front matter ''relates: [{1: a, "1": b}]'' or top-level ''true: a'' plus ''"true": b'' passes the check and keeps only the last value (probed against serde_yaml 0.9.34). The reverse also happens: ''~'' vs ''null'' and ''.nan'' vs ''.inf'' are rejected as "duplicate key ''Null''" although they map to distinct string keys. Not reachable from atelier-written records (keys are always plain words), so deferring is reasonable. Fix: read keys with map.next_key::<String>() and keep them in a BTreeSet, which matches the target type exactly and gives a readable key name.'
target: atelier-tqru
identity: Independent reviewer (Claude subagent)
commit: cabfe6b186ffc8b5d08c83295090f341843a5334
created_at: 2026-10-16T22:10:37.575938595Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: f1704ae119e304c37463ac237e2ef3dcef1fab91
    recorded_at: 2026-10-16T22:18:43.254950011Z
    resolution_commit: 4ff56376f14ac333b3df565a079cc17605cec399
    reason: 'Resolved: keys are now read with next_key::<String>(), the same coercion BTreeMap<String, Value> and serde_json''s KeyClassifier apply, and checked against the map being built. Probed against serde_yaml 0.9.34: 1 vs "1", true vs ''true'', ~ vs ''~'', 1.0 vs ''1.0'' (top-level and nested) are now rejected as duplicates, and ~ vs null or .nan vs .inf are accepted as the distinct keys they become. The typed test case fails without the fix.'
    migrated_from: atelier-ygnm
---

crates/atelier-records/src/lib.rs:1155: UniqueKeysVisitor compares keys as serde_json::Value, but the real parse (lib.rs:1131, BTreeMap<String, Value> and nested serde_json maps) coerces keys to strings. Keys that differ only by YAML type therefore still collapse silently: front matter 'relates: [{1: a, "1": b}]' or top-level 'true: a' plus '"true": b' passes the check and keeps only the last value (probed against serde_yaml 0.9.34). The reverse also happens: '~' vs 'null' and '.nan' vs '.inf' are rejected as "duplicate key 'Null'" although they map to distinct string keys. Not reachable from atelier-written records (keys are always plain words), so deferring is reasonable. Fix: read keys with map.next_key::<String>() and keep them in a BTreeSet, which matches the target type exactly and gives a readable key name.
