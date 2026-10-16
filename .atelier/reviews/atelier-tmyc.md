---
schema: atelier.review
schema_version: 1
id: atelier-tmyc
title: 'crates/atelier-cli/src/commands/bundle.rs:399 / :555: two update entries with different client_refs naming the same id (or a full id and its suffix) are accepted and applied in sequence, last writer wins. Reproduced: x1 {id:rv4n7v-bdyd,title:''From x1'',labels:[''one'']} and x2 {id:''bdyd'',title:''From x2'',priority:high}. The result has title ''From x2'' and x1''s label is silently lost, because x2''s omitted fields were filled from the pre-bundle record. Output still lists both ''x1 -> ... (updated)'' and ''x2 -> ... (updated)''. Fix: after fill_update_from_record resolves ids, reject duplicate update ids in validate_bundle (''Duplicate update of <id>: x1, x2''), and test it.'
target: atelier-4n7v
identity: Independent reviewer (Claude subagent)
commit: a9234343ba772ab7e86fa682752f752f73f4af7e
created_at: 2026-10-16T00:40:13.323878566Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/bundle.rs:399 / :555: two update entries with different client_refs naming the same id (or a full id and its suffix) are accepted and applied in sequence, last writer wins. Reproduced: x1 {id:rv4n7v-bdyd,title:'From x1',labels:['one']} and x2 {id:'bdyd',title:'From x2',priority:high}. The result has title 'From x2' and x1's label is silently lost, because x2's omitted fields were filled from the pre-bundle record. Output still lists both 'x1 -> ... (updated)' and 'x2 -> ... (updated)'. Fix: after fill_update_from_record resolves ids, reject duplicate update ids in validate_bundle ('Duplicate update of <id>: x1, x2'), and test it.
