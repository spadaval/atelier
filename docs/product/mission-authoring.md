# Mission And Work Authoring

Atelier is a pleasant interface over repository planning files. It helps agents
find work, see relationships, and preserve useful proof. It is not an approval
system for plans.

## Keep The Model Small

- A mission states the desired outcome, boundaries, and major workstreams.
- An epic states a coherent outcome that owns one integration and review
  boundary. Product-facing and pure-engineering outcomes use the same epic
  shape.
- An issue owns one accountable slice of work.
- Evidence records what was actually checked.

Use `advances` to connect a mission to root work, hierarchy to group issue work
under epics, and blockers only for real execution prerequisites.

## Plan Only What Is Useful

Describe the whole mission as outcomes, but create detailed issues only as far
as current evidence supports. A later epic may remain in `draft` when its
outcome is meaningful but its implementation and proof route are not yet clear.
If even the outcome or integration boundary is speculative, do not create the
epic yet.

Expanding a draft epic refines its outcome and preservation claims, creates the
immediate child work, represents known dependencies, and selects the proof
route together. Transition it to `ready` only after that current horizon is
understandable. There is no separate expansion record or approval.

Changing the graph during execution is normal. Add, split, reorder, block,
defer, or supersede work when evidence changes the implementation route.
Preserve completed work and evidence; do not rewrite history to make the new
route appear inevitable.

## Ready Means Understandable

A draft mission is ready when:

- its Outcome describes an observable finished state;
- immediate work is understandable without private chat history;
- known blockers are represented;
- major unresolved product or architecture decisions are called out; and
- the next useful increment can begin safely.

Use:

```text
atelier issue transition <mission-id> ready
atelier issue transition <mission-id> start
```

No separate mission-plan approval, reviewer identity, exact graph digest, or
provenance ceremony is required.

A draft epic is ready when:

- its Outcome describes one result that can be reviewed and validated as a
  coherent branch;
- preservation claims are explicit when no external behavior should change;
- immediate child work is understandable without private context;
- dependencies and blockers are represented; and
- the Manager has selected an appropriate proof route, including independent
  validation when the claim or risk requires it.

Use:

```text
atelier issue transition <epic-id> ready
atelier issue transition <epic-id> start
```

Draft epics remain valid planning records but are not ready work.

## Proof

Strategy states mission-level assurance. During epic expansion, the Manager
chooses the proof route appropriate to the now-understood increment. Workers
still choose focused commands while doing ordinary work unless an exact check
is part of the contract. Independent validation is useful for risky behavior
and parent completion claims, but it is ordinary accountable work rather than
permission to edit or execute a plan.

## Anti-Red-Tape Test

Before adding planning policy, ask:

- Does this help an agent understand the outcome or next action?
- Does it expose a real blocker or dependency?
- Does it preserve information another agent will need?
- Does it prevent a failure we have actually observed?

If not, leave it out.
