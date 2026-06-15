# Workflow Configuration Contract

Version 1 issue workflow policy is a fixed repository artifact:

```text
.atelier/workflow.yaml
```

There is no config-selected policy path, environment fallback, or alternate
workflow source in v1. Commands that inspect issue workflow state or execute
issue transitions load and validate this file directly. If the file is missing
or invalid, the command fails with a stable workflow-config error.

## Operator Surface

Issue workflow execution is explicit:

```text
atelier issue transition <id> <transition>
atelier issue transition <id> --options
```

`atelier issue transition <id> --options` renders the transitions available
from the issue's current workflow status. Each option reports whether the
transition is currently allowed, the configured validator results, rendered
guidance, and the exact command to run next. A blocked attempt records a
`transition_blocked` issue activity entry. A successful attempt records a
`transition_applied` activity entry and updates the canonical issue `status`
field to the destination workflow status.

## Scope

Version 1 workflow policy applies to issues only. The contract defines:

- a shared status catalog with explicit status categories;
- named issue workflows and their allowed transitions;
- terminal done states for each workflow;
- built-in issue-type to workflow mappings;
- configured built-in validators, including validator params;
- simple guidance templates rendered with transitions; and
- strict configuration errors for invalid or deferred config.

Version 1 does not define mission, milestone, plan, or evidence lifecycles.
Those records keep their own product contracts outside `.atelier/workflow.yaml`.

## Fixed V1 Shape

The file is strict YAML with explicit schema identity:

```yaml
schema: atelier.workflow
schema_version: 1

issue_types:
  bug: standard_review_proof
  closeout: standard_review_proof
  epic: standard_review_proof
  feature: standard_review_proof
  spike: lightweight_spike
  task: standard_review_proof
  validation: standard_review_proof

statuses:
  todo:
    category: todo
  in_progress:
    category: active
  blocked:
    category: blocked
  review:
    category: review
  validation:
    category: validation
  done:
    category: done
  archived:
    category: done

validators:
  durable_current:
    builtin: durable_state_current
  review_ready:
    builtin: review_complete
  proof_attached:
    builtin: evidence_attached
    params:
      min_count: 1
  blockers_clear:
    builtin: no_open_blockers
  lint_clear:
    builtin: no_blocking_lints
  closeout_clean:
    builtin: git_worktree_clean

guidance_templates:
  close_with_proof:
    format: markdown
    template: |
      Closing {{ issue.id }} requires attached evidence and no open blockers.
  record_spike_outcome:
    format: markdown
    template: |
      Record a concise close reason that captures what {{ issue.id }} learned
      and what follow-up work remains.

workflows:
  standard_review_proof:
    initial_status: todo
    done_statuses: [done, archived]
    transitions:
      start:
        from: [todo, blocked]
        to: in_progress
      block:
        from: [todo, in_progress, review, validation]
        to: blocked
      request_review:
        from: [in_progress]
        to: review
      request_validation:
        from: [in_progress, review]
        to: validation
        validators: [review_ready]
      close:
        from: [validation]
        to: done
        required_fields: [close_reason]
        validators:
          - proof_attached
          - blockers_clear
          - lint_clear
          - durable_current
          - closeout_clean
        guidance: [close_with_proof]

  lightweight_spike:
    initial_status: todo
    done_statuses: [done]
    transitions:
      start:
        from: [todo, blocked]
        to: in_progress
      block:
        from: [todo, in_progress, review]
        to: blocked
      request_review:
        from: [in_progress]
        to: review
      revise:
        from: [review]
        to: in_progress
      close:
        from: [review]
        to: done
        required_fields: [close_reason]
        validators:
          - review_ready
          - durable_current
        guidance: [record_spike_outcome]
```

Required top-level fields are `schema`, `schema_version`, `issue_types`,
`statuses`, `validators`, `guidance_templates`, and `workflows`.

Unknown fields are hard errors. Unknown references are hard errors. Schema
version 1 does not permit compatibility aliases, partial parsing, or silent
fallback behavior.

## Statuses And Categories

`statuses` is a shared catalog of named status objects. Status names use stable
lowercase ASCII identifiers: `^[a-z][a-z0-9_]*$`.

Each status object currently has one required field:

