# `atelier repair`

Primary role: Worker.

Primary question: "How do I clear stale active work after a worktree disappeared
or cleanup was interrupted?"

## Assessment

- Name: Misleading for the target workflow. Root `repair` overlaps with
  `doctor --fix` and `worktree repair`, and the active-pointer cleanup concept
  itself is no longer part of the product model.
- Documentation: Should classify this as a removal candidate, not normal worker
  guidance.
- Design: Remove or replace. Missing-worktree recovery should inspect checkout
  and worktree context, then reconcile canonical issue status through normal
  issue transitions or record edits rather than clearing hidden runtime state.
- Output hierarchy: If it remains temporarily for compatibility, stale
  issue/path first and an explicit statement that the command is legacy.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier repair [issue-id]` | Worker | Legacy cleanup for hidden active-pointer state. | Remove or replace. |
