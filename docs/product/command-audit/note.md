# `atelier note`

Primary role: Worker.

Primary question: "Can I add a generic note to any record?"

## Assessment

- Name: Incorrect as a public surface. Generic notes collapse record ownership
  and conflict with record-specific `issue note`.
- Documentation: Removed. It should stay out of root help and role guides.
- Design: Deleted from dispatch. Compatibility aliases should not be kept
  unless a human requests a transition window.
- Output hierarchy: Invoking `atelier note` rejects as an unrecognized
  subcommand.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier note add ...` | Worker | Legacy generic note entry. | Removed; use `atelier issue note <id> "..."`. |
