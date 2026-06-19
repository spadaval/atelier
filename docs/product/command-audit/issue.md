# `atelier issue`

Primary role: Manager/orchestrator.

Primary question: "How do I create, inspect, mutate, and advance accountable
issue work?"

`issue` is intentionally shared by roles. The root noun is correct because the
command owns issue records and issue workflow state, not a single user persona.
`issue create` and `issue transition` belong in this family because they mutate
issue state. Normal lifecycle movement routes through configured transitions;
the removed `issue close` surface is no longer part of routine operation.

## Assessment

- Name: Correct. The noun maps to the durable accountability unit.
- Documentation: Needs role-aware examples. Generic `issue --help` is accurate
  but does not tell a worker which subset matters.
- Design: Mostly correct. The family is broad, but cohesion is still issue
  state and issue relationships.
- Output hierarchy: For reads, current state and blockers before metadata. For
  mutations, changed fields or transition result first, then canonical path and
  next commands.

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `issue create` | Manager/orchestrator | Create actionable task/bug/feature/epic work; attach parent; seed a body template. | New ID, type, priority, labels, parent, canonical file path. | Edit body if needed, run `atelier issue show <id>`, link to mission, or run `atelier lint`. | Good. Next-action text should avoid pushing directly to implementation before orientation. |
| `issue list [--ready|--blocked|--status ...]` | Worker, manager/orchestrator | Find ready work; scan blocked queue; filter by workflow status/category/priority/label. | Actionable IDs, title, status/category, priority, blockers, and enough context to choose a record. | Run `atelier issue show <id>` or `atelier issue blocked <id>`. | Good concept. Empty results should say what filter or command to try next. |
| `issue show <id>` | Worker, reviewer, validator | Understand assignment; inspect proof gaps; continue handoff; review transition readiness. | Status/category/type, file path, body sections, blockers, evidence status, review/session/activity cues, transition options. | Record evidence, add a note, inspect history/review, or run `issue transition <id> --options`. | Strongest orientation surface; keep current action and proof gaps near the top. |
| `issue transition <id> [transition] [--options]` | Worker, reviewer, validator | Preview gates; execute allowed workflow move; diagnose blockers, validators, and declared effects. | Allowed/blocked transitions, validators, declared effects, skip-effects audit requirements. | Fix blockers/evidence, execute a transition, or inspect `review`/`evidence`. | Correct lifecycle inspection and execution surface. |
| `issue update <id>` | Manager/orchestrator | Correct title, priority, type, labels, or parent; clean up scope. | Changed fields, current metadata, canonical path. | Run `atelier issue show <id>` and `atelier lint <id>`. | Good. Help could give stronger cues for parent/type risk. |
| `issue note <id> <text>` | Worker, reviewer, manager/orchestrator | Add progress, handoff, blocker, resolution, observation, or human-context note. | Confirmation, note kind, target issue. | Run `atelier issue show <id>` or `atelier history --issue <id>`. | Good continuity surface. Handoff notes should route to history/show after mutation. |
| `issue block <blocked-id> <blocker-id>` | Manager/orchestrator | Record that one issue prevents another; make readiness dependency visible. | Blocked issue, blocker issue, relationship result. | Run `atelier issue blocked <blocked-id>` or `atelier issue show <blocker-id>`. | Good. Mutation output should name both records and inspection command. |
| `issue unblock <blocked-id> <blocker-id>` | Manager/orchestrator | Remove resolved dependency; return work to ready consideration. | Removed relationship and whether blocked issue is now ready. | Run `atelier issue list --ready` or `atelier issue blocked`. | Good. Output should say what can now move. |
| `issue blocked [id]` | Reviewer, manager/orchestrator | Inspect blocked queue; show blockers for one issue; triage dependency resolution. | Blocking issues, blocked issues, status, and drill-down commands. | Resolve blocker, show blocked issue, or unblock after proof. | Good fit and useful for readiness reviews. |

## Role Guide Implication

Workers should see `list --ready`, `show`, `note`, `transition`, and proof
commands.
Reviewers should see `transition --options`, `blocked`, and evidence commands.
Managers should see `create`, `update`, `block`, `unblock`, and queue filters.
