# `atelier bundle`

Primary role: Manager/orchestrator.

Primary question: "How do I preview and apply a reviewed batch of mission,
issue, relationship, and evidence records?"

## Assessment

- Name: Correct. `bundle` is a better product term than resurrecting
  first-class `plan` CRUD for one-shot batch creation.
- Documentation: Now visible in root help and should stay in manager guidance,
  not in routine worker loops.
- Design: Correct if `preview` remains deterministic and non-mutating, and
  `apply` continues to require explicit `--yes`.
- Output hierarchy: Bundle validity, created or referenced records, relationship
  counts, evidence links, then `lint` or focused show commands.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `bundle preview <file>` | Manager/orchestrator | Validate an authored record bundle without mutating tracker state. | Good. |
| `bundle apply <file> --yes` | Manager/orchestrator | Create the reviewed batch of records and links. | Good, with explicit confirmation. |

## Cutting Note

The current implementation delegates `commands::bundle` to `commands::plan`.
That boundary keeps a removed/deferred product name alive in code. Rename the
implementation module to `bundle` once any tests or internal imports that still
refer to plan semantics are removed.
