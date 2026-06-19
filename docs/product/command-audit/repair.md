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

## Retired Surface Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| removed `atelier repair [issue-id]` | Worker, admin | Legacy cleanup for hidden active work after worktree disappearance or interrupted cleanup. | Replacement repair boundary and whether state is canonical or ignored runtime. | Use `atelier doctor --fix` for ignored runtime/projection repair or `atelier worktree repair <id>` for stale worktree associations. | Remove or keep rejected; root repair overlaps with better-scoped commands. |
