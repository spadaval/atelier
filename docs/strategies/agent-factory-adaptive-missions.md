---
strategy: agent-factory-adaptive-missions
mission: atelier-5kej
revision: 2
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
  adaptation authority, and mission-level assurance. It is not a backlog or a
  container for user stories.
- Epics are outcome-bearing delivery increments. The outcome may be product,
  system, architecture, operational, quantitative, destructive, or explicitly
  behavior-preserving, but it must fit one coherent branch, review, and parent
  validation boundary.
- Draft epics form the visible provisional outcome map. A draft has a
  meaningful outcome and known uncertainty, but it need not have child work,
  an implementation route, or exact proof criteria and is not executable.
- Ready epics and their ordinary issues are the current implementation
  hypothesis. Expanding an epic refines its outcome, preservation claims,
  immediate work, dependencies, and proof route together through the nearest
  evidence boundary.
- Typed relationships remain the source of truth for mission scope,
  dependencies, and hierarchy.
- Normal Atelier views make the strategy reference, outcome map, executable
  horizon, blockers, and useful expansion signals legible. Plain Markdown
  remains a valid fallback.
- Managers may revise the implementation graph within this strategy's
  adaptation authority. Completed work and evidence remain historical facts;
  obsolete speculative work is superseded rather than rewritten away.

# Governing Decisions

1. Governing strategy and delivery planning are separate layers. Strategy says
   what governs every plausible route. An epic says what one coherent increment
   must make true. An ordinary issue says what one Worker can presently do to
   advance it.
2. Plan the full outcome, not the full task graph. Distant implementation work
   is not created merely to make a mission appear complete.
3. Use one epic type for product-facing and pure-engineering outcomes. Vary the
   authored outcome and proof, not the lifecycle shape, unless evidence later
   demonstrates a genuinely different workflow.
4. Prefer workflow state, existing Markdown sections, typed relationships, and
   presentation over new persistent machinery. Add schema or commands only
   when an observed operator scenario cannot be made clear with those facts.
5. Replanning is ordinary Manager work. It does not require an approval state,
   authenticated planning actor, graph digest, or provenance ceremony.
6. Strategy revision and implementation replanning are different. Changes to
   outcome, target system, governing tradeoffs, boundaries, assurance, or
   adaptation authority return to a commissioned strategy session. Changes to
   issues, dependencies, sequencing, proof, or expansion within those bounds
   do not.
7. Proof planning is part of epic expansion. Draft work carries only the
   mission-level assurance and known risks that are currently meaningful;
   exact epic and issue proof is authored when the intended increment is clear.
8. Atelier owns tracker mechanics and repository-specific command guidance.
   Agent Factory owns portable orchestration behavior and routes to Atelier
   rather than duplicating its command manual.

# Boundaries

- Do not introduce first-class plan records, a `.atelier/plans/` lifecycle, or
  an always-current desired-state manifest.
- Do not require whole-mission task completeness before execution.
- Do not add planning approval, reviewer identity, actor authorization, graph
  freezing, exact graph digests, or stale-plan invalidation.
- Do not add an `expanded` flag, a validation profile, outcome anchors, work
  dispositions, or a replanning record. Draft/ready workflow state, authored
  outcomes, relationships, evidence, Git history, and optional mission notes
  carry the required truth.
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

- The exact representation used to reference strategy and present draft epic
  uncertainty, provided the durable source remains direct and inspectable.
- CLI wording and the placement of strategy, horizon, and expansion information
  in normal mission views.
- The current issue decomposition, dependency graph, sequencing, proof
  expectations, and ready-work buffer.
- Whether a demonstrated gap needs documentation, presentation, workflow, or
  persistence changes.
- The order and size of outcome-bearing increments after the current horizon
  supplies evidence.

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

1. Create a mission that references a governing strategy and advances both
   product-facing and pure-engineering outcome epics.
2. Inspect the strategy revision, draft outcome map, current ready horizon, and
   blockers through normal product surfaces without treating draft epics as
   executable work.
3. Keep a draft epic structurally valid without requiring child work or exact
   proof, then use predecessor evidence to refine its outcome, preservation
   claims, immediate issues, dependencies, and validation route before making
   it ready.
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
- Revision 2 — Defined strategy as governing intent rather than a backlog;
  defined one outcome-bearing epic type for product and engineering work;
  introduced draft-to-ready expansion with just-in-time proof planning; and
  rejected outcome anchors, validation profiles, work dispositions, and
  persistent replanning records.
