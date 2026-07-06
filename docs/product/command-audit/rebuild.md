# `atelier rebuild`

Primary role: Admin.

Category: Hidden advanced diagnostic or admin repair primitive. It is not
normal workflow and is intentionally omitted from root help.

Primary question: "How do I rebuild the local SQLite domain cache from record
files?"

## Assessment

- Name: Correct for a low-level repair verb.
- Documentation: Should be hidden from root normal workflow help. Normal
  operators should use `check --fix` unless a diagnostic explicitly names
  `rebuild`.
- Design: Acceptable as a maintenance primitive, but too implementation-shaped
  for routine workflow. If retained, it should sit behind targeted diagnostics
  or `check --fix` ownership rather than ordinary handoff guidance.
- Output hierarchy: Input source, rebuilt local state, failures by record/path,
  next `check` or `check`.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| hidden advanced `atelier rebuild` | Admin/debug | Recreate the ignored SQLite domain cache from record files during targeted repair or testing. | Should stay out of normal role guides except admin recovery; `check --fix` owns the operator repair path. |

## Boundary

Rebuild must only recreate ignored local domain-cache/runtime state from
tracked record files. It must not change durable `.atelier/` records,
and a successful rebuild is not evidence that issue content, mission closeout,
or validation proof is correct. Ordinary proof uses `check` and the workflow
command being retried after repair.