| Field | Rule |
| --- | --- |
| `category` | Required. One of `todo`, `active`, `blocked`, `review`, `validation`, or `done`. |

Status categories are operator-facing summary buckets. They help `atelier
status`, ready queues, issue detail, and mission status describe where work is
without changing workflow semantics. Categories do not replace transitions.

Statuses in a workflow's `done_statuses` list are terminal for that workflow:

- every `done_statuses` value must exist in `statuses`;
- every terminal status must have category `done`; and
- no transition may leave a terminal done status.

## Issue-Type Mappings

`issue_types` maps each built-in issue type to one named workflow. Version 1
accepts only these built-in issue types:

- `bug`
- `closeout`
- `epic`
- `feature`
- `spike`
- `task`
- `validation`

Repositories may remap those built-in types to any defined workflow, but they
may not invent custom issue types in v1.

The starter policy is:

| Issue type | Default workflow |
| --- | --- |
| `bug` | `standard_review_proof` |
| `closeout` | `standard_review_proof` |
| `epic` | `standard_review_proof` |
| `feature` | `standard_review_proof` |
| `spike` | `lightweight_spike` |
| `task` | `standard_review_proof` |
| `validation` | `standard_review_proof` |

## Workflows And Transitions

`workflows` is a map of named workflow definitions. Each workflow defines:

| Field | Rule |
| --- | --- |
| `initial_status` | Required status name. Must exist in `statuses` and must not be terminal. |
| `done_statuses` | Required non-empty list of terminal status names. |
| `transitions` | Required map of named transition objects. |

Each transition object defines:

| Field | Rule |
| --- | --- |
| `from` | Required status name or non-empty list of status names. |
| `to` | Required destination status name. |
| `required_fields` | Optional list of canonical issue field names that must be non-empty before the transition succeeds. |
| `validators` | Optional ordered list of validator definition names from the top-level `validators` map. |
| `guidance` | Optional list of guidance template names from `guidance_templates`. |

Transition names use the same stable identifier rule as statuses and workflows.
A transition is invalid when it references an unknown status, targets a status
outside the workflow, duplicates another transition name in the same workflow,
or attempts to leave a terminal done status.

Version 1 required-field enforcement is intentionally narrow. `close_reason` is
the key required field used by the starter workflows to make low-risk closure
inspectable even when first-class evidence is not required.

## Validator Definitions

`validators` is a map of repository-defined validator entries. Each entry uses:

| Field | Rule |
| --- | --- |
| `builtin` | Required built-in validator name. |
| `params` | Optional params object validated by the chosen built-in validator. |

Transition validators reference these validator entry names, not raw built-in
strings, so repositories can reuse one configured validator in multiple
transitions.

Version 1 built-in validator names are fixed:

| Built-in | Params | Behavior |
| --- | --- | --- |
| `durable_state_current` | none | Fails when canonical tracker state or required export freshness is stale for the transition. |
| `evidence_attached` | `min_count` (required integer >= 1), `kind` (optional evidence kind) | Fails when the issue does not have enough attached evidence records matching the params. |
| `review_complete` | none | Fails when the issue has not gone through the expected review path for the transition. |
| `validation_criteria_satisfied` | none | Fails when the issue or parent closeout criteria required by the workflow remain unproven. |
| `no_open_blockers` | none | Fails when blocking issue dependencies remain open. |
| `no_blocking_lints` | none | Fails when tracker lint reports blocking defects for the issue or transition. |
| `git_worktree_clean` | none | Fails when the current worktree has tracked or untracked changes that make closeout non-clean. |

Unknown built-in names, missing required params, wrong param types, and
unexpected params are strict configuration errors.

## Guidance Templates

`guidance_templates` is a map of named advisory templates rendered near a
transition or failure path. Each template currently defines:

| Field | Rule |
| --- | --- |
| `format` | Required. `markdown` only in v1. |
| `template` | Required template string. |

Template rendering is strict. Unknown variables or malformed template syntax
fail configuration validation. The supported template context is intentionally
small:

- `issue.id`
- `issue.type`
- `transition.name`
- `transition.from`
- `transition.to`

Guidance is descriptive only. It does not replace validators and it does not
run commands.

## Strict Configuration Errors

Workflow-dependent commands report stable error names. Human-readable text may
change, but these names are the contract for diagnostics and validation proof.

