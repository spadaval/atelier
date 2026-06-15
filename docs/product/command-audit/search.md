# `atelier search`

Primary role: Worker.

Primary question: "How do I find the record when I do not know the ID?"

## Assessment

- Name: Correct.
- Documentation: Correct as an orientation helper.
- Design: Currently issue-text oriented by help text. If search now covers
  missions, plans, or evidence, the docs and output should say "records" rather
  than "issues."
- Output hierarchy: Echoed query, bounded matches, kind/status cues, drill-down
  commands.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier search <query>` | Worker | Recover an ID from remembered text. | Good, but naming text should match the actual search scope. |
