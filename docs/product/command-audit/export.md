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

## Hidden Surface Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| hidden/admin `atelier export` | Admin/migration | Materialize deterministic renderer output during migration; inspect canonical rendering while debugging storage. | Output path, rendered records, skipped/failing records, source state. | Use `lint` or targeted migration checks; avoid normal workflow use. | Temporary migration/test surface only. |
| hidden/admin `atelier export --check` | Admin/debug | Check deterministic renderer/projection freshness during targeted diagnostics. | Fresh/stale result, paths, differing records, repair route. | Run `atelier lint` for canonical errors or `doctor` for local state. | Not normal health; do not use as handoff or proof gate. |

## Boundary

Tracked `.atelier/` Markdown is authoritative. Ignored projection, runtime,
diagnostic, lock, and cache state is repairable checkout state. Normal commands
should refresh projections safely when possible. `doctor` and `doctor --fix`
own explicit ignored-state inspection and repair. `export` must not overwrite
canonical records during recovery and must not be presented as the ordinary
proof that a handoff, validation, or terminal check is ready.
