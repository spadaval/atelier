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

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `doctor` | Admin | Inspect runtime/projection/install health. | Good. |
| `doctor --fix` | Admin | Repair ignored local runtime/cache/projection state. | Good. |
