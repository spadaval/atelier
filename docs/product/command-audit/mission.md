# `atelier mission`

Primary role: Manager/orchestrator.

Primary question: "How do I create, focus, inspect, coordinate, and close a
durable mission?"

## Assessment

- Name: Correct. Mission is the durable purpose boundary.
- Documentation: Strong, but role guides should distinguish daily mission
  status from terminal-check detail and lifecycle mutation.
- Design: Mostly correct. The family owns mission purpose, focus, linked work,
  blockers, and terminal checks.
- Output hierarchy: Mission identity and lifecycle first, current work/blockers
  next, proof/health/terminal readiness next, then specific next actions.

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `mission create` | Manager/orchestrator | Create durable purpose boundary; capture intent, constraints, risks, and validation expectations. | Mission ID, generated sections, status, canonical path, and missing setup detail. | Add work with `mission add-work`, create/link issues, or run `mission status <id>`. | Good. Help says generated sections, but docs should explain why missions are purpose boundaries. |
| `mission show <id>` | Manager/orchestrator, reviewer | Inspect mission intent; see linked work, blockers, evidence, and notes; audit scope. | Rich mission body, linked issues, blockers, evidence, current health. | Run `mission status <id>`, `issue show <id>`, or `history --mission <id>`. | Good drill-down surface. |
| `mission start <id> --switch` | Manager/orchestrator | Focus active orchestration context; replace stale mission focus. | Active mission result, previous focus if replaced, next status command. | Run `mission status` or prepare mission worktree. | Clear enough. |
| `mission status [id]` | Manager/orchestrator, worker, reviewer | Daily control view; blocker triage; choose next issue; inspect terminal readiness summary. | Health, ready/blocked/done/backlog counts, current work, blockers, evidence gaps, terminal readiness. | Start/show next issue, record proof, clear blocker, or close mission. | Best mission-aligned surface, but currently blocked in this checkout by workflow policy drift. |
| `mission status --verbose [id]` | Reviewer, validator | Inspect terminal-check detail; audit advanced validators; classify closeout blockers. | Validator results, missing proof, reliability/detail failures, exact next repair commands. | Record validation evidence, fix blockers, or rerun non-verbose status. | Good if verbose detail remains secondary. |
| `mission close <id> --reason ...` | Manager/orchestrator | Close after gates pass; record close reason; verify terminal checks. | Terminal blockers or confirmation, close reason, evidence/validator status. | Run `history --mission <id>` or inspect any blocking child. | Strong if failures name next commands. |
| `mission list [--status ...]` | Manager/orchestrator | Select current mission; find historical/closed missions; orient after clone. | Mission IDs/titles, status, health/ordering cues, active focus. | Run `mission status <id>` or `mission show <id>`. | Good hierarchy. |
| `mission update <id>` | Manager/orchestrator | Adjust title/status/body/sections as scope changes. | Changed fields, resulting status, canonical path. | Run `mission show <id>` and `mission status <id>`. | Good, but help should discourage `--status closed` as ordinary terminal path. |
| `mission note <id>` | Manager/orchestrator | Add durable coordination, decision, blocker, result, or handoff context. | Note kind, mission ID, activity path. | Run `mission show <id>` or `history --mission <id>`. | Good continuity surface. |
| `mission add-work <mission> <issue>` | Manager/orchestrator | Link accountable issue work into mission scope; curate graph after issue creation. | Mission ID, issue ID, relationship result. | Run `mission status <id>` or `graph tree --compact`. | Good distinct concept. |
| `mission unlink <mission> <issue>` | Manager/orchestrator | Remove issue from mission scope; correct graph mistakes. | Removed relationship and current mission scope. | Run `mission status <id>` or `graph tree --compact`. | Good. |
| `mission add-blocker <mission> <issue>` | Manager/orchestrator | Make mission-level blocker visible; prevent premature closeout. | Blocker issue, mission state, readiness consequence. | Resolve/show blocker or run `mission status`. | Good coordination visibility. |

## Guidance Finding

The mission family is conceptually aligned with `zen.md`: purpose, current work,
blockers, and proof stay visible. The main gap in this checkout is operability:
repo-aware mission/status guidance fails when the installed binary rejects local
workflow `effects`, even though the checked-out source appears to support them.
