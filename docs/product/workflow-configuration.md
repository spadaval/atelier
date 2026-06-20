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

## Ownership Boundary

Atelier has three separate configuration surfaces. A setting belongs to exactly
one surface unless a later ADR explicitly changes that ownership.

| Surface | Owns | Must not own |
| --- | --- | --- |
| `.atelier/config.toml` | Tracked project config: project schema/version, `project_slug`, canonical `state_root`, ignored runtime/cache paths, active review mode, provider backend identity, provider remote coordinates, and the environment variable name that supplies any provider admin token. | Issue statuses, transitions, validators, workflow actions, branch templates, required transition fields, workflow-action role attribution, provider secret values, local runtime contents, projection data, diagnostics, locks, or caches. |
| `.atelier/workflow.yaml` | Tracked workflow policy: branch policy, status catalog, workflow applicability, transitions, terminal statuses, required transition fields, read-only validators, static descriptions, ordered transition actions, and action-owned review provider parameters such as action role attribution. | Provider host/owner/repo/token settings, environment variable values, local path overrides, projection/cache content, or hidden defaults. |
| Local runtime and environment | Ignored machine-local state under `.atelier/runtime/` and `.atelier/cache/`, local diagnostics, locks, rebuilt SQLite projections, and secret values supplied through environment variables such as the provider token variable named in config. | Durable project records or project policy. Runtime/cache state must be rebuildable or disposable, and environment variables must not be required for ordinary non-provider development commands. |

The boundary is intentionally split for review integration. `.atelier/config.toml`
selects the review backend, such as `review.mode = "provider"` with
`review.provider = "forgejo"`, and records the provider identity needed to
normalize and verify review artifacts. `.atelier/workflow.yaml` decides when a
transition opens or links the branch owner's review artifact through explicit
actions such as `review.open` or `review.link`. Provider
review actions declare the workflow role and any provider role-author mapping
they use; provider secrets remain environment-only through the token variable
named in `.atelier/config.toml`. Provider
approval rules, branch protection, and merge authorization remain with the
provider or native room implementation; workflow validators only read enough
review state to decide whether an Atelier transition may proceed.

## Operator Surface

Issue workflow execution is explicit:

```text
atelier issue transition <id> <transition>
atelier issue transition <id> --options
```

`atelier issue transition <id> --options` renders transitions available from
the issue's current status. Each option reports whether the transition is
currently allowed, configured read-only validator results, configured
transition actions, static transition descriptions, branch context, and the
next command to run. A blocked attempt records a `transition_blocked` issue
activity entry without running actions. A successful attempt runs declared
actions in order, records a `transition_applied` activity entry, and updates
the canonical issue `status`.

## Scope

Workflow policy applies to issues. The contract defines:

- a required branch policy for owner branch names, base branch, and merge
  strategy;
- a repository-defined issue type registry;
- a shared status catalog with explicit status categories;
- named issue workflows and their allowed transitions;
- terminal done states for each workflow;
- workflow-owned issue type applicability;
- inline read-only built-in validators and validator params;
- inline built-in transition actions;
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

issue_types:
  bug: { label: Bug }
  epic: { label: Epic }
  feature: { label: Feature }
  spike: { label: Spike }
  task: { label: Task }
  validation: { label: Validation }

statuses:
  todo: { category: todo }
  in_progress: { category: active }
  blocked: { category: blocked }
  review: { category: active }
  validation: { category: active }
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
        description: "Closing requires attached evidence and no open blockers."
        validators:
          - evidence.attached: { min_count: 1 }
          - blockers.none_open
          - lint.none_blocking
          - tracker.current

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
        actions:
          - review.open:
              provider: forgejo
              role: worker
              role_authors:
                worker: atelier-worker
                reviewer: atelier-reviewer
                validator: atelier-validator
                manager: atelier-manager
      request_validation:
        from: [in_progress, review]
        to: validation
        validators: [review.complete]
      close:
        from: [validation]
        to: done
        description: "Closing requires attached evidence, complete child proof, a merged pull request, and a clean worktree."
        validators:
          - evidence.attached: { min_count: 1 }
          - children.proof_complete
          - blockers.none_open
          - lint.none_blocking
          - review.linked_pr_merged
          - tracker.current
          - git.worktree_clean

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
        actions:
          - review.open: { role: worker }
      request_validation:
        from: [in_progress, review]
        to: validation
        validators: [review.complete]
      close:
        from: [validation]
        to: done
        description: "Closing requires attached evidence, complete child proof, and a clean worktree."
        validators:
          - evidence.attached: { min_count: 1 }
          - children.proof_complete
          - blockers.none_open
          - lint.none_blocking
          - tracker.current
          - git.worktree_clean

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
        description: "Closing requires complete review and current durable state."
        validators:
          - review.complete
          - tracker.current
