# `atelier abandon`

Primary role: Worker.

Primary question: "What replaces local active-work cleanup when work is paused
or redirected?"

## Assessment

- Name: Remove. The verb only makes sense when a local active pointer exists.
- Documentation: Should not teach `abandon` as normal workflow. Current work is
  the set of checked-out issue records with status `in_progress`.
- Design: Remove with the active-pointer model. Pausing or redirecting work is
  represented by issue workflow transitions, blockers, notes, and evidence.
- Output hierarchy: If the legacy command remains during migration, it should
  reject with corrective guidance to `issue transition`, `issue note`, `status`,
  and `worktree status`.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier abandon [issue-id] --reason "..."` | Worker | Legacy active-pointer cleanup. | Remove. Replacement is workflow status change plus durable note/evidence as needed. |
