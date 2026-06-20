# Retired `atelier graph`

Primary role: Manager/orchestrator.

Primary question: "What relationship context is missing from issue and mission
views?"

## Assessment

- Name: Weak. `graph` is implementation language; operators usually start from
  an issue, mission, blocker, or proof question.
- Documentation: This namespace is retired and should not be taught as an
  ordinary manager path.
- Design: Duplicative. `graph tree` overlaps with issue hierarchy and
  objective status views. `graph impact` exposed useful blast-radius logic; the
  replacement home is now `issue show`, which renders downstream impact
  alongside blockers, subissues, and proof context.
- Output hierarchy: If any implementation survives temporarily, it should
  report the source record, relationship reason, affected record, and the
  domain command that should own the next drill-down.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `graph impact <id>` | Manager/orchestrator | Understand downstream consequences before mutation or terminal validation. | Removed. Issue-sourced impact now belongs in `issue show`; objective work health belongs in `issue status <objective-id>`. |
| `graph tree [--compact]` | Manager/orchestrator | Inspect mission and issue hierarchy. | Removed. Objective hierarchy and work buckets belong in `issue status <objective-id>` and issue detail. |
| `graph tree --status <status>` | Manager/orchestrator | Filter the hierarchy by status. | Remove. It is redundant with issue list/status/category filters and currently hard-codes predecessor statuses. |

## Cutting Direction

The abstract `graph` namespace is removed. The replacement views now cover the
useful operator questions:

- `issue show <id>` renders parent/children, blockers, evidence, and downstream
  impact records that may need review before changing or closing the source.
- `issue status <objective-id>` renders objective health, linked work buckets,
  blockers, proof gaps, terminal checks, and next actions.
- `issue blocked [<id>]` remains the focused blocker inspection surface.

A compact blast-radius command can return later as `issue impact` only if the
strengthened issue views still leave a concrete operator question unanswered.
