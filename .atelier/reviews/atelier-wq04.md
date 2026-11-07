---
schema: atelier.review
schema_version: 2
id: atelier-wq04
title: finding on atelier-s66u @9603ad9e
target: atelier-s66u
identity: lighter-workflow reviewer for atelier-s66u (claude-opus-5-5)
commit: 9603ad9e630db3d0da6f7018d9abe5d8b3576b71
created_at: 2026-11-07T00:56:44.782151348Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: atelier-n4mj orchestrator (claude-opus-5-5)
    commit: 9ab8da5a716e87aa5497a10c7e82d52a497b5e01
    recorded_at: 2026-11-07T01:07:48.709506922Z
    reason: Any later pass clearing a failure, whatever its command or pin, is the rule the strategy chose (Shape, evidence bullet; Simplicity wins); evidence list shows which pass superseded the fail, which meets Show, don't enforce.
    migrated_from: atelier-op4x
---

low: crates/atelier-app/src/evidence_freshness.rs:455-479 any pass captured after a fail now clears it, whatever its command, kind or pin, including a pin not in HEAD's history (the old non-fresh path required an ancestor pin). A different check (a fmt pass clearing a failing test) could already clear a fail before this branch if fresh, so the new loss is only freshness and ancestry. Acceptable under the strategy: Shape removes 'the rule that a pass stops clearing an older failure once later commits make it stale', 'Simplicity wins' prefers no new condition, and 'Show, don't enforce' is met because evidence list marks the fail 'superseded by newer pass <id>' and that pass row shows its command for the reviewer. Recorded so the choice is visible; atelier-ll3p may want to show a fail cleared by a pass of a different command.
