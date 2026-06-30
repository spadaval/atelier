# Context

## Domain Terms

- Agent: an AI or human operator performing work in the repository.
- Agent-factory: the coordinated operating model used to plan, assign,
  validate, review, and hand off agent work.
- Atelier: the product and live repository tracker: a local-first, agent-native work tracker for
  complex software missions.
- Beads: the predecessor tracker replaced by Atelier. Beads data was imported
  and the repository archive was purged; external Beads data is migration input,
  not ongoing Atelier state.
- Beads migration input: repo-local `.beads/issues.manual.jsonl` data that
  `atelier init --import-beads` may import when explicitly requested.
- Canonical record tree: deterministic tracked Markdown files under `.atelier/`
  that can rebuild the local SQLite projection database.
- RecordStore: the target component that owns canonical Markdown record reads,
  writes, validation, deterministic rendering, and ID allocation.
- ProjectionIndex: the target rebuildable SQLite index derived from
  RecordStore records for global queries, graph traversal, search, validation,
  and Mission Control inputs. It may keep covered-index metadata but not
  project facts that cannot be recreated from canonical Markdown.
- Local diagnostics and cache: local-only ignored files such as command
  diagnostics, locks, and UI caches. These are not SQLite tracker state and
  must not define durable project records or current work.
- Local command diagnostics: user-local command telemetry used for performance
  and failure analysis. It is local diagnostic data, not a canonical work
  record, SQLite tracker state, or exported run/session record.
- Chainlink: the inherited Rust CLI codebase this repository starts from.
- Virtual workspace root: the target repository root `Cargo.toml` shape after
  the crate migration. It owns workspace membership and shared package metadata,
  not a root `atelier` library or binary package.
- Atelier crate layers: the target internal Rust crates under `crates/`:
  `atelier-core` for pure domain types, `atelier-workflow` for workflow policy,
  `atelier-records` for canonical Markdown storage, `atelier-sqlite` for
  rebuildable projection SQLite state, `atelier-app` for
  use-case orchestration, and `atelier-cli` for the public `atelier` binary.
- Evidence: a durable proof record for validation, such as test output, logs,
  screenshots, reports, or benchmark results.
- Strong proof: claim-specific evidence that is reproducible from durable
  repository state or recorded transcripts, attached to the accountable work,
  classified with a result, scoped to the claim being made, and independently
  validated when the claim's risk requires a different reviewer.
- Weak proof: evidence that is too broad, summary-only, unattached,
  unverifiable, stale, or not mapped to the specific claim it is supposed to
  prove. Broad green test suites and mission summaries can support strong proof,
  but they are weak when they are the only proof for a concrete outcome.
- Workflow: repository-owned policy that defines issue workflow statuses,
  transitions, terminal states, validators, descriptions, and applicability.
- Issue type registry: the repository-owned `issue_types` map in workflow
  policy. It defines valid issue type names and user-facing labels before
  workflows claim coverage.
- Workflow applicability: the issue type coverage owned by each workflow through
  `workflows.*.applies_to`. Every registered issue type must be covered exactly
  once by the committed workflow policy.
- Workflow status: the canonical issue `status` value defined by workflow
  policy. It is durable repository state, not a derived summary or a local
  runtime marker.
- Status category: derived orientation metadata that groups workflow statuses
  into stable operator-facing buckets such as todo, active, blocked, or done.
  Categories help commands summarize work but do not replace workflow status in
  durable state or transition checks. `review` and `validation` are issue types
  or workflow statuses when configured, not required global categories.
- Transition: a named workflow rule that moves a record from one workflow
  status to another after required fields, evidence, and validators succeed.
- Transition action: configured work run by an explicit issue transition after
  required fields and validators succeed. Actions may write canonical tracker
  records, local branch commits, review artifact links, provider requests, or
  provider-owned merge/sync operations, but they are scoped to the transition
  that declares them and are not hidden automation hooks.
- Validator: a machine-readable workflow transition check that controls whether
  a transition can proceed and returns an actionable failure reason.
- Transition description: static workflow text rendered near an action, status,
  or failure to explain the next operator move. Descriptions inform;
  validators decide.
- Base branch: the named integration branch from `branch_policy.base_branch`.
  It is the repository default target for owner-branch integration unless an
  explicit mission or epic branch base is recorded by workflow actions.
- Work branch: the Git branch that owns mutation and review for a branch-owning
  record. Its canonical name is `<issue_type>/<issue_id>`, such as
  `epic/atelier-li5h` or `mission/atelier-sszj`; the issue type is the
  registry key and the issue ID is the canonical record ID.
