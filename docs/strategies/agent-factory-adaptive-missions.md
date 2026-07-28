---
strategy: agent-factory-adaptive-missions
mission: atelier-5kej
revision: 1
status: active
sources:
  - PRODUCT_INTENT.md
  - CONTEXT.md
  - docs/product/mission-authoring.md
  - docs/product/work-model.md
  - docs/product/cli-surface.md
  - docs/architecture/markdown-first-record-store.md
  - /root/.agents/skills/agent-factory/SKILL.md
  - /root/.agents/skills/agent-factory/references/strategic-plans.md
  - /root/.agents/skills/agent-factory/procedures/plan.md
  - /root/.agents/skills/agent-factory/procedures/orchestrate.md
---

# Outcome

Atelier supports Agent Factory's adaptive mission model without becoming a
plan-approval or portfolio-management system. A Manager can see the governing
strategy, the outcome map, the current evidence-backed execution horizon, and
the conditions for expanding later work. As evidence changes, the Manager can
reconcile the implementation graph and continue safely without resubmitting or
freezing the whole plan.

# Target System

- Strategy is durable mission-scoped Markdown referenced by the mission. It
  owns the intended outcome, target shape, governing tradeoffs, boundaries,
  adaptation authority, and assurance.
- Missions and epics describe product outcomes. An epic may remain unexpanded
  when its constraints, unknowns, and expansion condition are clear.
- Ordinary issues are the current implementation hypothesis. They are created
  only through the nearest evidence boundary and own bounded executable work.
- Typed relationships remain the source of truth for mission scope,
  dependencies, and hierarchy.
- Normal Atelier views make the strategy reference, outcome map, executable
  horizon, blockers, and useful expansion signals legible. Plain Markdown
  remains a valid fallback.
- Managers may revise the implementation graph within this strategy's
  adaptation authority. Completed work and evidence remain historical facts;
  obsolete speculative work is superseded rather than rewritten away.

# Governing Decisions

1. Product requirements and implementation tasks are separate layers.
   Strategy and outcome records say what must become true. Issues say what one
   Worker can presently do to advance it.
2. Plan the full outcome, not the full task graph. Distant implementation work
   is not created merely to make a mission appear complete.
3. Prefer conventions and presentation over new persistent machinery. Add
   schema or commands only when an observed operator scenario cannot be made
   clear with existing Markdown and typed relationships.
4. Replanning is ordinary Manager work. It does not require an approval state,
   authenticated planning actor, graph digest, or provenance ceremony.
5. Strategy revision and implementation replanning are different. Changes to
   outcome, target system, governing tradeoffs, boundaries, assurance, or
   adaptation authority return to a commissioned strategy session. Changes to
   issues, dependencies, sequencing, proof, or expansion within those bounds
   do not.
6. Atelier owns tracker mechanics and repository-specific command guidance.
   Agent Factory owns portable orchestration behavior and routes to Atelier
   rather than duplicating its command manual.

# Boundaries

- Do not introduce first-class plan records, a `.atelier/plans/` lifecycle, or
  an always-current desired-state manifest.
- Do not require whole-mission task completeness before execution.
- Do not add planning approval, reviewer identity, actor authorization, graph
  freezing, exact graph digests, or stale-plan invalidation.
- Do not turn expansion conditions into workflow gates. They are coordination
  information for Managers.
- Do not redesign code review, provider authentication, evidence capture, or
  mission publication except where their existing presentation directly
  contradicts the adaptive planning model.
- Do not preserve obsolete planning commands or compatibility shims unless a
  human explicitly requests a compatibility window.

# Adaptation

## Must Preserve

- Atelier remains a pleasant interface over repository planning files.
- Durable strategy is distinguishable from the mutable implementation graph.
- Mission scope remains direct `advances` roots plus their descendants.
- Dependencies represent real execution prerequisites.
- Completed work, failed proof, and material replanning history remain
  inspectable.
- Operators can continue from durable repository state without private chat
  history.

## Managers May Change

- The exact Markdown conventions used to reference strategy or describe an
  epic's expansion condition.
- CLI wording and the placement of strategy, horizon, and expansion information
  in normal mission views.
- The current issue decomposition, dependency graph, sequencing, proof
  expectations, and ready-work buffer.
- Whether a demonstrated gap needs documentation, presentation, workflow, or
  persistence changes.
- The order and size of increments after the contract epic supplies evidence.

## Return To Strategy When

- Supporting the operator scenarios appears to require first-class plan state,
  a new planning lifecycle, or authorization.
- Evidence shows that strategy cannot remain ordinary repository Markdown.
- Mission scope semantics or the product's core record model must change.
- The distinction between strategy revision and implementation replanning
  proves unusable in practice.
- Assurance would require a mandatory gate that conflicts with Atelier's
  anti-red-tape boundary.

# Assurance

The mission must demonstrate, from a fresh repository state, that a Manager
can:

1. Create a mission that references a governing strategy and advances
   outcome-oriented epics.
2. Inspect the strategy revision, outcome map, current ready horizon, blockers,
   and an unexpanded epic's expansion condition through normal product
   surfaces.
3. Complete an increment, use its evidence to expand the next epic, and add or
   resequence only the newly justified issues.
4. Supersede obsolete speculative work while preserving completed work and
   evidence.
5. Continue execution without plan approval, actor identity, graph digest,
   frozen-plan checks, or whole-graph completeness requirements.
6. Follow repository-owned Agent Factory guidance that agrees with the evolved
   global skill and routes tracker mechanics through Atelier.

Focused tests, CLI scenarios, documentation residue checks, and independent
behavior validation must establish these claims. A broad green suite alone is
not sufficient.

# Revision History

- Revision 1 — Established the lightweight adaptive mission model after
  removing the independent plan-review subsystem. The initial execution
  horizon is the contract epic; later epics expand only after that contract and
  its red-line validation produce evidence.
