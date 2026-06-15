# `atelier start`

Primary role: Worker.

Primary question: "How do I mark this issue as current work in this tracker
copy?"

## Assessment

- Name: Keep as a thin convenience helper. It is acceptable only as a
  workflow-status operation, not a separate active-work system.
- Documentation: Should teach `atelier issue transition <id> start` because the
  current-work source of truth is canonical issue status.
- Design: Must not create local runtime association, session rows, or hidden
  claim state. Starting work means the issue record moves to `in_progress` in
  the checked-out Markdown tracker copy.
- Output hierarchy: Issue ID, old and new workflow status, canonical path, and
  next commands.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier issue transition <issue-id> start` | Worker | Mark the issue `in_progress` in canonical Markdown. | Good. |
| `atelier start <issue-id>` | Worker | Convenience start surface. | Performs only the workflow transition and must not write a runtime active pointer. |
