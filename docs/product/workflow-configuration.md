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
| `.atelier/config.toml` | Tracked project config: project schema/version, `project_slug`, canonical `state_root`, prune defaults, configured custom context-only issue link types, active review mode, provider backend identity, and provider remote coordinates. | Issue statuses, transitions, validators, branch naming, required transition fields, provider secret values, local runtime paths or contents, diagnostics, or locks. |
| `.atelier/workflow.yaml` | Tracked workflow policy: branch policy, status catalog, workflow applicability, transitions, terminal statuses, required transition fields, read-only validators, and static descriptions. | Side effects of any kind, provider host/owner/repo/token settings, local path overrides, or hidden defaults. |
| `~/.config/atelier.toml` | User-local global config, including plaintext provider secrets such as `review.providers.forgejo.admin_token`. | Durable project records, project policy, workflow policy, provider remote coordinates, diagnostics, or locks. |
| Local runtime and environment | Ignored machine-local state under `.atelier/runtime/` (the canonical mutation lock), local diagnostics, and optional environment variables for local behavior overrides such as diagnostics and agent labels. | Durable project records or project policy. Runtime state must be disposable, and environment variables must not be required for provider credentials. |

Custom issue links belong to `.atelier/config.toml` because they are project
vocabulary, not workflow behavior:

```toml
[issue_links]
custom_context_types = ["references", "informs"]
```

Configured custom link types are accepted by `atelier issue link --role <type>`
and stored in issue `relationships.relates[]`. They are context-only: commands
may display, search, preserve, and unlink them, but they do not affect mission
progress, readiness, blockers, branch ownership, review ownership, or workflow
transition validators. Built-in workflow-driving roles such as `advances` and
`blocked_by` remain hard-coded semantics. Evidence `validates` is reserved for
evidence attachments and is rejected as an issue-to-issue link role. Unknown
custom link roles are rejected until listed in
`issue_links.custom_context_types`.

## Operator Surface

Issue workflow execution is explicit:

```text
atelier issue transition <id> <transition>
atelier issue transition <id>
atelier start <id>
```

`atelier issue transition <id>` renders transitions available from
the issue's current status. Each option reports whether the transition is
currently allowed, configured read-only validator results, static transition
descriptions, and the next command to run. A blocked attempt reports its failed
requirements without writing anything. A successful attempt writes the
canonical issue record and nothing else: transitions never commit, push, merge,
or switch branches ([ADR 0019](../adr/0019-verified-repo-native-tracker.md)).

`atelier start <id>` is the one explicit side-effecting convenience. It
resolves the issue's branch owner (see [Branch Policy](#branch-policy)), runs
the `start` transition's checks when the workflow offers `start` from the
current status, and only then acts: it refuses a dirty checkout when it has to
switch branches, creates the owner branch from the current `HEAD` or checks out
the existing one, records the branch and its base on the owner record, and
applies `start`. It prints every Git operation it runs and never commits;
agents commit the record changes with the work they describe. Rerunning it
is a no-op once the branch is checked out and recorded. Missions have no
branch, so `atelier start <mission-id>` fails.

## Scope

Workflow policy applies to issue records, including mission-shaped objectives
whose front matter declares `issue_type: mission`. The contract defines:

- a required branch policy for the base branch and merge strategy;
- a repository-defined issue type registry;
- a shared status catalog with explicit status categories;
- named issue workflows and their allowed transitions;
- terminal done states for each workflow;
- workflow-owned issue type applicability;
- inline read-only built-in validators and validator params;
- optional static transition descriptions; and
- strict configuration errors for invalid or obsolete config.

Evidence and future durable record lifecycles stay outside
`.atelier/workflow.yaml`. Mission lifecycle is workflow-owned because missions
are issue records in v3, not a separate record kind or command namespace.

`blockers.transitive_none_open` walks only declared `blocked_by` dependency
edges. It rejects an incomplete direct or transitive prerequisite with the full
path from the issue being started, and rejects a dependency cycle with its
cycle path instead of treating the graph as ready. A prerequisite is complete
only when its status is listed in its issue type workflow's `done_statuses`;
the shared status category alone does not make it terminal. Mission `advances`
scope and ordinary hierarchy are not dependency edges, so incomplete internal
mission work does not block starting the mission shell.

