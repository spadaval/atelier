# `atelier start`

Primary role: Worker.

Primary question: "How do I establish this issue as my active local work?"

## Assessment

- Name: Correct. A root lifecycle verb is clearer than a role-prefixed command.
- Documentation: Correct when paired with `status`, `issue show`, and
  `abandon`.
- Design: Correct if it creates only local runtime association and does not
  claim durable completion or replace workflow transitions.
- Output hierarchy: Issue ID, active association, workspace/branch context, next
  commands.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier start <issue-id>` | Worker | Begin implementation or validation work on a slice. | Good. |
