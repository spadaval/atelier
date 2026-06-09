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
- Canonical projection: deterministic repo-state files that can rebuild the
  local SQLite runtime database.
- Chainlink: the inherited Rust CLI codebase this repository starts from.
- Evidence: a durable proof record for validation, such as test output, logs,
  screenshots, reports, or benchmark results.
- Workflow validator: a machine-readable transition check that controls whether
  a workflow transition can proceed and returns an actionable failure reason.
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
- SQLite state: fast local runtime state, currently inherited from Chainlink and
  targeted to live under `.atelier/`.

## Ambiguities

- The product name is Atelier. Use Chainlink for provenance and inherited
  behavior that is still intentionally documented as source history; use Atelier
  for the current package, binary, runtime directory, resources, and target-state
  product design.
- Export/import in the inherited code is backup-oriented. The target
  architecture needs canonical projection and rebuild semantics instead.
- Dependencies should represent actual sequencing. Use typed links for other
  relationships such as related, validates, implements, or evidenced_by.
- Missions, milestone checkpoint records, plans, evidence, and runs are target
  first-class concepts, not just labels on issues.
- Workflow validators belong to workflow policy, not to milestone records.
  Milestones own validation criteria; validators enforce transitions.
