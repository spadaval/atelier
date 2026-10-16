---
schema: atelier.review
schema_version: 1
id: atelier-tkbf
title: 'crates/atelier-cli/src/commands/migrate.rs:23-34. The migration only removes epic from the policy. It does not add work.terminal to the close transition of the --to type''s workflow, and the parent-close rule depends on that validator (the starter policy and this repo''s hand-edited .atelier/workflow.yaml have it). Repro on a /tmp copy of astromech-atelier: run ''migrate epics --apply'', then ''issue transition astromech-0474 start'' and ''issue transition astromech-0474 close --reason x''. The close succeeds with 3 open todo children (cgrw, gl7q, i0th), and check passes. This breaks the Done-when ''cannot reach a terminal status until every child is terminal'' for exactly the adopters the migration is for. The merge gate still blocks the merge, but the record says done, and the open children are then classed as retired under a terminal ancestor. docs/product/work-model.md:46-48 and ADR 0021 point 2 only mention the starter policy, so adopters are not told. Fix: have the migration add ''- work.terminal'' to the close validators of the workflow(s) that apply to --to, or at least print it as a required manual step and document it; or enforce the rule in code regardless of policy.'
target: atelier-o6h9
identity: Independent reviewer (Claude subagent)
commit: 4f66aab9fee9062a0f8cf6b3f586acc04a0cc218
created_at: 2026-10-16T00:37:30.941670794Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/migrate.rs:23-34. The migration only removes epic from the policy. It does not add work.terminal to the close transition of the --to type's workflow, and the parent-close rule depends on that validator (the starter policy and this repo's hand-edited .atelier/workflow.yaml have it). Repro on a /tmp copy of astromech-atelier: run 'migrate epics --apply', then 'issue transition astromech-0474 start' and 'issue transition astromech-0474 close --reason x'. The close succeeds with 3 open todo children (cgrw, gl7q, i0th), and check passes. This breaks the Done-when 'cannot reach a terminal status until every child is terminal' for exactly the adopters the migration is for. The merge gate still blocks the merge, but the record says done, and the open children are then classed as retired under a terminal ancestor. docs/product/work-model.md:46-48 and ADR 0021 point 2 only mention the starter policy, so adopters are not told. Fix: have the migration add '- work.terminal' to the close validators of the workflow(s) that apply to --to, or at least print it as a required manual step and document it; or enforce the rule in code regardless of policy.