| Error name | Meaning |
| --- | --- |
| `workflow_config_missing` | `.atelier/workflow.yaml` is required for the action and is absent. |
| `workflow_config_not_file` | The workflow path exists but is not a regular file. |
| `workflow_config_parse_error` | YAML parsing failed. |
| `workflow_config_schema_missing` | `schema` or `schema_version` is absent. |
| `workflow_config_schema_unsupported` | The schema name or version is unsupported. |
| `workflow_config_unknown_field` | A top-level or nested field is not part of schema version 1. |
| `workflow_config_duplicate_name` | A map or list defines the same status, workflow, transition, validator, or guidance name more than once. |
| `workflow_config_invalid_status` | A status entry is malformed or uses an unsupported category. |
| `workflow_config_invalid_workflow` | A workflow entry is malformed or internally inconsistent. |
| `workflow_config_invalid_transition` | A transition entry is malformed, unreachable, or violates terminal-state rules. |
| `workflow_config_invalid_validator` | A validator entry is malformed, names an unsupported built-in, or uses invalid params. |
| `workflow_config_invalid_guidance_template` | A guidance template is malformed or references unsupported template variables. |
| `workflow_config_invalid_issue_type_mapping` | An issue type mapping is missing, uses an unsupported issue type, or points at an undefined workflow. |
| `workflow_config_unknown_reference` | A transition, workflow, validator, or guidance block references an undefined name. |
| `workflow_config_deferred_feature` | The config uses a feature that version 1 intentionally does not support. |

Error payloads should include `path`, `error`, and `message`, plus `line`,
`column`, `field`, or `reference` when that detail is available.

## Standard Review/Proof Workflow Example

The standard starter workflow is the contract for most issue types. It makes
review and proof explicit before `done`, and it keeps `archived` available as a
terminal legacy-migration status:

```yaml
workflows:
  standard_review_proof:
    initial_status: todo
    done_statuses: [done, archived]
    transitions:
      start:
        from: [todo, blocked]
        to: in_progress
      block:
        from: [todo, in_progress, review, validation]
        to: blocked
      request_review:
        from: [in_progress]
        to: review
      request_validation:
        from: [in_progress, review]
        to: validation
        validators: [review_ready]
      close:
        from: [validation]
        to: done
        required_fields: [close_reason]
        validators:
          - proof_attached
          - blockers_clear
          - lint_clear
          - durable_current
          - closeout_clean
        guidance: [close_with_proof]
```

This workflow is intentionally strict at close:

- work must pass through review and validation/proof states;
- `close_reason` must be recorded;
- at least one evidence record must be attached;
- blockers and blocking lints must be clear; and
- durable tracker state and the worktree must be current enough for closeout.

## Lightweight Spike Workflow Example

The lightweight spike workflow is deliberately smaller. It still uses the review
path, but it does not require first-class evidence for low-risk closure:

```yaml
workflows:
  lightweight_spike:
    initial_status: todo
    done_statuses: [done]
    transitions:
      start:
        from: [todo, blocked]
        to: in_progress
      block:
        from: [todo, in_progress, review]
        to: blocked
      request_review:
        from: [in_progress]
        to: review
      revise:
        from: [review]
        to: in_progress
      close:
        from: [review]
        to: done
        required_fields: [close_reason]
        validators:
          - review_ready
          - durable_current
        guidance: [record_spike_outcome]
```

This example makes the intended trade-off explicit:

- spikes still move through review before they close;
- spikes still record an inspectable `close_reason`; and
- spikes do not require attached evidence unless a repository intentionally maps
  them to a stricter workflow.

## Diagnostics

Hidden advanced/internal workflow diagnostics remain separate from the normal
operator workflow. Use `atelier lint` and `atelier doctor` for the human-facing
record and runtime health checks, and reserve `atelier workflow validate` for
explicit diagnostic or closeout contracts that name it.

## Deferred Features

These features are outside version 1 and must be rejected with
`workflow_config_deferred_feature` when they appear in
`.atelier/workflow.yaml`:

- custom issue types;
- custom validator execution;
- expression validators;
- hooks;
- triggers;
- post-functions;
- waivers; and
- workflow projection tables.

Version 1 keeps the contract small on purpose. Future workflow work can extend
the schema with a new version once those behaviors have an explicit execution
and validation model.
