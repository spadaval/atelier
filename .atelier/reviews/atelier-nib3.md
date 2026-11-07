---
schema: atelier.review
schema_version: 2
id: atelier-nib3
title: 'crates/atelier-records/src/lib.rs:1130-1137: the single-pass visitor matches the old parse for every accepted input I probed (i64/u64, hex/octal, floats, .nan/.inf -> null, 1e400 -> string, nulls, !!str/!!int/!!float/!!bool core tags, anchors/aliases, ''<<'', non-string scalar keys, empty/comment-only front matter, non-mapping errors). Three small differences remain, none reachable from atelier-written records: (1) a local tag on the top-level mapping (''!tag'' then ''a: 1'') was accepted on master and by the two-pass version and is now rejected with ''invalid type: enum'', because serde_yaml calls visit_enum for deserialize_any but ignores the tag for deserialize_map; (2) messages for errors the plain parse used to word changed: nested local tags now say ''expected YAML without repeated mapping keys'' instead of ''expected any valid JSON value'', and a non-string key in a nested map says ''expected a string'' instead of ''expected a string key''; the expecting() text reads oddly for a type error (u128 overflow already said this in the two-pass version); (3) duplicates no longer take precedence over type errors: the first problem in document order is reported (e.g. a tag before a repeated key now reports the tag). Fix, if wanted: fall back to the plain parse when the visitor fails with a non-duplicate error, or reword expecting() to ''any valid JSON value''.'
target: atelier-tqru
identity: Independent reviewer (Claude subagent)
commit: f1704ae119e304c37463ac237e2ef3dcef1fab91
created_at: 2026-10-16T22:18:43.344324839Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: no_action
    identity: Claude Opus 5.5 (orchestrator)
    commit: 627d164a18989c65d02119840ad2968d249a326a
    recorded_at: 2026-10-16T22:19:33.658939300Z
    reason: 'Accepted: the differences only affect hand-written YAML Atelier never produces, and each one refuses input rather than accepting it.'
    migrated_from: atelier-n1ta
---

crates/atelier-records/src/lib.rs:1130-1137: the single-pass visitor matches the old parse for every accepted input I probed (i64/u64, hex/octal, floats, .nan/.inf -> null, 1e400 -> string, nulls, !!str/!!int/!!float/!!bool core tags, anchors/aliases, '<<', non-string scalar keys, empty/comment-only front matter, non-mapping errors). Three small differences remain, none reachable from atelier-written records: (1) a local tag on the top-level mapping ('!tag' then 'a: 1') was accepted on master and by the two-pass version and is now rejected with 'invalid type: enum', because serde_yaml calls visit_enum for deserialize_any but ignores the tag for deserialize_map; (2) messages for errors the plain parse used to word changed: nested local tags now say 'expected YAML without repeated mapping keys' instead of 'expected any valid JSON value', and a non-string key in a nested map says 'expected a string' instead of 'expected a string key'; the expecting() text reads oddly for a type error (u128 overflow already said this in the two-pass version); (3) duplicates no longer take precedence over type errors: the first problem in document order is reported (e.g. a tag before a repeated key now reports the tag). Fix, if wanted: fall back to the plain parse when the visitor fails with a non-duplicate error, or reword expecting() to 'any valid JSON value'.
