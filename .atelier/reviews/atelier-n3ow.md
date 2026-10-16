---
schema: atelier.review
schema_version: 1
id: atelier-n3ow
title: 'crates/atelier-cli/src/commands/bundle.rs:394-408 (validate_bundle_hierarchy). Before, depth was implicit: only epics could own children, and epics had no parent. The validator now checks only for mission parents, so ''bundle preview'' accepts nested chains. Examples: a parent of b parent of c inside one bundle, or a new issue whose parent is an existing child (a parent of {id: <child>}). Preview shows them as valid relationships, and only ''bundle apply --yes'' rejects them at staged-state validation (''parents are one level deep''). Nothing is corrupted, but preview is no longer a faithful dry run of the hierarchy rules. Fix: in the preview validator, reject a bundle parent that itself has a parent (in the bundle or existing), and a bundle child that has children of its own. Add a fixture or test.'
target: atelier-o6h9
identity: Independent reviewer (Claude subagent)
commit: 4f66aab9fee9062a0f8cf6b3f586acc04a0cc218
created_at: 2026-10-16T00:37:31.128950295Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/bundle.rs:394-408 (validate_bundle_hierarchy). Before, depth was implicit: only epics could own children, and epics had no parent. The validator now checks only for mission parents, so 'bundle preview' accepts nested chains. Examples: a parent of b parent of c inside one bundle, or a new issue whose parent is an existing child (a parent of {id: <child>}). Preview shows them as valid relationships, and only 'bundle apply --yes' rejects them at staged-state validation ('parents are one level deep'). Nothing is corrupted, but preview is no longer a faithful dry run of the hierarchy rules. Fix: in the preview validator, reject a bundle parent that itself has a parent (in the bundle or existing), and a bundle child that has children of its own. Add a fixture or test.
