# `atelier rebuild`

Primary role: Admin.

Category: Hidden debug diagnostic or admin repair primitive. It is not normal
workflow.

Primary question: "How do I rebuild local SQLite runtime/projection state from
canonical records?"

## Assessment

- Name: Correct for a low-level repair verb.
- Documentation: Should be admin-only. Normal operators should use `doctor
  --fix` unless a diagnostic explicitly names `rebuild`.
- Design: Acceptable as a maintenance primitive, but too implementation-shaped
  for routine workflow. If retained, it should sit behind targeted diagnostics
  or `doctor --fix` ownership rather than ordinary handoff guidance.
- Output hierarchy: Input source, rebuilt local state, failures by record/path,
  next `doctor` or `lint`.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| hidden/admin `atelier rebuild` | Admin/debug | Recreate ignored projection/runtime state from canonical Markdown during targeted repair or testing. | Should stay out of normal role guides except admin recovery; `doctor --fix` owns the operator repair path. |

## Boundary

Rebuild must only recreate ignored local projection/runtime/cache state from
tracked canonical Markdown. It must not change canonical `.atelier/` records,
and a successful rebuild is not evidence that issue content, mission closeout,
or validation proof is correct. Ordinary proof uses `lint` and the workflow
command being retried after repair.