- Branch base: the recorded branch/ref and commit from which a work branch was
  prepared. Review targets, sync, and integration checks use the recorded
  branch base instead of recomputing a target from the current parent graph.
- Mission integration branch: an optional mission work branch named
  `mission/<issue_id>`. It exists only when the mission workflow opts in through
  explicit validators and branch actions; missions do not implicitly create a
  hidden branch, and mission scope is still defined by direct `advances` links.
- Review artifact action: the v1 transition action that opens or links the
  configured review artifact for the branch-owning issue or epic and writes the
  canonical `review` field. It follows the active review mode and must not
  merge, approve, comment on, or close review or issue workflow by itself.
- Mission: a high-level objective declared through repository workflow policy.
  A mission may span multiple epics, issues, evidence records, agents, and
  deferred run metadata, but it does not own a hidden command lifecycle,
  active-focus pointer, or universal evidence rule outside the workflow model.
  A mission may still be the shared background workspace boundary when
  repository workflow and operator assignment choose that coordination shape.
- Epic: the normal branch and review boundary beneath a mission. One epic
  normally owns one reviewable branch or PR-equivalent changeset.
- Issue: a durable accountability unit and implementation slice. It does not
  have to map one-to-one to an agent run, worktree, branch, or independent
  review.
- Blocking relationship: an issue-owned relationship where one issue prevents
  another issue from being ready.
- Milestone/checkpoint semantics: a deferred v1 concept for validated
  intermediate target states. Checkpoint intent may be described in mission,
  epic, issue, or evidence bodies, but there is no active first-class
  `.atelier/milestones/` record table or milestone command surface.
- Mission Control: the target projection or UI surface that summarizes active
  missions, checkpoint progress, blockers, agents, workflow validator failures,
  and evidence.
- Status role: optional workflow policy on an active issue status that names
  the role currently responsible for that work state. Status roles are declared
  in `.atelier/workflow.yaml` and are surfaced by `atelier status`, role guides,
  and review attribution.
- Review mode: the repository-wide review backend selected in
  `.atelier/config.toml`. Exactly one mode is active for a project:
  `room` for native Atelier review rooms or `provider` for a hosted
  PR-equivalent provider such as Forgejo.
- Review artifact: the configured review object linked from issue state
  through the canonical `review` field. In room mode this is a native
  `.atelier/reviews/<id>.yaml` room. In provider mode this is the normalized
  provider-local review number plus provider kind. Review artifacts are code
  review workspaces, not Atelier workflow transitions, and they do not replace
  evidence records.
- Review room: a native Atelier review artifact stored as tracked YAML under
  `.atelier/reviews/<id>.yaml`. Its current state is derived from room metadata
  plus ordered events such as comments, findings, approvals, change requests,
  resolutions, stale-approval invalidations, and merge.
- Room event: an append-only review-room timeline entry. Events are the durable
  source for comments, decisions, approvals, finding resolution, stale approval
  invalidation, and merge state.
- Finding: a review-room decision event that names a blocking or non-blocking
  problem. Blocking findings must be resolved before room merge can succeed.
- Approval: a review-room decision event that can satisfy merge readiness until
  it becomes stale. New commits or change-request events invalidate previous
  approvals rather than mutating them in place.
- Merge authority: the `atelier review merge` boundary. In room mode it checks
  room approvals, stale approvals, blocking findings, and expected branch state
  and records a room merge event. In provider mode it delegates merge or merge
  confirmation to the configured provider. In both modes it does not transition
  Atelier issue workflow.
- Provider terminal actions: provider-backed workflow actions such as
  `tracker.commit`, `branch.push`, `review.merge`, and `base.sync` that make
  the provider review artifact and remote base branch the integration
  authority. They are not aliases for local `branch_integrate`.
- Plan: execution intent that matters beyond ephemeral context. In v1, plans are
  ordinary Markdown artifacts or prose referenced from accountable work or
  evidence; they are not first-class `.atelier/plans/` records.
- Run: execution metadata for a slice of work, not the primary unit of product
  planning.
- Graph: the cross-record relationship shape among missions, issues, blockers,
  evidence, and other first-class records.
- Current work: the set of canonical issue records in one checkout's tracked
  `.atelier/` tree whose workflow status is `in_progress`, interpreted with the
  mission worktree and epic branch context visible in that checkout. Different
  Git worktrees may legitimately have different current-work sets until their
  Markdown records reconcile through Git.
- Abandon: a legacy active-pointer cleanup concept that should not remain the
  normal way to leave current work once status-derived current work replaces
  runtime work associations.
- SQLite state: fast local projection state, currently living at ignored
  `.atelier/runtime/state.db`.
- Doctor: an operator health surface that reports whether the repository and
  local runtime are usable and may perform safe repair when explicitly asked.

