# `atelier repair`

Primary role: Worker.

Primary question: "How do I clear stale active work after a worktree disappeared
or cleanup was interrupted?"

## Assessment

- Name: Needs care. Root `repair` overlaps conceptually with `doctor --fix` and
  `worktree repair`; documentation must keep the scope narrow.
- Documentation: Should emphasize stale active-work association only.
- Design: Acceptable if it refuses to clear a live path and points to `abandon`
  for intentional context switches.
- Output hierarchy: Stale issue/path, refusal or clearance result, next
  `status` and `worktree status` commands.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier repair [issue-id]` | Worker | Recover local active-work state. | Needs narrow wording to avoid admin-repair confusion. |
