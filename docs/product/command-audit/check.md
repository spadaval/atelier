# `atelier check`

Primary role: Admin, reviewer, validator.

Primary question: "Is tracker state healthy enough to trust, and can ignored
local runtime state be repaired safely?"

## Assessment

- Name: Correct. `check` is the visible health command.
- Documentation: Should replace routine references to `lint`, `doctor`,
  `workflow check`, `rebuild`, and projection repair commands.
- Design: Correct if it remains one health surface and does not become a raw
  diagnostics console.
- Output hierarchy: blocking health failures, safe `--fix` repairs, focused
  issue check result, then recovery commands.

## Subcommands And Flags

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `check` | Reviewer/validator | Validate tracker health. | Keep. |
| `check <issue-id>` | Reviewer/validator | Validate one issue and its reachable tracker state. | Keep. |
| `check --fix` | Admin | Repair ignored runtime/cache/projection state without editing canonical records. | Keep. |

## Complexity Budget

`check` owns normal health. Hidden diagnostics may exist for development or
admin recovery, but role guides should not teach them as ordinary validation
paths.

Surfaces over budget unless explicitly routed by `check` output:

- `lint`
- `doctor`
- `rebuild`
- `workflow check`
- diagnostic export/projection probes
