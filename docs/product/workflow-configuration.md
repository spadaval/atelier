# Workflow Configuration Contract

Atelier workflow policy is a fixed tracked repository artifact:

```text
.atelier/workflow.yaml
```

Schema version 3 is the only supported workflow contract. There is no
compatibility parser, migration command, environment fallback, config-selected
policy path, or hidden default policy. Commands that inspect issue workflow
state or execute issue transitions load and validate this file directly. If the
file is missing, obsolete, or invalid, the command fails with a workflow config
error.

## Operator Surface

Issue workflow execution is explicit:

```text
atelier issue transition <id> <transition>
atelier issue transition <id> --options
```

`atelier issue transition <id> --options` renders transitions available from
the issue's current status. Each option reports whether the transition is
currently allowed, configured validator results, static transition descriptions,
branch context, and the next command to run. A blocked attempt records a
`transition_blocked` issue activity entry. A successful attempt records a
`transition_applied` activity entry and updates the canonical issue `status`.

## Scope

Workflow policy applies to issues. The contract defines:

- a required branch policy for owner branch names, base branch, and merge
  strategy;
- a shared status catalog with explicit status categories;
- named issue workflows and their allowed transitions;
- terminal done states for each workflow;
- workflow-owned issue type applicability;
- inline built-in validators and validator params;
- optional static transition descriptions; and
- strict configuration errors for invalid or obsolete config.

Mission, evidence, activity, and future durable record lifecycles stay outside
`.atelier/workflow.yaml`.

## Fixed V3 Shape

The file is strict YAML with explicit schema identity:

```yaml
schema: atelier.workflow
schema_version: 3

branch_policy:
  base_branch: master
  merge_strategy: squash
  branch_templates:
    epic: epic/{{ issue.id }}
    issue: codex/{{ issue.id }}

statuses:
  todo: { category: todo }
  in_progress: { category: active }
  blocked: { category: blocked }
  review: { category: review }
  validation: { category: validation }
  done: { category: done }
  archived: { category: done }

workflows:
  standard:
    applies_to: [bug, feature, task]
    initial_status: todo
    done_statuses: [done, archived]
    transitions:
      start:
        from: [todo, blocked]
        to: in_progress
      block:
        from: [todo, in_progress, validation]
        to: blocked
      close:
        from: [in_progress, validation]
        to: done
        required_fields: [close_reason]
        description: "Closing requires attached evidence and no open blockers."
        validators:
          - evidence_attached: { min_count: 1 }
          - no_open_blockers
          - no_blocking_lints
          - durable_state_current

  epic_reviewed:
    applies_to: [epic]
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
        validators: [review_complete]
      close:
        from: [validation]
        to: done
        required_fields: [close_reason]
        description: "Closing requires attached evidence, complete child proof, a merged pull request, and a clean worktree."
        validators:
          - evidence_attached: { min_count: 1 }
          - epic_child_proof_complete
          - no_open_blockers
          - no_blocking_lints
          - linked_pr_merged
          - durable_state_current
          - git_worktree_clean

  validation_reviewed:
    applies_to: [validation]
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
        validators: [review_complete]
      close:
        from: [validation]
        to: done
        required_fields: [close_reason]
        description: "Closing requires attached evidence, complete child proof, and a clean worktree."
        validators:
          - evidence_attached: { min_count: 1 }
          - epic_child_proof_complete
          - no_open_blockers
          - no_blocking_lints
          - durable_state_current
          - git_worktree_clean

  spike:
    applies_to: [spike]
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
        description: "Record a concise close reason that captures the spike outcome."
        validators:
          - review_complete
          - durable_state_current
```

Required top-level fields are `schema`, `schema_version`, `branch_policy`,
`statuses`, and `workflows`. Unknown top-level fields are hard errors.
Obsolete top-level fields such as `issue_types`, `branch_lifecycle`,
`validators`, `guidance_templates`, and `fields` are rejected.

