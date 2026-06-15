# `atelier start`

Primary role: Worker.

Primary question: "How do I make this issue part of the current work in this
checkout?"

## Assessment

- Name: Correct. A root lifecycle verb is clearer than a role-prefixed command.
- Documentation: Correct when paired with `status`, `issue show`, and
  `issue transition`.
- Design: Correct if it moves the issue to canonical `in_progress` state in the
  current checkout's tracked `.atelier/` copy and does not create a separate
  durable active pointer outside the workflow.
- Output hierarchy: Issue ID, resulting workflow status, workspace/branch
  context, next commands.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier start <issue-id>` | Worker | Begin implementation or validation work on a slice by adding it to the checkout's canonical current-work set. | Good. |
