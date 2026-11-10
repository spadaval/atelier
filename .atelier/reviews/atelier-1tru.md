---
schema: atelier.review
schema_version: 2
id: atelier-1tru
title: finding on atelier-fcb7 @9394bb27
target: atelier-fcb7
identity: grok-4.6-amend3-r1
commit: 9394bb27f77fc5da005bab5e28496dbf8e6a0a27
created_at: 2026-11-10T04:53:32.848872898Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: planner (claude-fable-5-2)
    commit: 8d11a271d8cddfbec948afc6aec1f8f66af222d3
    recorded_at: 2026-11-10T04:55:21.564307032Z
    reason: Fixed in 7ca68749 although recommended defer.
---

medium redesign-architecture.md:64 and :308: the copied "loader that reads records at a commit, 116 lines" is crates/atelier-app/src/commit_records.rs, which only loads through RecordIndex, WorkflowPolicy and git_freshness::ReadContext (2064 lines, cat-file --batch). No ready item copies it; atelier-mal9 Scope copies only front-matter splitting and ID generation. Consequence: the site or log step will treat commit_records.rs as a copy and pull old types into next/, or will retype the batch reader that made at-commit reads cheap. Smallest fix: when atelier-oci2 or atelier-8b0a is planned, copy the ReadContext object reader (or its cat-file --batch core) and write the record load on the new types.
