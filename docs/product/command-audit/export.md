# `atelier export`

Primary role: Admin.

Primary question: "How do I inspect or check canonical export state during
storage migration or repair?"

## Assessment

- Name: Potentially misleading. Product docs say export/rebuild are low-level
  mechanics, not normal handoff commands.
- Documentation: Should be admin-only and absent from root normal workflow help.
- Design: Acceptable as a diagnostic or migration surface. It should not be an
  automation contract for agents selecting work or proving validation.
- Output hierarchy: Export/check result, paths, freshness result, next `lint` or
  `doctor` command.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier export` | Admin | Materialize canonical export during migration/debugging. | Needs admin framing. |
| `atelier export --check` | Admin | Check canonical tracker freshness. | Consider routing normal users to `lint` instead. |
