# `atelier rebuild`

Primary role: Admin.

Primary question: "How do I rebuild local SQLite runtime/projection state from
canonical records?"

## Assessment

- Name: Correct for a low-level repair verb.
- Documentation: Should be admin-only. Normal operators should use `doctor
  --fix` unless a diagnostic explicitly names `rebuild`.
- Design: Acceptable as a maintenance primitive, but too implementation-shaped
  for routine workflow.
- Output hierarchy: Input source, rebuilt local state, failures by record/path,
  next `doctor` or `lint`.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier rebuild` | Admin | Recreate ignored projection/runtime state. | Should stay out of normal role guides except admin recovery. |
