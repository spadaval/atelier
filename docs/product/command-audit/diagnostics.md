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

## Hidden Surface Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| hidden `diagnostics slow` | Admin, performance investigator | Find slow command telemetry; inspect local-only command failures; support performance debugging. | Time window, threshold, command family, durations, workspace grouping, local-only warning. | File a performance issue, adjust threshold/window, or return to `doctor`/`status`. | Good as hidden local diagnostic. It must not become workflow state or mission/session evidence by default. |
