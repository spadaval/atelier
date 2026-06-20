# `atelier prune`

Primary role: Admin.

Primary question: "How do I inspect and explicitly remove accumulated local
artifacts?"

## Assessment

- Name: Correct. It signals cleanup rather than repair or validation.
- Documentation: Visible in root help as admin maintenance. It should not be
  cited as proof for normal workflow completion.
- Design: Correct if dry-run stays the default and `--apply` only removes
  cleanup classes with implemented retention contracts.
- Output hierarchy: Candidate class, retention window, protected/deferred
  classes, removed paths for `--apply`, failures.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `prune` | Admin | Preview eligible local cleanup. | Good. |
| `prune --apply` | Admin | Remove eligible local cleanup candidates. | Good with explicit confirmation. |
| `prune --retention-days <days>` | Admin | Adjust diagnostics retention for the cleanup pass. | Good. |

## Cutting Note

Keep the initial scope narrow. If future pruning grows to records, worktrees, or
branches, those classes need explicit retention contracts before `prune --apply`
removes anything beyond supported local artifacts.
