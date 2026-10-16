---
schema: atelier.review
schema_version: 1
id: atelier-apxi
title: 'crates/atelier-cli/src/commands/bundle.rs:560-564 (validate_bundle_hierarchy parent_is_child): when the parent ref is the client_ref of an update entry, the check looks only at that entry''s bundle parent field, not at the existing record''s parent. Reproduced in /tmp/rv4n7v2: b {update gupb, parent:{client_ref:''a''}} with a {update 0zub} (0zub is already a child of bmzh). Preview passes and lists ''issue/a -> issue/b (parent)''. Apply fails closed at the staged index: ''workflow_issue_hierarchy_invalid: issue rv4n7v2-0zub is a child of rv4n7v2-bmzh and has children of its own''. Nothing changes. The same case written as {id:''0zub''} is caught in preview. Fix: in the client_ref branch, also treat an update entry whose resolved id already has a parent_id as a child.'
target: atelier-4n7v
identity: Independent reviewer (Claude subagent)
commit: a14de2e608c00dd108b1f4f397432cfb0ac441c1
created_at: 2026-10-16T00:47:52.858114465Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/bundle.rs:560-564 (validate_bundle_hierarchy parent_is_child): when the parent ref is the client_ref of an update entry, the check looks only at that entry's bundle parent field, not at the existing record's parent. Reproduced in /tmp/rv4n7v2: b {update gupb, parent:{client_ref:'a'}} with a {update 0zub} (0zub is already a child of bmzh). Preview passes and lists 'issue/a -> issue/b (parent)'. Apply fails closed at the staged index: 'workflow_issue_hierarchy_invalid: issue rv4n7v2-0zub is a child of rv4n7v2-bmzh and has children of its own'. Nothing changes. The same case written as {id:'0zub'} is caught in preview. Fix: in the client_ref branch, also treat an update entry whose resolved id already has a parent_id as a child.
