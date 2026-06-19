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

## Retired Surface Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| removed/hidden `atelier note add ...` | Worker | Legacy generic note entry; attempt to add context without choosing record owner. | Replacement record-specific command. | Use `atelier issue note <id> ...` or `atelier mission note <id> ...`. | Generic notes should remain removed because they blur record ownership. |
