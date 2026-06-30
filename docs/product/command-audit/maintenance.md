# `atelier maintenance`

Primary role: Admin.

Primary question: "How do I perform explicit destructive record surgery?"

## Assessment

- Name: Correct. It signals danger and keeps destructive actions out of normal
  workflows.
- Documentation: Hidden from root help. It should not appear in normal role
  guidance.
- Design: Over budget as a normal surface unless an explicit recovery flow names
  it.
- Output hierarchy: Target record, consequence, confirmation/refusal, deleted
  record, recovery guidance through history/lint/Git.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `maintenance delete <kind> <id> --force` | Admin recovery | Delete a record deliberately. | Hide or remove unless explicit recovery proves it is still needed. |

## Complexity Budget

Destructive record surgery is not a routine command. Keep this only as a hidden
escape hatch with explicit routing from a failed repair/check flow, or delete it
if normal record edits and Git recovery cover the real cases.
