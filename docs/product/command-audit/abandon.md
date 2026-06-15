# `atelier abandon`

Primary role: Worker.

Primary question: "How do I stop this local work association without claiming
the issue is done?"

## Assessment

- Name: Correct. The verb distinguishes local context cleanup from issue
  closure.
- Documentation: Correct when it is described as local runtime cleanup, not a
  workflow transition.
- Design: Correct. The required reason helps preserve continuity.
- Output hierarchy: Cleared issue, recorded reason, remaining local cleanup
  hints, next status command.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier abandon [issue-id] --reason "..."` | Worker | Drop local active work safely. | Good. |