## Fixed V3 Shape

The file is strict YAML with explicit schema identity. `atelier init` writes
this starter policy:

```yaml
schema: atelier.workflow
schema_version: 3
branch_policy:
  base_branch: main
  merge_strategy: squash
issue_types:
  bug:
    label: Bug
  epic:
    label: Epic
  feature:
    label: Feature
  mission:
    label: Mission
  spike:
    label: Spike
  task:
    label: Task
  validation:
    label: Validation
statuses:
  draft:
    category: todo
  todo:
    category: todo
  in_progress:
    category: active
  blocked:
    category: blocked
  review:
    category: active
  done:
    category: done
  superseded:
    category: done
workflows:
  objective:
    applies_to:
    - mission
    - epic
    initial_status: draft
    done_statuses:
    - done
    - superseded
    transitions:
      ready:
        from: [draft]
        to: todo
        description: The outcome, immediate work, and proof route are clear enough to execute.
        validators:
        - issue.sections_parseable
      start:
        from: [todo, blocked]
        to: in_progress
        description: Start work.
        validators:
        - blockers.transitive_none_open
      block:
        from: [todo, in_progress, review]
        to: blocked
        description: Record a blocker.
      request_review:
        from: [in_progress]
        to: review
        description: Ready for independent review.
      revise:
        from: [review]
        to: in_progress
        description: Return to active work after review findings.
      close:
        from: [in_progress, review]
        to: done
        required_fields: [close_reason]
        description: Done claim; commit it on the work branch before review.
        validators:
        - blockers.none_open
      supersede:
        from: [draft, todo, blocked, in_progress]
        to: superseded
        required_fields: [close_reason]
        description: Retire work that evidence made obsolete.
  work:
    applies_to:
    - bug
    - feature
    - spike
    - task
    - validation
    initial_status: todo
    done_statuses:
    - done
    - superseded
    transitions:
      start:
        from: [todo, blocked]
        to: in_progress
        description: Start work.
        validators:
        - blockers.transitive_none_open
      block:
        from: [todo, in_progress, review]
        to: blocked
        description: Record a blocker.
      request_review:
        from: [in_progress]
        to: review
        description: Ready for independent review.
      revise:
        from: [review]
        to: in_progress
        description: Return to active work after review findings.
      close:
        from: [in_progress, review]
        to: done
        required_fields: [close_reason]
        description: Done claim; commit it on the work branch before review.
        validators:
        - blockers.none_open
      supersede:
        from: [todo, blocked, in_progress]
        to: superseded
        required_fields: [close_reason]
        description: Retire work that is no longer needed.
```

Required top-level fields are `schema`, `schema_version`, `branch_policy`,
`issue_types`, `statuses`, and `workflows`. Unknown fields at any level are
hard errors, including obsolete top-level fields such as `branch_lifecycle`,
`validators`, `guidance_templates`, and `fields`, and obsolete transition or
status fields such as `effects`, `actions`, and `role`.

## Branch Policy

Branch ownership is derived from the tracker graph:

- child issues under an epic work on the nearest parent epic's branch;
- standalone issues own their issue branch;
- epics own their epic branch; and
- missions own no branch; they are coordinated, not worked on a branch.

| Field | Rule |
| --- | --- |
| `base_branch` | Required non-empty Git branch name. |
| `merge_strategy` | Optional. One of `squash` (default), `merge_commit`, or `fast_forward_only`. |

`base_branch` is the repository integration branch used when no recorded base
applies. It is configuration, not a statement that every transition must run
from that branch.

Work branch names are canonical and are not configurable:
`<issue_type>/<issue_id>`, for example `epic/atelier-4p7q` or
`task/atelier-z1p8`. The issue type is the registered `issue_types` key and the
issue ID is the canonical record ID. Slugs and branch templates are rejected.

`atelier start` records the owner's branch and base in the owner's
`workflow_branch` field:

