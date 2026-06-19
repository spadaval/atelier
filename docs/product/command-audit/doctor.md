# `atelier doctor`

Primary role: Admin.

Primary question: "Is this local runtime/projection/install state healthy, and
can safe ignored-state repairs be applied?"

## Assessment

- Name: Correct.
- Documentation: Correct. It belongs in admin and reviewer preflight guidance,
  but `--fix` is admin/recovery behavior.
- Design: Correct if `--fix` never edits tracked canonical records.
- Output hierarchy: Health checks, degraded reasons, repairability, exact
  repaired ignored paths when `--fix` is supplied.

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `doctor` | Admin, reviewer preflight | Inspect runtime/projection/install health; distinguish canonical-state errors from ignored local repair; check provider-related health. | Health checks, degraded reasons, repairability, config paths, and whether `--fix` is allowed. | Run `atelier lint` for canonical errors, `atelier doctor --fix` for ignored-state repair, or provider setup commands. | Good admin surface. Output should print concrete next command when a check is not ok. |
| `doctor --fix` | Admin | Rebuild or repair ignored runtime/cache/projection state; recover from stale local state after checkout/merge. | Exact ignored paths changed, what was rebuilt, skipped repairs, and any remaining errors. | Run `atelier doctor`, then `atelier status` or `atelier mission status`. | Correct if it never edits tracked canonical records. |

## Guidance Finding

`doctor` is the repair-orientation command, so failed checks should always name
whether the operator should run `lint`, `doctor --fix`, or a provider-specific
setup command. It must not become a silent tracked-record repair tool.
