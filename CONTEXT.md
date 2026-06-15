# Context

## Domain Terms

- Agent: an AI or human operator performing work in the repository.
- Agent-factory: the coordinated operating model used to plan, assign,
  validate, review, and hand off agent work.
- Atelier: the product and live repository tracker: a local-first, agent-native work tracker for
  complex software missions.
- Beads: the predecessor tracker replaced by Atelier. Beads data was imported
  and the repository archive was purged; only the external `import-beads`
  input format remains supported.
- Canonical record tree: deterministic tracked Markdown files under `.atelier/`
  that can rebuild the local SQLite projection/runtime database.
- RecordStore: the target component that owns canonical Markdown record reads,
  writes, validation, deterministic rendering, and ID allocation.
- ProjectionIndex: the target rebuildable SQLite index derived from
  RecordStore records for global queries, graph traversal, search, validation,
  and Mission Control inputs.
- RuntimeState: local-only ignored data under `.atelier/runtime/` and
  `.atelier/cache/` such as current work association, sessions used by that
  association, agent identity, diagnostics, locks, and UI caches. It can
  reference canonical IDs but is not the durable project record source.
- Local command diagnostics: user-local command telemetry used for performance
  and failure analysis. It is RuntimeState-adjacent diagnostic data, not a
  canonical work record or exported run/session record.
- Chainlink: the inherited Rust CLI codebase this repository starts from.
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
- Issue: a durable accountability unit. It does not have to map one-to-one to an
  agent run.
- Milestone: a validated intermediate checkpoint state with scope boundaries,
  validation criteria, accepted evidence, and completion state. It is not a
  work container or super-epic.
- Mission: a high-level objective that may span multiple issues, milestones,
  plans, agents, and runs.
- Mission Control: the target projection or UI surface that summarizes active
  missions, checkpoint progress, blockers, agents, workflow validator failures,
  and evidence.
- Plan: durable execution intent that matters beyond ephemeral context.
- Run: execution metadata for a session or slice of work, not the primary unit
  of product planning.
- Active work: the local runtime association between an agent, an issue, and
  the current branch/worktree for in-progress execution. It is coordination
  state, not the durable workflow status of the issue.
- Abandon: an explicit action that clears active work without claiming
  completion or changing issue workflow status.
- SQLite state: fast local projection and runtime state, currently inherited
  from Chainlink and currently living at ignored `.atelier/state.db`.

## Ambiguities

- The product name is Atelier. Use Chainlink for provenance and inherited
  behavior that is still intentionally documented as source history; use Atelier
  for the current package, binary, runtime directory, resources, and target-state
  product design.
- Export/import in the inherited code is backup-oriented. The target
  architecture needs canonical projection and rebuild semantics instead.
- The canonical-state target is Markdown-first in a single `.atelier/` tree:
  successful durable mutations should write record files through RecordStore,
  then refresh ProjectionIndex. SQLite is not the destination source of truth
  for canonical records.
- Dependencies should represent actual sequencing. Canonical state groups
  record relationships under `relationships`: use `blocks` for readiness,
  `children` for hierarchy and mission work, `attachments` for plans/evidence,
  and `relates` for peer semantic relationships.
- Missions, milestone checkpoint records, plans, evidence, and runs are target
  first-class concepts, not just labels on issues.
- Validators belong to workflow policy, not to milestone records. Milestones
  own validation criteria; validators enforce transitions.
