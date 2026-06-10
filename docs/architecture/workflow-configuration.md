# Workflow Configuration Contract

Atelier workflow policy is owned by the repository in a committed YAML file:

```text
atelier.workflow.yaml
```

This file lives at the repository root. It is not stored under `.atelier/`
because `.atelier/` is local runtime state, and it is not stored under
`.atelier-state/` because `.atelier-state/` is the deterministic exported record
projection. Workflow configuration is hand-authored repository policy that
commands load alongside the exported tracker state.

## Scope

The workflow configuration defines:

- record types and their default workflows;
- workflow states, transitions, and terminal states;
- required fields, evidence, and workflow validators for transitions;
- hook commands that run around workflow actions;
- action-aware guidance rendered at the point of use.

It does not define direct agent-run management, coding-agent process
supervision, durable agent-run rows, retry queues, token accounting, or live
session metrics. Future run records may reference workflow outcomes, but the
workflow contract itself only controls repository tracker transitions,
validation, hooks, and guidance.

## Schema Shape

The top-level file is strict YAML with explicit schema identity:

```yaml
schema: atelier.workflow_config
schema_version: 1

defaults:
  workflow: tiny_task
  hook_timeout_ms: 30000

record_types:
  issue:
    default_workflow: tiny_task
    workflows: [tiny_task]

workflows:
  tiny_task:
    states: [open, claimed, done]
    initial_state: open
    terminal_states: [done]
    transitions:
      claim:
        from: open
        to: claimed
      close:
        from: claimed
        to: done
        validators: [durable_state_current]

validators:
  durable_state_current:
    kind: builtin
    builtin: durable_state_current

hooks: {}
guidance: {}
```

Required top-level fields are `schema`, `schema_version`, `record_types`,
`workflows`, and `validators`. Optional top-level fields are `defaults`, `hooks`,
and `guidance`.

Names for workflows, states, transitions, validators, hooks, and guidance blocks
use stable lowercase ASCII identifiers: `^[a-z][a-z0-9_]*$`. Human-readable
labels may be added later, but identifiers are the machine contract.

Unknown fields are configuration errors. Unknown references are configuration
errors. Future schema versions are unsupported unless an implementation
explicitly ships a migration or compatibility reader.

## Transition Fields

Each transition may define:

| Field | Rule |
| --- | --- |
| `from` | Required source state or source-state array. |
| `to` | Required destination state. |
| `required_fields` | Optional record fields that must be non-empty before the transition. |
| `evidence` | Optional evidence requirements such as `min_count`, `types`, or linked validation criteria. |
| `validators` | Optional ordered list of validator identifiers. |
| `hooks` | Optional hook lists keyed by event, such as `before_transition`. |
| `guidance` | Optional guidance block identifiers rendered for the action. |
| `allow_override` | Optional boolean. Overrides are rejected unless this is true and a reason is recorded. |

Workflow completion is represented by transitions into `terminal_states`.
Closure rules should be expressed through `required_fields`, `evidence`, and
`validators` on those transitions rather than by adding special milestone fields.

## Invalid Configuration Errors

Configuration loading must report stable error names. Human-readable text may
change, but these names are the API contract for JSON output, diagnostics, lint,
and Mission Control projections.

| Error name | Meaning |
| --- | --- |
| `workflow_config_missing` | `atelier.workflow.yaml` is required for the action and is absent. |
| `workflow_config_not_file` | The configured path exists but is not a regular file. |
| `workflow_config_parse_error` | YAML parsing failed. |
| `workflow_config_schema_missing` | `schema` or `schema_version` is absent. |
| `workflow_config_schema_unsupported` | The schema name or version is unsupported. |
| `workflow_config_unknown_field` | A top-level or nested field is not part of schema version 1. |
| `workflow_config_invalid_name` | A workflow, state, transition, validator, hook, or guidance identifier is malformed. |
| `workflow_config_duplicate_name` | A list or map defines the same identifier more than once. |
| `workflow_config_unknown_reference` | A transition, record type, validator, hook, or guidance block references an undefined identifier. |
| `workflow_config_invalid_transition` | A transition has invalid `from` or `to` states, cannot be reached, or conflicts with terminal-state rules. |
| `workflow_config_invalid_validator` | A validator definition is malformed or names an unsupported validator kind. |
| `workflow_config_invalid_hook` | A hook definition is malformed or names an unsupported hook event. |
| `workflow_config_invalid_env` | An environment-variable reference is malformed, missing when required, or used in a field that does not permit expansion. |
| `workflow_config_invalid_template` | Guidance rendering references an unknown variable, filter, or context field. |
| `workflow_config_reload_failed` | A reload check found a different file hash but the new file could not be loaded. |

Every error payload should include `path`, `error`, `message`, and, when
available, `line`, `column`, `field`, and `reference`.

