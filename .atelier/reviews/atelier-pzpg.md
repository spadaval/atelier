---
schema: atelier.review
schema_version: 1
id: atelier-pzpg
title: 'crates/atelier-cli/src/commands/bundle.rs fill_update_from_record parent check and validate_bundle_hierarchy: three low-impact parent-ref gaps remain. All fail closed, and none damages data. (1) An update that gives a parent to a record that already has children passes preview (the is_parent check only looks at bundle entries) but apply fails at the staged index: ''workflow_issue_hierarchy_invalid: ... has children of its own''. Fix: also check index.child_ids(id) for update entries. (2) Restating the current parent as {client_ref} of an update entry for that parent is refused with ''cannot move'', because only parent.id is compared. Fix: resolve a client_ref that names an update entry to its id before comparing. (3) This predates the change: a parent ref given as an id suffix ({id:''bmzh''}) fails with ''Issue parent for a must resolve to an issue'', because bundle_issue_type_for_ref uses index.get_issue without resolve_issue_ref, while validate_ref_exists accepts suffixes. Fix: resolve the ref first. Reproduced in /tmp/rv4n7v2.'
target: atelier-4n7v
identity: Independent reviewer (Claude subagent)
commit: 12cd9e849b50fda58f3ab0887c995a6e81a33999
created_at: 2026-10-16T00:45:10.144573661Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/bundle.rs fill_update_from_record parent check and validate_bundle_hierarchy: three low-impact parent-ref gaps remain. All fail closed, and none damages data. (1) An update that gives a parent to a record that already has children passes preview (the is_parent check only looks at bundle entries) but apply fails at the staged index: 'workflow_issue_hierarchy_invalid: ... has children of its own'. Fix: also check index.child_ids(id) for update entries. (2) Restating the current parent as {client_ref} of an update entry for that parent is refused with 'cannot move', because only parent.id is compared. Fix: resolve a client_ref that names an update entry to its id before comparing. (3) This predates the change: a parent ref given as an id suffix ({id:'bmzh'}) fails with 'Issue parent for a must resolve to an issue', because bundle_issue_type_for_ref uses index.get_issue without resolve_issue_ref, while validate_ref_exists accepts suffixes. Fix: resolve the ref first. Reproduced in /tmp/rv4n7v2.
