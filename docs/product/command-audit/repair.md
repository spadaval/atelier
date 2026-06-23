# Retired `atelier repair`

Primary role: Worker.

Primary question: "How do I clear stale active work after a checkout disappeared
or cleanup was interrupted?"

## Assessment

- Name: Misleading for the target workflow. Root `repair` overlaps with
  `doctor --fix`, and the active-pointer cleanup concept itself is no longer
  part of the product model.
- Documentation: Removed. It should not appear in normal worker guidance.
- Design: Removed. Missing-checkout recovery should inspect checkout
  state, then reconcile canonical issue status through normal issue transitions
  or record edits rather than clearing hidden runtime state.
- Output hierarchy: Invoking `atelier repair` rejects as an unrecognized
  subcommand.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier repair [issue-id]` | Worker | Legacy cleanup for hidden active-pointer state. | Removed; use `doctor --fix` for ignored local state or issue transitions for durable workflow state. |