## Environment Expansion

Environment values are never expanded implicitly. Plain YAML strings are literal,
including strings that contain `$NAME` or `${NAME}`.

Expansion is allowed only where the schema says an expandable scalar is valid,
and the value must use this object shape:

```yaml
value:
  from_env: ATELIER_TOKEN
  required: true
```

Optional environment values may provide a literal default:

```yaml
value:
  from_env: ATELIER_BRANCH_PREFIX
  required: false
  default: codex
```

Workflow names, state names, transition names, validator names, hook names,
record type names, and schema fields are not expandable. Repository policy must
remain visible in the committed file; environment indirection is only for local
secrets or machine-specific scalar values.

## Hooks

Hooks are side-effect commands declared by workflow policy. They are not a
replacement for validators: validators decide whether a transition is allowed,
while hooks run integration steps around an allowed action.

Hook definitions use explicit argv arrays and do not run through a shell unless
the repository intentionally invokes one:

```yaml
hooks:
  test_before_milestone_close:
    event: before_transition
    transitions: [close]
    command:
      argv: [cargo, test]
      env: {}
    timeout_ms: 120000
    failure_mode: block
```

Supported hook events for schema version 1 are:

- `before_transition`: runs after static transition checks and validators pass,
  but before the tracker mutation is committed;
- `after_transition`: runs after the canonical record write and projection
  refresh or stale-projection marking complete;
- `worktree_setup`: runs when a worktree helper prepares a worktree.

The staged `atelier worktree for` implementation executes hooks whose
definition has `event: worktree_setup`. The hook command runs in the created or
located worktree with `ATELIER_WORKTREE_PATH`, `ATELIER_WORK_ISSUE_ID`, and
`ATELIER_WORK_BRANCH` set. Non-zero exit status blocks the helper and is
reported in command output.

`timeout_ms` is required unless `defaults.hook_timeout_ms` is set. Hook
timeouts and non-zero exits produce machine-readable results. A hook with
`failure_mode: block` prevents the action and reports `hook_timeout`,
`hook_failed`, or `hook_spawn_failed`. A hook with `failure_mode: warn` allows
the action to continue and emits the same result names as warnings.

The default failure mode is `block` for `before_transition` and `worktree_setup`
hooks, and `warn` for `after_transition` hooks. Blocking `after_transition`
hooks are not allowed because the record mutation has already happened.

Hook stdin receives the same context shape used by workflow validators plus the
hook identifier and event. Hook stdout and stderr are captured, truncated for
display, and available in JSON output.

Schema version 1 examples still use `export_current` for compatibility with the
existing Mission Control projection vocabulary. New schema work should prefer
separate durable-record and projection-index freshness fields.

## Workflow Validators

Workflow validators are transition checks attached to workflow policy. The
validator identifier is stable and appears in transition definitions, JSON
results, lint output, and Mission Control projections.

Validator definitions have one of these schema version 1 kinds:

```yaml
validators:
  durable_state_current:
    kind: builtin
    builtin: durable_state_current

  evidence_attached:
    kind: builtin
    builtin: evidence_attached
    params:
      min_count: 1

  milestone_acceptance_met:
    kind: builtin
    builtin: validation_criteria_satisfied
```

Builtin validator names are implementation-owned stable names. The initial
stable builtin namespace is:

- `durable_state_current`
- `evidence_attached`
- `validation_criteria_satisfied`
- `no_open_blockers`
- `no_blocking_lints`
- `review_complete`

Custom validators are deferred until an implementation issue defines execution,
sandboxing, caching, and result validation. Hook commands must not be treated as
custom validators.

### Validator Context

Validators receive a deterministic input context:

```json
{
  "schema": "atelier.workflow_validator_context",
  "schema_version": 1,
  "config": {
    "path": "atelier.workflow.yaml",
    "sha256": "<config-file-sha256>"
  },
  "workflow": "milestone_task",
  "transition": {
    "name": "close",
    "from": "validation",
    "to": "done"
  },
  "record": {
    "kind": "issue",
    "id": "atelier-z1p8",
    "status": "validation"
  },
  "linked_records": [],
  "blockers": [],
  "evidence": [],
  "milestone": null,
  "projection": {
    "state_path": ".atelier-state",
    "export_current": true
  },
  "git": {
    "branch": "codex/example",
    "dirty": false
  },
  "now": "2026-06-09T00:00:00Z"
}
```

Implementations may add fields in a future schema version. Schema version 1
validators must not silently ignore missing required context: if a builtin
cannot use required context, it fails with an actionable message.

### Validator Result

Validator results are machine-readable and use a pass/fail shape:

