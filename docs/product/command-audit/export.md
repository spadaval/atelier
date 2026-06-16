# `atelier export`

Primary role: Admin.

Category: Hidden debug diagnostic or temporary migration. It is not normal
workflow.

Primary question: "How do I inspect or check canonical export state during
storage migration or repair?"

## Assessment

- Name: Potentially misleading if visible. Product docs say export/rebuild are
  low-level mechanics, not normal handoff, health, validation, or terminal
  commands.
- Documentation: Should be hidden/admin-only, absent from root normal workflow
  help, and absent from worker/reviewer handoff recipes.
- Design: Acceptable only as a deterministic-renderer test, projection
  diagnostic, or temporary migration surface. It should not be an automation
  contract for agents selecting work, proving validation, or deciding terminal readiness.
- Output hierarchy: Export/check result, paths, freshness result, next `lint` or
  `doctor` command.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| hidden/admin `atelier export` | Admin/migration | Materialize deterministic renderer output during migration or debugging. | Temporary migration or test-only. |
| hidden/admin `atelier export --check` | Admin/debug | Check deterministic renderer/projection freshness during migration or targeted diagnostics. | Not normal health; route normal operators to `lint` and `doctor`. |

## Boundary

Tracked `.atelier/` Markdown is authoritative. Ignored projection, runtime,
diagnostic, lock, and cache state is repairable checkout state. Normal commands
should refresh projections safely when possible, `doctor` reports local health,
and `doctor --fix` owns explicit ignored-state repair. `export` must not
overwrite canonical records during recovery and must not be presented as the
ordinary proof that a handoff, validation, or terminal check is ready.