## Branch Policy

`branch_policy` is the shared branch policy used by workflow commands, status
surfaces, PR validation, and branch/worktree helpers. It is derived from the
tracker graph rather than duplicated in command handlers:

- child issues under an epic use the nearest parent epic as branch owner;
- standalone issues own their issue branch;
- epics own their epic branch;
- child issue close commits tracker state on the epic branch and does not merge
  to base; and
- standalone issue and epic close integrate their owner branch to base.

| Field | Rule |
| --- | --- |
| `base_branch` | Required non-empty Git branch name. |
| `merge_strategy` | Required. One of `squash`, `merge_commit`, or `fast_forward_only`. |
| `branch_templates.epic` | Required branch template for epic owners. |
| `branch_templates.issue` | Required branch template for standalone issue owners and exceptional issue worktrees. |

Branch templates support only `{{ issue.id }}` and `{{ issue.type }}`. In this
context, `issue` means the branch owner, not necessarily the child issue being
started or closed.

## Statuses And Categories

`statuses` is a shared catalog of named status objects. Status names use stable
lowercase ASCII identifiers: `^[a-z][a-z0-9_]*$`.

Each status object currently has one required field:

| Field | Rule |
| --- | --- |
| `category` | Required. One of `todo`, `active`, `blocked`, `review`, `validation`, or `done`. |

Status categories are operator-facing summary buckets. They help commands
summarize work but do not replace workflow status in canonical issue records.

Statuses in a workflow's `done_statuses` list are terminal for that workflow:

- every `done_statuses` value must exist in `statuses`;
- every terminal status must have category `done`; and
- no transition may leave a terminal done status.

## Workflow Applicability

Each workflow owns its issue type coverage through `applies_to`.

Built-in issue types are `bug`, `epic`, `feature`, `spike`, `task`, and
`validation`. Every built-in issue type must appear exactly once across all
workflows. Missing, duplicate, or unknown issue types are hard config errors.

Starter workflow names are:

| Workflow | Applies to |
| --- | --- |
| `standard` | `bug`, `feature`, `task` |
| `epic_reviewed` | `epic` |
| `validation_reviewed` | `validation` |
| `spike` | `spike` |

## Transitions

Each workflow defines named transitions:

| Field | Rule |
| --- | --- |
| `from` | Required non-empty list of source statuses. Each status must exist and must not be terminal for the workflow. |
| `to` | Required destination status. It must exist. |
| `required_fields` | Optional list of required command inputs. Currently `close_reason` is supported. |
| `validators` | Optional list of inline built-in validators. |
| `description` | Optional static text rendered near transition options and blocked transition output. |

`description` is static text. There is no template registry and no template
variable expansion.

## Validators

Transition validators use built-in names directly:

```yaml
validators:
  - no_open_blockers
  - durable_state_current
```

Parameterized validators use single-key map syntax:

```yaml
validators:
  - evidence_attached: { min_count: 1 }
```

There is no top-level validator alias registry. Unknown validators and invalid
params are hard config errors.

Supported built-ins include:

| Validator | Purpose |
| --- | --- |
| `durable_state_current` | Canonical state and local projection are current enough for the transition. |
| `issue_sections_parseable` | Issue Markdown sections can be parsed. |
| `evidence_attached` | Required evidence is attached; supports `min_count`. |
| `review_complete` | Required review artifact state is complete enough for the configured transition; the configured review provider remains the authority for approval rules and branch protection. |
| `epic_child_proof_complete` | Epic child work is closed with validating proof. |
| `no_open_blockers` | Target has no open blockers. |
| `no_blocking_lints` | Blocking lint checks pass. |
| `git_worktree_clean` | Worktree cleanliness gate passes. |
| `linked_pr_merged` | The linked provider-local review artifact number, remote identity, source/target branches, and merged state match the Atelier workflow branch policy. |

## Review Field

