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
  transitions, terminal states, validators, and guidance.
- Workflow status: the canonical issue `status` value defined by workflow
  policy. It is durable repository state, not a derived summary or a local
  runtime marker.
- Status category: derived orientation metadata that groups workflow statuses
  into stable operator-facing buckets such as ready, active, blocked, done, or
  archived. Categories help commands summarize work but do not replace workflow
  status in durable state or transition checks.
- Transition: a named workflow action that moves a record from one workflow
  status to another after required fields, evidence, and validators succeed.
- Validator: a machine-readable workflow transition check that controls whether
  a transition can proceed and returns an actionable failure reason.
- Guidance: advisory workflow text rendered near an action, status, or failure
  to explain the next operator move. Guidance informs; validators decide.
- Mission: a high-level objective that may span multiple epics, issues,
  evidence records, agents, and deferred session/run metadata. It is also the
  shared background workspace boundary: one mission normally owns one shared
  worktree for coordinated agent work.
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
- Plan: execution intent that matters beyond ephemeral context. In v1, plans are
  ordinary Markdown artifacts or prose referenced from accountable work or
  evidence; they are not first-class `.atelier/plans/` records.
- Run: execution metadata for a session or slice of work, not the primary unit
  of product planning.
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
- Missions, issues, evidence, workflow policy, and activity sidecars are the v1
  first-class durable concepts. Milestone/checkpoint records, first-class plan
  records, and runs/sessions remain deferred until a later contract reintroduces
  them directly.
- Validators belong to workflow policy. Checkpoint or plan prose may describe
  desired proof, but validators enforce issue transitions.
- Durable claim/assignment and current work are easy to confuse. Current work
  is derived from canonical `in_progress` issue status in the checkout's
  tracked Markdown records, not from runtime work associations or a parallel
  hidden claim system.
- Workspace, branch, and review boundaries are distinct. Missions own shared
  worktrees/background checkouts, epics own reviewable branches, and ordinary
  issues own local implementation proof. Per-issue worktrees or branches are
  exceptional isolation tools, not the default assignment model.
- Branch lifecycle is workflow-owned rather than a separate routine setup step.
  `atelier start <id>` prepares the owner branch from the work graph: child
  issues use the nearest parent epic branch, standalone issues use an issue
  branch, and epics use an epic branch. `atelier issue close <id>` commits the
  close state on the owner branch; child issues stop there, while standalone
  issues and epics merge that owner branch to the configured base branch.
  Squash merge is the default integration strategy, with repository policy able
  to select alternatives and branch naming templates. A failed close-time
  commit or merge must not leave the item closed in the integration branch.
- The layered Cargo workspace is the target architecture, not a parallel
  scaffold. The repository root is a virtual workspace; remaining monolithic
  modules under `crates/atelier-cli/src/` are migration input for lower crates,
  and old root-package compatibility paths must not return.
