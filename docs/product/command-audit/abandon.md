# Retired `atelier abandon`

Primary role: Worker.

Primary question: "How do I stop this local work association without claiming
the issue is done?"

## Assessment

- Name: No longer matches the target work model. Stopping work is either no
  durable change at all or a normal issue workflow change, not hidden active
  pointer cleanup.
- Documentation: Removed. It should not appear in normal worker guidance.
- Design: Removed. When the durable state changed, use `issue note`
  plus `atelier issue transition <id> --options` to move the canonical issue
  record out of `in_progress`; when the durable state did not change, no extra
  cleanup command is required.
- Output hierarchy: Invoking `atelier abandon` rejects as an unrecognized
  subcommand.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier abandon [issue-id] --reason "..."` | Worker | Legacy cleanup for hidden active-pointer state. | Removed; use `issue note` plus configured issue transitions only when durable state needs to change. |
