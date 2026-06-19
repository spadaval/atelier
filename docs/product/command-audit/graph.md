# `atelier graph`

Primary role: Manager/orchestrator.

Primary question: "What depends on this record, and how does the work hierarchy
fit together?"

## Assessment

- Name: Correct for relationship inspection, though it is more abstract than
  `issue` or `mission`.
- Documentation: Good if it stays tied to impact and hierarchy rather than raw
  graph internals.
- Design: Correct. This is a cross-record relationship read surface.
- Output hierarchy: Starting record, bounded relationship set, status/blocker
  cues, then `mission show`, `issue show`, or `issue blocked`.

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `graph impact <id>` | Manager/orchestrator, reviewer | Assess downstream consequences before changing, closing, blocking, or invalidating a record; understand what a blocker affects. | Starting record, affected missions/issues, relationship type, status/blocker cues, and risk of mutation. | Run `atelier issue show <id>`, `atelier mission show <id>`, or `atelier issue blocked <id>` before mutating. | Concept is right. Output should include concrete drill-down commands, not only relationship warnings. |
| `graph tree [--compact]` | Manager/orchestrator | Inspect mission/issue hierarchy; choose coordination focus; scan blocked or stale areas. | Bounded hierarchy, status/category cues, omitted counts, blocker markers, and active mission context. | Run `atelier mission status <id>`, `atelier mission show <id>`, or `atelier issue show <id>`. | Good fit; `--compact` should be the role-guide default for scan-oriented work. |

## Guidance Finding

The graph surface fits the product model, but relationship output should end
with drill-down commands. Impact inspection is usually a pre-mutation or
pre-validation step, so the next action must be explicit enough for agents to
avoid guessing.