```

Required top-level fields are `schema`, `schema_version`, `branch_policy`,
`issue_types`, `statuses`, and `workflows`. Unknown top-level fields are hard
errors. Obsolete top-level fields such as `branch_lifecycle`, `validators`,
`guidance_templates`, and `fields` are rejected. Obsolete transition fields
such as `effects` are rejected for the target contract.

## Branch Policy

`branch_policy` is the shared branch policy used by workflow commands, status
surfaces, PR validation, and branch/worktree helpers. It is derived from the
tracker graph rather than duplicated in command handlers:

- child issues under an epic use the nearest parent epic as branch owner;
- standalone issues own their issue branch;
- epics own their epic branch;
- child issue completion actions commit tracker state on the epic branch and do
  not merge to base; and
- standalone issue and epic completion actions integrate their owner branch to
  base when the workflow declares that action.

| Field | Rule |
| --- | --- |
| `base_branch` | Required non-empty Git branch name. |
| `merge_strategy` | Required. One of `squash`, `merge_commit`, or `fast_forward_only`. |
| `branch_templates.epic` | Required branch template for epic owners. |
| `branch_templates.issue` | Required branch template for standalone issue owners and exceptional issue worktrees. |

Branch templates support only `{{ issue.id }}` and `{{ issue.type }}`. In this
context, `issue` means the branch owner, not necessarily the child issue being
started or closed.

## Issue Types

`issue_types` is the repository issue type registry. It makes the set of valid
issue type names explicit before workflows claim coverage.

Issue type names use stable lowercase ASCII identifiers:
`^[a-z][a-z0-9_]*$`.

Each issue type object currently has one required field:

| Field | Rule |
| --- | --- |
| `label` | Required non-empty user-facing label. |

The starter registry defines `bug`, `epic`, `feature`, `spike`, `task`, and
`validation`. Repositories may add custom issue types by adding registry
entries and then covering them from exactly one workflow. Unknown issue types
in issue records are hard config or lint errors because the workflow resolver
cannot know which status and transition rules apply.

## Statuses And Categories

`statuses` is a shared catalog of named status objects. Status names use stable
lowercase ASCII identifiers: `^[a-z][a-z0-9_]*$`.

Each status object currently has one required field:

| Field | Rule |
| --- | --- |
| `category` | Required. One of `todo`, `active`, `blocked`, or `done`. |

Status categories are operator-facing summary buckets. They help commands
summarize work but do not replace workflow status in canonical issue records.
`review` and `validation` may be configured as user-facing workflow statuses or
issue types, but they are not required global categories.

Statuses in a workflow's `done_statuses` list are terminal for that workflow:

- every `done_statuses` value must exist in `statuses`;
- every terminal status must have category `done`; and
- no transition may leave a terminal done status.

## Workflow Applicability

Each workflow owns registry coverage through `applies_to`.

Every `issue_types` registry key must appear exactly once across all workflow
`applies_to` lists. Missing coverage, duplicate coverage, and workflow
references to an unregistered issue type are hard config errors.

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
| `validators` | Optional list of inline read-only built-in validators. |
| `actions` | Optional ordered list of inline built-in transition actions. |
| `description` | Optional static text rendered near transition options and blocked transition output. |

`description` is static text. There is no template registry and no template
variable expansion.

## Transition Actions

Transition actions are configured work run by explicit issue transitions after
required fields and validators pass. They are declared on a transition, planned
in declaration order, and rendered separately from validators so operators can
see what readiness checked and what the command intends to mutate.

Built-in actions are:

| Action | Purpose |
| --- | --- |
| `branch_prepare` | Create or check out the workflow-derived owner branch when the transition needs branch preparation. |
| `branch_commit` | Commit the transition's canonical tracker changes on the workflow-derived owner branch. |
| `branch_integrate` | Integrate the owner branch to the configured base branch using `branch_policy.merge_strategy`. |
| `review.open` | Open or reuse the branch owner's configured review artifact and write the canonical `review` link. |
| `review.link` | Normalize an existing configured provider review artifact and write the canonical `review` link. |

The workflow engine intrinsically writes the canonical issue status and
transition activity entry for a successful transition. That status write is not
a configurable action.

Review artifact actions use the configured review mode from `.atelier/config.toml`.
In room mode they create or reuse a native review room. In provider mode they
create, fetch, or link the configured provider artifact. They do not approve,
comment, request changes, resolve findings, merge review artifacts, close
issues, add `pr` aliases, or replace explicit `atelier issue transition`.

Review artifact actions require parameter objects. Room-mode actions declare
the local Atelier role:

```yaml
actions:
  - review.open: { role: worker }
