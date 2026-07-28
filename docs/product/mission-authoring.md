# Mission And Work Authoring

Atelier is a pleasant interface over repository planning files. It helps agents
find work, see relationships, and preserve useful proof. It is not an approval
system for plans.

## Keep The Model Small

- A mission states the desired outcome, boundaries, and major workstreams.
- An epic groups a coherent integration or review boundary.
- An issue owns one accountable slice of work.
- Evidence records what was actually checked.

Use `advances` to connect a mission to root work, hierarchy to group issue work
under epics, and blockers only for real execution prerequisites.

## Plan Only What Is Useful

Describe the whole mission as outcomes, but create detailed issues only as far
as current evidence supports. Later work may remain an outcome-oriented epic
with known constraints, unknowns, and a condition for expanding it.

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

## Proof

Planning text describes what should become true. Workers choose focused proof
while doing the work. Independent validation is useful for risky behavior and
parent completion claims, but it is ordinary accountable work rather than
permission to edit or execute a plan.

## Anti-Red-Tape Test

Before adding planning policy, ask:

- Does this help an agent understand the outcome or next action?
- Does it expose a real blocker or dependency?
- Does it preserve information another agent will need?
- Does it prevent a failure we have actually observed?

If not, leave it out.
