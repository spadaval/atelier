# `atelier prune`

Primary role: Admin.

Primary question: "How do I inspect and explicitly remove accumulated local,
canonical, branch, and worktree artifacts?"

## Assessment

- Name: Correct. It signals cleanup rather than repair or validation.
- Documentation: Visible in root help as admin maintenance. It should not be
  cited as proof for normal workflow completion.
- Design: Correct if dry-run stays the default and `--apply` only removes
  cleanup classes with implemented retention contracts from
  [Retention And Prune Policy](../retention-and-prune.md).
- Output hierarchy: Candidate class, retention window, protected/deferred
  classes, removed paths or IDs for `--apply`, Git-history recovery shape for
  canonical cleanup, failures.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `prune` | Admin | Preview eligible cleanup across implemented retention classes. | Good. |
| `prune --apply` | Admin | Remove eligible cleanup candidates and report skipped protected state. | Good with explicit confirmation and Git recovery guidance for canonical records. |
| `prune --retention-days <days>` | Admin | Override diagnostics and canonical retention for the cleanup pass. | Good. |

## Cutting Note

Canonical records, branches, and worktrees may be pruned only after their
retention contracts are implemented. Until then, `prune` must keep those classes
visible as deferred or protected instead of deleting them.
