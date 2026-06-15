# `atelier diagnostics`

Primary role: Admin.

Primary question: "How do I inspect local command telemetry for performance or
failure analysis?"

## Assessment

- Name: Correct for a hidden diagnostic family.
- Documentation: Correctly hidden from normal root help.
- Design: Correct if JSON remains local telemetry, not a workflow automation
  contract.
- Output hierarchy: Diagnostic query parameters, summarized slow commands,
  local-only warning.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| hidden `diagnostics slow` | Admin | Find slow command telemetry by window and threshold. | Good as hidden admin diagnostic. |
