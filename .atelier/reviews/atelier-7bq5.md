---
schema: atelier.review
schema_version: 1
id: atelier-7bq5
title: 'Re-reviewed 36387dcb, merge b098beee and 9adeaad0 against findings n3f5, t7hl, fvyl and gz88. Read inherited_blockers, dependency_readiness_batch, dependency_closure, the issue show section and the work-model docs. inherited_blockers reads parent.relationships.children, the same source as the index children map, returns early when the parent has no blockers, and no longer copies sibling records. Reran in /tmp scratch repos with a fresh build: a parent blocked by its own child leaves both children ready (n3f5); a done child X of Q leaves C (blocked_by X) ready after Q gains an open blocker Z (t7hl); issue show, issue transition and start for a child of a blocked parent show ''Blocked via parent P'' and the path child -> parent -> blocker, and work blocked and work parent list the child (fvyl); the docs match this behaviour (gz88). Also checked that superseded and done parent blockers release the child, that close (blockers.none_open) still checks only direct blockers, and that the completion-claim gate no longer sees self-cycles. Evidence zkvv: cargo nextest run --workspace passes at 9adeaad0, which has the same code as HEAD. Open: one defer finding, a parent-routed cycle worded as an incomplete path.'
target: atelier-e4c5
identity: Independent reviewer (Claude subagent)
commit: 5b0ab3a3537f4297187238df94641fa10a9eaedd
created_at: 2026-10-16T00:55:52.551120546Z
event:
  event: approval
---

Re-reviewed 36387dcb, merge b098beee and 9adeaad0 against findings n3f5, t7hl, fvyl and gz88. Read inherited_blockers, dependency_readiness_batch, dependency_closure, the issue show section and the work-model docs. inherited_blockers reads parent.relationships.children, the same source as the index children map, returns early when the parent has no blockers, and no longer copies sibling records. Reran in /tmp scratch repos with a fresh build: a parent blocked by its own child leaves both children ready (n3f5); a done child X of Q leaves C (blocked_by X) ready after Q gains an open blocker Z (t7hl); issue show, issue transition and start for a child of a blocked parent show 'Blocked via parent P' and the path child -> parent -> blocker, and work blocked and work parent list the child (fvyl); the docs match this behaviour (gz88). Also checked that superseded and done parent blockers release the child, that close (blockers.none_open) still checks only direct blockers, and that the completion-claim gate no longer sees self-cycles. Evidence zkvv: cargo nextest run --workspace passes at 9adeaad0, which has the same code as HEAD. Open: one defer finding, a parent-routed cycle worded as an incomplete path.
