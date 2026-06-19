# `atelier search`

Primary role: Worker.

Primary question: "How do I find the record when I do not know the ID?"

## Assessment

- Name: Correct.
- Documentation: Correct as an orientation helper.
- Design: Currently issue-text oriented by help text. If search now covers
  missions or evidence, the docs and output should say "records" rather
  than "issues."
- Output hierarchy: Echoed query, bounded matches, kind/status cues, drill-down
  commands.

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `atelier search <query>` | Worker, reviewer, manager/orchestrator | Recover an unknown issue ID from remembered text; find possible duplicate work before creating an issue; locate prior handoff context. | Matching issue IDs, titles, status/type/priority cues, query echo, and enough surrounding text to choose a drill-down. | Run `atelier issue show <id>`, `atelier history --issue <id>`, or `atelier graph tree --compact`. | Current help and implementation are issue-text oriented. Broader docs that promise record search should either narrow to issue search or expand implementation. |

## Guidance Finding

`search` is named broadly, but root help says "Search issue text" and the
current implementation searches issue title/body/activity. If the product
intends search to recover missions, evidence, and review artifacts too, this is
an implementation gap. If not, product docs should say issue search so agents do
not rely on it for continuity across all records.
