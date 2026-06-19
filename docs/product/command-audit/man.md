# `atelier man`

Primary role: Worker.

Primary question: "Which existing Atelier commands matter for my current role?"

## Assessment

- Name: Correct. `man` is a guide layer, not a new command namespace.
- Documentation: Should be visible in root help and product docs as the
  role-specific replacement for the removed `prime` signpost.
- Design: Correct if valid roles are exact: `worker`, `reviewer`, `validator`,
  `manager`, and `admin`. Do not add `orchestrator` aliases.
- Output hierarchy: Role title, current state, ranked relevant commands, normal
  loop, then commands not usually for the role.

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `atelier man` | Any operator | Discover valid role guides; choose a role; recover from command-surface uncertainty. | Valid role names and one-line role purposes. | Run a role guide such as `atelier man worker` or `atelier man admin`. | Good. Bare guide works without choosing a role-specific namespace. |
| `atelier man worker` | Worker | Start assigned work; find ready work; learn proof and handoff loop. | Current work, ready work, core worker commands, commands to avoid. | Run `atelier status`, `issue list --ready`, or `issue show <id>`. | Good target contract, but currently blocked by invalid workflow config in this checkout. |
| `atelier man reviewer` | Reviewer | Check proof; review transition readiness; inspect review/evidence paths. | Mission/issue readiness, review/evidence commands, blocked/missing-proof recovery. | Run `mission status`, `issue transition --options`, `review status`, or `evidence show`. | Good role, needs degraded output when workflow policy is unreadable. |
| `atelier man validator` | Validator | Run independent validation; record proof; report pass/fail/deferred results. | Target issue, evidence expectations, review context, validation recording command. | Run `issue show <id>` and `evidence record --target issue/<id> --kind validation -- <command>`. | Good role. Should keep validation separate from implementation. |
| `atelier man manager` | Manager/orchestrator | Coordinate missions; create graph work; inspect blockers and hierarchy; prepare workspaces. | Current mission, ready/blocked counts, graph/bundle/worktree commands, recovery cues. | Run `mission status`, `bundle preview`, `graph tree --compact`, or `worktree for-mission`. | Good role. Should keep branch commands in advanced recovery. |
| `atelier man admin` | Admin | Set up, repair, migrate, and maintain Atelier; recover from broken tracker state. | Health checks, config paths, repairable local state, dangerous maintenance commands. | Run `doctor`, `lint`, `doctor --fix`, or provider setup commands. | Contract says it should degrade before init or broken state; live checkout shows it can fail on workflow policy parsing. |

## Role Term

`manager` is the broad CLI role class for work coordination. `orchestrator` is a
specific Agent Factory agent type within that class, not a `man` role alias.

## Guidance Finding

`man` is the recovery entry point, so `man admin` must be able to render useful
setup and repair guidance when tracker state is partially invalid. In this
checkout, `atelier man worker`, `reviewer`, `validator`, `manager`, and `admin`
all fail on `.atelier/workflow.yaml` before producing role guidance:
`workflow_config_unknown_field` at `workflows.standard.transitions.start`
because of an `effects` field. That violates the `zen.md` resilience and
friction-point guidance principles. Non-admin guides may hard-stop on invalid
workflow state, but admin should still explain `lint`, `doctor`, and config
repair paths.
