# `atelier prune`

Primary role: Admin.

Primary question: "How do I inspect and explicitly remove accumulated local,
canonical, branch, and worktree artifacts without bypassing their safety
contracts?"

## Assessment

- Name: Correct. It signals cleanup rather than repair or validation.
- Documentation: Visible in root help as admin maintenance. It should not be
  cited as proof for normal workflow completion.
- Design: `atelier prune` itself is the dry run (not `--dry-run`). `--apply`
  only removes cleanup classes with implemented retention contracts from
  [Retention And Prune Policy](../retention-and-prune.md).
- Output hierarchy: Candidate class, retention window, protected/deferred
  classes, class-specific protection reason, removed paths or IDs for
  `--apply`, Git-history recovery shape for canonical cleanup, failures.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `prune` | Admin | Preview eligible cleanup across implemented retention classes. | Good. |
| `prune --apply` | Admin | Remove eligible cleanup candidates and report skipped protected state. | Good: the explicit apply request is still bounded by class safety checks and Git recovery guidance for canonical records. |
| `prune --retention-days <days>` | Admin | Override diagnostics and canonical retention for the cleanup pass. | Good; it does not waive relationship, lock, checkout, branch, or worktree protections. |

## Cutting Note

Every class remains visible in the dry run. If a class's cleanup capability is
not implemented, `prune` must call it `deferred`; `--apply` must not silently
ignore it or delete it. Canonical, branch, and worktree cleanup always retains
the policy protections even when the class is implemented.