## Ambiguities

- The product name is Atelier. Use Chainlink for provenance and inherited
  behavior that is still intentionally documented as source history; use Atelier
  for the current package, binary, runtime directory, resources, and target-state
  product design.
- Export/import in the inherited code is backup-oriented. The target
  architecture needs canonical projection and rebuild semantics instead.
- Export and rebuild are low-level diagnostic mechanics, not normal operator
  workflow. Cache and projection state should be transparent and repaired by
  ordinary commands or by an explicit doctor repair path.
- Beads migration is explicit during setup. `atelier init` may detect the
  standard repo-local Beads migration input, but import requires an explicit
  setup option rather than a silent automatic conversion.
- Doctor repair may change ignored projection/cache state but must not edit
  tracked `.atelier/` canonical records.
- Graph commands should inspect cross-record relationships, including missions
  and issues. If a view is issue-only, its help should say so explicitly.
- The canonical-state target is Markdown-first in a single `.atelier/` tree:
  successful durable mutations should write record files through RecordStore,
  then refresh ProjectionIndex. SQLite is not the destination source of truth
  for canonical records.
- Blocking relationships represent issue readiness, not a separate dependency
  domain. Canonical state groups record relationships under `relationships`:
  use `blocks` for issue-owned blockers, `children` for hierarchy and mission
  work, `attachments` for evidence, and `relates` for peer semantic
  relationships.
- Missions, issues, evidence, workflow policy, review artifacts, and activity
  sidecars are the v1 first-class durable concepts. Milestone/checkpoint
  records, first-class plan records, sessions, attempts, and runs remain
  deferred until a later contract reintroduces them directly.
- Validators belong to workflow policy. Checkpoint or plan prose may describe
  desired proof, but validators enforce issue transitions.
- Durable claim/assignment and current work are easy to confuse. Current work
  is derived from active-category issue status in the checkout's tracked
  Markdown records, not from runtime work associations, sessions, attempts, or
  a parallel hidden claim system.
- Status roles and local command diagnostics serve different purposes. Status
  roles are committed workflow policy for active work ownership. Local command
  diagnostics are ignored runtime telemetry for command health and are not
  exported work records.
- Review artifacts and validators are distinct. `atelier review` commands
  operate on native rooms or provider-backed review artifacts and record their
  issue or epic linkage, while workflow validators such as
  `review.linked_pr_merged` only read review state to decide whether an Atelier
  transition is allowed.
- Review artifact actions and review commands are distinct. A workflow
  transition may declare an action that opens or links the branch owner's review
  artifact after validators pass, but approval, comments, request-changes,
  finding resolution, merge, and workflow status changes remain owned by
  explicit review and issue commands.
- Review command role attribution is status-derived by default. Mutating review
  commands use explicit `--role` when supplied; otherwise they infer the role
  from the linked owner issue's current `status.role` and fail if that status
  has no configured role.
- Review links and evidence attachments are distinct. The `review` field stores
  the active review artifact for a branch-owning issue or epic; evidence
  attachments prove claims with command transcripts, review summaries, or
  validation records. Legacy `pull_request` fields are migration input, not the
  durable target shape.
- Workspace, branch, and review boundaries are distinct. Missions own shared
  worktrees/background checkouts when declared as the coordination objective,
  epics own reviewable branches, and ordinary issues own local implementation
  proof. Per-issue worktrees or branches are exceptional isolation tools, not
  the default assignment model.
- Mission lifecycle and closeout policy is workflow-owned. The CLI may provide
  objective graph summaries and migration helpers, but the valid statuses,
  transitions, validators, and evidence requirements for mission-shaped work
  come from `.atelier/workflow.yaml`.
- Branch policy is workflow-owned rather than a separate routine setup step.
  `atelier start <id>` prepares the owner branch from the work graph: child
  issues use the nearest parent epic branch, standalone issues use an issue
  branch, and epics use an epic branch. Terminal transitions commit tracker
  state through explicit workflow actions. Child issues normally stop at the
  owner branch; standalone issues and epics use provider terminal actions in
  provider mode or explicit `branch_integrate` in local room mode. Squash merge
  is the default local integration strategy, with repository policy able to
  select merge alternatives but not configurable branch-name templates. Owner
  branches use canonical `<issue_type>/<issue_id>` names, and optional mission
  integration branches are created only by configured workflow actions. A failed
  close-time commit or merge must not leave the item closed in the integration
  branch.
- The layered Cargo workspace is the target architecture, not a parallel
  scaffold. The repository root is a virtual workspace; remaining monolithic
  modules under `crates/atelier-cli/src/` are migration input for lower crates,
  and old root-package compatibility paths must not return.