`review` is the built-in canonical issue field for the active review artifact
link. In room mode it points at a native room record:

```yaml
review:
  kind: room
  id: atelier-rvw1
```

In provider mode it stores the provider-backed review artifact as structured
data:

```yaml
review:
  kind: pull_request
  provider: forgejo
  number: 42
```

Room records live in tracked YAML under `.atelier/reviews/<id>.yaml`:

```yaml
id: atelier-rvw1
issue: atelier-epic
mode: room
source_branch: epic/atelier-epic
target_branch: master
events:
  - id: evt-0001
    kind: comment
    actor: reviewer
    body: "Initial review note"
  - id: evt-0002
    kind: finding
    actor: reviewer
    severity: blocking
    body: "Fix the failing path"
```

The current room status is derived from metadata plus ordered events. Room
projections may index open findings, approvals, stale approvals, and merge
state, but canonical records must not store a second mutable summary that can
drift from the event timeline.

Provider-mode `review link` inputs may accept a review number or a full
provider URL, but canonical issue records store only the normalized structured
field. URL inputs must match the configured review provider, host, owner, and
repository before they normalize to a number. The current provider
implementation is Forgejo.

The active review link belongs to the branch-owning issue or epic. Child issues
inherit the nearest parent epic's `review`; defining `review` directly on a
child issue is invalid unless the child owns its own branch by policy. Legacy
top-level `pull_request` fields are migration input only: migrated records must
render the structured `review` field, and strict validation rejects the old
shape after migration. The starter policy attaches
`linked_pr_merged` only to epic close, so validation issues and ordinary child
issues can close on their own proof while the epic remains the merged review
artifact boundary. In provider mode `linked_pr_merged` derives provider
host/owner/repo from `.atelier/config.toml` and expected source/target branches
from `branch_policy`. In room mode equivalent review readiness comes from the
room merge event rather than provider PR state.

`linked_pr_merged` is deliberately a fact check, not a second review-provider
policy engine. Atelier validates the review artifact link, remote identity,
branch match, and merged state because those facts decide whether the local
workflow gate is satisfied. The configured review provider owns branch
protection, required approvals, allowed merge strategies, and final merge
authorization. If a repository needs Atelier to enforce additional review
policy locally, that is a new product decision rather than an extension of the
starter workflow.

## Review Artifact Guidance

Code-changing epic work should have a review artifact when the workflow
requires review or merged-review closeout. Ordinary child implementation
issues use the nearest parent epic's review artifact; standalone code-changing
issues may own their own artifact. Planning, tracker-only, docs-only, and
scenario-validation work do not need a review artifact unless their workflow or
human assignment explicitly asks for one.

Agents use the review artifact for code discussion: worker context for the diff,
reviewer findings and review decisions, validator bugs tied to changed code or
tests, and worker responses plus follow-up commits. Agents keep Atelier as the
durable work record: issue status, blockers, evidence transcripts, scenario
validation, mission or epic closeout, and proof summaries remain in canonical
records. Native Markdown comments or activity sidecars may capture durable
notes, but they are not a second PR system and do not satisfy review-provider
merge gates.

## Errors

Workflow config errors should name the rejected field or reference. Important
diagnostic families include:

| Error | Meaning |
| --- | --- |
| `workflow_config_invalid_schema` | Missing, obsolete, or unsupported schema identity/version. |
| `workflow_config_unknown_field` | Unknown or removed top-level config field. |
| `workflow_config_invalid_branch_policy` | Branch policy is malformed. |
| `workflow_config_invalid_issue_type` | Workflow `applies_to` coverage is missing, duplicated, or unknown. |
| `workflow_config_invalid_status` | Status name, category, transition source, or terminal status is invalid. |
| `workflow_config_unknown_validator` | Transition references an unsupported validator. |
| `workflow_config_invalid_validator` | Validator params are malformed. |
| `workflow_issue_field_invalid` | Canonical issue fields violate built-in workflow field rules. |
