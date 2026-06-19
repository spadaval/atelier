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

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `maintenance delete <kind> <id> --force` | Admin | Deliberately remove a bad record; recover from obsolete imported issue state; perform explicit destructive surgery. | Target kind/id, consequence, confirmation/refusal, deleted path, and recovery route through Git/lint/history. | Run `atelier lint`, inspect Git diff, and use `history` or Git to recover if needed. | Concept is right, but docs/help advertise generic record deletion while current dispatch supports issue records only. |

## Guidance Finding

There is an implementation/contract mismatch: `maintenance delete` accepts
`<TARGET_KIND> <TARGET_ID>` and docs describe generic record surgery, but the
current dispatch rejects non-issue kinds with "currently supports issue records
only." Either narrow help/docs to issue deletion or implement the advertised
record-kind behavior. Keep this command out of ordinary next-action guidance.
