---
schema: atelier.review
schema_version: 1
id: atelier-ayr9
title: 'crates/atelier-cli/src/commands/bundle.rs:49 — bundle apply -q prints only summary.records issues/updated/evidence IDs, but apply also writes existing records it does not list: a parent relationship to an existing ID calls add_issue_child on that parent (bundle.rs:1416), and blocks/advances declared by an existing target write the target. The Quiet Tokens row says ''the IDs of the records it created or updated'', so a script that stages or reports the quiet IDs misses the modified existing parent (e.g. a bundle adding children under an existing mission/parent, as the fixture''s parent_scope suggests). Either list touched existing records or narrow the doc row to ''created, or updated by operation: update''.'
target: atelier-pr4k
identity: Independent reviewer (Claude subagent)
commit: db348b9eb46038270d76680937219362478b5526
created_at: 2026-10-17T08:28:19.609352847Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/bundle.rs:49 — bundle apply -q prints only summary.records issues/updated/evidence IDs, but apply also writes existing records it does not list: a parent relationship to an existing ID calls add_issue_child on that parent (bundle.rs:1416), and blocks/advances declared by an existing target write the target. The Quiet Tokens row says 'the IDs of the records it created or updated', so a script that stages or reports the quiet IDs misses the modified existing parent (e.g. a bundle adding children under an existing mission/parent, as the fixture's parent_scope suggests). Either list touched existing records or narrow the doc row to 'created, or updated by operation: update'.
