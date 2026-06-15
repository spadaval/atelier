# `atelier repair`

Primary role: Worker.

Primary question: "What replaces root active-pointer repair after current work
becomes status-derived?"

## Assessment

- Name: Remove. Root `repair` is too broad, and stale active-work pointers are
  not a target product concept.
- Documentation: Should route concrete failure modes to their owning surfaces:
  `lint` for canonical Markdown errors, `doctor --fix` for ignored
  runtime/cache/projection repair, and `worktree status/remove` or Git recovery
  for worktree path problems.
- Design: Remove active-pointer repair. It must not clear or synthesize current
  work because current work is derived from `in_progress` issue status.
- Output hierarchy: If the legacy command remains during migration, it should
  reject with the specific replacement surface for the detected problem.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier repair [issue-id]` | Worker | Legacy stale active-pointer cleanup. | Remove. Replacement is `lint`, `doctor --fix`, `worktree status/remove`, or ordinary Git recovery depending on the problem. |