```json
{
  "schema": "atelier.workflow_validator_result",
  "schema_version": 1,
  "validator": "evidence_attached",
  "workflow": "milestone_task",
  "transition": "close",
  "record": {
    "kind": "issue",
    "id": "atelier-z1p8"
  },
  "result": "fail",
  "message": "Attach at least 1 evidence record before closing atelier-z1p8.",
  "actions": [
    "Run the required validation command.",
    "Record the result with atelier evidence add.",
    "Retry the close transition."
  ],
  "details": {
    "required_count": 1,
    "actual_count": 0
  }
}
```

`result` is `pass` or `fail`. A failing result must name the failed condition,
the affected record, and at least one next action. Commands that cannot build a
validator context may report the command-level validation state `blocked`, but a
validator that runs returns only `pass` or `fail`.

## Guidance Rendering

Guidance blocks are advisory text rendered close to the action they affect. They
may be attached to record types, workflows, states, or transitions:

```yaml
guidance:
  milestone_close:
    applies_to:
      workflow: milestone_task
      transition: close
    format: markdown
    template: |
      Closing {{ record.id }} requires current export state and attached
      evidence for every milestone validation criterion.
```

Template rendering is strict. Unknown variables, filters, or context paths fail
with `workflow_config_invalid_template` at reload-check time when statically
detectable, or at action time when dependent on runtime context. Guidance
receives the same context as validators and must not read environment variables
except through explicit expandable values in the config.

## Reload Behavior

Every CLI invocation that depends on workflow policy reads and validates
`atelier.workflow.yaml` before acting. The command captures the config file hash
in validator, hook, and diagnostic output.

Long-lived future surfaces such as Mission Control, file watchers, or local
helpers must perform a reload check before each config-dependent action:

1. Hash `atelier.workflow.yaml`.
2. If the hash is unchanged, keep the loaded config.
3. If the hash changed, parse and validate the new file.
4. If the new file is valid, use it for the action and report the new hash.
5. If the new file is invalid, reject mutating config-dependent actions with
   `workflow_config_reload_failed`.

Read-only projections may display the last valid config only if they also show
the current reload failure. They must not silently fall back for transitions,
validators, hooks, or guidance.

The future `atelier workflow validate` command should validate the config and
emit JSON containing `path`, `sha256`, `result`, `errors`, and `warnings`.
Until that command is promoted by its owning implementation issue, config health
may be surfaced through `atelier lint` and `atelier doctor`.

## Tiny Task Example

Tiny tasks avoid milestone ceremony but still protect durable state freshness:

```yaml
schema: atelier.workflow_config
schema_version: 1

defaults:
  workflow: tiny_task
  hook_timeout_ms: 30000

record_types:
  issue:
    default_workflow: tiny_task
    workflows: [tiny_task]

workflows:
  tiny_task:
    states: [open, claimed, done]
    initial_state: open
    terminal_states: [done]
    transitions:
      claim:
        from: open
        to: claimed
      close:
        from: claimed
        to: done
        validators: [durable_state_current]

validators:
  durable_state_current:
    kind: builtin
    builtin: durable_state_current
```

This workflow lets small issues move from open to claimed to done. It does not
require evidence unless repository policy opts into that stricter validator.

## Stricter Milestone Workflow Example

Milestone-bound work can require planning, review, validation evidence, and a
current export before completion:

```yaml
schema: atelier.workflow_config
schema_version: 1

defaults:
  workflow: milestone_task
  hook_timeout_ms: 30000

record_types:
  issue:
    default_workflow: milestone_task
    workflows: [milestone_task]

workflows:
  milestone_task:
    states:
      - research
      - planning
      - implementation
      - review
      - validation
      - done
    initial_state: research
    terminal_states: [done]
    transitions:
      plan:
        from: research
        to: planning
      implement:
        from: planning
        to: implementation
      request_review:
        from: implementation
        to: review
      validate:
        from: [implementation, review]
        to: validation
        validators: [review_complete]
      close:
        from: validation
        to: done
        validators:
          - validation_criteria_satisfied
          - evidence_attached
          - durable_state_current
          - no_open_blockers
        hooks:
          before_transition: [test_before_milestone_close]

validators:
  durable_state_current:
    kind: builtin
    builtin: durable_state_current
  evidence_attached:
    kind: builtin
    builtin: evidence_attached
    params:
      min_count: 1
  no_open_blockers:
    kind: builtin
    builtin: no_open_blockers
  review_complete:
    kind: builtin
    builtin: review_complete
  validation_criteria_satisfied:
    kind: builtin
    builtin: validation_criteria_satisfied

hooks:
  test_before_milestone_close:
    event: before_transition
    transitions: [close]
    command:
      argv: [cargo, test]
      env: {}
    timeout_ms: 120000
    failure_mode: block
```

This stricter workflow scales process to milestone risk without making that
ceremony the default for tiny tasks.
