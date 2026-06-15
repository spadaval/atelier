# `atelier note`

Primary role: Worker.

Primary question: "Can I add a generic note to any record?"

## Assessment

- Name: Incorrect as a public surface. Generic notes collapse record ownership
  and conflict with record-specific `issue note` and `mission note`.
- Documentation: Correctly hidden and described as removed.
- Design: Should remain hidden until deleted. Compatibility aliases should not
  be kept unless a human requests a transition window.
- Output hierarchy: If invoked, it should reject with specific replacements.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| hidden `atelier note add ...` | Worker | Legacy generic note entry. | Should be removed or continue to point to record-specific note commands. |
