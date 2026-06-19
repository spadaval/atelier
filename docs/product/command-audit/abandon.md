# `atelier abandon`

Primary role: Worker.

Primary question: "How do I stop this local work association without claiming
the issue is done?"

## Assessment

- Name: No longer matches the target work model. Stopping work is either no
  durable change at all or a normal issue workflow change, not hidden active
  pointer cleanup.
- Documentation: Should classify this as a removal or replacement candidate,
  not normal worker guidance.
- Design: Remove or replace. When the durable state changed, use `issue note`
  plus `atelier issue transition <id> --options` to move the canonical issue
  record out of `in_progress`; when the durable state did not change, no extra
  cleanup command is required.
- Output hierarchy: If kept temporarily, the output should say it is legacy and
  point operators back to `status`, `issue transition`, and `worktree status`.

## Retired Surface Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| removed `atelier abandon [issue-id] --reason "..."` | Worker | Legacy cleanup for hidden active-pointer state; stop work without completion. | Replacement path for current-work state. | Add `issue note` if context matters, inspect `issue transition --options`, or leave no durable change if no workflow state changed. | Remove or keep rejected; do not restore hidden active-pointer cleanup. |
