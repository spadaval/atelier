# `atelier maintenance`

Primary role: Admin.

Primary question: "How do I perform explicit destructive record surgery?"

## Assessment

- Name: Correct. It signals danger and keeps destructive actions out of normal
  workflows.
- Documentation: Correct as a visible specialized surface.
- Design: Correct if commands require explicit target kind and force or
  confirmation.
- Output hierarchy: Target record, consequence, confirmation/refusal, deleted
  record, recovery guidance through history/lint/Git.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `maintenance delete <kind> <id> --force` | Admin | Delete a record deliberately. | Good. Should never appear as routine next action. |