```yaml
fields:
  workflow_branch:
    work_branch: epic/atelier-4p7q
    branch_base: main
```

The base is the branch that was checked out when `start` created the work
branch. A recorded branch wins over the derived name.

## Issue Types

`issue_types` is the repository issue type registry. It makes the set of valid
issue type names explicit before workflows claim coverage.

Issue type names use stable lowercase ASCII identifiers:
`^[a-z][a-z0-9_]*$`.

Each issue type object currently has one required field:

| Field | Rule |
| --- | --- |
| `label` | Required non-empty user-facing label. |

The starter registry defines `bug`, `epic`, `feature`, `mission`, `spike`,
`task`, and `validation`. Repositories may add custom issue types by adding registry
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
`review` may be configured as a user-facing workflow status, but it is not a
required global category.

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
| `objective` | `mission`, `epic` |
| `work` | `bug`, `feature`, `spike`, `task`, `validation` |

## Transitions

Each workflow defines named transitions:

| Field | Rule |
| --- | --- |
| `from` | Required non-empty list of source statuses. Each status must exist and must not be terminal for the workflow. |
| `to` | Required destination status. It must exist. |
| `required_fields` | Optional list of required command inputs. Currently `close_reason` is supported. |
| `validators` | Optional list of inline read-only built-in validators. |
| `description` | Optional static text rendered near transition options and blocked transition output. |

`description` is static text. There is no template registry and no template
variable expansion.

## Validators

Transition validators use namespaced built-in names directly:

```yaml
validators:
  - blockers.none_open
  - git.worktree_clean
```

Parameterized validators use single-key map syntax:

```yaml
validators:
  - evidence.attached: { min_count: 1 }
```

There is no top-level validator alias registry. Unknown validators, obsolete
flat validator names, and invalid params are hard config errors.
Validators must be read-only. They may inspect canonical records, worktree
state, evidence, and blockers, but they must not write records, create commits,
or change branches. There are no transition actions.
Record-file validity is checked whenever a command loads the records; it is
not a user-configurable workflow validator.

The built-in validators are:

| Validator | Purpose |
| --- | --- |
| `issue.sections_parseable` | The issue's required Markdown sections are present and non-empty. |
| `blockers.none_open` | The issue has no open direct `blocked_by` blockers. |
| `blockers.transitive_none_open` | The issue's declared dependency closure is complete and acyclic. |
| `evidence.attached` | Passing validating evidence is attached; supports `min_count` and `kind`. |
| `lint.none_blocking` | Blocking lint checks pass. |
| `git.worktree_clean` | The Git checkout has no uncommitted or untracked changes. |
| `work.terminal` | Every child and, for missions, all work linked by `advances` (with its children) is in a done-category status. |

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

The current room status is derived from metadata plus ordered events. Record
files must not store a second mutable summary that can drift from the event
timeline.

Provider-mode `review open --existing` inputs may accept a review number or a
full provider URL, but canonical issue records store only the normalized
structured field. URL inputs must match the configured review provider, host,
owner, and repository before they normalize to a number. The current provider
implementation is Forgejo. This is a recovery/import path; routine `review
open` derives creation fields from issue and workflow state.

The active review link belongs to the branch-owning issue or epic. Child issues
inherit the nearest parent epic's `review`; defining `review` directly on a
child issue is invalid unless the child owns its own branch by policy. Legacy
top-level `pull_request` fields are migration input only: migrated records must
render the structured `review` field, and strict validation rejects the old
shape after migration. The starter policy requires `review.complete` for the
epic and validation review flow, while ordinary child issues close on their own
proof. Repositories that require an epic close to verify merged provider state
may add `review.linked_pr_merged` to that close transition. In provider mode
`review.linked_pr_merged` derives provider host/owner/repo from
`.atelier/config.toml` and expected source/target branches from `branch_policy`.
In room mode equivalent review readiness comes from the room merge event rather
than provider PR state.

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
validation, mission publish readiness, epic closeout, and proof summaries
remain in canonical records. Native Markdown comments or activity sidecars may
capture durable notes, but they are not a second PR system and do not satisfy
review-provider merge gates.

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
