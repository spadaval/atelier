# `atelier lint`

Primary role: Reviewer.

Primary question: "Are committed tracker records and workflow configuration
valid?"

## Assessment

- Name: Correct.
- Documentation: Correct. It should be in reviewer and admin guides.
- Design: Correct if it validates canonical state and does not silently repair
  tracked records.
- Output hierarchy: Pass/fail summary, offending record/config path, actionable
  repair guidance, next `doctor` only when runtime state is implicated.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `lint` | Reviewer | Validate committed tracker state before handoff or closeout. | Good. |
| `lint <id>` | Reviewer | Validate a focused record. | Good. |