```

Forgejo-backed provider actions also declare `provider: forgejo` and the role
author mapping used by the action's provider calls:

```yaml
actions:
  - review.open:
      provider: forgejo
      role: worker
      role_authors:
        worker: atelier-worker
        reviewer: atelier-reviewer
        validator: atelier-validator
        manager: atelier-manager
```

The Forgejo admin token value is not a workflow parameter. It remains a secret
read from the environment variable named by `.atelier/config.toml`.

Failure behavior is part of the action contract:

- Preflight failures stop before actions mutate state. This includes invalid
  source status, missing required fields, failed validators, invalid review mode,
  and invalid action configuration.
- Local write failures name the failed Markdown, activity, branch, commit, or
  integration step and leave recovery commands that can inspect the preserved
  state.
- Provider failures name the failed provider step and preserve local state only
  when retry is idempotent or the command can provide an explicit repair path.
- Idempotent retry must tolerate an already-created review artifact,
  already-written review link, already-applied activity entry, or already-made
  owner-branch commit.
- Recovery text must name the failed action, what state was preserved, and next
  commands such as `atelier issue show <id>`, `atelier issue transition <id>
  --options`, `atelier review status --issue <id>`, or `atelier lint <id>`.

## Validators

Transition validators use namespaced built-in names directly:

```yaml
validators:
  - blockers.none_open
  - tracker.current
```

Parameterized validators use single-key map syntax:

```yaml
validators:
  - evidence.attached: { min_count: 1 }
```

There is no top-level validator alias registry. Unknown validators, obsolete
flat validator names, and invalid params are hard config errors.
Validators must be read-only. They may inspect canonical records, projection
freshness, worktree state, evidence, blockers, and review artifacts, but they
must not write records, create commits, change branches, open reviews, or merge
anything. Mutating behavior belongs in transition actions.

Supported built-ins include:

| Validator | Purpose |
| --- | --- |
| `tracker.current` | Canonical state and local projection are current enough for the transition. |
| `issue.sections_parseable` | Issue Markdown sections can be parsed. |
| `evidence.attached` | Required evidence is attached; supports `min_count`. |
| `review.complete` | Required review artifact state is complete enough for the configured transition; the configured review provider remains the authority for approval rules and branch protection. |
| `children.proof_complete` | Child work is closed with validating proof. |
| `blockers.none_open` | Target has no open blockers. |
| `lint.none_blocking` | Blocking lint checks pass. |
| `git.worktree_clean` | Worktree cleanliness gate passes. |
| `review.linked_pr_merged` | The linked provider-local review artifact number, remote identity, source/target branches, and merged state match the Atelier workflow branch policy. |

Actions are not validators. They may mutate Git state, canonical review
records, provider review artifacts, and committed tracker state. Unknown
actions or action params unsupported by the built-in action
are hard config errors.

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
`review.linked_pr_merged` only to epic close, so validation issues and ordinary
child issues can close on their own proof while the epic remains the merged
review artifact boundary. In provider mode `review.linked_pr_merged` derives
provider host/owner/repo from `.atelier/config.toml` and expected source/target
branches from `branch_policy`. In room mode equivalent review readiness comes
from the room merge event rather than provider PR state.

`review.linked_pr_merged` is deliberately a fact check, not a second
review-provider policy engine. Atelier validates the review artifact link,
remote identity, branch match, and merged state because those facts decide
whether the local workflow gate is satisfied. The configured review provider
owns branch protection, required approvals, allowed merge strategies, and final
merge authorization. If a repository needs Atelier to enforce additional
review policy locally, that is a new product decision rather than an extension
of the starter workflow.

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
| `workflow_config_unknown_action` | Transition references an unsupported action. |
| `workflow_config_invalid_action` | Action params are malformed or unsupported for that built-in action. |
| `workflow_issue_field_invalid` | Canonical issue fields violate built-in workflow field rules. |
